![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# {{crate}}: calculate timer period for MIDI key number
Copyright © 2024 Bart Massey (Version {{version}})

{{readme}}

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
rounding earlier and then do a rounded integer
division. Note that the integer division rounds a division
whose numerator and denominator have already been rounded:
this "double rounding" is a source of error that is not
easily avoidable, and will not affect the result too much
given appropriate choices.

Let's start by scaling the numerator and denominator by some
scaling factor $s$. (The code currently has $s=512$
hardwired.) We now have

$$
p = \left\langle \frac{s\cdot F}{s\cdot f_0 \cdot 2^{k/12}} \right\rangle
$$

The numerator is already an integer, but the denominator is not. We will
use an identity to fix things up:

$$
k / 12 = k\mathbin{\textrm{ div }}12 + (k \bmod 12) / 12
$$

We can use this identity to do some algebra and rearrangement of terms.

$$
\begin{eqnarray*}
p &=& \left\langle \frac{s\cdot F}{s\cdot f_0 \cdot 2^{k/12}} \right\rangle \\
  &=& \left\langle \frac{s\cdot F}{s\cdot f_0 \cdot 2^{k\mathbin{\textrm{ div }}12 + (k \bmod 12) / 12}} \right\rangle \\
  &=& \left\langle \frac{s\cdot F}{2^{k\mathbin{\textrm{ div }}12} \cdot \big(s\cdot f_0 \cdot 2^{(k \bmod 12) / 12}\big)} \right\rangle
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
p = \left\langle \frac{s\cdot F}{2^{k\mathbin{\textrm{ div }}12} \cdot E(k \bmod 12)} \right\rangle
$$

where all the runtime computation is integer except the rounded division
at the end. To round, we use the identity

$$
\begin{eqnarray*}
\textrm{round}(n/d) &=& \lfloor n/d + 0.5 \rfloor \\
                    &=& (2\cdot n + d)\mathbin{\textrm{ div }}(2\cdot d)
\end{eqnarray*}
$$

with the numerators and denominators above.

The choice of $s$ and $F$ is crucial here. Too small and our
calculation will become inaccurate at higher
frequencies. Too large and the numerator or denominator will
overflow.

## License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
