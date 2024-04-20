mod run;
mod state;

use run::run;

fn main() {
    pollster::block_on(run());
}
