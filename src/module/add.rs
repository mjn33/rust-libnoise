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

/// Noise module that outputs the sum of the two output values from two
/// source modules.
///
/// This noise module requires two source modules.
pub struct Add<M1: Module, M2: Module> {
    module1: M1,
    module2: M2,
}

impl<M1: Module, M2: Module> Add<M1, M2> {
    /// Create a new `Add` noise module around the specified modules.
    pub fn new(module1: M1, module2: M2) -> Add<M1, M2> {
        Add {
            module1: module1,
            module2: module2,
        }
    }

    /// Returns a reference to the first source module.
    pub fn module1(&self) -> &M1 {
        &self.module1
    }

    /// Returns a mutable reference to the first source module used.
    pub fn module1_mut(&mut self) -> &mut M1 {
        &mut self.module1
    }

    /// Returns a reference to the second source module.
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

impl<M1: Module, M2: Module> Module for Add<M1, M2> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        self.module1.get_value(x, y, z) + self.module2.get_value(x, y, z)
    }
}

impl<M1: Module + Clone, M2: Module + Clone> Clone for Add<M1, M2> {
    fn clone(&self) -> Add<M1, M2> {
        Add {
            module1: self.module1.clone(),
            module2: self.module2.clone(),
        }
    }
}
