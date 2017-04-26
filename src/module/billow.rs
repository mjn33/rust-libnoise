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
use noisegen::{gradient_coherent_noise3d, make_i32_range, NoiseQuality};

/// Default frequency for the [`Billow`](struct.Billow.html) noise module.
pub const DEFAULT_BILLOW_FREQUENCY: f64 = 1.0;

/// Default lacunarity for the the [`Billow`](struct.Billow.html) noise module.
pub const DEFAULT_BILLOW_LACUNARITY: f64 = 2.0;

/// Default number of octaves for the the [`Billow`](struct.Billow.html) noise
/// module.
pub const DEFAULT_BILLOW_OCTAVE_COUNT: i32 = 6;

/// Default persistence value for the the [`Billow`](struct.Billow.html) noise
/// module.
pub const DEFAULT_BILLOW_PERSISTENCE: f64 = 0.5;

/// Default noise quality for the the [`Billow`](struct.Billow.html) noise
/// module.
pub const DEFAULT_BILLOW_QUALITY: NoiseQuality = NoiseQuality::Standard;

/// Default noise seed for the the [`Billow`](struct.Billow.html) noise module.
pub const DEFAULT_BILLOW_SEED: i32 = 0;

/// Maximum number of octaves for the the [`Billow`](struct.Billow.html) noise
/// module.
pub const BILLOW_MAX_OCTAVE: i32 = 30;

/// Noise module that outputs three-dimensional "billowy" noise.
///
/// This noise module generates "billowy" noise suitable for clouds and
/// rocks.
///
/// This noise module is nearly identical to
/// [`Perlin`](../perlin/struct.Perlin.html) except this noise module modifies
/// each octave with an absolute-value function.  See the documentation of
/// `Perlin` for more information.
#[derive(Clone)]
pub struct Billow {
    frequency: f64,
    lacunarity: f64,
    quality: NoiseQuality,
    octave_count: i32,
    persistence: f64,
    seed: i32,
}

impl Default for Billow {
    /// Create a new `Billow` noise module with default parameters.
    fn default() -> Billow {
        Billow {
            frequency: DEFAULT_BILLOW_FREQUENCY,
            lacunarity: DEFAULT_BILLOW_LACUNARITY,
            quality: DEFAULT_BILLOW_QUALITY,
            octave_count: DEFAULT_BILLOW_OCTAVE_COUNT,
            persistence: DEFAULT_BILLOW_PERSISTENCE,
            seed: DEFAULT_BILLOW_SEED,
        }
    }
}

impl Billow {
    /// Create a new `Billow` noise module with default parameters.
    pub fn new() -> Billow {
        Default::default()
    }

    /// Returns the frequency of the first octave.
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Returns the lacunarity of the billowy noise.
    ///
    /// The lacunarity is the frequency multiplier between successive octaves.
    pub fn lacunarity(&self) -> f64 {
        self.lacunarity
    }

    /// Returns the quality of the billowy noise.
    ///
    /// See [`NoiseQuality`](../../noisegen/enum.NoiseQuality.html) for
    /// definitions of the various coherent-noise qualities.
    pub fn quality(&self) -> NoiseQuality {
        self.quality
    }

    /// Returns the number of octaves that generate the billowy noise.
    ///
    /// The number of octaves controls the amount of detail in the billowy noise.
    pub fn octave_count(&self) -> i32 {
        self.octave_count
    }

    /// Returns the persistence value of the billowy noise.
    ///
    /// The persistence value controls the roughness of the billowy noise.
    pub fn persistence(&self) -> f64 {
        self.persistence
    }

    /// Returns the seed value used by the billowy-noise function.
    pub fn seed(&self) -> i32 {
        self.seed
    }

    /// Sets the frequency of the first octave.
    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
    }

    /// Sets the lacunarity of the billowy noise.
    ///
    /// The lacunarity is the frequency multiplier between successive octaves.
    ///
    /// For best results, set the lacunarity to a number between 1.5 and 3.5.
    pub fn set_lacunarity(&mut self, lacunarity: f64) {
        self.lacunarity = lacunarity;
    }

    /// Sets the quality of the billowy noise.
    ///
    /// See [`NoiseQuality`](../../noisegen/enum.NoiseQuality.html) for
    /// definitions of the various coherent-noise qualities.
    pub fn set_quality(&mut self, quality: NoiseQuality) {
        self.quality = quality;
    }

    /// Sets the number of octaves that generate the billowy noise.
    ///
    /// The number of octaves controls the amount of detail in the billowy
    /// noise.
    ///
    /// The larger the number of octaves, the more time required to
    /// calculate the billowy-noise value.
    ///
    /// # Panics
    ///
    /// Panics if the given octave count is outside the range from 1 to
    /// [`BILLOW_MAX_OCTAVE`](constant.BILLOW_MAX_OCTAVE.html) inclusive.
    pub fn set_octave_count(&mut self, octave_count: i32) {
        if octave_count < 1 || octave_count > BILLOW_MAX_OCTAVE {
            panic!("`octave_count` must be in the range [{}, {}]", 1, BILLOW_MAX_OCTAVE);
        }
        self.octave_count = octave_count;
    }

    /// Sets the persistence value of the billowy noise.
    ///
    /// The persistence value controls the roughness of the billowy noise.
    ///
    /// For best results, set the persistence to a number between 0.0 and 1.0.
    pub fn set_persistence(&mut self, persistence: f64) {
        self.persistence = persistence;
    }

    /// Sets the seed value used by the billowy-noise function.
    pub fn set_seed(&mut self, seed: i32) {
        self.seed = seed;
    }
}

impl Module for Billow {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut value = 0.0;
        let mut cur_persistence = 1.0;
        let mut x = x * self.frequency;
        let mut y = y * self.frequency;
        let mut z = z * self.frequency;

        for cur_octave in 0..self.octave_count {
            // Make sure that these floating-point values have the same range as
            // a 32-bit integer so that we can pass them to the coherent-noise
            // functions.
            let nx = make_i32_range(x);
            let ny = make_i32_range(y);
            let nz = make_i32_range(z);

            // Get the coherent-noise value from the input value and add it to
            // the final result.
            let seed = self.seed + cur_octave;
            let signal = gradient_coherent_noise3d(nx, ny, nz, seed, self.quality);
            let signal = 2.0 * signal.abs() - 1.0;
            value += signal * cur_persistence;

            // Prepare the next octave.
            x *= self.lacunarity;
            y *= self.lacunarity;
            z *= self.lacunarity;
            cur_persistence *= self.persistence;
        }
        value += 0.5;

        value
    }
}
