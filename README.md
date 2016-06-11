# pngencoder-rust

Basically [this](https://github.com/KCreate/pngencoder), but written in [Rust](https://www.rust-lang.org/).

# Installation

You will need [Cargo](https://crates.io/) to build this.
```sh
git clone https://github.com/KCreate/pngencoder-rust.git
cd pngencoder-rust
cargo build --release
sudo cp ./target/release/pngencoder /usr/bin/
```

If you install to another location, you will need to make sure that it is in your $PATH variable.

# Usage

You can just pipe the output of every command directly into it.
In this example I pipe the result of a curl request into it.

```sh
curl -v -silent https://github.com/KCreate/pngencoder-rust 2>&1 | pngencoder
```

This will produce an image called __output.png__ in the directory the command is run in.

![Example](./output.png)


# License
The MIT License (MIT)
Copyright (c) <2016 - present> <Leonard Schütz>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
