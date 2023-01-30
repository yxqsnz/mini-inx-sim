use inx::{Simulator, Step, World};

mod ui;

// info!("World: ");
// info!(" R. Population: {}", sim.world.peoples.len());
// info!(" Couples: {}", sim.world.couples.len());
// info!(" Year: {}", sim.world.year);
// info!(" News: {:?}", sim.history.pop());
fn main() {
    env_logger::init();
    info!("Starting version {}", env!("CARGO_PKG_VERSION"));
    info!("Creating world");

    loop {
        let world = World::new(25);
        let mut sim = Simulator::new(world, 5);

        for _ in 0..1000 {
            sim.step();
        }

        if sim.created_generations < 5 {
            continue;
        } else {
            info!("Done");

            println!("{:#?}", sim.world.couples);
            break;
        }
    }
}
