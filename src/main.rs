mod run;
mod state;
mod buffer;
mod texture;
mod camera;

use run::run;

fn main() {
    pollster::block_on(run());
}
