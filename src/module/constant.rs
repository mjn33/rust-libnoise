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

/// Default constant value for the [`Constant`](struct.Constant.html) noise
/// module.
pub const DEFAULT_CONST_VALUE: f64 = 0.0;

/// Noise module that outputs a constant value.
///
/// This noise module is not useful by itself, but it is often used as a source
/// module for other noise modules.
///
/// This noise module does not require any source modules.
#[derive(Clone)]
pub struct Constant {
    val: f64,
}

impl Default for Constant {
    /// Create a new `Constant` noise module with default parameters.
    fn default() -> Constant {
        Constant {
            val: DEFAULT_CONST_VALUE,
        }
    }
}

impl Constant {
    /// Create a new `Constant` noise module with default parameters.
    pub fn new() -> Constant {
        Default::default()
    }

    /// Returns the constant output value for this noise module.
    pub fn const_value(&self) -> f64 {
        self.val
    }

    /// Sets the constant output value for this noise module.
    pub fn set_const_value(&mut self, val: f64) {
        self.val = val;
    }
}

impl Module for Constant {
    fn get_value(&self, _x: f64, _y: f64, _z: f64) -> f64 {
        self.val
    }
}

