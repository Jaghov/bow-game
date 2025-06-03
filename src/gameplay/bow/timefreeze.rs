use bevy::prelude::*;

use super::{Bow, BowAssets, animation};
use crate::{
    gameplay::{
        GameState,
        arrow::{Arrow, ArrowAssets},
        timefreeze::{FreezeLocation, Frozen},
    },
    world::GAME_PLANE,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::TimeFreeze), spawn_bow);
}

pub fn spawn_bow(
    mut commands: Commands,
    bow_assets: Res<BowAssets>,
    arrow_assets: Res<ArrowAssets>,
    freeze: Res<FreezeLocation>,
    bows: Query<Entity, With<Bow>>,
) {
    for bow in bows {
        commands.entity(bow).insert(Frozen);
    }

    info!("Spawning bow");
    commands
        .spawn((
            Bow,
            SceneRoot(bow_assets.scene.clone()),
            Transform::from_xyz(freeze.location.x, freeze.location.y, GAME_PLANE),
        ))
        .observe(animation::setup_animations);

    commands.spawn((
        Name::new("Frozen Arrow"),
        Arrow::default(),
        SceneRoot(arrow_assets.glowing.clone()),
    ));
}
