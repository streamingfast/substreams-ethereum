pub mod rpc {
    #[link(wasm_import_module = "rpc")]
    extern "C" {
        pub fn eth_call(
            rpc_call_offset: *const u8,
            rpc_call_len: u32,
            rpc_response_ptr: *const u8,
        );
    }
}
