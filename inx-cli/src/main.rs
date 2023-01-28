use inx::{Simulator, World};

use crate::ui::info;

mod ui;

fn main() {
    info!("Starting version {}", env!("CARGO_PKG_VERSION"));
    info!("Creating world");
    let world = World::new(10, 2000);
    let mut sim = Simulator::new(world);

    while let Some(res) = sim.step() {
        info!("World: ");
        info!(" Population: {}", sim.world.peoples.len());
        info!(" Year: {}", sim.world.year);
        info!(" News: {:?}", sim.history.pop());

        if sim.world.peoples.len() == 0 {
            break;
        }
    }

    info!("History: {:#?}", sim.history);
}
