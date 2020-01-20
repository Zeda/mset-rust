# mset
This is just a quick program I put together for practice with Rust.

![*Firey Mandelbrot set*](https://i.imgur.com/cwabDAUm.png)
![*Mandelbrot set in blue*](https://i.imgur.com/BdTNO19m.png)

See this [Imgur album](https://imgur.com/a/TcSF8UE) for more!

# "License"
Feel free to use this at your own risk and do whatever you want with the code.

# Building
Whelp, this is written in Rust (and Cargo), so you'll need to install them. See
[this](https://doc.rust-lang.org/cargo/getting-started/installation.html).
You'll also need to clone this project :) Once cloned, navigate to the project
folder and run:
```
cargo build --release
```

The binary will be found at `target/release/mset`.

# Executing
This generates the Mandelbrot set from `x` on `[-2, .5]` and `y` on
`[-1.25, 1.25]`. The output is stored to `output.png`. It takes input via the
command line. Syntax looks like:
```
zeda@zeda:~/mset$ ./mset width height iter cm0 cm1 cm2
```

- `width` is the width in pixels of the output
- `height` is the height in pixels of the output
- `iter` is the maximum iterations per pixel
- `cm0` is a multiplier used to generate the red component of the color
- `cm1` is a multiplier used to generate the green component of the color
- `cm2` is a multiplier used to generate the blue component of the color

I couldn't figure out how to parse a string as an integer, so I wrote my own
routine. It does no error checking, so you will get wonky results if you use
non-numeric values!


Some of my favorite inputs:
```
./mset 1024 1024 256 32 32 12
./mset 1024 1024 256 1 3 6
./mset 1024 1024 256 3 6 1
./mset 1024 1024 256 6 1 3
./mset 1024 1024 256 6 3 1
./mset 1024 1024 256 253 22 1
```
