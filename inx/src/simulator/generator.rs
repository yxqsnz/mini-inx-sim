use crate::{
    human::Couple,
    util::{extract_two, VecTake},
    Simulator, Step, World,
};

pub fn build_generation(coup_a: &mut Couple, coup_b: &mut Couple, max_generations: u64) -> u64 {
    let mut found = 0;

    if let Some((fm_a, fm_b)) = extract_two(coup_a.children.as_slice(), coup_b.children.as_slice())
    {
        log::debug!("Uuh! Found a new couple.");

        log::debug!("Target (fm_a): {fm_a:#?}");
        log::debug!("Target (fm_b): {fm_b:#?}");

        let mut sim_a = Simulator::new(World::with_peoples(fm_a), 1);
        let mut sim_b = Simulator::new(World::with_peoples(fm_b), 1);

        loop {
            sim_a.step();

            if sim_a.step == Step::GenerationgGenerations {
                break;
            }
        }

        loop {
            sim_b.step();

            if sim_b.step == Step::GenerationgGenerations {
                break;
            }
        }

        log::debug!(
            "Found {}/{} families from inner simulations",
            sim_a.world.couples.len(),
            sim_b.world.couples.len()
        );

        if let Some(couple) = sim_a.world.couples.take(0) {
            coup_a.descedent = Some(Box::new(couple));
            coup_a.descedent_count += 1;
            found += 1;
        }

        if let Some(couple) = sim_b.world.couples.take(0) {
            coup_b.descedent = Some(Box::new(couple));
            coup_b.descedent_count += 1;

            found += 1;
        }

        if coup_a.descedent_count < max_generations && coup_b.descedent_count < max_generations {
            if let (Some(ca), Some(cb)) = (coup_a.descedent.as_mut(), coup_b.descedent.as_mut()) {
                found += build_generation(ca, cb, max_generations);
            }
        }
    }

    found
}
