mod run;
mod state;
mod texture;
mod camera;
mod instance;
mod model;
mod resources;

use run::run;

fn main() {
    pollster::block_on(run());
}
