use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use serde::Deserialize;

pub(crate) fn hash(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

pub const DEFAULT_PAGE: usize = 1;
pub const DEFAULT_PER_PAGE: usize = 30;
pub const MAX_PER_PAGE: usize = 100;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Pagination {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

impl Pagination {
    pub fn page(&self) -> usize {
        self.page.unwrap_or(DEFAULT_PAGE).max(1)
    }

    pub fn per_page(&self) -> usize {
        self.per_page.unwrap_or(DEFAULT_PER_PAGE).clamp(1, MAX_PER_PAGE)
    }
}

pub fn paginate<T: Clone>(items: &[T], pagination: Pagination) -> Vec<T> {
    let page = pagination.page();
    let per_page = pagination.per_page();

    let start = (page - 1) * per_page;
    if start >= items.len() {
        return Vec::new();
    }

    let end = (start + per_page).min(items.len());
    items[start..end].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_defaults() {
        let p = Pagination {
            page: None,
            per_page: None,
        };
        assert_eq!(p.page(), DEFAULT_PAGE);
        assert_eq!(p.per_page(), DEFAULT_PER_PAGE);
    }

    #[test]
    fn test_pagination_custom() {
        let p = Pagination {
            page: Some(2),
            per_page: Some(50),
        };
        assert_eq!(p.page(), 2);
        assert_eq!(p.per_page(), 50);
    }

    #[test]
    fn test_pagination_limits() {
        let p = Pagination {
            page: Some(0),
            per_page: Some(200),
        };
        assert_eq!(p.page(), 1);
        assert_eq!(p.per_page(), MAX_PER_PAGE);

        let p2 = Pagination {
            page: Some(1),
            per_page: Some(0),
        };
        assert_eq!(p2.per_page(), 1);
    }

    #[test]
    fn test_paginate_logic() {
        let items = vec![1, 2, 3, 4, 5];

        // Page 1, per_page 2 -> [1, 2]
        let p1 = Pagination { page: Some(1), per_page: Some(2) };
        assert_eq!(paginate(&items, p1), vec![1, 2]);

        // Page 2, per_page 2 -> [3, 4]
        let p2 = Pagination { page: Some(2), per_page: Some(2) };
        assert_eq!(paginate(&items, p2), vec![3, 4]);

        // Page 3, per_page 2 -> [5]
        let p3 = Pagination { page: Some(3), per_page: Some(2) };
        assert_eq!(paginate(&items, p3), vec![5]);

        // Page 4, per_page 2 -> []
        let p4 = Pagination { page: Some(4), per_page: Some(2) };
        assert_eq!(paginate(&items, p4), Vec::<i32>::new());

        // Huge page
        let ph = Pagination { page: Some(100), per_page: Some(2) };
        assert_eq!(paginate(&items, ph), Vec::<i32>::new());
    }
}
