use std::sync::Mutex;
use spade::Point2;


#[repr(u32)]
pub enum CorrectionKey {
    Pos = 0,
    Vel = 1,
    Acc = 2,
    Apos = 3,
    Avel = 4,
    Aacc = 5,
    Grav = 6,
    Layer = 7,
    Prevapos = 8,
}
#[macro_export]
macro_rules! ffi {
    //for functions that return a value
    (fn $name:ident($($arg:ident: $ty:ty),*) -> $ret:ty $body:block) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $name($($arg: $ty),*) -> $ret $body
    };
    //for functions that return nothing (void in C#)
    (fn $name:ident($($arg:ident: $ty:ty),*) $body:block) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $name($($arg: $ty),*) $body
    };
    //for functions with no arguments
    (fn $name:ident() -> $ret:ty $body:block) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $name() -> $ret $body
    };
    //for functions with no arguments and returns nothing
    (fn $name:ident() $body:block) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $name() $body
    };
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CorrectionData {
    pub index: u32,
    pub data_type: u32,
    
    // Generic storage
    pub x: f32,
    pub y: f32,
    pub value: f32,
    pub uvalue: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: Point2<f32>,
    pub id: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Matrix2 {
    pub col1: Vector2,
    pub col2: Vector2,
}
pub struct DVect {
    pub radians: f32,
    pub magnitude: f32,
} //just in case i need it
pub struct Object1l {
    pub pos: Vector2,
    pub vel: Vector2,
    pub acc: Vector2,
    pub apos: f32,
    pub avel: f32, // radians, not degrees
    pub aacc: f32,
    pub grav: Vector2,
}
pub struct Object2l {
    pub id: u32,
    pub obj: Object1l,
    pub collision: Vec<Vector2>,
    pub nonwarp_collision: Vec<Vector2>,
    pub collision_order: Vec<u32>,
    pub layer: u32,
    pub coobjs: Vec<u32>,
    pub prevapos: f32,
}
pub struct Island3l {
    pub objids: Vec<u32>,
}
pub struct Archi4l {
    pub objids: Vec<u32>,
}
#[repr(C)]
pub struct PhisiksObject {
    pub id: u32,
}