# fractalgen
A rust based fractal generator, essentially a rust clone of my other fractal renderer project in C.

## Getting started
``cargo run --release -- <args>``

Is the main syntax for making renders.
The ``--mapfile`` argument requires a file with a list of hexcodes as an argument, see
[libcmap](https://github.com/tritoke/libcmap) for a repository with many of these.

See ``cargo run -- --help`` for more information on what arguments are available and how to use them.

## Examples
``cargo run --release -- --mapfile path/to/libcmap/colourmaps/lesbian.cmap  -s -x 3              --image_centre -0.6,0``
![Mandelbrot with lesbian flag colourmap](https://github.com/tritoke/fractalgen/blob/main/examples/mandelbrot-lesbian.jpg)

``cargo run --release -- --mapfile path/to/libcmap/colourmaps/blues.cmap    -s -x 0.2    -i 2000 --image_centre -0.8,0.15``
![Mandelbrot - seahorse valley in blues colourmap](https://github.com/tritoke/fractalgen/blob/main/examples/seahorse-blues.jpg)

``cargo run --release -- --mapfile path/to/libcmap/colourmaps/bud5.cmap     -s -x 0.0002 -i 2000 --image_centre -0.7439057990393522,0.1317168628468215``
![Mandelbrot - deep zoom in bud5 colourmap](https://github.com/tritoke/fractalgen/blob/main/examples/zoom-bud5.jpg)

``cargo run --release -- --mapfile path/to/libcmap/colourmaps/binary.cmap   -s -f julia --julia_centre -0.8,0.156``
![Julia set - example render 1 in binary colourmap](https://github.com/tritoke/fractalgen/blob/main/examples/julia1-binary.jpg)

``cargo run --release -- --mapfile path/to/libcmap/colourmaps/Skydye01.cmap -s -f julia --julia_centre -0.4,0.6``
![Julia set - example render 2 in Skydye01 colourmap](https://github.com/tritoke/fractalgen/blob/main/examples/julia2-Skydye01.jpg)

``cargo run --release -- --mapfile path/to/libcmap/colourmaps/Gallet01.cmap -s -f julia --julia_centre 0.285,0.01``
![Julia set - example render 3 in Gallet01 colourmap](https://github.com/tritoke/fractalgen/blob/main/examples/julia3-Gallet01.jpg)

``cargo run --release -- --mapfile path/to/libcmap/colourmaps/Gallet06.cmap -s -f julia --julia_centre -0.835,-0.2321``
![Julia set - example render 4 in Gallet06 colourmap](https://github.com/tritoke/fractalgen/blob/main/examples/julia4-Gallet06.jpg)

![blues julia set spin](https://imgur.com/a/VRTneUs)
