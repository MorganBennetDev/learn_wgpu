mod run;
mod state;
mod buffer;

use run::run;

fn main() {
    pollster::block_on(run());
}
