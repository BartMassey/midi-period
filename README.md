![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# midi-period: calculate timer period for MIDI key number
Copyright © 2024 Bart Massey (Version 0.1.0)


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


## Derivation

Here's the derivation used in the code. We are given a MIDI
key number $k$ and a timer frequency $F$ in ticks per
second. Let $f_0$ be the frequency in Hz of MIDI key number
0, given by

$$
f_0 = 440 \cdot 2^{-69 / 12}
$$

Given real arithmetic, the desired integer timer period $p$
in ticks is given by

$$
p = \left\langle \frac{F}{f_0 \cdot 2^{k/12}} \right\rangle
$$

where $\langle \cdot \rangle$ is round-to-nearest.

To make this work as an integer operation, we will do the
rounding earlier and then do an integer division. Note that
the integer division truncates a division whose numerator
and denominator have already been rounded: this is a source
of error that is not easily avoidable, and will not affect
the result too much given appropriate choices. Let's start
by scaling the numerator and denominator by some scaling
factor $s$. (The code currently has $s=1024$
hardwired.) We now have

$$
p = \left\lfloor \frac{s\cdot F}{s\cdot f_0 \cdot 2^{k/12}} \right\rfloor
$$

The numerator is already an integer, but the denominator is not. We will
use an identity to fix things up:

$$
k / 12 = k~\mathbin{\textrm{div}}~12 + (k \bmod 12) / 12
$$

We can use this identity to do some algebra and rearrangement of terms.

$$
\begin{eqnarray*}
p &=& \left\lfloor \frac{s\cdot F}{s\cdot f_0 \cdot 2^{k/12}} \right\rfloor \\
  &=& \left\lfloor \frac{s\cdot F}{s\cdot f_0 \cdot 2^{k~\mathbin{\textrm{div}}~12 + (k \bmod 12) / 12}} \right\rfloor \\
  &=& \left\lfloor \frac{s\cdot F}{2^{k~\mathbin{\textrm{div}}~12} \cdot \big(s\cdot f_0 \cdot 2^{(k \bmod 12) / 12}\big)} \right\rfloor
\end{eqnarray*}
$$

The parenthesized term can now be extracted into a function that rounds
its result to an integer.

$$
E(i) = \left\langle s \cdot f_0 \cdot 2^{i/12}\right\rangle
$$

Since the input of $E$ is the result of a computation mod 12, it
will be an integer between 0 and 11 inclusive. This means that
we can build a memo table for $E$ at compile-time, and compute
its values by table lookup. We thus end up with

$$
p = \left\lfloor \frac{s\cdot F}{2^{k~\mathbin{\textrm{div}}~12} \cdot E(k \bmod 12)} \right\rfloor
$$

where all the runtime computation is integer. The choice of
$s$ is crucial here: it must be large enough to produce
accurate rounding, but small enough that neither the
numerator nor the denominator computations overflow.

## License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
