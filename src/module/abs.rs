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

/// Noise module that outputs the absolute value of the output value from a
/// source module.
pub struct Abs<M: Module> {
    module: M,
}

impl<M: Module> Abs<M> {
    /// Create a new `Abs` noise module around the specified module.
    pub fn new(module: M) -> Abs<M> {
        Abs {
            module: module
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

    /// Set the source module to be used.
    pub fn set_module(&mut self, module: M) {
        self.module = module;
    }
}

impl<M: Module> Module for Abs<M> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        self.module.get_value(x, y, z).abs()
    }
}

impl<M: Module + Clone> Clone for Abs<M> {
    fn clone(&self) -> Abs<M> {
        Abs {
            module: self.module.clone(),
        }
    }
}
