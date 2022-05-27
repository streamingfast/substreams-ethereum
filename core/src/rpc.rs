use crate::externs;
use substreams::memory;
use substreams::proto;
use crate::pb;

fn eth_call_internal(input: Vec<u8>) -> Vec<u8> {
    unsafe {
        let rpc_response_ptr = memory::alloc(8);
        externs::rpc::eth_call(input.as_ptr(), input.len() as u32, rpc_response_ptr);
        return memory::get_output_data(rpc_response_ptr);
    }
}

pub fn eth_call(input: &pb::eth::rpc::RpcCalls) -> pb::eth::rpc::RpcResponses {
    let raw_resp: Vec<u8> = eth_call_internal(proto::encode(input).unwrap());
    let resp: pb::eth::rpc::RpcResponses = proto::decode(&raw_resp).unwrap();

    return resp;
}
