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

/// Performs linear interpolation between two values.
///
///  * `n0` - The first value.
///  * `n1` - The second value.
///  * `a` - The alpha value.
///
/// The alpha value should range from 0.0 to 1.0.  If the alpha value is
/// 0.0, this function returns `n0`.  If the alpha value is 1.0, this
/// function returns `n1`.
pub fn linear_interp(n0: f64, n1: f64, a: f64) -> f64
{
    ((1.0 - a) * n0) + (a * n1)
}

/// Performs cubic interpolation between two values bound between two other
/// values.
///
///   * `n0` - The value before the first value.
///   * `n1` - The first value.
///   * `n2` - The second value.
///   * `n3` - The value after the second value.
///   * `a` - The alpha value.
///
/// The alpha value should range from 0.0 to 1.0.  If the alpha value is 0.0,
/// this function returns `n1`.  If the alpha value is 1.0, this function
/// returns `n2`.
pub fn cubic_interp(n0: f64, n1: f64, n2: f64, n3: f64, a: f64) -> f64
{
    let p = (n3 - n2) - (n0 - n1);
    let q = (n0 - n1) - p;
    let r = n2 - n0;
    let s = n1;
    p * a * a * a + q * a * a + r * a + s
}

/// Maps a value onto a cubic S-curve. The input should range from 0.0 to 1.0.
///
/// The derivitive of a cubic S-curve is zero at `a` = 0.0 and `a` = 1.0
pub fn scurve3 (a: f64) -> f64
{
    a * a * (3.0 - 2.0 * a)
}

/// Maps a value onto a quintic S-curve. The input should range from 0.0 to 1.0.
///
/// The first derivitive of a quintic S-curve is zero at `a` = 0.0 and `a` = 1.0
///
/// The second derivitive of a quintic S-curve is zero at `a` = 0.0 and `a` =
/// 1.0
pub fn scurve5(a: f64) -> f64 {
    let a3 = a * a * a;
    let a4 = a3 * a;
    let a5 = a4 * a;
    (6.0 * a5) - (15.0 * a4) + (10.0 * a3)
}

pub fn clamp<T: Ord>(value: T, lower_bound: T, upper_bound: T) -> T {
    if value < lower_bound {
        lower_bound
    } else if value > upper_bound {
        upper_bound
    } else {
        value
    }
}
