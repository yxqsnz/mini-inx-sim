use inx::{Simulator, World};

mod ui;

fn main() {
    info!("Starting version {}", env!("CARGO_PKG_VERSION"));
    info!("Creating world");
    let world = World::new(5, 2);
    let mut sim = Simulator::new(world);

    sim.prepare();

    for _ in 0..100 {
        sim.step();
        info!("World: ");
        info!(" R. Population: {}", sim.world.peoples.len());
        info!(" Couples: {}", sim.world.couples.len());
        info!(" Year: {}", sim.world.year);
        info!(" News: {:?}", sim.history.pop());

        if sim.world.peoples.len() == 0 {
            break;
        }
    }

    info!("History: {:#?}", sim.history);

    println!("{:#?}", sim.world.couples)
}
