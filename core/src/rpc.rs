use crate::{externs, Function};
use crate::pb;
use substreams::memory;
use substreams::proto;
use crate::pb::eth::rpc::{RpcCall, RpcCalls, RpcResponse, RpcResponses};

pub trait RPCDecodable<R> {
    fn output(data: &[u8]) -> Result<R, String>;
}

pub struct RpcBatch {
    store: RpcCalls,
}

pub fn batch() -> RpcBatch {
    RpcBatch {
        store: RpcCalls {
            ..Default::default()
        },
    }
}

impl RpcBatch {
    pub fn new() -> RpcBatch {
        RpcBatch {
            store: RpcCalls { calls: vec![] },
        }
    }

    pub fn add<F: Function>(mut self, call: F, address: Vec<u8>) -> Self {
        self.store.calls.push(RpcCall {
            to_addr: address,
            data: call.encode(),
        });
        self
    }

    pub fn execute(self) -> Result<RpcResponses, String> {
        Ok(eth_call(&self.store))
    }

    pub fn decode<R, T: RPCDecodable<R> + Function>(response: &RpcResponse) -> Option<R> {
        if response.failed {
            return None;
        }

        match T::output(response.raw.as_ref()) {
            Ok(data) => Some(data),
            Err(err) => {
                substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            T::NAME, err
                        );
                None
            }
        }
    }
}

fn eth_call_internal(input: Vec<u8>) -> Vec<u8> {
    unsafe {
        let rpc_response_ptr = memory::alloc(8);
        externs::rpc::eth_call(input.as_ptr(), input.len() as u32, rpc_response_ptr);
        return memory::get_output_data(rpc_response_ptr);
    }
}

pub fn eth_call(input: &RpcCalls) -> RpcResponses {
    let raw_resp: Vec<u8> = eth_call_internal(proto::encode(input).unwrap());
    let resp: RpcResponses = proto::decode(&raw_resp).unwrap();

    return resp;
}
