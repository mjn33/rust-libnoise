// Copyright (C) 2004 Owen Jacobson, 2016 Matthew Nicholls
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

/// Noise module that raises the output value from a first source module to the
/// power of the output value from a second source module.
///
/// This noise module requires two source modules.
pub struct Power<M1: Module, M2: Module> {
    module1: M1,
    module2: M2,
}

impl<M1: Module, M2: Module> Power<M1, M2> {
    /// Create a new `Power` noise module around the specified modules.
    pub fn new(module1: M1, module2: M2) -> Power<M1, M2> {
        Power {
            module1: module1,
            module2: module2,
        }
    }

    /// Returns a reference to the first source module used.
    pub fn module1(&self) -> &M1 {
        &self.module1
    }

    /// Returns a mutable reference to the first source module used.
    pub fn module1_mut(&mut self) -> &mut M1 {
        &mut self.module1
    }

    /// Returns a reference to the second source module used.
    pub fn module2(&self) -> &M2 {
        &self.module2
    }

    /// Returns a mutable reference to the second source module used.
    pub fn module2_mut(&mut self) -> &mut M2 {
        &mut self.module2
    }

    /// Set the first source module to be used.
    pub fn set_module1(&mut self, module: M1) {
        self.module1 = module;
    }

    /// Set the second source module to be used.
    pub fn set_module2(&mut self, module: M2) {
        self.module2 = module;
    }
}

impl<M1: Module, M2: Module> Module for Power<M1, M2> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let value1 = self.module1.get_value(x, y, z);
        let value2 = self.module2.get_value(x, y, z);
        value1.powf(value2)
    }
}

impl<M1: Module + Clone, M2: Module + Clone> Clone for Power<M1, M2> {
    fn clone(&self) -> Power<M1, M2> {
        Power {
            module1: self.module1.clone(),
            module2: self.module2.clone(),
        }
    }
}
