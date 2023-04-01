use mongodb::Database;

use self::{blocks::BlockRepository, tracks::TrackRepository, users::UserRepository};

pub mod blocks;
pub mod entities;
pub mod playlists;
pub mod tracks;
pub mod users;

/// Overall struct that houses all entity repos.
#[derive(Debug, Clone)]
pub struct Repository {
    pub tracks: TrackRepository,
    pub users: UserRepository,
    pub blocks: BlockRepository,
}

impl Repository {
    pub fn new(db: Database) -> Self {
        Self {
            tracks: TrackRepository::new(&db),
            users: UserRepository::new(&db),
            blocks: BlockRepository::new(&db),
        }
    }
}
