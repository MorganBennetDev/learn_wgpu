#![feature(random)]
mod run;
mod state;
mod texture;
mod camera;
mod instance;
mod model;
mod resources;
mod light;
mod renderer;
mod compute;

use run::run;

fn main() {
    pollster::block_on(run());
}
