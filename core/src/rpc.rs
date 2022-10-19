use crate::pb::eth::rpc::{RpcCall, RpcCalls, RpcResponse, RpcResponses};
use crate::Function;
use substreams::proto;

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
                    T::NAME,
                    err
                );
                None
            }
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
fn eth_call_internal(input: Vec<u8>) -> Vec<u8> {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        let rpc_response_ptr = substreams::memory::alloc(8);
        crate::externs::rpc::eth_call(input.as_ptr(), input.len() as u32, rpc_response_ptr);
        return memory::get_output_data(rpc_response_ptr);
    }

    #[cfg(not(target_arch = "wasm32"))]
    unimplemented!("this method is not implemented outside of 'wasm32' target compilation")
}

pub fn eth_call(input: &RpcCalls) -> RpcResponses {
    let raw_resp: Vec<u8> = eth_call_internal(proto::encode(input).unwrap());
    let resp: RpcResponses = proto::decode(&raw_resp).unwrap();

    return resp;
}
