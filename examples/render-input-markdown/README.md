This example demonstrates
 - a responsive layout
 - a textarea
 - rendering the textarea's content as markdown

Run this example with

    cargo run --example render-input-markdown

If you want to have a log file generated, run

    TERMIMAD_LOG=debug cargo run --example render-input-markdown

Quit with <kbd>ctrl</kbd>-<kbd>Q</kbd>

If you're on linux and there's a compilation error you may have to install xorg-dev and libxcb-composite0-dev, which can be done on apt based distributions with

    sudo apt install xorg-dev libxcb-composite0-dev

If you're on macOS and there's a compilation error you may have to install `libxcb` with

    brew install libxcb

and then `brew list libxcb` to see the library path (something like `/opt/homebrew/Cellar/libxcb/1.16/lib/libxcb.1.1.0.dylib`) 

and `export LIBRARY_PATH="$LIBRARY_PATH:<YOUR_XCB_DYLIB_DIRECTORY_PATH>"` (for example `export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/Cellar/libxcb/1.16/lib`) to add it to the library path.

Then run `cargo run --example render-input-markdown`.
