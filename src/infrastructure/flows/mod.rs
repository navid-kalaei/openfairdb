mod archive_comments;
mod archive_events;
mod archive_ratings;
mod change_user_role;
mod create_event;
mod create_place;
mod create_rating;
mod reset_password;
mod review_places;
mod update_event;
mod update_place;

pub mod prelude {
    pub use super::{
        archive_comments::*, archive_events::*, archive_ratings::*, change_user_role::*,
        create_event::*, create_place::*, create_rating::*, reset_password::*, review_places::*,
        update_event::*, update_place::*,
    };
}

pub type Result<T> = std::result::Result<T, error::AppError>;

pub(crate) use super::{db::sqlite, error};
pub(crate) use crate::core::{prelude::*, usecases};

#[cfg(test)]
pub(crate) mod tests;
