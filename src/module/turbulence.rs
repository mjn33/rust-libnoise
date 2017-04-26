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
use module::perlin;

/// Default frequency for the [`Turbulence`](struct.Turbulence.html) noise
/// module.
pub const DEFAULT_TURBULENCE_FREQUENCY: f64 = perlin::DEFAULT_PERLIN_FREQUENCY;

/// Default power for the [`Turbulence`](struct.Turbulence.html) noise module.
pub const DEFAULT_TURBULENCE_POWER: f64 = 1.0;

/// Default roughness for the [`Turbulence`](struct.Turbulence.html) noise
/// module.
pub const DEFAULT_TURBULENCE_ROUGHNESS: i32 = 3;

/// Default noise seed for the [`Turbulence`](struct.Turbulence.html) noise
/// module.
pub const DEFAULT_TURBULENCE_SEED: i32 = perlin::DEFAULT_PERLIN_SEED;

/// Noise module that randomly displaces the input value before returning the
/// output value from a source module.
///
/// `Turbulence` is the pseudo-random displacement of the input value.  The
/// [`get_value()`](struct.Turbulence.html#method.get_value) method randomly
/// displaces the (`x`, `y`, `z`) coordinates of the input value before
/// retrieving the output value from the source module.  To control the
/// turbulence, an application can modify its frequency, its power, and its
/// roughness.
///
/// The frequency of the turbulence determines how rapidly the displacement
/// amount changes.  To specify the frequency, call the
/// [`set_frequency()`](struct.Turbulence.html#method.set_frequency) method.
///
/// The power of the turbulence determines the scaling factor that is applied to
/// the displacement amount.  To specify the power, call the
/// [`set_power()`](struct.Turbulence.html#method.set_power) method.
///
/// The roughness of the turbulence determines the roughness of the changes to
/// the displacement amount.  Low values smoothly change the displacement
/// amount.  High values roughly change the displacement amount, which produces
/// more "kinky" changes.  To specify the roughness, call the
/// [`set_roughness()`](struct.Turbulence.html#method.set_roughness) method.
///
/// Use of this noise module may require some trial and error.  Assuming that
/// you are using a generator module as the source module, you should first:
///   * Set the frequency to the same frequency as the source module.
///   * Set the power to the reciprocal of the frequency.
///
/// From these initial frequency and power values, modify these values until
/// this noise module produce the desired changes in your terrain or texture.
/// For example:
///
///   * Low frequency (1/8 initial frequency) and low power (1/8 initial power)
///     produces very minor, almost unnoticeable changes.
///   * Low frequency (1/8 initial frequency) and high power (8 times initial
///     power) produces "ropey" lava-like terrain or marble-like textures.
///   * High frequency (8 times initial frequency) and low power (1/8 initial
///     power) produces a noisy version of the initial terrain or texture.
///   * High frequency (8 times initial frequency) and high power (8 times
///     initial power) produces nearly pure noise, which isn't entirely useful.
///
/// Displacing the input values result in more realistic terrain and textures.
/// If you are generating elevations for terrain height maps, you can use this
/// noise module to produce more realistic mountain ranges or terrain features
/// that look like flowing lava rock.  If you are generating values for
/// textures, you can use this noise module to produce realistic marble-like or
/// "oily" textures.
///
/// Internally, there are three [`Perlin`](../perlin/struct.Perlin.html) noise
/// modules that displace the input value; one for the `x`, one for the `y`, and
/// one for the `z` coordinate.
///
/// This noise module requires one source module.
pub struct Turbulence<M: Module> {
    power: f64,
    msource: M,
    x_distort: perlin::Perlin,
    y_distort: perlin::Perlin,
    z_distort: perlin::Perlin,
}

impl<M: Module> Turbulence<M> {
    /// Create a new `Turbulence` noise module around the specified module,
    /// using default parameters.
    pub fn new(module: M) -> Turbulence<M> {
        let x_distort = perlin::Perlin::default();
        let y_distort = perlin::Perlin::default();
        let z_distort = perlin::Perlin::default();
        let mut rv = Turbulence {
            power: DEFAULT_TURBULENCE_POWER,
            msource: module,
            x_distort: x_distort,
            y_distort: y_distort,
            z_distort: z_distort,
        };

        rv.set_seed(DEFAULT_TURBULENCE_SEED);
        rv.set_frequency(DEFAULT_TURBULENCE_FREQUENCY);
        rv.set_roughness(DEFAULT_TURBULENCE_ROUGHNESS);

        rv
    }

    /// Returns a reference to the module whose input values are being randomly
    /// displaced.
    pub fn module(&self) -> &M {
        &self.msource
    }

    /// Returns a mutable reference to the module whose input values are being
    /// randomly displaced.
    pub fn module_mut(&mut self) -> &mut M {
        &mut self.msource
    }

    /// Returns the frequency of the turbulence.
    ///
    /// The frequency of the turbulence determines how rapidly the displacement
    /// amount changes.
    pub fn frequency(&self) -> f64 {
        self.x_distort.frequency()
    }

    /// Returns the power of the turbulence.
    ///
    /// The power of the turbulence determines the scaling factor that is
    /// applied to the displacement amount.
    pub fn power(&self) -> f64 {
        self.power
    }

    /// Returns the roughness of the turbulence.
    ///
    /// The roughness of the turbulence determines the roughness of the changes
    /// to the displacement amount.  Low values smoothly change the displacement
    /// amount.  High values roughly change the displacement amount, which
    /// produces more "kinky" changes.
    pub fn roughness(&self) -> i32 {
        self.x_distort.octave_count()
    }

    /// Returns the seed value of the internal Perlin-noise modules that are
    /// used to displace the input values.
    ///
    /// Internally, there are three [`Perlin`](../perlin/struct.Perlin.html)
    /// noise modules that displace the input value; one for the `x`, one for
    /// the `y`, and one for the `z` coordinate.
    pub fn seed(&self) -> i32 {
        self.x_distort.seed()
    }

    /// Sets the module whose input values are going to be displaced randomly.
    pub fn set_module(&mut self, module: M) {
        self.msource = module;
    }

    /// Sets the frequency of the turbulence.
    ///
    /// The frequency of the turbulence determines how rapidly the
    /// displacement amount changes.
    pub fn set_frequency(&mut self, frequency: f64) {
        self.x_distort.set_frequency(frequency);
        self.y_distort.set_frequency(frequency);
        self.z_distort.set_frequency(frequency);
    }

    /// Sets the power of the turbulence.
    ///
    /// The power of the turbulence determines the scaling factor that is
    /// applied to the displacement amount.
    pub fn set_power(&mut self, power: f64) {
        self.power = power;
    }

    /// Sets the roughness of the turbulence.
    ///
    /// The roughness of the turbulence determines the roughness of the changes
    /// to the displacement amount.  Low values smoothly change the displacement
    /// amount.  High values roughly change the displacement amount, which
    /// produces more "kinky" changes.
    ///
    /// Internally, there are three [`Perlin`](../perlin/struct.Perlin.html)
    /// noise modules that displace the input value; one for the `x`, one for
    /// the `y`, and one for the `z` coordinate.  The roughness value is equal
    /// to the number of octaves used by `Perlin` noise modules.
    ///
    /// # Panics
    ///
    /// Panics if the given `roughness` is outside the valid range for
    /// `octave_count` accepted by
    /// [`Perlin::set_octave_count()`](../perlin/struct.Perlin.html#method.set_octave_count).
    pub fn set_roughness(&mut self, roughness: i32) {
        self.x_distort.set_octave_count(roughness);
        self.y_distort.set_octave_count(roughness);
        self.z_distort.set_octave_count(roughness);
    }

    /// Sets the seed value of the internal noise modules that are used to
    /// displace the input values.
    ///
    /// Internally, there are three [`Perlin`](../perlin/struct.Perlin.html)
    /// noise modules that displace the input value; one for the `x`, one for
    /// the `y`, and one for the `z` coordinate.  This noise module assigns the
    /// following seed values to the `Perlin` noise modules:
    ///
    ///   * It assigns the seed value (`seed + 0`) to the `x` noise module.
    ///   * It assigns the seed value (`seed + 1`) to the `y` noise module.
    ///   * It assigns the seed value (`seed + 2`) to the `z` noise module.
    pub fn set_seed(&mut self, seed: i32) {
        // Set the seed of each `Perlin` noise modules.  To prevent any sort of
        // weird artifacting, use a slightly different seed for each noise
        // module.
        self.x_distort.set_seed(seed);
        self.y_distort.set_seed(seed + 1);
        self.z_distort.set_seed(seed + 2);
    }
}

impl<M: Module> Module for Turbulence<M> {
    fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        // Get the values from the three `Perlin` noise modules and add each
        // value to each coordinate of the input value.  There are also some
        // offsets added to the coordinates of the input values.  This prevents
        // the distortion modules from returning zero if the (x, y, z)
        // coordinates, when multiplied by the frequency, are near an integer
        // boundary.  This is due to a property of gradient coherent noise,
        // which returns zero at integer boundaries.
        let x0 = x + (12414.0 / 65536.0);
        let y0 = y + (65124.0 / 65536.0);
        let z0 = z + (31337.0 / 65536.0);
        let x1 = x + (26519.0 / 65536.0);
        let y1 = y + (18128.0 / 65536.0);
        let z1 = z + (60493.0 / 65536.0);
        let x2 = x + (53820.0 / 65536.0);
        let y2 = y + (11213.0 / 65536.0);
        let z2 = z + (44845.0 / 65536.0);
        let x_distort = x + self.x_distort.get_value(x0, y0, z0) * self.power;
        let y_distort = y + self.y_distort.get_value(x1, y1, z1) * self.power;
        let z_distort = z + self.z_distort.get_value(x2, y2, z2) * self.power;

        // Retrieve the output value at the offsetted input value instead of the
        // original input value.
        self.msource.get_value(x_distort, y_distort, z_distort)
    }
}

impl<M: Module + Clone> Clone for Turbulence<M> {
    fn clone(&self) -> Turbulence<M> {
        Turbulence {
            power: self.power,
            msource: self.msource.clone(),
            x_distort: self.x_distort.clone(),
            y_distort: self.y_distort.clone(),
            z_distort: self.z_distort.clone(),
        }
    }
}
