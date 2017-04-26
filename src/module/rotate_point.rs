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

/// Default `x` rotation angle for the [`RotatePoint`](struct.RotatePoint.html)
/// noise module.
pub const DEFAULT_ROTATE_X: f64 = 0.0;

/// Default `y` rotation angle for the [`RotatePoint`](struct.RotatePoint.html)
/// noise module.
pub const DEFAULT_ROTATE_Y: f64 = 0.0;

/// Default `z` rotation angle for the [`RotatePoint`](struct.RotatePoint.html)
/// noise module.
pub const DEFAULT_ROTATE_Z: f64 = 0.0;

/// Noise module that rotates the input value around the origin before returning
/// the output value from a source module.
///
/// The [`get_value()`](struct.RotatePoint.html#method.get_value) method rotates
/// the coordinates of the input value around the origin before returning the
/// output value from the source module.  To set the rotation angles, call the
/// [`set_angles()`](struct.RotatePoint.html#method.set_angles) method.  To set
/// the rotation angle around the individual `x`, `y`, or `z` axes, call the
/// [`set_x_angle()`](struct.RotatePoint.html#method.set_x_angle),
/// [`set_y_angle()`](struct.RotatePoint.html#method.set_y_angle) or
/// [`set_z_angle()`](struct.RotatePoint.html#method.set_z_angle) methods,
/// respectively.
///
/// The coordinate system of the input value is assumed to be "left-handed" (`x`
/// increases to the right, `y` increases upward, and `z` increases inward.)
///
/// This noise module requires one source module.
pub struct RotatePoint<M: Module> {
    module: M,
    /// `x`, `y` and `z` rotation angle applied to the input value, in degrees.
    angles: (f64, f64, f64),
    /// The 3x3 rotation matrix used for rotating the input value.
    matrix: [[f64; 3]; 3],
}

impl<M: Module> RotatePoint<M> {
    /// Create a new `RotatePoint` noise module around the specified module,
    /// using default parameters.
    pub fn new(module: M) -> RotatePoint<M> {
        let mut tmp = RotatePoint {
            module: module,
            angles: (DEFAULT_ROTATE_X, DEFAULT_ROTATE_Y, DEFAULT_ROTATE_Z),
            matrix: [[0.0; 3]; 3],
        };
        tmp.update_matrix();
        tmp
    }

    /// Returns a reference to the source module used.
    pub fn module(&self) -> &M {
        &self.module
    }

    /// Returns a mutable reference to the source module used.
    pub fn module_mut(&mut self) -> &mut M {
        &mut self.module
    }

    /// Returns the rotation angle around the `x` axis to apply to the input
    /// value (in degrees).
    pub fn x_angle(&self) -> f64 {
        self.angles.0
    }

    /// Returns the rotation angle around the `y` axis to apply to the input
    /// value (in degrees).
    pub fn y_angle(&self) -> f64 {
        self.angles.1
    }

    /// Returns the rotation angle around the `z` axis to apply to the input
    /// value (in degrees).
    pub fn z_angle(&self) -> f64 {
        self.angles.2
    }

    /// Set the source module to be used.
    pub fn set_module(&mut self, module: M) {
        self.module = module;
    }

    /// Sets the rotation angles around all three axes to apply to the
    /// input value.
    ///
    /// The [`get_value()`](struct.RotatePoint.html#method.get_value) method
    /// rotates the coordinates of the input value around the origin before
    /// returning the output value from the source module.
    pub fn set_angles(&mut self, x: f64, y: f64, z: f64) {
        self.angles = (x, y, z);
        self.update_matrix();
    }

    /// Sets the rotation angle around the `x` axis to apply to the input value.
    ///
    /// The [`get_value()`](struct.RotatePoint.html#method.get_value) method
    /// rotates the coordinates of the input value around the origin before
    /// returning the output value from the source module.
    pub fn set_x_angle(&mut self, x: f64) {
        self.angles.0 = x;
        self.update_matrix();
    }

    /// Sets the rotation angle around the `y` axis to apply to the input value.
    ///
    /// The [`get_value()`](struct.RotatePoint.html#method.get_value) method
    /// rotates the coordinates of the input value around the origin before
    /// returning the output value from the source module.
    pub fn set_y_angle(&mut self, y: f64) {
        self.angles.0 = y;
        self.update_matrix();
    }

    /// Sets the rotation angle around the `z` axis to apply to the input value.
    ///
    /// The [`get_value()`](struct.RotatePoint.html#method.get_value) method
    /// rotates the coordinates of the input value around the origin before
    /// returning the output value from the source module.
    pub fn set_z_angle(&mut self, z: f64) {
        self.angles.0 = z;
        self.update_matrix();
    }

    /// Updates the rotation matrix after the angles have been changed.
    fn update_matrix(&mut self) {
        let (x_sin, x_cos) = f64::sin_cos(self.angles.0.to_radians());
        let (y_sin, y_cos) = f64::sin_cos(self.angles.1.to_radians());
        let (z_sin, z_cos) = f64::sin_cos(self.angles.2.to_radians());

        self.matrix[0][0] = y_sin * x_sin * z_sin + y_cos * z_cos;
        self.matrix[0][1] = x_cos * z_sin;
        self.matrix[0][2] = y_sin * z_cos - y_cos * x_sin * z_sin;
        self.matrix[1][0] = y_sin * x_sin * z_cos - y_cos * z_sin;
        self.matrix[1][1] = x_cos * z_cos;
        self.matrix[1][2] = -y_cos * x_sin * z_cos - y_sin * z_sin;
        self.matrix[2][0] = -y_sin * x_cos;
        self.matrix[2][1] = x_sin;
        self.matrix[2][2] = y_cos * x_cos;
    }
}

impl<M: Module> Module for RotatePoint<M> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let nx = self.matrix[0][0] * x + self.matrix[0][1] * y + self.matrix[0][2] * z;
        let ny = self.matrix[1][0] * x + self.matrix[1][1] * y + self.matrix[1][2] * z;
        let nz = self.matrix[2][0] * x + self.matrix[2][1] * y + self.matrix[2][2] * z;
        self.module.get_value(nx, ny, nz)
    }
}

impl<M: Module + Clone> Clone for RotatePoint<M> {
    fn clone(&self) -> RotatePoint<M> {
        RotatePoint {
            module: self.module.clone(),
            angles: self.angles,
            matrix: self.matrix,
        }
    }
}
