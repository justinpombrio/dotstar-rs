# Installation

Following the
[cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart)
instructions, add additional build targets for rust (not all of these are
needed):

    rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf

And install cargo generate:

    cargo install cargo-generate

Install `gdb-multiarch`:

    sudo apt-get install gdb-multiarch

Clone the `dotstar-bluepill` repository:

   git clone https://github.com/e-matteson/dotstar-bluepill.git

(Said repo assumes that this repo is located at `../dotstar/`.)

You shouldn't need to run this, but I did:

    sudo dpkg-divert --package gdb --add /usr/share/man/man1/gdb.1.gz

Install `gdb` for the architecture:

    sudo apt-get install gcc-arm-none-eabi gdb-arm-none-eabi

Finally you should be able to build:

    cargo build
    
and run:

    cargo run

which should start `gdb`, after which you can run the program:

    run

