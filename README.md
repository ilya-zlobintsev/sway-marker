# Simple popup for using marks in Sway

This allows you to use vim-like marks in sway easily.

![image](https://i.imgur.com/gQW40rq.png)

Usage:
```
bindsym --to-code $mod+m exec sway-marker mark
bindsym --to-code $mod+grave exec sway-marker goto
```
After adding this to your sway config you can use marks like this:

mod+m and then type a character will create a mark on the currently focused container (window)

mod+grave(`) then type a character will focus the previously marked container.

## Installation

Install rust, gtk and gtk-layer-shell and build with `cargo build --release`. The file will be in `target/release/sway-marker`.