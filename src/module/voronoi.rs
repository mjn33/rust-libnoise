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

use consts;
use module::Module;
use noisegen::{value_noise3d};

/// Default displacement to apply to each cell for the
/// [`Voronoi`](struct.Voronoi.html) noise module.
pub const DEFAULT_VORONOI_DISPLACEMENT: f64 = 1.0;

/// Default frequency of the seed points for the
/// [`Voronoi`](struct.Voronoi.html) noise module.
pub const DEFAULT_VORONOI_FREQUENCY: f64 = 1.0;

/// Default seed of the noise function for the [`Voronoi`](struct.Voronoi.html)
/// noise module.
pub const DEFAULT_VORONOI_SEED: i32 = 0;

/// Noise module that outputs Voronoi cells.
///
/// In mathematics, a *Voronoi cell* is a region containing all the points that
/// are closer to a specific *seed point* than to any other seed point.  These
/// cells mesh with one another, producing polygon-like formations.
///
/// By default, this noise module randomly places a seed point within each unit
/// cube.  By modifying the *frequency* of the seed points, an application can
/// change the distance between seed points.  The higher the frequency, the
/// closer together this noise module places the seed points, which reduces the
/// size of the cells.  To specify the frequency of the cells, call the
/// [`set_frequency()`](struct.Voronoi.html#method.set_frequency) method.
///
/// This noise module assigns each Voronoi cell with a random constant value
/// from a coherent-noise function.  The *displacement value* controls the range
/// of random values to assign to each cell.  The range of random values is +/-
/// the displacement value.  Call the
/// [`set_displacement()`](struct.Voronoi.html#method.set_displacement) method
/// to specify the displacement value.
///
/// To modify the random positions of the seed points, call the
/// [`set_seed()`](struct.Voronoi.html#method.set_seed) method.
///
/// This noise module can optionally add the distance from the nearest seed to
/// the output value.  To enable this feature, call the
/// [`enable_distance()`](struct.Voronoi.html#method.enable_distance) method.
/// This causes the points in the Voronoi cells to increase in value the further
/// away that point is from the nearest seed point.
///
/// Voronoi cells are often used to generate cracked-mud terrain formations or
/// crystal-like textures
///
/// This noise module requires no source modules.
#[derive(Clone)]
pub struct Voronoi {
    /// Scale of the random displacement to apply to each Voronoi cell.
    displacement: f64,
    enable_distance: bool,
    frequency: f64,
    seed: i32,
}

impl Default for Voronoi {
    /// Create a new `Voronoi` noise module with default parameters.
    fn default() -> Voronoi {
        Voronoi {
            displacement: DEFAULT_VORONOI_DISPLACEMENT,
            enable_distance: false,
            frequency: DEFAULT_VORONOI_FREQUENCY,
            seed: DEFAULT_VORONOI_SEED,
        }
    }
}

impl Voronoi {
    /// Create a new `Voronoi` noise module with default parameters.
    pub fn new() -> Voronoi {
        Default::default()
    }

    /// Determines if the distance from the nearest seed point is applied
    /// to the output value.
    ///
    /// Returns `true` if the distance is applied to the output value, otherwise
    /// `false` if not.
    ///
    /// Applying the distance from the nearest seed point to the output value
    /// causes the points in the Voronoi cells to increase in value the further
    /// away that point is from the nearest seed point.
    pub fn is_distance_enabled(&self) -> bool {
        self.enable_distance
    }

    /// Returns the displacement value of the Voronoi cells.
    ///
    /// This noise module assigns each Voronoi cell with a random constant value
    /// from a coherent-noise function.  The *displacement value* controls the
    /// range of random values to assign to each cell.  The range of random
    /// values is +/- the displacement value.
    pub fn displacement(&self) -> f64 {
        self.displacement
    }

    /// Returns the frequency of the seed points.
    ///
    /// The frequency determines the size of the Voronoi cells and the distance
    /// between these cells.
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Returns the seed value used by the Voronoi cells
    ///
    /// The positions of the seed values are calculated by a coherent-noise
    /// function.  By modifying the seed value, the output of that function
    /// changes.
    pub fn seed(&self) -> i32 {
        self.seed
    }

    /// Enables or disables applying the distance from the nearest seed point to
    /// the output value.
    ///
    /// Applying the distance from the nearest seed point to the output value
    /// causes the points in the Voronoi cells to increase in value the further
    /// away that point is from the nearest seed point.  Setting this value to
    /// `true` (and setting the displacement to a near-zero value) causes this
    /// noise module to generate cracked mud formations.
    pub fn enable_distance(&mut self, enabled: bool) {
        self.enable_distance = enabled;
    }

    /// Sets the displacement value of the Voronoi cells.
    ///
    /// This noise module assigns each Voronoi cell with a random constant value
    /// from a coherent-noise function.  The *displacement value* controls the
    /// range of random values to assign to each cell.  The range of random
    /// values is +/- the displacement value.
    pub fn set_displacement(&mut self, displacement: f64) {
        self.displacement = displacement;
    }

    /// Sets the frequency of the seed points.
    ///
    /// The frequency determines the size of the Voronoi cells and the distance
    /// between these cells.
    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
    }

    /// Sets the seed value used by the Voronoi cells
    ///
    /// The positions of the seed values are calculated by a coherent-noise
    /// function.  By modifying the seed value, the output of that function
    /// changes.
    pub fn set_seed(&mut self, seed: i32) {
        self.seed = seed;
    }
}

impl Module for Voronoi {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        // This method could be more efficient by caching the seed values.  Fix
        // later.

        let x = x * self.frequency;
        let y = y * self.frequency;
        let z = z * self.frequency;

        let x_int = if x > 0.0 { x as i32 } else { (x - 1.0) as i32 };
        let y_int = if y > 0.0 { y as i32 } else { (y - 1.0) as i32 };
        let z_int = if z > 0.0 { z as i32 } else { (z - 1.0) as i32 };

        let mut min_dist = 2147483647.0;
        let mut x_candidate = 0.0;
        let mut y_candidate = 0.0;
        let mut z_candidate = 0.0;

        // Inside each unit cube, there is a seed point at a random position.
        // Go through each of the nearby cubes until we find a cube with a seed
        // point that is closest to the specified position.
        // FIXME: inclusive range syntax unstable, replace when something becomes stable
        for z_cur in (z_int - 2)..(z_int + 3) {
            for y_cur in (y_int - 2)..(y_int + 3) {
                for x_cur in (x_int - 2)..(x_int + 3) {
                    // Calculate the position and distance to the seed point
                    // inside of this unit cube.
                    let x_pos = x_cur as f64 + value_noise3d(x_cur, y_cur, z_cur, self.seed);
                    let y_pos = y_cur as f64 + value_noise3d(x_cur, y_cur, z_cur, self.seed + 1);
                    let z_pos = z_cur as f64 + value_noise3d(x_cur, y_cur, z_cur, self.seed + 2);
                    let x_dist = x_pos - x;
                    let y_dist = y_pos - y;
                    let z_dist = z_pos - z;
                    let dist = x_dist * x_dist + y_dist * y_dist + z_dist * z_dist;

                    if dist < min_dist {
                        // This seed point is closer to any others found so far,
                        // so record this seed point.
                        min_dist = dist;
                        x_candidate = x_pos;
                        y_candidate = y_pos;
                        z_candidate = z_pos;
                    }
                }
            }
        }

        let value = if self.enable_distance {
            // Determine the distance to the nearest seed point.
            let x_dist = x_candidate - x;
            let y_dist = y_candidate - y;
            let z_dist = z_candidate - z;
            (x_dist * x_dist + y_dist * y_dist + z_dist * z_dist).sqrt() * consts::SQRT_3 - 1.0
        } else {
            0.0
        };

        // Return the calculated distance with the displacement value applied.
        value + (self.displacement * value_noise3d(
            x_candidate.floor() as i32,
            y_candidate.floor() as i32,
            z_candidate.floor() as i32,
            0))
    }
}
