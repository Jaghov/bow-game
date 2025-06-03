use bevy::{input::common_conditions::input_just_released, prelude::*};

use super::{Bow, BowAssets, animation};
use crate::{
    gameplay::{
        GameState,
        arrow::{FireArrow, ReadyArrow},
        bow::BowArrow,
        timefreeze::FreezeLocation,
    },
    world::GAME_PLANE,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::TimeFreeze), spawn_bow)
        .add_systems(
            Update,
            on_mouse_up.run_if(
                in_state(GameState::TimeFreeze).and(input_just_released(MouseButton::Left)),
            ),
        );
}

pub fn spawn_bow(mut commands: Commands, bow_assets: Res<BowAssets>, freeze: Res<FreezeLocation>) {
    info!("Spawning bow");
    let bow = commands
        .spawn((
            Name::new("Frozen Bow"),
            Bow,
            StateScoped(GameState::TimeFreeze),
            SceneRoot(bow_assets.scene.clone()),
            Transform::from_xyz(freeze.location.x, freeze.location.y, GAME_PLANE),
        ))
        .observe(animation::setup_animations)
        .id();

    commands.trigger(ReadyArrow::for_bow(bow));
}
fn on_mouse_up(mut commands: Commands, bow_arrows: Query<&BowArrow>) {
    for arrow in &bow_arrows {
        commands.trigger_targets(FireArrow, arrow.arrow());
    }
}
