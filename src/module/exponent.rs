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

/// Default exponent for the [`Exponent`](struct.Exponent.html) noise module.
pub const DEFAULT_EXPONENT: f64 = 1.0;

/// Noise module that maps the output value from a source module onto an
/// exponential curve.
///
/// Because most noise modules will output values that range from -1.0 to +1.0,
/// this noise module first normalizes this output value (the range becomes 0.0
/// to 1.0), maps that value onto an exponential curve, then rescales that value
/// back to the original range.
///
/// This noise module requires one source module.
pub struct Exponent<M: Module> {
    module: M,
    exponent: f64,
}

impl<M: Module> Exponent<M> {
    /// Create a new `Exponent` noise module around the specified module, using
    /// default parameters.
    pub fn new(module: M) -> Exponent<M> {
        Exponent {
            module: module,
            exponent: DEFAULT_EXPONENT,
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

    /// Returns the exponent value to apply to the output value from the source
    /// module.
    ///
    /// Because most noise modules will output values that range from -1.0 to
    /// +1.0, this noise module first normalizes this output value (the range
    /// becomes 0.0 to 1.0), maps that value onto an exponential curve, then
    /// rescales that value back to the original range.
    pub fn exponent(&self) -> f64 {
        self.exponent
    }

    /// Set the source module to be used.
    pub fn set_module(&mut self, module: M) {
        self.module = module;
    }

    /// Sets the exponent value to apply to the output value from the source
    /// module.
    ///
    /// Because most noise modules will output values that range from -1.0 to
    /// +1.0, this noise module first normalizes this output value (the range
    /// becomes 0.0 to 1.0), maps that value onto an exponential curve, then
    /// rescales that value back to the original range.
    pub fn set_exponent(&mut self, exponent: f64) {
        self.exponent = exponent;
    }
}

impl<M: Module> Module for Exponent<M> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let value = self.module.get_value(x, y, z);
        ((value + 1.0) / 2.0).abs().powf(self.exponent) * 2.0 - 1.0
    }
}

impl<M: Module + Clone> Clone for Exponent<M> {
    fn clone(&self) -> Exponent<M> {
        Exponent {
            module: self.module.clone(),
            exponent: self.exponent,
        }
    }
}
