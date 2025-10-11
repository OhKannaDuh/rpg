prelude!();
public!(core, animator);

use std::hash::Hash;

pub struct AnimationPlugin;
game_module_build!(AnimationPlugin);

impl GameModule for AnimationPlugin {
    fn systems(&self, app: &mut App) {
        app.on_playing_game_update((advance_animators::<String>,));
    }
}

pub fn advance_animators<Key: Eq + Hash + Clone + Send + Sync + 'static>(
    mut query: Query<(&mut Animator<Key>, &mut AnimationBank<Key>, &mut Sprite)>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (mut animator, mut bank, mut sprite) in &mut query {
        if animator.target != animator.current {
            animator.current = animator.target.clone();
            animator.reset_current(&mut bank);
        }

        let Some(clip) = bank.clips.get_mut(&animator.current) else {
            continue;
        };

        if clip.frames.is_empty() {
            continue;
        }

        if clip.frames[clip.frame].duration <= 0.0 {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = clip.frames[clip.frame].atlas_index;
            }

            continue;
        }

        animator.elapsed += delta * animator.speed;

        while clip.frames[clip.frame].duration > 0.0
            && animator.elapsed >= clip.frames[clip.frame].duration
        {
            animator.elapsed -= clip.frames[clip.frame].duration;
            clip.frame += 1;
            if clip.frame >= clip.frames.len() {
                if animator.looping {
                    clip.frame = 0;
                } else {
                    clip.frame = clip.frames.len() - 1;
                    break;
                }
            }
        }

        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = clip.frames[clip.frame].atlas_index;
        }
    }
}
