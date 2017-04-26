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

/// Default lower bound of the clamping range for the
/// [`Clamp`](struct.Clamp.html) noise module.
pub const DEFAULT_CLAMP_LOWER_BOUND: f64 = -1.0;

/// Default upper bound of the clamping range for the
/// [`Clamp`](struct.Clamp.html) noise module.
pub const DEFAULT_CLAMP_UPPER_BOUND: f64 = 1.0;

/// Noise module that clamps the output value from a source module to a range of
/// values.
///
/// The range of values in which to clamp the output value is called the
/// *clamping range*.
///
/// If the output value from the source module is less than the lower bound of
/// the clamping range, this noise module clamps that value to the lower bound.
/// If the output value from the source module is greater than the upper bound
/// of the clamping range, this noise module clamps that value to the upper
/// bound.
///
/// To specify the upper and lower bounds of the clamping range, call the
/// [`set_bounds()`](struct.Clamp.html#method.set_bounds) method.
///
/// This noise module requires one source module.
pub struct Clamp<M: Module> {
    module: M,
    lower_bound: f64,
    upper_bound: f64,
}

impl<M: Module> Clamp<M> {
    /// Create a new `Clamp` noise module around the specified module, using
    /// default parameters.
    pub fn new(module: M) -> Clamp<M> {
        Clamp {
            module: module,
            lower_bound: DEFAULT_CLAMP_LOWER_BOUND,
            upper_bound: DEFAULT_CLAMP_UPPER_BOUND,
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

    /// Returns the lower bound of the clamping range.
    ///
    /// If the output value from the source module is less than the lower bound
    /// of the clamping range, this noise module clamps that value to the lower
    /// bound.
    pub fn lower_bound(&self) -> f64 {
        self.lower_bound
    }

    /// Returns the upper bound of the clamping range.
    ///
    /// If the output value from the source module is greater than the upper
    /// bound of the clamping range, this noise module clamps that value to the
    /// upper bound.
    pub fn upper_bound(&self) -> f64 {
        self.upper_bound
    }

    /// Set the source module to be used.
    pub fn set_module(&mut self, module: M) {
        self.module = module;
    }

    /// Sets the lower and upper bounds of the clamping range.
    ///
    /// If the output value from the source module is less than the lower bound
    /// of the clamping range, this noise module clamps that value to the lower
    /// bound.  If the output value from the source module is greater than the
    /// upper bound of the clamping range, this noise module clamps that value
    /// to the upper bound.
    ///
    /// # Panics
    ///
    /// Panics if the given lower bound is greater than the given upper bound.
    pub fn set_bounds(&mut self, lower_bound: f64, upper_bound: f64) {
        if lower_bound > upper_bound {
            panic!("Lower bound is larger than upper bound!");
        }
        self.lower_bound = lower_bound;
        self.upper_bound = upper_bound;
    }
}

impl<M: Module> Module for Clamp<M> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let value = self.module.get_value(x, y, z);
        if value < self.lower_bound {
            self.lower_bound
        } else if value > self.upper_bound {
            self.upper_bound
        } else {
            value
        }
    }
}

impl<M: Module + Clone> Clone for Clamp<M> {
    fn clone(&self) -> Clamp<M> {
        Clamp {
            module: self.module.clone(),
            lower_bound: self.lower_bound,
            upper_bound: self.upper_bound,
        }
    }
}
