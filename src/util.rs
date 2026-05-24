use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub(crate) fn hash(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}
