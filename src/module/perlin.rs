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

/// Default frequency for the [`Perlin`](struct.Perlin.html) noise module.
pub const DEFAULT_PERLIN_FREQUENCY: f64 = 1.0;

/// Default lacunarity for the [`Perlin`](struct.Perlin.html) noise module.
pub const DEFAULT_PERLIN_LACUNARITY: f64 = 2.0;

/// Default number of octaves for the [`Perlin`](struct.Perlin.html) noise
/// module.
pub const DEFAULT_PERLIN_OCTAVE_COUNT: i32 = 6;

/// Default persistence value for the [`Perlin`](struct.Perlin.html) noise
/// module.
pub const DEFAULT_PERLIN_PERSISTENCE: f64 = 0.5;

/// Default noise quality for the [`Perlin`](struct.Perlin.html) noise module.
pub const DEFAULT_PERLIN_QUALITY: NoiseQuality = NoiseQuality::Standard;

/// Default noise seed for the [`Perlin`](struct.Perlin.html) noise module.
pub const DEFAULT_PERLIN_SEED: i32 = 0;

/// Maximum number of octaves for the [`Perlin`](struct.Perlin.html) noise
/// module.
pub const PERLIN_MAX_OCTAVE: i32 = 30;

/// Noise module that outputs 3-dimensional Perlin noise.
///
/// Perlin noise is the sum of several coherent-noise functions of
/// ever-increasing frequencies and ever-decreasing amplitudes.
///
/// An important property of Perlin noise is that a small change in the input
/// value will produce a small change in the output value, while a large change
/// in the input value will produce a random change in the output value.
///
/// This noise module outputs Perlin-noise values that usually range from -1.0
/// to +1.0, but there are no guarantees that all output values will exist
/// within that range.
///
/// For a better description of Perlin noise, see the links in the *References
/// and Acknowledgments* section.
///
/// This noise module does not require any source modules.
///
/// ## Octaves
///
/// The number of octaves control the *amount of detail* of the Perlin noise.
/// Adding more octaves increases the detail of the Perlin noise, but with the
/// drawback of increasing the calculation time.
///
/// An octave is one of the coherent-noise functions in a series of
/// coherent-noise functions that are added together to form Perlin noise.
///
/// An application may specify the frequency of the first octave by calling the
/// [`set_frequency()`](struct.Perlin.html#method.set_frequency) method.
///
/// An application may specify the number of octaves that generate Perlin noise
/// by calling the
/// [`set_octave_count()`](struct.Perlin.html#method.set_octave_count) method.
///
/// These coherent-noise functions are called octaves because each octave has,
/// by default, double the frequency of the previous octave.  Musical tones have
/// this property as well; a musical C tone that is one octave higher than the
/// previous C tone has double its frequency.
///
/// ## Frequency
///
/// An application may specify the frequency of the first octave by calling the
/// [`set_frequency()`](struct.Perlin.html#method.set_frequency) method.
///
/// ## Persistence
///
/// The persistence value controls the *roughness* of the Perlin noise.  Larger
/// values produce rougher noise.
///
/// The persistence value determines how quickly the amplitudes diminish for
/// successive octaves.  The amplitude of the first octave is 1.0.  The
/// amplitude of each subsequent octave is equal to the product of the previous
/// octave's amplitude and the persistence value.  So a persistence value of 0.5
/// sets the amplitude of the first octave to 1.0; the second, 0.5; the third,
/// 0.25; etc.
///
/// An application may specify the persistence value by calling the
/// [`set_persistence()`](struct.Perlin.html#method.set_persistence) method.
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
/// [The Noise Machine](http://www.noisemachine.com/talk1/) - From the master,
/// Ken Perlin himself.  This page contains a presentation that describes Perlin
/// noise and some of its variants.  He won an Oscar for creating the Perlin
/// noise algorithm!
///
/// [Perlin Noise](http://freespace.virgin.net/hugo.elias/models/m_perlin.htm) -
/// Hugo Elias's webpage contains a very good description of Perlin noise and
/// describes its many applications.  This page gave me the inspiration to
/// create libnoise in the first place.  Now that I know how to generate Perlin
/// noise, I will never again use cheesy subdivision algorithms to create
/// terrain (unless I absolutely need the speed.)
///
/// [Perlin noise math FAQ](http://www.robo-murito.net/code/perlin-noise-math-faq.html) -
/// A good page that describes Perlin noise in plain English with only a minor
/// amount of math.  During development of libnoise, I noticed that my
/// coherent-noise function generated terrain with some "regularity" to the
/// terrain features.  This page describes a better coherent-noise function
/// called *gradient noise*.  This version of Perlin uses gradient coherent
/// noise to generate Perlin noise.
#[derive(Clone)]
pub struct Perlin {
    frequency: f64,
    lacunarity: f64,
    quality: NoiseQuality,
    octave_count: i32,
    persistence: f64,
    seed: i32,
}

impl Default for Perlin {
    /// Create a new `Perlin` noise module with default parameters.
    fn default() -> Perlin {
        Perlin {
            frequency: DEFAULT_PERLIN_FREQUENCY,
            lacunarity: DEFAULT_PERLIN_LACUNARITY,
            quality: DEFAULT_PERLIN_QUALITY,
            octave_count: DEFAULT_PERLIN_OCTAVE_COUNT,
            persistence: DEFAULT_PERLIN_PERSISTENCE,
            seed: DEFAULT_PERLIN_SEED,
        }
    }
}

impl Perlin {
    /// Create a new `Perlin` noise module with default parameters.
    pub fn new() -> Perlin {
        Default::default()
    }

    /// Returns the frequency of the first octave.
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Returns the lacunarity of the Perlin noise.
    ///
    /// The lacunarity is the frequency multiplier between successive octaves.
    pub fn lacunarity(&self) -> f64 {
        self.lacunarity
    }

    /// Returns the quality of the Perlin noise.
    ///
    /// See [`NoiseQuality`](../../noisegen/enum.NoiseQuality.html) for
    /// definitions of the various coherent-noise qualities.
    pub fn quality(&self) -> NoiseQuality {
        self.quality
    }

    /// Returns the number of octaves that generate the Perlin noise.
    ///
    /// The number of octaves controls the amount of detail in the Perlin noise.
    pub fn octave_count(&self) -> i32 {
        self.octave_count
    }

    /// Returns the persistence value of the Perlin noise.
    ///
    /// The persistence value controls the roughness of the Perlin noise.
    pub fn persistence(&self) -> f64 {
        self.persistence
    }

    /// Returns the seed value used by the Perlin-noise function.
    pub fn seed(&self) -> i32 {
        self.seed
    }

    /// Sets the frequency of the first octave.
    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
    }

    /// Sets the lacunarity of the Perlin noise.
    ///
    /// The lacunarity is the frequency multiplier between successive octaves.
    ///
    /// For best results, set the lacunarity to a number between 1.5 and 3.5.
    pub fn set_lacunarity(&mut self, lacunarity: f64) {
        self.lacunarity = lacunarity;
    }

    /// Sets the quality of the Perlin noise.
    ///
    /// See [`NoiseQuality`](../../noisegen/enum.NoiseQuality.html) for
    /// definitions of the various coherent-noise qualities.
    pub fn set_quality(&mut self, quality: NoiseQuality) {
        self.quality = quality;
    }

    /// Sets the number of octaves that generate the Perlin noise.
    ///
    /// The number of octaves controls the amount of detail in the Perlin
    /// noise.
    ///
    /// The larger the number of octaves, the more time required to
    /// calculate the Perlin-noise value.
    ///
    /// # Panics
    ///
    /// Panics if the given octave count is outside the range from 1 to
    /// [`PERLIN_MAX_OCTAVE`](constant.PERLIN_MAX_OCTAVE.html) inclusive.
    pub fn set_octave_count(&mut self, octave_count: i32) {
        if octave_count < 1 || octave_count > PERLIN_MAX_OCTAVE {
            panic!("`octave_count` must be in the range [{}, {}]", 1, PERLIN_MAX_OCTAVE);
        }
        self.octave_count = octave_count;
    }

    /// Sets the persistence value of the Perlin noise.
    ///
    /// The persistence value controls the roughness of the Perlin noise.
    ///
    /// For best results, set the persistence to a number between 0.0 and 1.0.
    pub fn set_persistence(&mut self, persistence: f64) {
        self.persistence = persistence;
    }

    /// Sets the seed value used by the Perlin-noise function.
    pub fn set_seed(&mut self, seed: i32) {
        self.seed = seed;
    }
}

impl Module for Perlin {
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
            value += signal * cur_persistence;

            // Prepare the next octave.
            x *= self.lacunarity;
            y *= self.lacunarity;
            z *= self.lacunarity;
            cur_persistence *= self.persistence;
        }

        value
    }
}
