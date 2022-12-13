# Advent of Code 2022: Day 7
Ben and Bart Massey

---

OK, I think we're done here.

Ben has kind of dropped out at this point. I spent six days
trying to figure out why my perfectly clean and
cromulent-looking code, which gave the right answer on the
example input for Part 1, gave a low answer for the actual
input. I tried a bunch of approaches to diagnose this, and
eventually got it.

---

Here's how I finally debugged Part 1:

* Carefully read and commented my code.
* Added unit tests to existing functions.
* Split solution code into function and added unit tests.
* Grabbed someone else's solution to verify that my solution
  was buggy. It was.
* Woke up one morning with an understanding of where the bug
  was.
    * Verified the bug by adding the bugged case to the unit tests.
    * Fixed the bug.
    * Verified that the example input still worked.
    * Verified that I got the same answer as the third-party
      code on the input.

The tl;dr is that I wasn't adding directories with only
directory children into the directory table, which meant
that my sum was low by the total size of these directories.

---

The fun has gone now. Enjoy Advent of Code!

---

Solution to [this problem](https://adventofcode.com/2022/day/7).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
