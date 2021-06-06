# nbody-3d

Newtonian gravity simulation for n massive bodies.

To build (need rustc and cargo):
* Clone/download the repository
* In top-level directory, `cargo build --release`

To run:
* `cargo run --release`

Controls:
* `W` to zoom in, `S` to zoom out.
* `SPACE` to play/pause the simulation.
* Left click anywhere in the window to add a new object at that location. If it is close enough to another object with a strong gravitational pull, it will attempt to orbit that object.
* Hold `LSHIFT` while clicking to add a more massive object, or `LCTRL` for an even more massive one.