//! Straight-line readers and writers for ABI-encoded values.
//!
//! `abigen` emits calls to these for every parameter it decodes, and for the
//! inputs it encodes for an `eth_call`. A reader takes the offset its value sits
//! at. A writer works against one buffer holding the whole payload: the caller
//! reserves a head section, fixed values are written into their slots, and a
//! dynamic value appends its tail and fills its offset word in afterwards.
//!
//! The readers deliberately reproduce `ethabi`'s accept/reject behaviour rather
//! than the stricter reading of the ABI spec. `ethabi` discards the high-order
//! padding of an `address` instead of requiring it to be zero, and accepts a
//! buffer longer than the parameters need. Rejecting either would drop logs that
//! decode today.

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
mod write_tests {
    use super::*;

    /// The hex `cast abi-encode` prints for the same value, without its `0x`.
    fn encoded(of: impl FnOnce(&mut Vec<u8>)) -> String {
        let mut out = Vec::new();
        of(&mut out);
        out.iter().map(|byte| format!("{:02x}", byte)).collect()
    }

    fn int(value: &str) -> substreams::scalar::BigInt {
        value.parse().unwrap()
    }

    #[test]
    fn it_writes_a_negative_int_sign_extended_across_the_word() {
        assert_eq!(
            encoded(|out| write_int(out, &int("-1"))),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
        assert_eq!(
            encoded(|out| write_int(out, &int("-12345"))),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffcfc7"
        );
    }

    #[test]
    fn it_sign_extends_a_narrow_int_across_the_whole_word() {
        // An `int128` fills the word rather than its declared width.
        assert_eq!(
            encoded(|out| write_int(out, &int("-1000000"))),
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0bdc0"
        );
    }

    #[test]
    fn it_writes_a_positive_int_zero_padded() {
        assert_eq!(
            encoded(|out| write_int(out, &int("5"))),
            "0000000000000000000000000000000000000000000000000000000000000005"
        );
    }

    #[test]
    fn it_writes_the_largest_uint() {
        assert_eq!(
            encoded(|out| {
                write_uint(
                out,
                &int("115792089237316195423570985008687907853269984665640564039457584007913129639935")
            )
            }),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
    }

    #[test]
    fn it_writes_zero_as_an_empty_word() {
        assert_eq!(
            encoded(|out| write_uint(out, &int("0"))),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            encoded(|out| write_int(out, &int("0"))),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    #[should_panic(expected = "negative numbers are not supported")]
    fn it_panics_on_a_negative_uint() {
        let mut out = Vec::new();
        write_uint(&mut out, &int("-1"));
    }

    #[test]
    fn it_writes_an_address_right_aligned() {
        let address = [
            0xff, 0xfd, 0xb7, 0x37, 0x73, 0x45, 0x37, 0x18, 0x17, 0xf2, 0xb4, 0xdd, 0x49, 0x03,
            0x19, 0x75, 0x5f, 0x58, 0x99, 0xec,
        ];

        assert_eq!(
            encoded(|out| write_address(out, &address)),
            "000000000000000000000000fffdb7377345371817f2b4dd490319755f5899ec"
        );
    }

    #[test]
    fn it_writes_a_bool_in_the_low_byte() {
        assert_eq!(
            encoded(|out| write_bool(out, &true)),
            "0000000000000000000000000000000000000000000000000000000000000001"
        );
        assert_eq!(
            encoded(|out| write_bool(out, &false)),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn it_writes_fixed_bytes_padded_on_the_right() {
        assert_eq!(
            encoded(|out| write_fixed_bytes(out, &[0xde, 0xad, 0xbe, 0xef])),
            "deadbeef00000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn it_writes_a_full_word_of_fixed_bytes_without_padding() {
        assert_eq!(
            encoded(|out| write_fixed_bytes(out, &[0x11; 32])),
            "11".repeat(32)
        );
    }

    #[test]
    fn it_writes_a_bytes_tail_as_a_length_then_padded_content() {
        assert_eq!(
            encoded(|out| write_bytes_tail(out, &[0xab, 0xde, 0xff, 0x90])),
            "0000000000000000000000000000000000000000000000000000000000000004\
             abdeff9000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn it_pads_a_bytes_tail_that_spills_into_a_second_word() {
        let value: Vec<u8> = (1u8..=33).collect();

        assert_eq!(
            encoded(|out| write_bytes_tail(out, &value)),
            "0000000000000000000000000000000000000000000000000000000000000021\
             0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20\
             2100000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn it_writes_an_empty_bytes_tail_as_a_bare_length() {
        assert_eq!(
            encoded(|out| write_bytes_tail(out, &[])),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn it_writes_a_head_offset() {
        assert_eq!(
            encoded(|out| write_offset(out, 32)),
            "0000000000000000000000000000000000000000000000000000000000000020"
        );
    }
}

/// Appends a 32-byte word holding `value` in its low bytes, zero-padded on the
/// left.
#[inline]
pub fn write_padded(out: &mut Vec<u8>, value: &[u8]) {
    out.extend(core::iter::repeat(0u8).take(32 - value.len()));
    out.extend_from_slice(value);
}

/// Writes an `address` as its low 20 bytes, right-aligned in the word.
///
/// An input longer than 20 bytes keeps its trailing 20, which is what
/// `ethabi::Address::from_slice` would have panicked on; a shorter one is
/// left-padded.
#[inline]
pub fn write_address(out: &mut Vec<u8>, value: &[u8]) {
    let from = value.len().saturating_sub(20);
    write_padded(out, &value[from..]);
}

/// Writes a `uintN` as a big-endian word.
///
/// A negative value has no `uint` encoding, and `encode` returns bytes rather
/// than a result, so there is nowhere to report one but a panic.
#[inline]
pub fn write_uint(out: &mut Vec<u8>, value: &substreams::scalar::BigInt) {
    let (sign, bytes) = value.to_bytes_be();
    if sign == num_bigint::Sign::Minus {
        panic!("negative numbers are not supported");
    }

    write_word_of(out, &bytes, 0x00);
}

/// Writes an `intN` as a big-endian two's-complement word.
///
/// `to_signed_bytes_be` returns the minimal width that carries the value, so a
/// negative one is sign-extended with `0xff` to fill the word rather than
/// zero-padded.
#[inline]
pub fn write_int(out: &mut Vec<u8>, value: &substreams::scalar::BigInt) {
    let bytes = value.to_signed_bytes_be();
    let fill = if bytes.first().is_some_and(|byte| byte & 0x80 == 0x80) {
        0xff
    } else {
        0x00
    };

    write_word_of(out, &bytes, fill);
}

/// Appends a word holding the low 32 bytes of `bytes`, padded on the left with
/// `fill`.
///
/// A value wider than a word keeps its low 32 bytes, matching what the integer
/// types do when they overflow their declared width.
#[inline]
fn write_word_of(out: &mut Vec<u8>, bytes: &[u8], fill: u8) {
    let from = bytes.len().saturating_sub(32);
    let value = &bytes[from..];

    out.extend(core::iter::repeat(fill).take(32 - value.len()));
    out.extend_from_slice(value);
}

/// Writes a `bool` as a word whose low byte is 0 or 1.
#[inline]
pub fn write_bool(out: &mut Vec<u8>, value: &bool) {
    out.extend(core::iter::repeat(0u8).take(31));
    out.push(*value as u8);
}

/// Writes `bytesN` as the leading `N` bytes of a word, padded on the right.
#[inline]
pub fn write_fixed_bytes(out: &mut Vec<u8>, value: &[u8]) {
    out.extend_from_slice(value);

    let padding = (32 - value.len() % 32) % 32;
    out.extend(core::iter::repeat(0u8).take(padding));
}

/// Writes a head word holding `offset`, the distance from the start of the
/// enclosing head section to the value's tail.
#[inline]
pub fn write_offset(out: &mut Vec<u8>, offset: usize) {
    write_padded(out, &(offset as u64).to_be_bytes());
}

/// Writes the tail of a `bytes` or `string`: a length word, then the bytes
/// padded out to a whole number of words.
#[inline]
pub fn write_bytes_tail(out: &mut Vec<u8>, value: &[u8]) {
    write_offset(out, value.len());
    out.extend_from_slice(value);

    let padding = (32 - value.len() % 32) % 32;
    out.extend(core::iter::repeat(0u8).take(padding));
}

/// Writes a word into the slot at `at`, taking the low 32 bytes of `bytes` and
/// padding the rest with `fill`.
///
/// The slot was reserved by `reserve_head`, so its bytes already belong to the
/// buffer and are overwritten in place. This is what lets a parameter list hold
/// its fixed values while dynamic tails are appended past the head.
#[inline]
fn write_word_at(out: &mut [u8], at: usize, bytes: &[u8], fill: u8) {
    let from = bytes.len().saturating_sub(32);
    let value = &bytes[from..];
    let pad = 32 - value.len();

    out[at..at + pad].fill(fill);
    out[at + pad..at + 32].copy_from_slice(value);
}

/// Writes an `address` into the slot at `at`, right-aligned in the word.
#[inline]
pub fn write_address_at(out: &mut [u8], at: usize, value: &[u8]) {
    let from = value.len().saturating_sub(20);
    write_word_at(out, at, &value[from..], 0x00);
}

/// Writes a `uintN` into the slot at `at`.
#[inline]
pub fn write_uint_at(out: &mut [u8], at: usize, value: &substreams::scalar::BigInt) {
    let (sign, bytes) = value.to_bytes_be();
    if sign == num_bigint::Sign::Minus {
        panic!("negative numbers are not supported");
    }

    write_word_at(out, at, &bytes, 0x00);
}

/// Writes an `intN` into the slot at `at`, sign-extended across the word.
#[inline]
pub fn write_int_at(out: &mut [u8], at: usize, value: &substreams::scalar::BigInt) {
    let bytes = value.to_signed_bytes_be();
    let fill = if bytes.first().is_some_and(|byte| byte & 0x80 == 0x80) {
        0xff
    } else {
        0x00
    };

    write_word_at(out, at, &bytes, fill);
}

/// Writes a `bool` into the slot at `at`, in the low byte of the word.
#[inline]
pub fn write_bool_at(out: &mut [u8], at: usize, value: &bool) {
    out[at..at + 31].fill(0);
    out[at + 31] = *value as u8;
}

/// Writes `bytesN` into the slot at `at`, the leading `N` bytes of the word.
#[inline]
pub fn write_fixed_bytes_at(out: &mut [u8], at: usize, value: &[u8]) {
    let len = value.len().min(32);

    out[at..at + len].copy_from_slice(&value[..len]);
    out[at + len..at + 32].fill(0);
}

/// Opens a head section of `width` bytes, returning where it starts.
///
/// The section is zeroed so each parameter can be written into its own slot in
/// any order, which is what lets a dynamic value append its tail to the same
/// buffer and fill its offset in afterwards.
#[inline]
pub fn reserve_head(out: &mut Vec<u8>, width: usize) -> usize {
    let base = out.len();
    out.resize(base + width, 0);
    base
}

/// Fills in the offset word of a head slot at `at`.
///
/// The slot was reserved by `reserve_head`, so the bytes it spans are already
/// part of the buffer and are overwritten rather than appended. The offset is
/// passed in rather than read from the buffer's length, so the call does not
/// have to sit at a particular point relative to the tail it points at.
#[inline]
pub fn backfill_offset(out: &mut Vec<u8>, at: usize, offset: usize) {
    out[at + 24..at + 32].copy_from_slice(&(offset as u64).to_be_bytes());
}

/// Where a value's tail begins, measured from the start of the head section it
/// is pointed at from.
#[inline]
pub fn tail_offset(out: &[u8], base: usize) -> usize {
    out.len() - base
}

#[cfg(test)]
mod single_buffer_tests {
    use super::*;

    /// The hex of a slot written into a freshly reserved head.
    fn slot_of(write: impl FnOnce(&mut [u8])) -> String {
        let mut out = Vec::new();
        reserve_head(&mut out, 32);
        write(&mut out);
        out.iter().map(|byte| format!("{:02x}", byte)).collect()
    }

    fn int(value: &str) -> substreams::scalar::BigInt {
        value.parse().unwrap()
    }

    #[test]
    fn it_writes_a_uint_into_its_slot() {
        assert_eq!(
            slot_of(|out| write_uint_at(out, 0, &int("42"))),
            "000000000000000000000000000000000000000000000000000000000000002a"
        );
    }

    #[test]
    fn it_writes_a_negative_int_into_its_slot_sign_extended() {
        assert_eq!(
            slot_of(|out| write_int_at(out, 0, &int("-12345"))),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffcfc7"
        );
    }

    #[test]
    fn it_writes_an_address_into_its_slot_right_aligned() {
        let address = [
            0xff, 0xfd, 0xb7, 0x37, 0x73, 0x45, 0x37, 0x18, 0x17, 0xf2, 0xb4, 0xdd, 0x49, 0x03,
            0x19, 0x75, 0x5f, 0x58, 0x99, 0xec,
        ];

        assert_eq!(
            slot_of(|out| write_address_at(out, 0, &address)),
            "000000000000000000000000fffdb7377345371817f2b4dd490319755f5899ec"
        );
    }

    #[test]
    fn it_writes_a_bool_into_its_slot() {
        assert_eq!(
            slot_of(|out| write_bool_at(out, 0, &true)),
            "0000000000000000000000000000000000000000000000000000000000000001"
        );
    }

    #[test]
    fn it_writes_fixed_bytes_into_its_slot_padded_right() {
        assert_eq!(
            slot_of(|out| write_fixed_bytes_at(out, 0, &[0xde, 0xad, 0xbe, 0xef])),
            "deadbeef00000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn it_overwrites_a_slot_that_already_held_bytes() {
        let mut out = vec![0xff; 64];
        write_uint_at(&mut out, 32, &int("1"));

        assert_eq!(&out[..32], &[0xff; 32]);
        assert_eq!(
            out[32..]
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>(),
            "0000000000000000000000000000000000000000000000000000000000000001"
        );
    }

    #[test]
    fn it_writes_each_slot_of_a_wider_head_independently() {
        let mut out = Vec::new();
        reserve_head(&mut out, 64);

        write_uint_at(&mut out, 0, &int("1"));
        write_uint_at(&mut out, 32, &int("2"));

        assert_eq!(&out[24..32], &1u64.to_be_bytes());
        assert_eq!(&out[56..64], &2u64.to_be_bytes());
    }

    #[test]
    fn it_reserves_a_zeroed_head_section() {
        let mut out = Vec::new();
        let base = reserve_head(&mut out, 64);

        assert_eq!(base, 0);
        assert_eq!(out, vec![0u8; 64]);
    }

    #[test]
    fn it_reserves_past_content_already_written() {
        let mut out = vec![0xff; 32];
        let base = reserve_head(&mut out, 32);

        assert_eq!(base, 32);
        assert_eq!(out.len(), 64);
        assert_eq!(&out[..32], &[0xff; 32]);
        assert_eq!(&out[32..], &[0u8; 32]);
    }

    #[test]
    fn it_backfills_an_offset_measured_from_the_head_start() {
        let mut out = Vec::new();
        let base = reserve_head(&mut out, 32);

        let at = tail_offset(&out, base);
        write_bytes_tail(&mut out, b"hi");
        backfill_offset(&mut out, base, at);

        // The tail begins one word past the head, so the slot holds 32.
        assert_eq!(&out[24..32], &32u64.to_be_bytes());
    }

    #[test]
    fn it_backfills_each_slot_of_a_wider_head() {
        let mut out = Vec::new();
        let base = reserve_head(&mut out, 64);

        let first = tail_offset(&out, base);
        write_bytes_tail(&mut out, b"first");
        backfill_offset(&mut out, base, first);

        let second = tail_offset(&out, base);
        write_bytes_tail(&mut out, b"second");
        backfill_offset(&mut out, base + 32, second);

        assert_eq!(&out[24..32], &64u64.to_be_bytes());
        assert_eq!(&out[56..64], &128u64.to_be_bytes());
    }

    #[test]
    fn it_measures_a_nested_offset_from_its_own_base() {
        let mut out = vec![0xaa; 96];
        let base = reserve_head(&mut out, 32);

        let at = tail_offset(&out, base);
        write_bytes_tail(&mut out, b"x");
        backfill_offset(&mut out, base, at);

        // The content before the section does not shift the offset.
        assert_eq!(&out[96 + 24..96 + 32], &32u64.to_be_bytes());
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
