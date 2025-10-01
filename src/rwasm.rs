#[link(wasm_import_module = "fluentbase_v1preview")]
extern "C" {
    pub fn _keccak256_permute(state_ptr: *mut [u64; 25]);
}

/// KeccakF function for Fluentbase (rWasm IR)
#[inline]
pub fn keccakf(state: &mut [u64; 25]) {
    unsafe {
        _keccak256_permute(state.as_mut_ptr() as *mut [u64; 25]);
    }
}
