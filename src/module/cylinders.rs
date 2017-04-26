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

/// Default frequency value for the [`Cylinders`](struct.Cylinders.html) noise
/// module.
pub const DEFAULT_CYLINDERS_FREQUENCY: f64 = 1.0;

/// Noise module that outputs concentric cylinders.
///
/// This noise module outputs concentric cylinders centered on the origin.
/// These cylinders are oriented along the `y` axis similar to the concentric
/// rings of a tree.  Each cylinder extends infinitely along the `y` axis.
///
/// The first cylinder has a radius of 1.0.  Each subsequent cylinder has
/// a radius that is 1.0 unit larger than the previous cylinder.
///
/// The output value from this noise module is determined by the distance
/// between the input value and the the nearest cylinder surface.  The input
/// values that are located on a cylinder surface are given the output value 1.0
/// and the input values that are equidistant from two cylinder surfaces are
/// given the output value -1.0.
///
/// An application can change the frequency of the concentric cylinders.
/// Increasing the frequency reduces the distances between cylinders.  To
/// specify the frequency, call the
/// [`set_frequency()`](struct.Cylinders.html#method.set_frequency) method.
///
/// This noise module, modified with some low-frequency, low-power turbulence,
/// is useful for generating wood-like textures.
///
/// This noise module does not require any source modules.
#[derive(Clone)]
pub struct Cylinders {
    frequency: f64,
}

impl Default for Cylinders {
    /// Create a new `Cylinders` noise module with default parameters.
    fn default() -> Cylinders {
        Cylinders {
            frequency: DEFAULT_CYLINDERS_FREQUENCY,
        }
    }
}

impl Cylinders {
    /// Create a new `Cylinders` noise module with default parameters.
    pub fn new() -> Cylinders {
        Default::default()
    }

    /// Returns the frequency of the concentric cylinders.
    ///
    /// Increasing the frequency increases the density of the concentric
    /// cylinders, reducing the distances between them.
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Sets the frequenct of the concentric cylinders.
    ///
    /// Increasing the frequency increases the density of the concentric
    /// cylinders, reducing the distances between them.
    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
    }
}

impl Module for Cylinders {
    fn get_value(&self, x: f64, _y: f64, z: f64) -> f64 {
        let x = x * self.frequency;
        let z = z * self.frequency;

        let dist_from_centre = (x * x + z * z).sqrt();
        let dist_from_smaller_sphere = dist_from_centre - dist_from_centre.floor();
        let dist_from_larger_sphere = 1.0 - dist_from_smaller_sphere;
        let nearest_dist = f64::min(dist_from_smaller_sphere, dist_from_larger_sphere);

        1.0 - nearest_dist * 4.0 // Puts it in the -1.0 to +1.0 range.
    }
}
