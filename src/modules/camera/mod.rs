prelude!();
use bevy::{camera::ScalingMode, window::PrimaryWindow};

use crate::modules::world::map::WorldBounds;

pub struct CameraPlugin;
game_module_build!(CameraPlugin);

impl GameModule for CameraPlugin {
    fn systems(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), spawn_camera);

        app.on_playing_game_update(follow_foci);
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct MainCamera {
    pub speed: f32,
    pub max_speed: f32,
    pub start_scale_on_distance: f32,
    pub max_speed_on_distance: f32,
    pub snap_at_distance: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
#[reflect(Component)]
#[require(Transform)]
pub struct CameraFocus;

pub fn spawn_camera(mut commands: Commands) {
    info!("Spawning camera...");
    commands.spawn((
        Name::new("Main Camera"),
        MainCamera {
            speed: TILE_SIZE,
            max_speed: TILE_SIZE * TILE_SIZE,
            start_scale_on_distance: TILE_SIZE * 2.0,
            max_speed_on_distance: PIXELS_PER_CHUNK / 2.0,
            snap_at_distance: PIXELS_PER_CHUNK,
        },
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: VIEWPORT_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn follow_foci(
    mut camera: Single<(&mut Transform, &MainCamera, &Projection), Without<CameraFocus>>,
    foci: Query<&Transform, With<CameraFocus>>,
    bounds: Res<WorldBounds>,
    time: Res<Time>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    if foci.is_empty() {
        return;
    }

    let mut focus = Vec3::ZERO;
    let count = foci.iter().count() as f32;

    for transform in foci.iter() {
        focus += transform.translation;
    }

    focus /= count;

    let (mut camera_transform, camera, projection) = camera.into_inner();

    let distance = camera_transform.translation.distance(focus);
    if distance >= camera.snap_at_distance {
        camera_transform.translation = focus;
        return;
    }

    let t = ((distance - camera.start_scale_on_distance)
        / (camera.max_speed_on_distance - camera.start_scale_on_distance))
        .clamp(0.0, 1.0);

    let speed = camera.speed + t * (camera.max_speed - camera.speed);

    camera_transform.translation = camera_transform
        .translation
        .lerp(focus, 0.1 * speed * time.delta_secs());

    if let Projection::Orthographic(ortho) = projection {
        let half_w = (ortho.area.max.x - ortho.area.min.x) * 0.5;
        let half_h = (ortho.area.max.y - ortho.area.min.y) * 0.5;

        let map_min = bounds.center - Vec2::new(bounds.width / 2.0, bounds.height / 2.0);
        let map_size = Vec2::new(bounds.width, bounds.height);
        let map_max = map_min + map_size;

        if map_size.x <= half_w * 2.0 {
            camera_transform.translation.x = (map_min.x + map_max.x) * 0.5;
        } else {
            camera_transform.translation.x = camera_transform
                .translation
                .x
                .clamp(map_min.x + half_w, map_max.x - half_w);
        }

        if map_size.y <= half_h * 2.0 {
            camera_transform.translation.y = (map_min.y + map_max.y) * 0.5;
        } else {
            camera_transform.translation.y = camera_transform
                .translation
                .y
                .clamp(map_min.y + half_h, map_max.y - half_h);
        }
    }
}
