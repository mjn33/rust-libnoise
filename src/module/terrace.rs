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
use util::{clamp, linear_interp};

/// Noise module that maps the output value from a source module onto a
/// terrace-forming curve.
///
/// This noise module maps the output value from the source module onto a
/// terrace-forming curve.  The start of this curve has a slope of zero; its
/// slope then smoothly increases.  This curve also contains *control points*
/// which resets the slope to zero at that point, producing a "terracing"
/// effect.
///
/// To add a control point to this noise module, call the
/// [`add_control_point()`](struct.Terrace.html#method.add_control_point)
/// method.
///
/// An application must add a minimum of two control points to the curve.  If
/// this is not done, the [`get_value()`](struct.Terrace.html#method.get_value)
/// method fails.  The control points can have any value, although no two
/// control points can have the same value.  There is no limit to the number of
/// control points that can be added to the curve.
///
/// This noise module clamps the output value from the source module if that
/// value is less than the value of the lowest control point or greater than the
/// value of the highest control point.
///
/// This noise module is often used to generate terrain features such as your
/// stereotypical desert canyon.
///
/// This noise module requires one source module.
pub struct Terrace<M: Module> {
    module: M,
    invert_terraces: bool,
    control_points: Vec<f64>,
}

impl<M: Module> Terrace<M> {
    /// Create a new `Terrace` noise module around the specified module, using
    /// default parameters.
    pub fn new(module: M) -> Terrace<M> {
        Terrace {
            module: module,
            invert_terraces: false,
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

    /// Determines if the terrace-forming curve between the control
    /// points is inverted.
    ///
    /// Returns `true` if the curve between the control points is inverted.
    /// Otherwise `false` if the curve between the control points is not
    /// inverted.
    pub fn invert_terraces(&self) -> bool {
        self.invert_terraces
    }

    /// Returns a slice of all the control points, in order.
    pub fn control_points(&self) -> &[f64] {
        &self.control_points
    }

    /// Set the source module to be used.
    pub fn set_module(&mut self, module: M) {
        self.module = module;
    }

    /// Enables or disables the inversion of the terrace-forming curve between
    /// the control points.
    pub fn set_invert_terraces(&mut self, invert: bool) {
        self.invert_terraces = invert;
    }

    /// Adds a control point to the terrace-forming curve.
    ///
    /// Two or more control points define the terrace-forming curve.  The start
    /// of this curve has a slope of zero; its slope then smoothly increases.
    /// At the control points, its slope resets to zero.
    ///
    /// It does not matter which order these points are added.
    ///
    /// # Panics
    ///
    /// Panics if `value` is NaN, or if the given `value` has already been
    /// added.
    pub fn add_control_point(&mut self, value: f64) {
        if value.is_nan() {
            // With this check the `unwrap()` in the binary search should always
            // succeed.
            panic!("Tried to insert NaN value!");
        }
        let f = |x: &f64| x.partial_cmp(&value).unwrap();
        match self.control_points.binary_search_by(f) {
            Ok(_) => {
                panic!("Control point with given value already exists!");
            },
            Err(idx) => {
                self.control_points.insert(idx, value);
            }
        }
    }

    /// Creates a number of equally-spaced control points that range from -1 to
    /// +1. The previous control points on the terrace-forming curve are
    /// deleted.
    ///
    /// Two or more control points define the terrace-forming curve.  The start
    /// of this curve has a slope of zero; its slope then smoothly increases.
    /// At the control points, its slope resets to zero.
    ///
    /// # Panics
    ///
    /// Panics if `count` is less than 2.
    pub fn make_control_points(&mut self, count: i32) {
        if count < 2 {
            panic!("The number of control points must be greater than or equal to 2!");
        }

        self.control_points.clear();

        let terrace_step = 2.0 / (count as f64 - 1.0);
        let mut cur_value = -1.0;
        for _ in 0..count {
            self.control_points.push(cur_value);
            cur_value += terrace_step;
        }
    }
}

impl<M: Module> Module for Terrace<M> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        if self.control_points.len() < 2 {
            panic!("Fewer than 2 control points on curve!");
        }

        // Get the output value from the source module.
        let source_value = self.module.get_value(x, y, z);

        // Find the first element in the control point array that has a value
        // larger than the output value from the source module.
        let f = |x: &f64| x.partial_cmp(&source_value).unwrap();
        let idx_pos = match self.control_points.binary_search_by(f) {
            Ok(idx) => idx as isize + 1,
            Err(idx) => idx as isize,
        };

        // Find the two nearest control points so that we can map their values
        // onto a quadratic curve.
        let idx0 = clamp(idx_pos - 1,
                         0, self.control_points.len() as isize - 1) as usize;
        let idx1 = clamp(idx_pos,
                         0, self.control_points.len() as isize - 1) as usize;

        // If some control points are missing (which occurs if the output value
        // from the source module is greater than the largest value or less than
        // the smallest value of the control point array), get the value of the
        // nearest control point and exit now.
        if idx0 == idx1 {
            return self.control_points[idx1];
        }

        // Compute the alpha value used for linear interpolation.
        let value0 = self.control_points[idx0];
        let value1 = self.control_points[idx1];
        let alpha = (source_value - value0) / (value1 - value0);
        let (value0, value1, alpha) = if self.invert_terraces {
            (value1, value0, 1.0 - alpha)
        } else {
            (value0, value1, alpha)
        };

        // Squaring the alpha produces the terrace effect.
        let alpha = alpha * alpha;

        // Now perform the linear interpolation given the alpha value.
        linear_interp(value0, value1, alpha)
    }
}

impl<M: Module + Clone> Clone for Terrace<M> {
    fn clone(&self) -> Terrace<M> {
        Terrace {
            module: self.module.clone(),
            invert_terraces: self.invert_terraces,
            control_points: self.control_points.clone(),
        }
    }
}
