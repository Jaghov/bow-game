use bevy::{animation::RepeatAnimation, prelude::*, scene::SceneInstanceReady};

use crate::AppSystems;

use super::{Bow, BowArrow, BowAssets};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_bow_pull.in_set(AppSystems::Update));
}

#[derive(Component)]
struct BowAnimation {
    player: Entity,
    index: AnimationNodeIndex,
}
// an observer for the spawned bow. not global.
pub(super) fn setup_animations(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    assets: Res<BowAssets>,
    children: Query<&Children>,
    animation_players: Query<(), With<AnimationPlayer>>,
    scene_root: Query<Entity, With<Bow>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let target = trigger.target();
    let Some(animation_player_entity) = children
        .iter_descendants(target)
        .find(|child| animation_players.get(*child).is_ok())
    else {
        error!("No animation player for scene!");
        return;
    };

    let bow = scene_root.get(trigger.target()).unwrap();

    let (animation_graph, node) = AnimationGraph::from_clip(assets.pull_string.clone());

    let graph_handle = graphs.add(animation_graph);

    commands
        .entity(animation_player_entity)
        .insert((AnimationGraphHandle(graph_handle)));

    commands.entity(bow).insert(BowAnimation {
        player: animation_player_entity,
        index: node,
    });
}

fn update_bow_pull(
    bow_pull: Query<(&BowAnimation, Option<&BowArrow>)>,
    mut animations: Query<&mut AnimationPlayer>,
) {
    for (anim_props, bow_arrow) in bow_pull {
        let pull_strength = bow_arrow
            .map(|bow_arrow| bow_arrow.strength())
            .unwrap_or_default();

        let mut anim_player = animations.get_mut(anim_props.player).unwrap();

        if !anim_player.is_playing_animation(anim_props.index) {
            let pull_animation = anim_player.play(anim_props.index);
            pull_animation
                .set_repeat(RepeatAnimation::Never)
                .set_speed(0.);
        }

        let pull_animation = anim_player.animation_mut(anim_props.index).unwrap();
        //the animation is at max at 0.83. This may be an off-by-one interp problem.

        const MAX: f32 = 0.83;

        let strength = pull_strength * MAX;
        pull_animation.seek_to(strength);
    }
}
