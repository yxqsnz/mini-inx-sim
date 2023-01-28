use std::iter::repeat_with;

use rand::random;

use super::Gene;

pub fn merge_genes(a: &[Gene], b: &[Gene], max_genes: usize) -> Vec<Gene> {
    let mut output = vec![];

    let av = (a.len() as f64 * 0.25) as usize;
    let bv = (b.len() as f64 * 0.25) as usize;

    output.extend_from_slice(&a[0..av]);
    output.extend_from_slice(&b[0..bv]);
    output.extend(repeat_with(random).take(max_genes / 2).collect::<Vec<_>>());

    output
}
