mod abs;
mod add;
mod billow;
mod blend;
mod cache;
mod checkerboard;
mod clamp;
mod constant;
mod curve;
mod cylinders;
mod displace;
mod exponent;
mod invert;
mod max;
mod min;
mod multiply;
mod perlin;
mod power;
mod ridged_multi;
mod rotate_point;
mod scale_bias;
mod scale_point;
mod select;
mod spheres;
mod terrace;
mod translate_point;
mod turbulence;
mod voronoi;

use std::ops::Deref;

pub use self::abs::*;
pub use self::add::*;
pub use self::billow::*;
pub use self::blend::*;
pub use self::cache::*;
pub use self::checkerboard::*;
pub use self::clamp::*;
pub use self::constant::*;
pub use self::curve::*;
pub use self::cylinders::*;
pub use self::displace::*;
pub use self::exponent::*;
pub use self::invert::*;
pub use self::max::*;
pub use self::min::*;
pub use self::multiply::*;
pub use self::perlin::*;
pub use self::power::*;
pub use self::ridged_multi::*;
pub use self::rotate_point::*;
pub use self::scale_bias::*;
pub use self::scale_point::*;
pub use self::select::*;
pub use self::spheres::*;
pub use self::terrace::*;
pub use self::translate_point::*;
pub use self::turbulence::*;
pub use self::voronoi::*;

pub trait Module {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64;
}


impl<T: Deref<Target=Module>> Module for T {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        self.deref().get_value(x, y, z)
    }
}
