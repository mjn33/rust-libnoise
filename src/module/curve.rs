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
use util::{clamp, cubic_interp};

/// This structure defines a control point.
///
/// Control points are used for defining splines.
#[derive(Copy, Clone)]
pub struct ControlPoint {
    pub input_value: f64,
    pub output_value: f64,
}

/// Noise module that maps the output value from a source module onto an
/// arbitrary function curve.
///
/// This noise module maps the output value from the source module onto an
/// application-defined curve.  This curve is defined by a number of *control
/// points*; each control point has an *input value* that maps to an *output
/// value*.
///
/// To add the control points to this curve, call the
/// [`add_control_point()`](struct.Curve.html#method.add_control_point) method.
///
/// Since this curve is a cubic spline, an application must add a minimum of
/// four control points to the curve.  If this is not done, the
/// [`get_value()`](struct.Curve.html#method.get_value) method panics.  Each
/// control point can have any input and output value, although no two control
/// points can have the same input value.  There is no limit to the number of
/// control points that can be added to the curve.
///
/// This noise module requires one source module.
pub struct Curve<M: Module> {
    module: M,
    control_points: Vec<ControlPoint>,
}

impl<M: Module> Curve<M> {
    /// Create a new `Curve` noise module around the specified module.
    pub fn new(module: M) -> Curve<M> {
        Curve {
            module: module,
            control_points: Vec::new(),
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

    /// Adds a control point to the curve.
    ///
    /// It does not matter which order these points are added.
    ///
    /// # Panics
    ///
    /// Panics if either `input_value` or `output_value` are NaN, or if the
    /// given `input_value` has already been added onto the `Curve`.
    pub fn add_control_point(&mut self, input_value: f64, output_value: f64) {
        if input_value.is_nan() || output_value.is_nan() {
            // With this check the `unwrap()` in the binary search should always
            // succeed.
            panic!("Tried to insert NaN input_value or output_value!");
        }
        let f = |x: &ControlPoint| x.input_value.partial_cmp(&input_value).unwrap();
        match self.control_points.binary_search_by(f) {
            Ok(_) => {
                panic!("Control point with given input value already exists!");
            },
            Err(idx) => {
                self.control_points.insert(idx, ControlPoint {
                    input_value: input_value,
                    output_value: output_value
                });
            }
        }
    }

    /// Deletes all the control points on the curve.
    pub fn clear_control_points(&mut self) {
        self.control_points.clear();
    }

    /// Returns a slice of all the control points on the curve, in order.
    pub fn control_points(&self) -> &[ControlPoint] {
        &self.control_points
    }
}

impl<M: Module> Module for Curve<M> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        if self.control_points.len() < 4 {
            panic!("Fewer than 4 control points on curve!");
        }

        // Get the output value from the source module.
        let source_value = self.module.get_value(x, y, z);

        // Find the first element in the control point array that has an input value
        // larger than the output value from the source module.
        let f = |x: &ControlPoint| x.input_value.partial_cmp(&source_value).unwrap();
        let idx_pos = match self.control_points.binary_search_by(f) {
            Ok(idx) => idx as isize + 1,
            Err(idx) => idx as isize,
        };

        // Find the four nearest control points so that we can perform cubic
        // interpolation.
        let idx0 = clamp(idx_pos - 2,
                         0, self.control_points.len() as isize - 1) as usize;
        let idx1 = clamp(idx_pos - 1,
                         0, self.control_points.len() as isize - 1) as usize;
        let idx2 = clamp(idx_pos,
                         0, self.control_points.len() as isize - 1) as usize;
        let idx3 = clamp(idx_pos + 1,
                         0, self.control_points.len() as isize - 1) as usize;

        // If some control points are missing (which occurs if the value from the
        // source module is greater than the largest input value or less than the
        // smallest input value of the control point array), get the corresponding
        // output value of the nearest control point and exit now.
        if idx1 == idx2 {
            return self.control_points[idx1].output_value
        }

        // Compute the alpha value used for cubic interpolation.
        let input0 = self.control_points[idx1].input_value;
        let input1 = self.control_points[idx2].input_value;
        let alpha = (source_value - input0) / (input1 - input0);

        // Now perform the cubic interpolation given the alpha value.
        cubic_interp(
            self.control_points[idx0].output_value,
            self.control_points[idx1].output_value,
            self.control_points[idx2].output_value,
            self.control_points[idx3].output_value,
            alpha)
    }
}

impl<M: Module + Clone> Clone for Curve<M> {
    fn clone(&self) -> Curve<M> {
        Curve {
            module: self.module.clone(),
            control_points: self.control_points.clone(),
        }
    }
}
