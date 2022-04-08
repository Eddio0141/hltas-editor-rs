# notice
I've worked on this graphical editor to the point it's starting to become usable, with it being able to edit anything in the hltas file.
However it doesn't seem to speedup or match normal text editing of hltas files.

A simple highlighting and language extension for hltas files in vscode / notepad++ is already faster than using this app, and I could keep refining it to be better with many shortcuts and other enhancements, but it probably still won't beat notepad editing.
On top of that, the usage of the tas editor in bunnymodXT makes the usual notepad editing less used, making this GUI app overkill.

This repository will be archived unless there is a great idea that makes the app extremely useful over vscode / notepad++

# hltas-editor-rs
Graphical editor for hltas files

# How to build
1. Install stable rust ([how to install](https://www.rust-lang.org/tools/install)).
2. Run `cargo build` for a debug build or `cargo build --release` for a release build.
3. Built binary goes in the `.\target\debug` or `.\target\release` directory.
