extern crate kb;

macro_rules! rterr {
    ( $($args:expr),+ $(,)?) => {
        crate::rterr(format!( $($args),+ ))
    };
}

mod dh;
mod ggezh;
mod m;

pub use kb::*;

pub use dh::*;
pub use ggezh::*;
pub use m::main;
