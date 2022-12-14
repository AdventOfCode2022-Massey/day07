# Advent of Code 2022: Day 7
Ben and Bart Massey

---

OK, I think we're almost done here.

Ben has kind of dropped out at this point. I (Bart) spent
six days trying to figure out why my perfectly clean and
cromulent-looking code, which gave the right answer on the
example input for Part 1, gave a low answer for the actual
input. I tried a bunch of approaches to diagnose this, and
eventually got it.

---

Here's how I finally debugged Part 1:

* Ran Cargo clippy and fixed the resulting lints.
* Added a bunch of logging (gated by a Cargo "logging"
  feature) to verify the rest of the computation on the
  example input. Sadly, the real input produced too much
  logging to carefully analyze.
* Carefully read (again and again) and commented my code.
* Added unit tests to existing functions.
* Split solution code into a function and added unit tests.
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

Given the debugged Part 1, the solution to Part 2 was easy.

The fun has gone now. I'll keep going until more like this
pop up shortly. Enjoy Advent of Code!

---

Solution to [this problem](https://adventofcode.com/2022/day/7).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.

---

Thanks to Github user Paul Ologeh for providing the Python
solution to this problem that Bart used to check results
when debugging mine. (This program was used as an "opaque
box": Bart did not inspect its interior.)
