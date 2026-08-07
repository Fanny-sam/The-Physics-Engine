#![no_std]

use spirv_std::spirv;

#[spirv(compute(threads(64)))]
pub fn run(#[spirv(global_invocation_id)] id: spirv::UVec3 /* id, idk how this works, but it works. */,  ) {
    let mut i = id;
    while i < 4096 {
        let m = batch[i];
    
        if m.is_zero() {
            break;
        }
        
        process(m);
        
        i += 64;
    }
}