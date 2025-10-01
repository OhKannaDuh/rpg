use crate::components::move_intent::MoveIntent;
use crate::data::config::TILE_SIZE;
use crate::data::world::MapLayer;
use crate::entities::player::*;
use crate::modules::player::plugin::PlayerPlugin;
use crate::modules::world::RpgWorld;
use crate::prelude::*;

#[add_system(schedule = OnEnter(RpgState::WorldLoaded), plugin = PlayerPlugin)]
fn spawn_player(mut commands: Commands, world_query: Query<&RpgWorld>) {
    info!("Spawning player...");

    let Ok(world) = world_query.single() else {
        warn!("No RpgWorld found when spawning player");
        return;
    };

    let Some(rpg_level) = &world.active_level else {
        return;
    };

    let nav = &rpg_level.nav;

    let tile = IVec2::new(24, 40);
    let pos = rpg_level.tile_center(tile);

    commands.spawn((
        Name::new("Player"),
        Player,
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..Default::default()
        },
        MoveIntent(None),
        GlobalTransform::from_xyz(pos.x, pos.y, MapLayer::Entities.z()),
        Transform::from_xyz(pos.x, pos.y, MapLayer::Entities.z()),
    ));
}
