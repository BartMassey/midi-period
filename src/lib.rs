/*!

This Rust `no_std` library crate provides a function for
converting a MIDI key number into a timer period.  The
function uses only 32-bit integer arithmetic, making it
suitable for embedded applications.

The code currently relies on a table built by its
`build.rs` build script. This will change when `const`
floating-point arithmetic is stabilized: expected in
Rust 1.81.0. In the mean time the build script does
floating-point arithmetic, which is fine for
cross-compilation but may not work in weird self-hosted
embedded environments.

*/

#![no_std]

mod exp_frac;
use exp_frac::*;

/// Function for converting a MIDI `key` number into a timer
/// period for the key's implied frequency, given a
/// `timer_frequency` in ticks per second. Returns `Some`
/// period if the `key` is in range and the resulting period
/// does not overflow the `u32`; `None` otherwise.
///
/// See this crate's `README` for a derivation and explanation
/// of this function.
///
/// # Examples
///
/// ```
/// let p = midi_period::midi_key_period(1_000_000, 69);
/// assert_eq!(p, (1_000_000 + 220) / 440);
/// ```
///
/// # Panics
///
/// * Panics if passed a `key` number > 127.
///
/// * Panics if an internal multiply overflows.
///   `timer_frequency` 4MHz or less is safe. (However, note
///   that `timer_frequency` less than 2MHz will give errors
///   in excess of 5 cents at higher frequencies.)
pub fn midi_key_period(timer_frequency: u32, key: u16) -> u32 {
    assert!(key <= 127);
    let num = timer_frequency * EXP_FRAC_SCALE;
    let p1 = u32::pow(2, key as u32 / 12);
    let p2 = EXP_FRAC[key as usize % 12];
    let den = p1 * p2;
    (2 * num + den) / (2 * den)
}

#[test]
fn test_period() {
    for key in 12..=127 {
        let cents = 5.0;
        let upper = 440.0 * f64::powf(2.0, (key as f64 - 69.0 + cents/100.0) / 12.0);
        let lower = 440.0 * f64::powf(2.0, (key as f64 - 69.0 - cents/100.0) / 12.0);
        let tf = 2_000_000;
        let p = midi_key_period(tf, key);
        let f = tf as f64 / p as f64;
        assert!(f < upper && f > lower, "{key} {lower} {f} {upper}");
    }
}
