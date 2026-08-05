#![no_std]

use spirv_std::spirv;

#[spirv(compute(threads(64)))]
pub fn run(
    #[spirv(global_invocation_id)] id: spirv::UVec3
) {
    let number = id.x;

    // GPU calculations here
}