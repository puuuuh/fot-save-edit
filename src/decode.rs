mod error;
pub(crate) mod primitive;
pub(crate) mod sections;
pub mod saveh;
pub mod campaign_save;
mod shared;
pub(crate) mod stream;

use std::{io::Read, ops::Shr};
