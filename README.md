# Advent of Code 2021

My Rust solutions and thoughts regarding Advent of Code 2021.
I am *not* attempting to compete for any leaderboards, just doing these for fun and to try out crates I haven't gotten around to using (enough).
So far these include:

- [`thiserror`](https://github.com/dtolnay/thiserror)
- [`array-init`](https://github.com/Manishearth/array-init)
- [`bitvec`](https://github.com/bitvecto-rs/bitvec)
- [`ndarray-linalg`](https://github.com/rust-ndarray/ndarray-linalg/)

## Day 1

Day 1 is pretty straightforward, however, there is a fun, not immediately obvious way to improve the second part.
In digital signal processing, computing a moving average in fixed point is often optimized by only adding the newest value to the running sum and subtracting the removed value.
Essentially, this is a lossless integrator (normally unstable), followed by a comb filter.
However, as soon as you realize this, it becomes clear that if you are comparing consecutive windows and the update is `sum += new - old` you can just compare `new` and `old` directly, without computing the sum at all.

## Day 2

Nothing particularly interesting here.
Meh.

## Day 3

Straightforward, but you can play around with bit twiddling hacks.
And by partitioning the input instead of really changing the vector size you can avoid all allocations except those required during file reading.

## Day 4

Another boring one.

## Day 5

Fairly straightforward, but I decided to go for more standard loops here instead of trying to do everything with iterators, including using `BufRead::read_line` instead of `BufRead::lines` for input.
This one would have been more interesting if step two had required angles other than 45° (requiring something like [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)).

## Day 6

This is the first one where the straightforward solution doesn't work for step two.
Also, [my initial step two solution](https://github.com/l0calh05t/advent-of-code-2021/blob/f694b9b13cfc00bbce58ffd09542bc645a7af981/day-06/src/main.rs) allowed me to channel my inner Sean Parent and exclaim [“That’s a rotate!”](https://www.youtube.com/watch?v=UZmeDQL4LaE)
But since everything worth doing is worth overdoing:
You can reduce the complexity from O(*n*) to O(log *n*) by converting the iteration to a 9×9 iteration matrix and using [exponentiation by squaring](https://electric-snow.net/2016/05/31/fibonacci-youre-also-doing-it-wrong/).
Or take it even further and diagonalize the matrix and computing the matrix power in O(1) as Re{P D⁸⁰ P⁻¹}, which is what I did for this one (which also gave me an excuse to try [`ndarray-linalg`](https://github.com/rust-ndarray/ndarray-linalg/)).

DISCLAIMER: Don't expect this solution to be any faster.
Quite the opposite, since *n* = 256 isn't exactly a large number.
