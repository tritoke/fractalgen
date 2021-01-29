# fractalgen
A rust based fractal generator, essentially a rust clone of my other fractal renderer project in C.

## Getting started
``cargo run --release -- <args>``

Is the main syntax for making renders.
The ``--mapfile`` argument requires a file with a list of hexcodes as an argument, see
[libcmap](https://github.com/tritoke/libcmap) for a repository with many of these.
