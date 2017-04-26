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

/// Noise module that uses three source modules to displace each coordinate of
/// the input value before returning the output value from a source module.
///
/// The [`get_value()`](struct.Displace.html#method.get_value) method modifies
/// the (`x`, `y`, `z`) coordinates of the input value using the output
/// values from the three displacement modules before retrieving the output
/// value from the source module.
///
/// The [`Turbulence`](../turbulence/struct.Turbulence.html) noise module is a
/// special case of the displacement module; internally, there are three
/// Perlin-noise modules that perform the displacement operation.
///
/// This noise module requires four source modules.
pub struct Displace<MS: Module, MX: Module, MY: Module, MZ: Module> {
    msource: MS,
    mdisp_x: MX,
    mdisp_y: MY,
    mdisp_z: MZ
}

impl<MS: Module, MX: Module, MY: Module, MZ: Module> Displace<MS, MX, MY, MZ> {
    /// Create a new `Displace` noise module around the specified modules.
    pub fn new(msource: MS, mdisp_x: MX, mdisp_y: MY, mdisp_z: MZ) -> Displace<MS, MX, MY, MZ> {
        Displace {
            msource: msource,
            mdisp_x: mdisp_x,
            mdisp_y: mdisp_y,
            mdisp_z: mdisp_z,
        }
    }

    /// Returns a reference to the module whose input values are being
    /// displaced.
    pub fn module(&self) -> &MS {
        &self.msource
    }

    /// Returns a mutable reference to the module whose input values are being
    /// displaced.
    pub fn module_mut(&mut self) -> &mut MS {
        &mut self.msource
    }

    /// Returns a reference to the `x`-displacement module.
    ///
    /// The [`get_value()`](struct.Displace.html#method.get_value) method
    /// displaces the input value by adding the output value from this
    /// displacement module to the `x` coordinate of the input value before
    /// returning the output value from the source module.
    pub fn x_displace_module(&self) -> &MX {
        &self.mdisp_x
    }

    /// Returns a mutable reference to the `x`-displacement module.
    pub fn x_displace_module_mut(&mut self) -> &mut MX {
        &mut self.mdisp_x
    }

    /// Returns the `y`-displacement module.
    ///
    /// The [`get_value()`](struct.Displace.html#method.get_value) method
    /// displaces the input value by adding the output value from this
    /// displacement module to the `y` coordinate of the input value before
    /// returning the output value from the source module.
    pub fn y_displace_module(&self) -> &MY {
        &self.mdisp_y
    }

    /// Returns a mutable reference to the `y`-displacement module.
    pub fn y_displace_module_mut(&mut self) -> &mut MY {
        &mut self.mdisp_y
    }

    /// Returns the `z`-displacement module.
    ///
    /// The [`get_value()`](struct.Displace.html#method.get_value) method
    /// displaces the input value by adding the output value from this
    /// displacement module to the `z` coordinate of the input value before
    /// returning the output value from the source module.
    pub fn z_displace_module(&self) -> &MZ {
        &self.mdisp_z
    }

    /// Returns a mutable reference to the `z`-displacement module.
    pub fn z_displace_module_mut(&mut self) -> &mut MZ {
        &mut self.mdisp_z
    }

    /// Sets the module whose input values are going to be displaced.
    pub fn set_module(&mut self, module: MS) {
        self.msource = module;
    }

    /// Sets the `x`-displacement module.
    ///
    /// The [`get_value()`](struct.Displace.html#method.get_value) method
    /// displaces the input value by adding the output value from this
    /// displacement module to the `x` coordinate of the input value before
    /// returning the output value from the source module.
    pub fn set_x_displace_module(&mut self, module: MX) {
        self.mdisp_x = module;
    }

    /// Sets the `y`-displacement module.
    ///
    /// The [`get_value()`](struct.Displace.html#method.get_value) method
    /// displaces the input value by adding the output value from this
    /// displacement module to the `y` coordinate of the input value before
    /// returning the output value from the source module.
    pub fn set_y_displace_module(&mut self, module: MY) {
        self.mdisp_y = module;
    }

    /// Sets the `z`-displacement module.
    ///
    /// The [`get_value()`](struct.Displace.html#method.get_value) method
    /// displaces the input value by adding the output value from this
    /// displacement module to the `z` coordinate of the input value before
    /// returning the output value from the source module.
    pub fn set_z_displace_module(&mut self, module: MZ) {
        self.mdisp_z = module;
    }
}

impl<MS: Module, MX: Module, MY: Module, MZ: Module> Module for Displace<MS, MX, MY, MZ> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        // Get the output values from the three displacement modules.  Add each
        // value to the corresponding coordinate in the input value.
        let x_displace = x + self.mdisp_x.get_value(x, y, z);
        let y_displace = y + self.mdisp_y.get_value(x, y, z);
        let z_displace = z + self.mdisp_z.get_value(x, y, z);

        // Retrieve the output value using the offsetted input value instead of
        // the original input value.
        self.msource.get_value(x_displace, y_displace, z_displace)
    }
}

impl<MS: Module + Clone,
     MX: Module + Clone,
     MY: Module + Clone,
     MZ: Module + Clone> Clone for Displace<MS, MX, MY, MZ> {
    fn clone(&self) -> Displace<MS, MX, MY, MZ> {
        Displace {
            msource: self.msource.clone(),
            mdisp_x: self.mdisp_x.clone(),
            mdisp_y: self.mdisp_y.clone(),
            mdisp_z: self.mdisp_z.clone(),
        }
    }
}
