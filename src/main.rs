mod object_frontend;
mod constants;
mod methods;
use crate::constants::*;
ffi! {
    fn phisiks_correct(obj: *mut Object2l, correction_data: CorrectionData) {
        unsafe {(*obj).correction(correction_data);} // call the safe rust method
    }
}
ffi! {
    fn phisiks_update(obj: *mut PhisiksObject, dt: f32) {
        unsafe {(*(obj as *mut Object2l)).update(dt);}
    }
}
fn main() {
    println!("Hello, World!");
}