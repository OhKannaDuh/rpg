use crate::{modules::ChunkId, prelude::*};

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChunkRoot {
    pub id: ChunkId,
}
