use mongodb::Database;

use self::{tracks::TrackRepository, users::UserRepository};

pub mod tracks;
pub mod users;

/// Overall struct that houses all entity repos.
#[derive(Debug, Clone)]
pub struct Repository {
    pub tracks: TrackRepository,
    pub users: UserRepository,
}

impl Repository {
    pub fn new(db: Database) -> Self {
        Self {
            tracks: TrackRepository::new(&db),
            users: UserRepository::new(&db),
        }
    }
}
