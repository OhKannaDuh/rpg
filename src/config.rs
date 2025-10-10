#![allow(unused)]

use bevy_rapier2d::prelude::{CollisionGroups, Group};

use crate::prelude::GameAppExt;
pub const TILE_SIZE: f32 = 16.0;
pub const TILES_PER_CHUNK: f32 = 32.0;
pub const PIXELS_PER_CHUNK: f32 = TILE_SIZE * TILES_PER_CHUNK;

pub const TILE_SIZE_I64: i64 = 16;
pub const TILES_PER_CHUNK_I64: i64 = 32;
pub const PIXELS_PER_CHUNK_I64: i64 = TILE_SIZE_I64 * TILES_PER_CHUNK_I64;

pub const TILE_SIZE_U32: u32 = 16;
pub const TILES_PER_CHUNK_U32: u32 = 32;
pub const PIXELS_PER_CHUNK_U32: u32 = TILE_SIZE_U32 * TILES_PER_CHUNK_U32;

pub const PIXELS_PER_METER: f32 = 32.0;
pub const TILES_PER_METER: f32 = PIXELS_PER_METER / TILE_SIZE;
pub const METERS_PER_TILE: f32 = TILE_SIZE / PIXELS_PER_METER;

pub const DEFAULT_ACTOR_COLLIDER_RADIUS: f32 = TILE_SIZE * METERS_PER_TILE * 0.9;
pub const DEFAULT_ACTOR_RESTITUTION: f32 = 0.7;
pub const DEFAULT_ACTOR_SPEED: f32 = PIXELS_PER_METER * 3.0;

pub const VIEWPORT_HEIGHT: f32 = 384.0;

pub const DEFAULT_WORLD_IDENTIFIER: &str = "TESTING_WORLD";

pub const ENTITY_COLLISION_GROUP: Group = Group::GROUP_1;
pub const WORLD_COLLISION_GROUP: Group = Group::GROUP_2;

pub const ENTITY_COLLISION: CollisionGroups =
    CollisionGroups::new(ENTITY_COLLISION_GROUP, WORLD_COLLISION_GROUP);

pub const WORLD_COLLISION: CollisionGroups =
    CollisionGroups::new(WORLD_COLLISION_GROUP, ENTITY_COLLISION_GROUP);
