// Copyright (C) 2003, 2004 Jason Bevins, 2016 Matthew Nicholls
//
// This library is free software; you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 2.1 of the License, or (at
// your option) any later version.
//
// This library is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
// License (COPYING.txt) for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this library; if not, write to the Free Software Foundation,
// Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA

use module::Module;

/// Default scaling factor applied to the `x` coordinate for the
/// [`ScalePoint`](struct.ScalePoint.html) noise module.
pub const DEFAULT_SCALE_POINT_X: f64 = 1.0;

/// Default scaling factor applied to the `y` coordinate for the
/// [`ScalePoint`](struct.ScalePoint.html) noise module.
pub const DEFAULT_SCALE_POINT_Y: f64 = 1.0;

/// Default scaling factor applied to the `z` coordinate for the
/// [`ScalePoint`](struct.ScalePoint.html) noise module.
pub const DEFAULT_SCALE_POINT_Z: f64 = 1.0;

/// Noise module that scales the coordinates of the input value before returning
/// the output value from a source module.
///
/// The [`get_value()`](struct.ScalePoint.html#method.get_value) method
/// multiplies the (`x`, `y`, `z`) coordinates of the input value with a scaling
/// factor before returning the output value from the source module.  To set the
/// scaling factor, call the
/// [`set_scale()`](struct.ScalePoint.html#method.set_scale) method.  To set the
/// scaling factor to apply to the individual `x`, `y`, or `z` coordinates, call
/// the [`set_x_scale()`](struct.ScalePoint.html#method.set_x_scale),
/// [`set_y_scale()`](struct.ScalePoint.html#method.set_y_scale) or
/// [`set_z_scale()`](struct.ScalePoint.html#method.set_z_scale) methods,
/// respectively.
///
/// This noise module requires one source module.
pub struct ScalePoint<M: Module> {
    module: M,
    scale: (f64, f64, f64),
}

impl<M: Module> ScalePoint<M> {
    /// Create a new `ScalePoint` noise module around the specified module,
    /// using default parameters.
    pub fn new(module: M) -> ScalePoint<M> {
        ScalePoint {
            module: module,
            scale: (DEFAULT_SCALE_POINT_X, DEFAULT_SCALE_POINT_Y, DEFAULT_SCALE_POINT_Z),
        }
    }

    /// Returns a reference to the source module used.
    pub fn module(&self) -> &M {
        &self.module
    }

    /// Returns a mutable reference to the source module used.
    pub fn module_mut(&mut self) -> &mut M {
        &mut self.module
    }

    /// Returns the scaling factor applied to the `x` coordinate of the input
    /// value.
    pub fn x_scale(&self) -> f64 {
        self.scale.0
    }

    /// Returns the scaling factor applied to the `y` coordinate of the input
    /// value.
    pub fn y_scale(&self) -> f64 {
        self.scale.1
    }

    /// Returns the scaling factor applied to the `z` coordinate of the input
    /// value.
    pub fn z_scale(&self) -> f64 {
        self.scale.2
    }

    /// Set the source module to be used.
    pub fn set_module(&mut self, module: M) {
        self.module = module;
    }

    /// Sets the scaling factor to apply to the input value.
    ///
    /// The [`get_value()`](struct.ScalePoint.html#method.get_value) method
    /// multiplies the (`x`, `y`, `z`) coordinates of the input value with a
    /// scaling factor before returning the output value from the source module.
    pub fn set_scale(&mut self, scale: f64) {
        self.scale = (scale, scale, scale);
    }

    /// Sets the scaling factor to apply to the (`x`, `y`, `z`) coordinates of
    /// the input value.
    ///
    /// The [`get_value()`](struct.ScalePoint.html#method.get_value) method
    /// multiplies the (`x`, `y`, `z`) coordinates of the input value with a
    /// scaling factor before returning the output value from the source module.
    pub fn set_xyz_scale(&mut self, x: f64, y: f64, z: f64) {
        self.scale = (x, y, z);
    }

    /// Sets the scaling factor to apply to the `x` coordinate of the input
    /// value.
    ///
    /// The [`get_value()`](struct.ScalePoint.html#method.get_value) method
    /// multiplies the (`x`, `y`, `z`) coordinates of the input value with a
    /// scaling factor before returning the output value from the source module.
    pub fn set_x_scale(&mut self, x: f64) {
        self.scale.0 = x;
    }

    /// Sets the scaling factor to apply to the `y` coordinate of the input
    /// value.
    ///
    /// The [`get_value()`](struct.ScalePoint.html#method.get_value) method
    /// multiplies the (`x`, `y`, `z`) coordinates of the input value with a
    /// scaling factor before returning the output value from the source module.
    pub fn set_y_scale(&mut self, y: f64) {
        self.scale.1 = y;
    }

    /// Sets the scaling factor to apply to the `z` coordinate of the input
    /// value.
    ///
    /// The [`get_value()`](struct.ScalePoint.html#method.get_value) method
    /// multiplies the (`x`, `y`, `z`) coordinates of the input value with a
    /// scaling factor before returning the output value from the source module.
    pub fn set_z_scale(&mut self, z: f64) {
        self.scale.2 = z;
    }
}

impl<M: Module> Module for ScalePoint<M> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        self.module.get_value(x * self.scale.0, y * self.scale.1, z * self.scale.2)
    }
}

impl<M: Module + Clone> Clone for ScalePoint<M> {
    fn clone(&self) -> ScalePoint<M> {
        ScalePoint {
            module: self.module.clone(),
            scale: self.scale,
        }
    }
}
