// use crate::data::config::*;
// use crate::modules::world::ldtk::LdtkMap;
// use bevy::{
//     log::info,
//     math::{UVec2, Vec2},
// };
// use ldtk_rust::Level;
// use thiserror::Error;

// #[derive(Debug, Clone, Default)]
// pub struct MapTransitionDestination {
//     pub map: String,
//     pub position: Vec2,
// }

// #[derive(Debug, Clone, Default)]
// pub struct MapTransition {
//     pub position: Vec2,
//     pub size: UVec2,
//     pub destination: MapTransitionDestination,
// }

// #[derive(Debug, Error)]
// pub enum MapTransitionError {
//     #[error("entity has identifier '{0}', expected '{1}'")]
//     WrongIdentifier(String, &'static str),

//     #[error("entity missing DESTINATION field")]
//     MissingDestinationField,

//     #[error("DESTINATION field is not an EntityRef object")]
//     DestinationNotObject,

//     #[error("DESTINATION missing key '{0}'")]
//     DestinationMissingKey(&'static str),

//     #[error("Level with iid '{0}' not found")]
//     LevelNotFound(String),

//     #[error("Layer with iid '{0}' not found in level '{1}'")]
//     LayerNotFound(String, String),

//     #[error("Entity with iid '{0}' not found in layer '{1}'")]
//     EntityNotFound(String, String),

//     #[error("destination entity has identifier '{0}', expected '{1}'")]
//     WrongDestinationIdentifier(String, &'static str),
// }

// impl MapTransition {
//     pub fn create(
//         entity: &ldtk_rust::EntityInstance,
//         map: &LdtkMap,
//         current_level: &Level,
//     ) -> Result<Self, MapTransitionError> {
//         if entity.identifier != ENTITY_MAP_TRANSITION_ID {
//             return Err(MapTransitionError::WrongIdentifier(
//                 entity.identifier.clone(),
//                 ENTITY_MAP_TRANSITION_ID,
//             ));
//         }

//         let half_w = current_level.px_wid as f32 * 0.5;
//         let half_h = current_level.px_hei as f32 * 0.5;
//         let src_w = entity.width as f32;
//         let src_h = entity.height as f32;
//         let src_cx_px = entity.px[0] as f32 + src_w * 0.5;
//         let src_cy_px = entity.px[1] as f32 + src_h * 0.5;
//         let src_center = Vec2::new(src_cx_px - half_w, half_h - src_cy_px);
//         let src_size = UVec2::new(entity.width as u32, entity.height as u32);

//         let field = entity
//             .field_instances
//             .iter()
//             .find(|f| f.identifier == FIELD_DESTINATION_ID)
//             .ok_or(MapTransitionError::MissingDestinationField)?;

//         let obj = field
//             .value
//             .as_ref()
//             .and_then(|v| v.as_object())
//             .ok_or(MapTransitionError::DestinationNotObject)?;

//         let level_iid = obj
//             .get("levelIid")
//             .and_then(|v| v.as_str())
//             .ok_or(MapTransitionError::DestinationMissingKey("levelIid"))?
//             .to_owned();

//         let layer_iid = obj
//             .get("layerIid")
//             .and_then(|v| v.as_str())
//             .ok_or(MapTransitionError::DestinationMissingKey("layerIid"))?
//             .to_owned();

//         let entity_iid = obj
//             .get("entityIid")
//             .and_then(|v| v.as_str())
//             .ok_or(MapTransitionError::DestinationMissingKey("entityIid"))?
//             .to_owned();

//         let level = map
//             .project
//             .levels
//             .iter()
//             .find(|lvl| lvl.iid == level_iid)
//             .ok_or_else(|| MapTransitionError::LevelNotFound(level_iid.clone()))?;

//         let level_name = if !level.identifier.is_empty() {
//             level.identifier.clone()
//         } else {
//             // fallback to IID if no name
//             level.iid.clone()
//         };

//         let layers = level.layer_instances.as_deref().unwrap_or(&[]);

//         let layer = layers
//             .iter()
//             .find(|ly| ly.iid == layer_iid)
//             .ok_or_else(|| {
//                 MapTransitionError::LayerNotFound(layer_iid.clone(), level_name.clone())
//             })?;

//         let dest_entity = layer
//             .entity_instances
//             .iter()
//             .find(|e| e.iid == entity_iid)
//             .ok_or_else(|| {
//                 MapTransitionError::EntityNotFound(entity_iid.clone(), layer_iid.clone())
//             })?;

//         if dest_entity.identifier != ENTITY_MAP_TRANSITION_DESTINATION_ID {
//             return Err(MapTransitionError::WrongDestinationIdentifier(
//                 dest_entity.identifier.clone(),
//                 ENTITY_MAP_TRANSITION_DESTINATION_ID,
//             ));
//         }

//         let half_w = level.px_wid as f32 * 0.5;
//         let half_h = level.px_hei as f32 * 0.5;
//         let cx = dest_entity.px[0] as f32 - half_w;
//         let cy = half_h - dest_entity.px[1] as f32;

//         info!(
//             "Transition destination level: {} (IID: {}), half_size: ({}, {}), center: ({}, {})",
//             level_name, level_iid, half_w, half_h, cx, cy
//         );

//         Ok(MapTransition {
//             position: src_center,
//             size: src_size,
//             destination: MapTransitionDestination {
//                 map: level_iid,
//                 position: Vec2::new(cx, cy),
//             },
//         })
//     }

//     pub fn contains_point(&self, point: Vec2) -> bool {
//         let half_size = self.size.as_vec2() * 0.5;
//         let min = self.position - half_size;
//         let max = self.position + half_size;

//         point.x >= min.x && point.x <= max.x && point.y >= min.y && point.y <= max.y
//     }
// }
