use std::ops::{FnOnce, FnMut, Fn};

pub struct Closure{
    pub i: isize,
}

impl FnOnce<(isize,)> for Closure {
    type Output = isize;
    extern "rust-call" fn call_once(self, (arg, ): (isize, )) -> Self::Output {
        self.i + arg
    }
}


pub struct Closure2{
    pub i: isize,
}

impl Fn<(isize,)> for Closure2 {
    extern "rust-call" fn call(&self, (arg, ): (isize, )) -> Self::Output {
        self.i + arg
    }
}

impl FnMut<(isize,)> for Closure2 {
    extern "rust-call" fn call_mut(&mut self, (arg, ): (isize, )) -> Self::Output {
        self.i + arg
    }
}

impl FnOnce<(isize,)> for Closure2 {
    type Output = isize;
    extern "rust-call" fn call_once(self, (arg, ): (isize, )) -> Self::Output {
        self.i + arg
    }
}
