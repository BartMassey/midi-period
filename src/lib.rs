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
/// does not overflow the `u16`; `None` otherwise.
///
/// See this crate's `README` for a derivation and explanation
/// of this function.
pub fn midi_key_period(timer_frequency: u32, key: u16) -> Option<u16> {
    if key > 127 {
        return None;
    }
    let num = timer_frequency * EXP_FRAC_SCALE;
    let p1 = u32::pow(2, key as u32 / 12);
    let p2 = EXP_FRAC[key as usize % 12];
    let den = p1 * p2;
    u16::try_from(num / den).ok()
}

#[test]
fn test_period() {
    for key in 12..=127 {
        let cents = 0.5;
        let upper = 440.0 * f64::powf(2.0, (key as f64 - 69.0 + cents) / 12.0);
        let lower = 440.0 * f64::powf(2.0, (key as f64 - 69.0 - cents) / 12.0);
        let p = midi_key_period(1_000_000, key).unwrap();
        let f = 1.0e6 / p as f64;
        assert!(f < upper && f > lower, "{key} {lower} {f} {upper}");
    }
}
