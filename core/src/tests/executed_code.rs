use crate::pb::eth::v2::{BoolOptional, BoolRequired, Call};
use prost::Message;

// BoolOptional: true / false / nil
#[test]
fn bool_optional_true() {
    let msg = BoolOptional { state: Some(true) };
    let got = BoolOptional::decode(msg.encode_to_vec().as_slice()).unwrap();
    assert_eq!(got.state, Some(true));
}

#[test]
fn bool_optional_false() {
    let msg = BoolOptional { state: Some(false) };
    let got = BoolOptional::decode(msg.encode_to_vec().as_slice()).unwrap();
    assert_eq!(got.state, Some(false));
}

#[test]
fn bool_optional_nil() {
    let msg = BoolOptional { state: None };
    let got = BoolOptional::decode(msg.encode_to_vec().as_slice()).unwrap();
    assert_eq!(got.state, None);
}

// BoolRequired: true / false
#[test]
fn bool_required_true() {
    let msg = BoolRequired { state: true };
    let got = BoolRequired::decode(msg.encode_to_vec().as_slice()).unwrap();
    assert_eq!(got.state, true);
}

#[test]
fn bool_required_false() {
    let msg = BoolRequired { state: false };
    let got = BoolRequired::decode(msg.encode_to_vec().as_slice()).unwrap();
    assert_eq!(got.state, false);
}

// Call.executed_code: true / false / nil
#[test]
fn call_executed_code_true() {
    let call = Call { executed_code: Some(true), ..Default::default() };
    let got = Call::decode(call.encode_to_vec().as_slice()).unwrap();
    assert_eq!(got.executed_code, Some(true));
}

#[test]
fn call_executed_code_false() {
    let call = Call { executed_code: Some(false), ..Default::default() };
    let got = Call::decode(call.encode_to_vec().as_slice()).unwrap();
    assert_eq!(got.executed_code, Some(false));
}

#[test]
fn call_executed_code_nil() {
    let call = Call { executed_code: None, ..Default::default() };
    let got = Call::decode(call.encode_to_vec().as_slice()).unwrap();
    assert_eq!(got.executed_code, None);
}

// nil and false must encode differently on the wire
#[test]
fn bool_optional_nil_vs_false_are_distinct() {
    let nil_data = BoolOptional { state: None }.encode_to_vec();
    let false_data = BoolOptional { state: Some(false) }.encode_to_vec();

    assert!(nil_data.is_empty(), "optional nil must encode to empty bytes");
    assert!(!false_data.is_empty(), "optional false must encode to non-empty bytes");

    let got_nil = BoolOptional::decode(nil_data.as_slice()).unwrap();
    let got_false = BoolOptional::decode(false_data.as_slice()).unwrap();
    assert_eq!(got_nil.state, None);
    assert_eq!(got_false.state, Some(false));
}

// Backward compatibility: old producer (BoolRequired) -> new consumer (BoolOptional)
// old true  -> new Some(true)
// old false -> new None (false is absent on the wire, indistinguishable from not set)
#[test]
fn bool_optional_backward_compatible() {
    let true_data = BoolRequired { state: true }.encode_to_vec();
    let false_data = BoolRequired { state: false }.encode_to_vec();

    let got_true = BoolOptional::decode(true_data.as_slice()).unwrap();
    let got_false = BoolOptional::decode(false_data.as_slice()).unwrap();

    assert_eq!(got_true.state, Some(true), "old true readable as optional true");
    assert_eq!(got_false.state, None, "old false is absent on the wire, reads as nil");
}

// Forward compatibility: new producer (BoolOptional) -> old consumer (BoolRequired)
// new true  -> old true
// new false -> old false
// new nil   -> old false (absent field decodes to zero value)
#[test]
fn bool_optional_forward_compatible() {
    let true_data = BoolOptional { state: Some(true) }.encode_to_vec();
    let false_data = BoolOptional { state: Some(false) }.encode_to_vec();
    let nil_data = BoolOptional { state: None }.encode_to_vec();

    let got_true = BoolRequired::decode(true_data.as_slice()).unwrap();
    let got_false = BoolRequired::decode(false_data.as_slice()).unwrap();
    let got_nil = BoolRequired::decode(nil_data.as_slice()).unwrap();

    assert_eq!(got_true.state, true);
    assert_eq!(got_false.state, false);
    assert_eq!(got_nil.state, false, "new nil decodes to required false (zero value)");
}
