use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("failed to read file {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse JSON from {path}: {source}")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
}

/// Load and parse JSON from a file using a buffered reader.
pub fn load_json_from_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, LoadError> {
    let path = path.as_ref();
    let file = std::fs::File::open(path).map_err(|source| LoadError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).map_err(|source| LoadError::Json {
        path: path.to_path_buf(),
        source,
    })
}

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

#[derive(Debug, Clone, Copy)]
pub struct PaginationMetadata {
    pub next_page: Option<usize>,
    pub per_page: usize,
}

pub fn paginate<T: Clone>(items: &[T], pagination: Pagination) -> (Vec<T>, PaginationMetadata) {
    let page = pagination.page();
    let per_page = pagination.per_page();

    let start = (page - 1) * per_page;
    let items_slice = if start >= items.len() {
        &[]
    } else {
        let end = (start + per_page).min(items.len());
        &items[start..end]
    };

    let has_next = start + per_page < items.len();
    let metadata = PaginationMetadata {
        next_page: if has_next { Some(page + 1) } else { None },
        per_page,
    };

    (items_slice.to_vec(), metadata)
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
    fn test_paginate_first_page_with_next() {
        let items = vec![1, 2, 3, 4, 5];
        let pagination = Pagination { page: Some(1), per_page: Some(2) };
        let (res, meta) = paginate(&items, pagination);
        assert_eq!(res, vec![1, 2]);
        assert_eq!(meta.next_page, Some(2));
        assert_eq!(meta.per_page, 2);
    }

    #[test]
    fn test_paginate_middle_page_with_next() {
        let items = vec![1, 2, 3, 4, 5];
        let pagination = Pagination { page: Some(2), per_page: Some(2) };
        let (res, meta) = paginate(&items, pagination);
        assert_eq!(res, vec![3, 4]);
        assert_eq!(meta.next_page, Some(3));
    }

    #[test]
    fn test_paginate_last_page_with_partial_results() {
        let items = vec![1, 2, 3, 4, 5];
        let pagination = Pagination { page: Some(3), per_page: Some(2) };
        let (res, meta) = paginate(&items, pagination);
        assert_eq!(res, vec![5]);
        assert_eq!(meta.next_page, None);
    }

    #[test]
    fn test_paginate_beyond_available_pages() {
        let items = vec![1, 2, 3, 4, 5];
        let pagination = Pagination { page: Some(4), per_page: Some(2) };
        let (res, meta) = paginate(&items, pagination);
        assert_eq!(res, Vec::<i32>::new());
        assert_eq!(meta.next_page, None);
    }

    #[test]
    fn test_paginate_huge_page_number() {
        let items = vec![1, 2, 3, 4, 5];
        let pagination = Pagination { page: Some(100), per_page: Some(2) };
        let (res, meta) = paginate(&items, pagination);
        assert_eq!(res, Vec::<i32>::new());
        assert_eq!(meta.next_page, None);
    }
}
