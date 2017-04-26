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
use util::linear_interp;

/// Noise module that outputs a weighted blend of the output values from two
/// source modules given the output value supplied by a control module.
///
/// This noise module uses linear interpolation to perform the blending
/// operation.
///
/// This noise module requires three source modules.
pub struct Blend<M1: Module, M2: Module, MC: Module> {
    module1: M1,
    module2: M2,
    mcontrol: MC,
}

impl<M1: Module, M2: Module, MC: Module> Blend<M1, M2, MC> {
    /// Create a new `Blend` noise module around the specified modules.
    pub fn new(module1: M1, module2: M2, control: MC) -> Blend<M1, M2, MC> {
        Blend {
            module1: module1,
            module2: module2,
            mcontrol: control,
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

    /// Returns a reference to the control module.
    ///
    /// The control module determines the weight of the blending operation.
    /// Negative values weigh the blend towards the output value from `module1`.
    /// Positive values weigh the blend towards the output value from `module2`.
    pub fn control_module(&self) -> &MC {
        &self.mcontrol
    }

    /// Returns a reference to the control module.
    ///
    /// The control module determines the weight of the blending operation.
    /// Negative values weigh the blend towards the output value from `module1`.
    /// Positive values weigh the blend towards the output value from `module2`.
    pub fn control_module_mut(&mut self) -> &mut MC {
        &mut self.mcontrol
    }

    /// Set the first module to be used.
    pub fn set_module1(&mut self, module1: M1) {
        self.module1 = module1;
    }

    /// Set the second module to be used.
    pub fn set_module2(&mut self, module2: M2) {
        self.module2 = module2;
    }

    /// Sets the control module.
    ///
    /// The control module determines the weight of the blending operation.
    /// Negative values weigh the blend towards the output value from `module1`.
    /// Positive values weigh the blend towards the output value from `module2`.
    pub fn set_control_module(&mut self, control: MC) {
        self.mcontrol = control;
    }
}

impl<M1: Module, M2: Module, MC: Module> Module for Blend<M1, M2, MC> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let v0 = self.module1.get_value(x, y, z);
        let v1 = self.module2.get_value(x, y, z);
        let alpha = (self.mcontrol.get_value(x, y, z) + 1.0) / 2.0;
        linear_interp(v0, v1, alpha)
    }
}

impl<M1: Module + Clone,
     M2: Module + Clone,
     MC: Module + Clone> Clone for Blend<M1, M2, MC> {
    fn clone(&self) -> Blend<M1, M2, MC> {
        Blend {
            module1: self.module1.clone(),
            module2: self.module2.clone(),
            mcontrol: self.mcontrol.clone(),
        }
    }
}
