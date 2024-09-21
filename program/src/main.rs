#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use spdemo_lib::{f, PublicValuesStruct};

pub fn main() {
    let x = sp1_zkvm::io::read::<u32>();
    let y = sp1_zkvm::io::read::<u32>();
    let z = f(x, y);
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { x, y, z });
    sp1_zkvm::io::commit_slice(&bytes);
}
