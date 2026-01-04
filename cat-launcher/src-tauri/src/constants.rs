use std::num::{NonZeroU16, NonZeroUsize};

pub const MAX_BACKUPS: NonZeroUsize = NonZeroUsize::new(5).unwrap();
pub const PARALLEL_REQUESTS: NonZeroU16 = NonZeroU16::new(4).unwrap();
