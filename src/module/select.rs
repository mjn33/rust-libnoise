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
use util::{linear_interp, scurve3};

/// Default edge-falloff value for the [`Select`](struct.Select.html) noise
/// module.
pub const DEFAULT_SELECT_EDGE_FALLOFF: f64 = 0.0;

/// Default lower bound of the selection range for the
/// [`Select`](struct.Select.html) noise module.
pub const DEFAULT_SELECT_LOWER_BOUND: f64 = -1.0;

/// Default upper bound of the selection range for the
/// [`Select`](struct.Select.html) noise module.
pub const DEFAULT_SELECT_UPPER_BOUND: f64 = 1.0;

/// Noise module that outputs the value selected from one of two source modules
/// chosen by the output value from a control module.
///
/// To specify the bounds of the selection range, call the
/// [`set_bounds()`](struct.Select.html#method.set_bounds) method.
///
/// By default, there is an abrupt transition between the output values from the
/// two source modules at the selection-range boundary.  To smooth the
/// transition, pass a non-zero value to the
/// [`set_edge_falloff()`](struct.Select.html#method.set_edge_falloff) method.
/// Higher values result in a smoother transition.
///
/// This noise module requires three source modules.
pub struct Select<M1: Module, M2: Module, MC: Module> {
    module1: M1,
    module2: M2,
    mcontrol: MC,
    edge_falloff: f64,
    lower_bound: f64,
    upper_bound: f64,
}

impl<M1: Module, M2: Module, MC: Module> Select<M1, M2, MC> {
    /// Create a new `Select` noise module around the specified modules, using
    /// default parameters.
    pub fn new(module1: M1, module2: M2, control: MC) -> Select<M1, M2, MC> {
        Select {
            module1: module1,
            module2: module2,
            mcontrol: control,
            edge_falloff: DEFAULT_SELECT_EDGE_FALLOFF,
            lower_bound: DEFAULT_SELECT_LOWER_BOUND,
            upper_bound: DEFAULT_SELECT_UPPER_BOUND,
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

    /// Returns a reference to the control module.
    ///
    /// The control module determines the output value to select.  If the output
    /// value from the control module is within a range of values known as the
    /// *selection range*, the
    /// [`get_value()`](struct.Select.html#method.get_value) method outputs the
    /// value from the second source module.  Otherwise, this method outputs the
    /// value from the first source module.
    pub fn control_module(&self) -> &MC {
        &self.mcontrol
    }

    /// Returns a mutable reference to the control module.
    pub fn control_module_mut(&mut self) -> &mut MC {
        &mut self.mcontrol
    }

    /// Returns the falloff value at the edge transition.
    ///
    /// The falloff value is the width of the edge transition at either edge of
    /// the selection range.
    ///
    /// By default, there is an abrupt transition between the output values from
    /// the two source modules at the selection-range boundary.
    pub fn edge_falloff(&self) -> f64 {
        self.edge_falloff
    }

    /// Returns the lower bound of the selection range.
    ///
    /// If the output value from the control module is within a range of values
    /// known as the *selection range*, the
    /// [`get_value()`](struct.Select.html#method.get_value) method outputs the
    /// value from the second source module.  Otherwise, this method outputs the
    /// value from the first source module.
    pub fn lower_bound(&self) -> f64 {
        self.lower_bound
    }

    /// Returns the upper bound of the selection range.
    ///
    /// If the output value from the control module is within a range of values
    /// known as the *selection range*, the
    /// [`get_value()`](struct.Select.html#method.get_value) method outputs the
    /// value from the second source module.  Otherwise, this method outputs the
    /// value from the first source module.
    pub fn upper_bound(&self) -> f64 {
        self.upper_bound
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
    /// The control module determines the output value to select.  If the output
    /// value from the control module is within a range of values known as the
    /// *selection range*, the
    /// [`get_value()`](struct.Select.html#method.get_value) method outputs the
    /// value from the second source module.  Otherwise, this method outputs the
    /// value from the first source module.
    pub fn set_control_module(&mut self, control: MC) {
        self.mcontrol = control;
    }

    /// Sets the falloff value at the edge transition.
    ///
    /// The falloff value is the width of the edge transition at either edge of
    /// the selection range.
    ///
    /// By default, there is an abrupt transition between the values from the
    /// two source modules at the boundaries of the selection range.
    ///
    /// For example, if the selection range is 0.5 to 0.8, and the edge falloff
    /// value is 0.1, then the
    /// [`get_value()`](struct.Select.html#method.get_value) method outputs:
    ///
    ///   * the output value from the source module with an index value of 0
    ///     if the output value from the control module is less than 0.4
    ///     (= 0.5 - 0.1).
    ///   * a linear blend between the two output values from the two source
    ///     modules if the output value from the control module is between
    ///     0.4 (= 0.5 - 0.1) and 0.6 (= 0.5 + 0.1).
    ///   * the output value from the source module with an index value of 1
    ///     if the output value from the control module is between 0.6
    ///     (= 0.5 + 0.1) and 0.7 (= 0.8 - 0.1).
    ///   * a linear blend between the output values from the two source
    ///     modules if the output value from the control module is between
    ///     0.7 (= 0.8 - 0.1) and 0.9 (= 0.8 + 0.1).
    ///   * the output value from the source module with an index value of 0
    ///     if the output value from the control module is greater than 0.9
    ///     (= 0.8 + 0.1).
    pub fn set_edge_falloff(&mut self, edge_falloff: f64) {
        self.edge_falloff = edge_falloff;
        self.clamp_falloff();
    }

    /// Sets the lower and upper bounds of the selection range.
    ///
    /// If the output value from the control module is within a range of values
    /// known as the *selection range*, the
    /// [`get_value()`](struct.Select.html#method.get_value) method outputs the
    /// value from the second source module.  Otherwise, this method outputs the
    /// value from the first source module.
    ///
    /// # Panics
    ///
    /// Panics if the given lower bound is greater than the given upper bound.
    pub fn set_bounds(&mut self, lower_bound: f64, upper_bound: f64) {
        if lower_bound > upper_bound {
            panic!("Lower bound is larger than upper bound!");
        }
        self.lower_bound = lower_bound;
        self.upper_bound = upper_bound;
        self.clamp_falloff();
    }

    /// Makes sure that the edge falloff curves do not overlap.
    fn clamp_falloff(&mut self) {
        let bound_size = self.upper_bound - self.lower_bound;
        if bound_size / 2.0 < self.edge_falloff {
            self.edge_falloff = bound_size / 2.0;
        }
    }
}

impl<M1: Module, M2: Module, MC: Module> Module for Select<M1, M2, MC> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let control_value = self.mcontrol.get_value(x, y, z);
        if self.edge_falloff > 0.0 {
            if control_value < self.lower_bound - self.edge_falloff {
                // The output value from the control module is below the
                // selector threshold; return the output value from the first
                // source module.
                self.module1.get_value(x, y, z)
            } else if control_value < self.lower_bound + self.edge_falloff {
                // The output value from the control module is near the lower
                // end of the selector threshold and within the smooth
                // curve. Interpolate between the output values from the first
                // and second source modules.
                let lower_curve = self.lower_bound - self.edge_falloff;
                let upper_curve = self.lower_bound + self.edge_falloff;
                let alpha = scurve3((control_value - lower_curve) / (upper_curve - lower_curve));
                linear_interp(self.module1.get_value(x, y, z),
                              self.module2.get_value(x, y, z),
                              alpha)
            } else if control_value < self.upper_bound - self.edge_falloff {
                // The output value from the control module is within the
                // selector threshold; return the output value from the second
                // source module.
                self.module2.get_value(x, y, z)
            } else if control_value < self.upper_bound + self.edge_falloff {
                // The output value from the control module is near the upper
                // end of the selector threshold and within the smooth
                // curve. Interpolate between the output values from the first
                // and second source modules.
                let lower_curve = self.upper_bound - self.edge_falloff;
                let upper_curve = self.upper_bound + self.edge_falloff;
                let alpha = scurve3((control_value - lower_curve) / (upper_curve - lower_curve));
                linear_interp(self.module2.get_value(x, y, z),
                              self.module1.get_value(x, y, z),
                              alpha)
            } else {
                // Output value from the control module is above the selector threshold;
                // return the output value from the first source module.
                self.module1.get_value(x, y, z)
            }
        } else {
            if control_value < self.lower_bound || control_value > self.upper_bound {
                self.module1.get_value(x, y, z)
            } else {
                self.module2.get_value(x, y, z)
            }
        }
    }
}

impl<M1: Module + Clone,
     M2: Module + Clone,
     MC: Module + Clone> Clone for Select<M1, M2, MC> {
    fn clone(&self) -> Select<M1, M2, MC> {
        Select {
            module1: self.module1.clone(),
            module2: self.module2.clone(),
            mcontrol: self.mcontrol.clone(),
            edge_falloff: self.edge_falloff,
            lower_bound: self.lower_bound,
            upper_bound: self.upper_bound,
        }
    }
}
