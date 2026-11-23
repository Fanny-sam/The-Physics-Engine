use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {U32(u32), I32(i32), F32(f32)}

impl Value {
    #[inline(always)]
    pub fn as_f32(&self) -> Option<f32> {
        match *self {Value::F32(v) => Some(v) _ => None}
    }
    #[inline(always)]
    pub fn as_i32(&self) -> Option<i32> {
        match *self {Value::I32(v) => Some(v) _ => None}
    }
    #[inline(always)]
    pub fn as_u32(&self) -> Option<u32> {
        match *self {Value::U32(v) => Some(v) _ => None}
    }
}

impl From<f32> for Value {fn from(v: f32) -> Self {Value::F32(v)}}
impl From<i32> for Value {fn from(v: i32) -> Self {Value::I32(v)}}
impl From<u32> for Value {fn from(v: u32) -> Self {Value::U32(v)}}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::F32(v) => write!(f, "{v}")
            Value::I32(v) => write!(f, "{v}")
            Value::U32(v) => write!(f, "{v}")
        }
    }
}

type EqFnN<const N: usize> = dyn Fn(&[Value], f32) -> [Value; N] + Send + Sync;
pub struct EqVarN<const N: usize> {f: Box<EqFnN<N>>, cache: Vec<Value>, name: &'static str}

impl<const N: usize> EqVarN<N> {
    pub fn new(name: &'static str, cache: Vec<Value>, f: impl Fn(&[Value], f32) -> [Value; N] + Send + Sync + 'static) ->
    Self {
        Self {f: Box::new(f), cache, name}
    }
    #[inline(always)]
    pub fn eval(&self, t: f32) ->
    [Value; N] {
        (self.f)(&self.cache, t)
    }
    pub fn set(&mut self, i: usize, v: impl Into<Value>) {
        match self.cache.get_mut(i) {
            Some(slot) => *slot = v.into();
            None => panic!("EqVarN '{}': idx {} OOR (len = {})", self.name, i, self.cache.len());
        }
    }
}
