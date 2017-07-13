# mandelbrot

This is a port of the @gotbadger implementation of the Mandelbrot generator
that was written in Go.  This one is written in Rust as a learning project and
to see how it compares (in difficulty) to the Go version.  See the original
[here](https://github.com/gotbadger/mandelbrot).

Mandelbrot in go. Image is drawn to the terminal so you will need to be using
iTerm2 The image is written to `fractal.png` (and will overwrite it) so you'll
need an image viewer to see the image generated.


### Build:

```bash
$ git clone https://github.com/ajkavanagh/rust-mandelbrot
$ cd rust-mandelbrot
$ cargo build --release
```

### Run:
```bash
target/release/mandelbrot
```

This will generate the default Mandelbrot image in `fractal.png` in the same
directory.

### Options:

you can see all options by running `target/release/mandelbrot -h`

```
Usage of mandelbrot:
  -i int
       	max number of iterations / colours (default 30)
  --step=float
       	a pixel is drawn for each step between coordinates (default 0.003)
  --x0=float
       	from X (default -2)
  --x1=float
       	to X (default 1)
  --y0=float
       	from Y (default -1.2)
  --y1=float
       	to Y (default 1.2)
```

Some interesting examples

```
cd target/release
mandelbrot --x0=-1 --x1=-0.65 --y0=0.1 --y1=0.3 --step=0.0002 -i 50
mandelbrot --x0=-0.745 --x1=-0.7 --y0=0.275 --y1=0.3 --step=0.00003 -i 200
```


### Output of the default Mandelbrot image

![img](https://raw.githubusercontent.com/ajkavanagh/rust-mandelbrot/master/fractal.png)
