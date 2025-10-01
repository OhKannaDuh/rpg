use crate::modules::world::ldtk::{LdtkMapConfig, LdtkMapHandle};
use crate::modules::world::map_transitions::MapTransitions;
use crate::prelude::*;

#[derive(Default, Bundle)]
pub struct LdtkMapBundle {
    pub ldtk_map: LdtkMapHandle,
    pub ldtk_map_config: LdtkMapConfig,
    pub transitions: MapTransitions,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
}
