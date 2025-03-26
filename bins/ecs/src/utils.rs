use std::collections::HashMap;
use std::hash::Hash;

pub fn get_two_mut<K: Eq + Hash + Copy + PartialOrd, V>(
    map: &mut HashMap<K, V>,
    a: K,
    b: K,
) -> Option<(&mut V, &mut V)> {
    if a == b {
        return None;
    }

    let (first, second) = if a < b { (a, b) } else { (b, a) };

    let mut first_ptr = None;
    let mut second_ptr = None;

    for (k, v) in map {
        if *k == first {
            first_ptr = Some(v as *mut V);
        } else if *k == second {
            second_ptr = Some(v as *mut V);
        }
    }

    unsafe { Some((&mut *first_ptr?, &mut *second_ptr?)) }
}
