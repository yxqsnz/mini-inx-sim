use std::iter::repeat_with;

use rand::random;

use crate::Characteristic;

#[must_use]
#[inline]
pub fn random_characteristics(size: usize) -> Vec<Characteristic> {
    let data = repeat_with(random).take(size);
    data.collect()
}

#[test]
fn rand_charas() {
    let data = random_characteristics(10);
    assert_eq!(data.len(), 10);
}
