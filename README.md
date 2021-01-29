# fractalgen
A rust based fractal generator, essentially a rust clone of my other fractal renderer project in C.

## Getting started
``cargo run --release -- <args>``

Is the main syntax for making renders.
The ``--mapfile`` argument requires a file with a list of hexcodes as an argument, see
[libcmap](https://github.com/tritoke/libcmap) for a repository with many of these.

See ``cargo run -- --help`` for more information on what arguments are available and how to use them.

## Examples
``cargo run --release -- --mapfile path/to/libcmap/colourmaps/lesbian.cmap -s -x 3 --image_centre -0.6,0``
