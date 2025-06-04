use bevy::prelude::*;

use crate::gameplay::{
    bow::Quiver,
    level::LevelState,
    sphere::{SpawnSphere, SphereType},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(LevelState::Debug), setup_debug_level);
}

fn setup_debug_level(mut commands: Commands, mut quiver: ResMut<Quiver>) {
    quiver.set_arrow_count(Some(1));

    commands.trigger(SpawnSphere::new(Vec2::new(10., 0.), SphereType::Multiplier));

    let l2 = 18.;
    let l2y = 6.;
    // behind it
    commands.trigger(SpawnSphere::new(Vec2::new(l2, l2y), SphereType::Multiplier));
    commands.trigger(SpawnSphere::new(Vec2::new(l2, 0.), SphereType::Multiplier));
    commands.trigger(SpawnSphere::new(
        Vec2::new(l2, -l2y),
        SphereType::Multiplier,
    ));

    commands.trigger(SpawnSphere::new(Vec2::new(24., 7.), SphereType::Normal));
    commands.trigger(SpawnSphere::new(Vec2::new(21., 3.), SphereType::Exploder));
    commands.trigger(SpawnSphere::new(Vec2::new(27., -2.), SphereType::Exploder));
    commands.trigger(SpawnSphere::new(Vec2::new(28., 3.), SphereType::Multiplier));
    commands.trigger(SpawnSphere::new(Vec2::new(37., 8.5), SphereType::Exploder));
    commands.trigger(SpawnSphere::new(
        Vec2::new(42., 8.8),
        SphereType::TimeFreeze,
    ));

    commands.trigger(SpawnSphere::new(Vec2::new(-9., 8.5), SphereType::Normal));
}
