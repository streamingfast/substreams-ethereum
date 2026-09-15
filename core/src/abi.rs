//! Straight-line readers for ABI-encoded values at known offsets.
//!
//! `abigen` emits calls to these for events whose parameters are all fixed-size,
//! where the byte layout is known when the ABI is read. Dynamic parameters still
//! go through `ethabi`.
//!
//! These deliberately reproduce `ethabi`'s accept/reject behaviour rather than the
//! stricter reading of the ABI spec. `ethabi` discards the high-order padding of an
//! `address` instead of requiring it to be zero, and accepts a buffer longer than
//! the parameters need. Rejecting either would drop logs that decode today.

/// Reads the 32-byte word at `offset`.
///
/// An offset read out of the data can be as large as `u32::MAX`, which is wider
/// than a `usize` on `wasm32`, so the end of the word is computed with a checked
/// add rather than one that would wrap into a valid index.
#[inline]
fn word<'a>(data: &'a [u8], offset: usize, name: &str) -> Result<&'a [u8], String> {
    offset
        .checked_add(32)
        .and_then(|end| data.get(offset..end))
        .ok_or_else(|| {
            format!(
                "unable to decode param '{}': need 32 bytes at offset {}, buffer is {} bytes",
                name,
                offset,
                data.len()
            )
        })
}

/// Reads an `address`, the low 20 bytes of the word.
#[inline]
pub fn read_address(data: &[u8], offset: usize, name: &str) -> Result<Vec<u8>, String> {
    Ok(word(data, offset, name)?[12..32].to_vec())
}

/// Reads a `uintN` as a big-endian unsigned integer over the full word.
#[inline]
pub fn read_uint(
    data: &[u8],
    offset: usize,
    name: &str,
) -> Result<substreams::scalar::BigInt, String> {
    Ok(substreams::scalar::BigInt::from_unsigned_bytes_be(word(
        data, offset, name,
    )?))
}

/// Reads an `intN` as a big-endian two's-complement integer over the full word.
///
/// The sign comes from the top bit of the word, so a value narrower than 256 bits
/// is read correctly only when the encoder sign-extended it.
#[inline]
pub fn read_int(
    data: &[u8],
    offset: usize,
    name: &str,
) -> Result<substreams::scalar::BigInt, String> {
    Ok(substreams::scalar::BigInt::from_signed_bytes_be(word(
        data, offset, name,
    )?))
}

/// Reads a `bool` from the low byte. Any non-zero low byte other than 1 reads as
/// `false`, matching `ethabi`.
#[inline]
pub fn read_bool(data: &[u8], offset: usize, name: &str) -> Result<bool, String> {
    let word = word(data, offset, name)?;
    if word[..31].iter().any(|byte| *byte != 0) {
        return Err(format!(
            "unable to decode param '{}': a bool word carries a non-zero byte above its low byte",
            name
        ));
    }

    Ok(word[31] == 1)
}

/// Reads `bytesN`, the leading `N` bytes of the word, into a fixed array.
#[inline]
pub fn read_fixed_bytes<const N: usize>(
    data: &[u8],
    offset: usize,
    name: &str,
) -> Result<[u8; N], String> {
    // `bytesN` occupies a whole word but only spans its leading `N` bytes, so a
    // buffer holding those bytes is enough even when it stops short of the word.
    // A declared `N` above 32 has no valid encoding and is reported rather than
    // read past the word, which would trap on `wasm32`.
    if N > 32 {
        return Err(format!(
            "unable to decode param '{}': bytes{} is wider than a word",
            name, N
        ));
    }

    let mut out = [0u8; N];
    out.copy_from_slice(take(data, offset, N, name)?);
    Ok(out)
}

/// Reads a head word as an offset into the data section.
///
/// Only the low four bytes carry the value, so an offset above `u32::MAX` is
/// rejected rather than truncated, and `offset + 32` cannot overflow a `usize`.
#[inline]
pub fn read_offset(data: &[u8], offset: usize, name: &str) -> Result<usize, String> {
    let word = word(data, offset, name)?;
    if word[..28].iter().any(|byte| *byte != 0) {
        return Err(format!(
            "unable to decode param '{}': an offset above u32::MAX",
            name
        ));
    }

    Ok(u32::from_be_bytes([word[28], word[29], word[30], word[31]]) as usize)
}

/// Takes `len` bytes at `offset`, rejecting a buffer that stops short.
#[inline]
fn take<'a>(data: &'a [u8], offset: usize, len: usize, name: &str) -> Result<&'a [u8], String> {
    data.get(offset..)
        .and_then(|rest| rest.get(..len))
        .ok_or_else(|| {
            format!(
                "unable to decode param '{}': need {} bytes at offset {}, buffer is {} bytes",
                name,
                len,
                offset,
                data.len()
            )
        })
}

/// Reads `bytes`: a head offset, then a length, then that many bytes.
///
/// The bytes are copied out rather than borrowed so the decoded event owns them
/// and carries no lifetime.
pub fn read_bytes(data: &[u8], offset: usize, name: &str) -> Result<Vec<u8>, String> {
    let at = read_offset(data, offset, name)?;
    let len = read_offset(data, at, name)?;

    let from = at.checked_add(32).ok_or_else(|| {
        format!(
            "unable to decode param '{}': a length word at the end of the address space",
            name
        )
    })?;

    Ok(take(data, from, len, name)?.to_vec())
}

/// Reads a `string`, decoding its bytes lossily.
///
/// A contract can write bytes that are not valid UTF-8, and rejecting those
/// would drop the whole log rather than the one field.
pub fn read_string(data: &[u8], offset: usize, name: &str) -> Result<String, String> {
    Ok(String::from_utf8_lossy(&read_bytes(data, offset, name)?).into_owned())
}

/// Resolves a `T[]` to its element count and the slice its elements are read
/// from, with element `i` at offset `i * 32` of that slice.
///
/// Reading the length word is what proves the tail's own start is inside the
/// buffer, so the returned slice is always in range.
pub fn read_array_tail<'a>(
    data: &'a [u8],
    offset: usize,
    name: &str,
) -> Result<(&'a [u8], usize), String> {
    let len_offset = read_offset(data, offset, name)?;
    let len = read_offset(data, len_offset, name)?;

    let tail = len_offset
        .checked_add(32)
        .and_then(|start| data.get(start..))
        .ok_or_else(|| {
            format!(
                "unable to decode param '{}': an array tail past the buffer",
                name
            )
        })?;

    Ok((tail, len))
}

/// Resolves a dynamic `T[N]` or tuple to the slice its fields are read from.
pub fn read_dynamic_tail<'a>(
    data: &'a [u8],
    offset: usize,
    name: &str,
) -> Result<&'a [u8], String> {
    let at = read_offset(data, offset, name)?;

    data.get(at..)
        .ok_or_else(|| format!("unable to decode param '{}': a tail past the buffer", name))
}

#[cfg(test)]
mod dynamic_tests {
    use super::*;

    /// A 32-byte word holding `value` in its low eight bytes.
    fn word_of(value: u64) -> Vec<u8> {
        let mut out = vec![0u8; 32];
        out[24..32].copy_from_slice(&value.to_be_bytes());
        out
    }

    fn concat(words: &[Vec<u8>]) -> Vec<u8> {
        words.iter().flat_map(|word| word.iter().copied()).collect()
    }

    #[test]
    fn it_reads_bytes_through_its_offset_and_length() {
        // head -> 32, len 4, then the bytes padded out to a word.
        let mut data = concat(&[word_of(32), word_of(4)]);
        data.extend_from_slice(&[0xab, 0xde, 0xff, 0x90]);
        data.extend_from_slice(&[0u8; 28]);

        assert_eq!(read_bytes(&data, 0, "p"), Ok(vec![0xab, 0xde, 0xff, 0x90]));
    }

    #[test]
    fn it_reads_a_string() {
        let mut data = concat(&[word_of(32), word_of(13)]);
        data.extend_from_slice(b"second string");
        data.extend_from_slice(&[0u8; 19]);

        assert_eq!(read_string(&data, 0, "p"), Ok("second string".to_string()));
    }

    #[test]
    fn it_reads_a_string_whose_bytes_are_not_utf8() {
        // `ethabi` decodes lossily here rather than failing the whole log.
        let mut data = concat(&[word_of(32), word_of(4)]);
        data.extend_from_slice(&[0xe4, 0xb8, 0x8d, 0xe5]);
        data.extend_from_slice(&[0u8; 28]);

        assert_eq!(read_string(&data, 0, "p"), Ok("不\u{fffd}".to_string()));
    }

    #[test]
    fn it_reads_an_empty_byte_string() {
        let data = concat(&[word_of(32), word_of(0)]);

        assert_eq!(read_bytes(&data, 0, "p"), Ok(vec![]));
    }

    #[test]
    fn it_reads_bytes_that_do_not_fill_a_whole_word() {
        let mut data = concat(&[word_of(32), word_of(5)]);
        data.extend_from_slice(&[0xaa; 5]);
        data.extend_from_slice(&[0u8; 27]);

        assert_eq!(read_bytes(&data, 0, "p"), Ok(vec![0xaa; 5]));
    }

    #[test]
    fn it_reports_a_length_reaching_past_the_buffer() {
        // The length claims far more than the buffer holds.
        let data = concat(&[word_of(32), word_of(0xffff_ffff)]);

        assert!(read_bytes(&data, 0, "p").is_err());
    }

    #[test]
    fn it_reports_an_offset_past_the_buffer() {
        let data = concat(&[word_of(224)]);

        assert!(read_bytes(&data, 0, "p").is_err());
    }

    #[test]
    fn it_reports_an_offset_above_u32_max() {
        let mut data = vec![0u8; 32];
        data[20..28].copy_from_slice(&1u64.to_be_bytes());

        let err = read_offset(&data, 0, "p").unwrap_err();
        assert!(err.contains("u32::MAX"), "{err}");
    }

    #[test]
    fn it_reads_an_offset_at_u32_max_without_overflowing() {
        let mut data = vec![0u8; 32];
        data[28..32].copy_from_slice(&u32::MAX.to_be_bytes());

        // The value is readable; the read that follows it is what fails.
        assert_eq!(read_offset(&data, 0, "p"), Ok(u32::MAX as usize));
        assert!(read_bytes(&data, 0, "p").is_err());
    }

    #[test]
    fn it_resolves_an_array_tail_and_length() {
        let data = concat(&[word_of(32), word_of(2), word_of(7), word_of(8)]);

        let (tail, len) = read_array_tail(&data, 0, "p").expect("resolves");
        assert_eq!(len, 2);
        assert_eq!(read_uint(tail, 0, "e").unwrap().to_string(), "7");
        assert_eq!(read_uint(tail, 32, "e").unwrap().to_string(), "8");
    }

    #[test]
    fn it_resolves_an_empty_array() {
        let data = concat(&[word_of(32), word_of(0)]);

        let (_, len) = read_array_tail(&data, 0, "p").expect("resolves");
        assert_eq!(len, 0);
    }

    #[test]
    fn it_reports_an_array_length_word_past_the_buffer() {
        // `ethabi` rejects this while reading the length, before it slices the
        // tail, which is why its own unchecked slice is never out of range.
        let data = concat(&[word_of(0xffff_fff0)]);

        assert!(read_array_tail(&data, 0, "p").is_err());
    }

    #[test]
    fn it_reports_an_array_whose_length_exceeds_the_tail() {
        // A length of 0xffffffff with a tail of one word. The caller reads
        // element by element, so the failure surfaces on the first read past
        // the end rather than here.
        let data = concat(&[word_of(32), word_of(0xffff_ffff), word_of(1)]);

        let (tail, len) = read_array_tail(&data, 0, "p").expect("the tail itself is in range");
        assert_eq!(len, 0xffff_ffff);
        assert!(
            read_uint(tail, 32, "e").is_err(),
            "the second element is past the tail"
        );
    }

    #[test]
    fn it_resolves_a_dynamic_tail() {
        let data = concat(&[word_of(32), word_of(9)]);

        let tail = read_dynamic_tail(&data, 0, "p").expect("resolves");
        assert_eq!(read_uint(tail, 0, "e").unwrap().to_string(), "9");
    }

    #[test]
    fn it_reports_a_dynamic_tail_past_the_buffer() {
        let data = concat(&[word_of(0xffff_fff0)]);

        assert!(read_dynamic_tail(&data, 0, "p").is_err());
    }

    /// A 32-byte word holding `value` in its low four bytes, the widest offset
    /// `read_offset` accepts.
    fn offset_word(value: u32) -> Vec<u8> {
        let mut out = vec![0u8; 32];
        out[28..32].copy_from_slice(&value.to_be_bytes());
        out
    }

    #[test]
    fn it_rejects_an_offset_whose_word_would_end_past_the_address_space() {
        // `usize` is 32 bits on wasm32, so the end of the word at `u32::MAX`
        // wraps to a small in-range index unless the add is checked.
        for value in [u32::MAX, u32::MAX - 31, u32::MAX - 16] {
            let mut data = offset_word(value);
            data.extend_from_slice(&[0xabu8; 64]);

            assert!(
                read_bytes(&data, 0, "p").is_err(),
                "offset {value} must be rejected"
            );
            assert!(
                read_array_tail(&data, 0, "p").is_err(),
                "offset {value} must be rejected"
            );
            assert!(
                read_dynamic_tail(&data, 0, "p").is_err(),
                "offset {value} must be rejected"
            );
        }
    }

    #[test]
    fn it_rejects_a_word_read_at_an_offset_near_the_address_space_end() {
        let data = vec![0u8; 64];

        assert!(read_uint(&data, usize::MAX, "p").is_err());
        assert!(read_uint(&data, usize::MAX - 31, "p").is_err());
        assert!(read_address(&data, usize::MAX, "p").is_err());
        assert!(read_bool(&data, usize::MAX, "p").is_err());
    }

    #[test]
    fn it_reports_an_empty_buffer_for_every_dynamic_read() {
        assert!(read_bytes(&[], 0, "p").is_err());
        assert!(read_string(&[], 0, "p").is_err());
        assert!(read_array_tail(&[], 0, "p").is_err());
        assert!(read_dynamic_tail(&[], 0, "p").is_err());
        assert!(read_offset(&[], 0, "p").is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(fill: u8) -> Vec<u8> {
        vec![fill; 32]
    }

    #[test]
    fn it_reads_an_address_from_the_low_twenty_bytes() {
        let mut data = vec![0u8; 32];
        data[12..32].copy_from_slice(&[0xaa; 20]);

        assert_eq!(read_address(&data, 0, "p"), Ok(vec![0xaa; 20]));
    }

    #[test]
    fn it_discards_the_high_order_padding_of_an_address() {
        // ethabi truncates rather than rejecting, and contracts do emit these.
        let mut data = vec![0xff; 12];
        data.extend_from_slice(&[0xaa; 20]);

        assert_eq!(read_address(&data, 0, "p"), Ok(vec![0xaa; 20]));
    }

    #[test]
    fn it_reads_a_word_at_a_non_zero_offset() {
        let mut data = w(0x00);
        data.extend_from_slice(&w(0x11));

        assert_eq!(read_uint(&data, 32, "p").unwrap().to_string(), {
            let mut expected = String::new();
            let big = substreams::scalar::BigInt::from_unsigned_bytes_be(&w(0x11));
            expected.push_str(&big.to_string());
            expected
        });
    }

    #[test]
    fn it_reports_a_buffer_too_short_for_the_offset() {
        let data = vec![0u8; 31];

        let err = read_uint(&data, 0, "amount").unwrap_err();
        assert!(err.contains("amount"), "error names the param: {err}");
        assert!(err.contains("31"), "error reports the buffer length: {err}");
    }

    #[test]
    fn it_reports_a_buffer_that_stops_before_a_later_param() {
        let data = vec![0u8; 32];

        assert!(read_uint(&data, 32, "second").is_err());
    }

    #[test]
    fn it_accepts_a_buffer_longer_than_the_params_need() {
        // ethabi ignores trailing bytes; a stricter reading would drop real logs.
        let data = vec![0u8; 64];

        assert!(read_uint(&data, 0, "p").is_ok());
    }

    #[test]
    fn it_reads_a_bool_from_the_low_byte() {
        let mut t = vec![0u8; 32];
        t[31] = 1;

        assert_eq!(read_bool(&t, 0, "p"), Ok(true));
        assert_eq!(read_bool(&vec![0u8; 32], 0, "p"), Ok(false));
    }

    #[test]
    fn it_reads_a_bool_other_than_one_as_false() {
        let mut two = vec![0u8; 32];
        two[31] = 2;

        assert_eq!(read_bool(&two, 0, "p"), Ok(false));
    }

    #[test]
    fn it_reports_a_bool_word_with_dirty_padding() {
        // `ethabi` rejects these rather than reading the low byte regardless.
        let mut dirty = vec![0x11u8; 32];
        dirty[31] = 0;

        assert!(read_bool(&dirty, 0, "p").is_err());
    }

    #[test]
    fn it_reads_fixed_bytes_from_the_front_of_the_word() {
        let mut data = vec![0u8; 32];
        data[..4].copy_from_slice(&[0xde, 0xad, 0xbe, 0xef]);

        assert_eq!(
            read_fixed_bytes::<4>(&data, 0, "p"),
            Ok([0xde, 0xad, 0xbe, 0xef])
        );
        assert_eq!(
            read_fixed_bytes::<32>(&data, 0, "p"),
            Ok(*<&[u8; 32]>::try_from(&data[..]).unwrap())
        );
    }

    #[test]
    fn it_reports_short_input_for_fixed_bytes_rather_than_panicking() {
        // `copy_from_slice` would panic; the word() check has to come first.
        let data = vec![0u8; 16];

        assert!(read_fixed_bytes::<32>(&data, 0, "p").is_err());
    }

    #[test]
    fn it_reads_a_negative_int_as_twos_complement() {
        let minus_one = vec![0xff; 32];

        assert_eq!(read_int(&minus_one, 0, "p").unwrap().to_string(), "-1");
    }

    #[test]
    fn it_reads_a_positive_int_with_the_sign_bit_clear() {
        let mut data = vec![0u8; 32];
        data[31] = 5;

        assert_eq!(read_int(&data, 0, "p").unwrap().to_string(), "5");
    }

    #[test]
    fn it_reads_an_unsigned_word_with_the_top_bit_set_as_positive() {
        let max = vec![0xff; 32];

        let value = read_uint(&max, 0, "p").unwrap();
        assert_eq!(
            value.to_string(),
            "115792089237316195423570985008687907853269984665640564039457584007913129639935"
        );
    }

    #[test]
    fn it_reports_an_empty_buffer() {
        assert!(read_address(&[], 0, "p").is_err());
        assert!(read_uint(&[], 0, "p").is_err());
        assert!(read_bool(&[], 0, "p").is_err());
        assert!(read_int(&[], 0, "p").is_err());
        assert!(read_fixed_bytes::<32>(&[], 0, "p").is_err());
    }
}
