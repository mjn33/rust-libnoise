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
use noisegen::{gradient_coherent_noise3d, make_i32_range, NoiseQuality};

/// Default frequency for the [`RidgedMulti`](struct.RidgedMulti.html) noise
/// module.
pub const DEFAULT_RIDGED_FREQUENCY: f64 = 1.0;

/// Default lacunarity for the [`RidgedMulti`](struct.RidgedMulti.html) noise
/// module.
pub const DEFAULT_RIDGED_LACUNARITY: f64 = 2.0;

/// Default number of octaves for the [`RidgedMulti`](struct.RidgedMulti.html)
/// noise module.
pub const DEFAULT_RIDGED_OCTAVE_COUNT: i32 = 6;

/// Default noise quality for the [`RidgedMulti`](struct.RidgedMulti.html) noise
/// module.
pub const DEFAULT_RIDGED_QUALITY: NoiseQuality = NoiseQuality::Standard;

/// Default noise seed for the [`RidgedMulti`](struct.RidgedMulti.html) noise
/// module.
pub const DEFAULT_RIDGED_SEED: i32 = 0;

/// Maximum number of octaves for the [`RidgedMulti`](struct.RidgedMulti.html)
/// noise module.
pub const RIDGED_MAX_OCTAVE: i32 = 30;

/// Noise module that outputs 3-dimensional ridged-multifractal noise.
///
/// This noise module, heavily based on the Perlin-noise module, generates
/// ridged-multifractal noise.  Ridged-multifractal noise is generated in much
/// of the same way as Perlin noise, except the output of each octave is
/// modified by an absolute-value function.  Modifying the octave values in this
/// way produces ridge-like formations.
///
/// Ridged-multifractal noise does not use a persistence value.  This is because
/// the persistence values of the octaves are based on the values generated from
/// from previous octaves, creating a feedback loop (or that's what it looks
/// like after reading the code.)
///
/// This noise module outputs ridged-multifractal-noise values that usually
/// range from -1.0 to +1.0, but there are no guarantees that all output values
/// will exist within that range.
///
/// **Note:** For ridged-multifractal noise generated with only one octave, the
/// output value ranges from -1.0 to 0.0.
///
/// Ridged-multifractal noise is often used to generate craggy mountainous
/// terrain or marble-like textures.
///
/// This noise module does not require any source modules.
///
/// ## Octaves
///
/// The number of octaves control the *amount of detail* of the
/// ridged-multifractal noise.  Adding more octaves increases the detail of the
/// ridged-multifractal noise, but with the drawback of increasing the
/// calculation time.
///
/// An application may specify the number of octaves that generate
/// ridged-multifractal noise by calling the
/// [`set_octave_count()`](struct.RidgedMulti.html#method.set_octave_count)
/// method.
///
/// ## Frequency
///
/// An application may specify the frequency of the first octave by calling the
/// [`set_frequency()`](struct.RidgedMulti.html#method.set_frequency) method.
///
/// ## Lacunarity
///
/// The lacunarity specifies the frequency multipler between successive octaves.
///
/// The effect of modifying the lacunarity is subtle; you may need to play with
/// the lacunarity value to determine the effects.  For best results, set the
/// lacunarity to a number between 1.5 and 3.5.
///
/// ## References & Acknowledgments
///
/// [F. Kenton "Doc Mojo" Musgrave's texturing page](http://www.texturingandmodeling.com/Musgrave.html) -
/// This page contains links to source code that generates ridged-multfractal
/// noise, among other types of noise.
///
/// The source file [fractal.c](http://www.texturingandmodeling.com/CODE/MUSGRAVE/CLOUD/fractal.c)
/// contains the code I used in my ridged-multifractal class (see the
/// `RidgedMultifractal()` function.)  This code was written by F. Kenton
/// Musgrave, the person who created [MojoWorld](http://www.pandromeda.com).  He
/// is also one of the authors in *Texturing and Modeling: A Procedural
/// Approach* (Morgan Kaufmann, 2002. ISBN 1-55860-848-6.)
#[derive(Clone)]
pub struct RidgedMulti {
    frequency: f64,
    lacunarity: f64,
    quality: NoiseQuality,
    octave_count: i32,
    /// Contains the spectral weights for each octave.
    spectral_weights: [f64; RIDGED_MAX_OCTAVE as usize],
    seed: i32,
}

/// Calculates the spectral weights for each octave.
fn calc_spectral_weights(spectral_weights: &mut [f64], lacunarity: f64) {
    // This exponent parameter should be user-defined; it may be exposed in a
    // future version of libnoise.
    let h = 1.0;

    let mut frequency: f64 = 1.0;
    for w in spectral_weights {
        *w = frequency.powf(-h);
        frequency *= lacunarity;
    }
}

impl Default for RidgedMulti {
    /// Create a new `RidgedMulti` noise module with default parameters.
    fn default() -> RidgedMulti {
        let mut spectral_weights = [0.0; RIDGED_MAX_OCTAVE as usize];
        calc_spectral_weights(&mut spectral_weights, DEFAULT_RIDGED_LACUNARITY);
        RidgedMulti {
            frequency: DEFAULT_RIDGED_FREQUENCY,
            lacunarity: DEFAULT_RIDGED_LACUNARITY,
            quality: DEFAULT_RIDGED_QUALITY,
            octave_count: DEFAULT_RIDGED_OCTAVE_COUNT,
            spectral_weights: spectral_weights,
            seed: DEFAULT_RIDGED_SEED,
        }
    }
}

impl RidgedMulti {
    /// Create a new `RidgedMulti` noise module with default parameters.
    pub fn new() -> RidgedMulti {
        Default::default()
    }

    /// Returns the frequency of the first octave.
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Returns the lacunarity of the ridged-multifractal-noise.
    ///
    /// The lacunarity is the frequency multiplier between successive octaves.
    pub fn lacunarity(&self) -> f64 {
        self.lacunarity
    }

    /// Returns the quality of the ridged-multifractal-noise.
    ///
    /// See [`NoiseQuality`](../../noisegen/enum.NoiseQuality.html) for
    /// definitions of the various coherent-noise qualities.
    pub fn quality(&self) -> NoiseQuality {
        self.quality
    }

    /// Returns the number of octaves that generate the
    /// ridged-multifractal-noise.
    ///
    /// The number of octaves controls the amount of detail in the
    /// ridged-multifractal-noise.
    pub fn octave_count(&self) -> i32 {
        self.octave_count
    }

    /// Returns the seed value used by the ridged-multifractal-noise function.
    pub fn seed(&self) -> i32 {
        self.seed
    }

    /// Sets the frequency of the first octave.
    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
    }

    /// Sets the lacunarity of the ridged-multifractal-noise.
    ///
    /// The lacunarity is the frequency multiplier between successive octaves.
    ///
    /// For best results, set the lacunarity to a number between 1.5 and 3.5.
    pub fn set_lacunarity(&mut self, lacunarity: f64) {
        self.lacunarity = lacunarity;
        calc_spectral_weights(&mut self.spectral_weights, self.lacunarity);
    }

    /// Sets the quality of the ridged-multifractal-noise.
    ///
    /// See [`NoiseQuality`](../../noisegen/enum.NoiseQuality.html) for
    /// definitions of the various coherent-noise qualities.
    pub fn set_quality(&mut self, quality: NoiseQuality) {
        self.quality = quality;
    }

    /// Sets the number of octaves that generate the ridged-multifractal-noise.
    ///
    /// The number of octaves controls the amount of detail in the
    /// ridged-multifractal-noise.
    ///
    /// The larger the number of octaves, the more time required to calculate
    /// the ridged-multifractal-noise value.
    ///
    /// # Panics
    ///
    /// Panics if the given octave count is outside the range from 1 to
    /// [`RIDGED_MAX_OCTAVE`](constant.RIDGED_MAX_OCTAVE.html) inclusive.
    pub fn set_octave_count(&mut self, octave_count: i32) {
        if octave_count < 1 || octave_count > RIDGED_MAX_OCTAVE {
            panic!("`octave_count` must be in the range [{}, {}]", 1, RIDGED_MAX_OCTAVE);
        }
        self.octave_count = octave_count;
    }

    /// Sets the seed value used by the ridged-multifractal-noise function.
    pub fn set_seed(&mut self, seed: i32) {
        self.seed = seed;
    }
}

impl Module for RidgedMulti {
    // Multifractal code originally written by F. Kenton "Doc Mojo" Musgrave,
    // 1998.  Modified by jas for use with libnoise.
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut x = x * self.frequency;
        let mut y = y * self.frequency;
        let mut z = z * self.frequency;

        let mut value = 0.0;
        let mut weight = 1.0;

        // These parameters should be user-defined; they may be exposed in a
        // future version of libnoise.
        let offset = 1.0;
        let gain = 2.0;

        for cur_octave in 0..self.octave_count {
            // Make sure that these floating-point values have the same range as
            // a 32-bit integer so that we can pass them to the coherent-noise
            // functions.
            let nx = make_i32_range(x);
            let ny = make_i32_range(y);
            let nz = make_i32_range(z);

            // Get the coherent-noise value.
            let seed = (self.seed + cur_octave) & 0x7fffffff;
            let mut signal = gradient_coherent_noise3d(nx, ny, nz, seed, self.quality);

            // Make the ridges.
            signal = signal.abs();
            signal = offset - signal;

            // Square the signal to increase the sharpness of the ridges.
            signal *= signal;

            // The weighting from the previous octave is applied to the signal.
            // Larger values have higher weights, producing sharp points along the
            // ridges.
            signal *= weight;

            // Weight successive contributions by the previous signal.
            weight = signal * gain;
            weight = if weight > 1.0 {
                1.0
            } else if weight < 0.0 {
                0.0
            } else {
                weight
            };

            // Add the signal to the output value.
            value += signal * self.spectral_weights[cur_octave as usize];

            // Go to the next octave.
            x *= self.lacunarity;
            y *= self.lacunarity;
            z *= self.lacunarity;
        }

        (value * 1.25) - 1.0
    }
}
