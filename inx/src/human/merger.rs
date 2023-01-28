use std::iter::repeat_with;

use rand::random;

use crate::util::RemovePercent;

use super::Gene;

pub fn merge_genes(a: &[Gene], b: &[Gene]) -> Vec<Gene> {
    let mut output = vec![];

    output.extend_from_slice(a);
    output.extend_from_slice(b);

    output = output.remove_percent(25, |p| !p.dominant);
    output = output.remove_percent(5, |p| p.dominant);
    output.extend(repeat_with(random).take(20).collect::<Vec<_>>());

    output
}
