use crate::{data::map_transition::MapTransition, prelude::*};

#[derive(Component, Debug, Clone, Default)]
pub struct MapTransitions(pub Vec<MapTransition>);
