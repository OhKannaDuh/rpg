prelude!();
plugins!(
    (physics, PhysicsPlugin),
    (pathfinding, PathfindingPlugin),
    (map, MapPlugin)
);

pub struct WorldPlugin;
game_module_build!(WorldPlugin);

impl GameModule for WorldPlugin {
    fn plugins(&self, app: &mut App) {
        app.add_plugins((PhysicsPlugin, PathfindingPlugin, MapPlugin));
    }
}
