# nbody-3d

Newtonian gravity simulation for n massive bodies.

To build (need rustc and cargo):
* Clone/download the repository
* In top-level directory, `cargo build --release`

To run:
* `cargo run --release`

Controls:
* Basic arcball camera.
* `SPACE` to play/pause the simulation.
* `↑ ↓ ← →` to pitch/yaw the camera about its focus point.
* `WASD` to move camera focus about xy plane.
* `Z` to zoom out, `X` to zoom in.
* `R` to spawn object with random orbital parameters.
* `P` to toggle trails.
* `C` to clear trails.
