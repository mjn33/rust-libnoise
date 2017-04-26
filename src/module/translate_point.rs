// Copyright (C) 2004 Jason Bevins, 2016 Matthew Nicholls
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

/// Default translation applied to the `x` coordinate for the
/// [`TranslatePoint`](struct.TranslatePoint.html) noise module.
pub const DEFAULT_TRANSLATE_POINT_X: f64 = 0.0;

/// Default translation applied to the `y` coordinate for the
/// [`TranslatePoint`](struct.TranslatePoint.html) noise module.
pub const DEFAULT_TRANSLATE_POINT_Y: f64 = 0.0;

/// Default translation applied to the `z` coordinate for the
/// [`TranslatePoint`](struct.TranslatePoint.html) noise module.
pub const DEFAULT_TRANSLATE_POINT_Z: f64 = 0.0;

/// Noise module that moves the coordinates of the input value before returning
/// the output value from a source module.
///
/// The [`get_value()`](struct.TranslatePoint.html#method.get_value) method
/// moves the (`x`, `y`, `z`) coordinates of the input value by a translation
/// amount before returning the output value from the source module.  To set the
/// translation amount, call the
/// [`set_trans()`](struct.TranslatePoint.html#method.set_trans)
/// method.  To set the translation amount to apply to the individual `x`, `y`,
/// or `z` coordinates, call the
/// [`set_x_trans()`](struct.TranslatePoint.html#method.set_x_trans),
/// [`set_y_trans()`](struct.TranslatePoint.html#method.set_y_trans)
/// or [`set_z_trans()`](struct.TranslatePoint.html#method.set_z_scale)
/// methods, respectively.
///
/// This noise module requires one source module.
pub struct TranslatePoint<M: Module> {
    module: M,
    trans: (f64, f64, f64),
}

impl<M: Module> TranslatePoint<M> {
    /// Create a new `TranslatePoint` noise module around the specified module,
    /// using default parameters.
    pub fn new(module: M) -> TranslatePoint<M> {
        TranslatePoint {
            module: module,
            trans: (DEFAULT_TRANSLATE_POINT_X, DEFAULT_TRANSLATE_POINT_Y, DEFAULT_TRANSLATE_POINT_Z),
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

    /// Returns the translation applied to the `x` coordinate of the input
    /// value.
    pub fn x_trans(&self) -> f64 {
        self.trans.0
    }

    /// Returns the translation applied to the `y` coordinate of the input
    /// value.
    pub fn y_trans(&self) -> f64 {
        self.trans.1
    }

    /// Returns the translation applied to the `z` coordinate of the input
    /// value.
    pub fn z_trans(&self) -> f64 {
        self.trans.2
    }

    /// Set the source module to be used.
    pub fn set_module(&mut self, module: M) {
        self.module = module;
    }

    /// Sets the translation amount to apply to the input value.
    ///
    /// The [`get_value()`](struct.TranslatePoint.html#method.get_value) method
    /// moves the (`x`, `y`, `z`) coordinates of the input value by a
    /// translation amount before returning the output value from the source
    /// module.
    pub fn set_trans(&mut self, trans: f64) {
        self.trans = (trans, trans, trans);
    }

    /// Sets the translation amount to apply to the (`x`, `y`, `z`) coordinates of
    /// the input value.
    ///
    /// The [`get_value()`](struct.TranslatePoint.html#method.get_value) method
    /// moves the (`x`, `y`, `z`) coordinates of the input value by a
    /// translation amount before returning the output value from the source
    /// module.
    pub fn set_xyz_trans(&mut self, x: f64, y: f64, z: f64) {
        self.trans = (x, y, z);
    }

    /// Sets the translation to apply to the `x` coordinate of the input value.
    ///
    /// The [`get_value()`](struct.TranslatePoint.html#method.get_value) method
    /// moves the (`x`, `y`, `z`) coordinates of the input value by a
    /// translation amount before returning the output value from the source
    /// module.
    pub fn set_x_trans(&mut self, x: f64) {
        self.trans.0 = x;
    }

    /// Sets the translation to apply to the `y` coordinate of the input value.
    ///
    /// The [`get_value()`](struct.TranslatePoint.html#method.get_value) method
    /// moves the (`x`, `y`, `z`) coordinates of the input value by a
    /// translation amount before returning the output value from the source
    /// module.
    pub fn set_y_trans(&mut self, y: f64) {
        self.trans.1 = y;
    }

    /// Sets the translation to apply to the `z` coordinate of the input value.
    ///
    /// The [`get_value()`](struct.TranslatePoint.html#method.get_value) method
    /// moves the (`x`, `y`, `z`) coordinates of the input value by a
    /// translation amount before returning the output value from the source
    /// module.
    pub fn set_z_trans(&mut self, z: f64) {
        self.trans.2 = z;
    }
}

impl<M: Module> Module for TranslatePoint<M> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        self.module.get_value(x + self.trans.0, y + self.trans.1, z + self.trans.2)
    }
}

impl<M: Module + Clone> Clone for TranslatePoint<M> {
    fn clone(&self) -> TranslatePoint<M> {
        TranslatePoint {
            module: self.module.clone(),
            trans: self.trans,
        }
    }
}
