mod add_entry;
mod add_rating;
mod update_entry;

pub mod prelude {
    pub use super::{add_entry::*, add_rating::*, update_entry::*};
}

pub type Result<T> = std::result::Result<T, error::AppError>;

pub(crate) use super::{db::sqlite, error, notify};

pub(crate) use crate::core::{prelude::*, usecases};