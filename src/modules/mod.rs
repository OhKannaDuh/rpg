prelude!();
public!(game_module);
plugins!(
    (world, WorldPlugin),
    (actor, ActorPlugin),
    (camera, CameraPlugin),
    (dev, DevPlugin)
);

pub struct ModulePlugin;
game_module_build!(ModulePlugin);

impl GameModule for ModulePlugin {
    fn plugins(&self, app: &mut App) {
        app.add_plugins((WorldPlugin, ActorPlugin, CameraPlugin));

        if cfg!(debug_assertions) {
            app.add_plugins((DevPlugin,));
        }
    }
}
