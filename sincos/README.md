Fixpoint sine / cosine table generator
======================================

using multiplication of complex numbers for rotation.

Starting at 1 (in other words `cos(0) + j sin(0)`), we
multiply by `cos(step) + j sin(step)`, this way stepping
along a quarter cirlce, exploiting symmetry writing down
the values at four positions in destination memory.

The prototype in Rust will check against truncation and
overflow issues at compile time and outputs a table
that can be loaded into a spreadsheet program (such as
LibreOffice Calc or MS Excel) to compare against ground
truth calculated from floatingpoint values.
