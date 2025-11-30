#!/usr/bin/env bash
# Run bouncing-ball with X11 backend workaround for Wayland issues
export WINIT_UNIX_BACKEND=x11
cargo run --release
