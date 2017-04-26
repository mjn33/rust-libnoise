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

/// Default bias for the [`ScaleBias`](struct.ScaleBias.html) noise module.
pub const DEFAULT_BIAS: f64 = 0.0;

/// Default scale for the [`ScaleBias`](struct.ScaleBias.html) noise module.
pub const DEFAULT_SCALE: f64 = 1.0;

/// Noise module that applies a scaling factor and a bias to the output value
/// from a source module.
///
/// The [`get_value()`](struct.ScaleBias.html#method.get_value) method retrieves
/// the output value from the source module, multiplies it with a scaling
/// factor, adds a bias to it, then outputs the value.
///
/// This noise module requires one source module.
pub struct ScaleBias<M: Module> {
    module: M,
    scale: f64,
    bias: f64,
}

impl<M: Module> ScaleBias<M> {
    /// Create a new `ScaleBias` noise module around the specified module, using
    /// default parameters.
    pub fn new(module: M) -> ScaleBias<M> {
        ScaleBias {
            module: module,
            scale: DEFAULT_SCALE,
            bias: DEFAULT_BIAS,
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

    /// Returns the scaling factor to apply to the output value from the source
    /// module.
    ///
    /// The [`get_value()`](struct.ScaleBias.html#method.get_value) method
    /// retrieves the output value from the source module, multiplies it with
    /// the scaling factor, adds the bias to it, then outputs the value.
    pub fn scale(&self) -> f64 {
        self.scale
    }

    /// Returns the bias to apply to the scaled output value from the source
    /// module.
    ///
    /// The [`get_value()`](struct.ScaleBias.html#method.get_value) method
    /// retrieves the output value from the source module, multiplies it with
    /// the scaling factor, adds the bias to it, then outputs the value.
    pub fn bias(&self) -> f64 {
        self.bias
    }

    /// Set the source module to be used.
    pub fn set_module(&mut self, module: M) {
        self.module = module;
    }

    /// Sets the scaling factor to apply to the output value from the source
    /// module.
    ///
    /// The [`get_value()`](struct.ScaleBias.html#method.get_value) method
    /// retrieves the output value from the source module, multiplies it with
    /// the scaling factor, adds the bias to it, then outputs the value.
    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    /// Sets the bias to apply to the scaled output value from the source
    /// module.
    ///
    /// The [`get_value()`](struct.ScaleBias.html#method.get_value) method
    /// retrieves the output value from the source module, multiplies it with
    /// the scaling factor, adds the bias to it, then outputs the value.
    pub fn set_bias(&mut self, bias: f64) {
        self.bias = bias;
    }
}

impl<M: Module> Module for ScaleBias<M> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let value = self.module.get_value(x, y, z);
        value * self.scale + self.bias
    }
}

impl<M: Module + Clone> Clone for ScaleBias<M> {
    fn clone(&self) -> ScaleBias<M> {
        ScaleBias {
            module: self.module.clone(),
            scale: self.scale,
            bias: self.bias,
        }
    }
}
