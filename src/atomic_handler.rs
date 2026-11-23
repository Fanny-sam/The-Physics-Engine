use std::ops::{Add, AddAssign, Div, Deref, Mul, Sub};
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Clone, Copy)]
pub struct GlobalInt {
    raw: &'static AtomicU32,
}

#[allow(non_snake_case)]
pub fn GlobalInt(n: u32) -> GlobalInt {
    GlobalInt {
        raw: Box::leak(Box::new(AtomicU32::new(n))),
    }
}

impl GlobalInt {
    pub const fn from_static(raw: &'static AtomicU32) -> Self {
        GlobalInt { raw }
    }

    pub fn get(&self) -> u32 {
        self.raw.load(Ordering::Relaxed)
    }
}

impl Deref for GlobalInt {
    type Target = AtomicU32;
    fn deref(&self) -> &AtomicU32 {
        self.raw
    }
}

impl Add<u32> for GlobalInt {
    type Output = GlobalInt;
    fn add(self, rhs: u32) -> GlobalInt {
        self.raw.fetch_add(rhs, Ordering::Relaxed);
        self
    }
}

impl Sub<u32> for GlobalInt {
    type Output = GlobalInt;
    fn sub(self, rhs: u32) -> GlobalInt {
        self.raw.fetch_sub(rhs, Ordering::Relaxed);
        self
    }
}

impl Mul<u32> for GlobalInt {
    type Output = GlobalInt;
    fn mul(self, rhs: u32) -> GlobalInt {
        self.raw
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| Some(v * rhs))
            .unwrap();
        self
    }
}

impl Div<u32> for GlobalInt {
    type Output = GlobalInt;
    fn div(self, rhs: u32) -> GlobalInt {
        self.raw
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| Some(v / rhs))
            .unwrap();
        self
    }
}

impl AddAssign<u32> for GlobalInt {
    fn add_assign(&mut self, rhs: u32) {
        self.raw.fetch_add(rhs, Ordering::Relaxed);
    }
}

impl std::fmt::Debug for GlobalInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.get(), f)
    }
}

impl std::fmt::Display for GlobalInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}