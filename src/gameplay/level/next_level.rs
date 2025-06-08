use std::time::Duration;

use bevy::prelude::*;

use crate::{
    Screen,
    gameplay::{
        GameSet, GameState,
        arrow::Arrow,
        bow::{Bow, PrimaryBow},
        level::{
            Level, LevelState, Levels, SPHERE_START_PLANE, WALL_START_PLANE, Walls,
            timer::LevelSetupTimer,
        },
        sphere::Sphere,
    },
    world::{GAME_PLANE, light::SetLightPosition},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(LevelState::NextLevel),
        (init_timer, set_light_position),
    )
    .add_systems(
        Update,
        (update_wall_transform, update_sphere_transform)
            .in_set(GameSet::Update)
            .run_if(in_state(LevelState::NextLevel)),
    )
    .add_systems(
        PostUpdate,
        get_ready_for_next_level_state.run_if(in_state(LevelState::NextLevel)),
    );
}

fn set_light_position(mut commands: Commands) {
    commands.trigger(SetLightPosition::to_gameplay().with_duration(Duration::from_millis(700)));
}
fn init_timer(mut commands: Commands) {
    commands.insert_resource(LevelSetupTimer::new(Duration::from_millis(500)));
}

fn update_wall_transform(
    time: Res<LevelSetupTimer>,
    mut walls: Query<&mut Transform, (With<Walls>, Without<Sphere>)>,
) {
    let mut walls = walls
        .single_mut()
        .expect("No wall for level loading. This is unrecoverable!");

    let progress = time.fraction();
    let eased_progress = progress * progress * (3.0 - 2.0 * progress);

    let wall_z = GAME_PLANE.lerp(WALL_START_PLANE, eased_progress);

    walls.translation.z = wall_z;
}

fn update_sphere_transform(
    time: Res<LevelSetupTimer>,
    mut spheres: Query<&mut Transform, (With<Sphere>, Without<Walls>)>,
) {
    let progress = time.fraction();
    let eased_progress = progress * progress * (3.0 - 2.0 * progress);

    let sphere_z = GAME_PLANE.lerp(SPHERE_START_PLANE, eased_progress);
    for mut sphere in &mut spheres {
        sphere.translation.z = sphere_z;
    }
}

fn get_ready_for_next_level_state(
    mut commands: Commands,
    timer: Res<LevelSetupTimer>,
    mut level_state: ResMut<NextState<LevelState>>,
    spheres: Query<Entity, With<Sphere>>,
    arrows: Query<Entity, With<Arrow>>,
    bows: Query<Entity, (With<Bow>, Without<PrimaryBow>)>,
    walls: Query<Entity, With<Walls>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    level: Res<Level>,
    levels: Res<Levels>,
    mut screen_state: ResMut<NextState<Screen>>,
) {
    if !timer.finished() {
        return;
    }
    for sphere in spheres {
        commands.entity(sphere).try_despawn();
    }
    for walls in walls {
        commands.entity(walls).try_despawn();
    }
    for arrow in arrows {
        commands.entity(arrow).try_despawn();
    }
    for bow in bows {
        commands.entity(bow).try_despawn();
    }

    if level.0 == levels.num_levels() {
        // all levels have been played
        screen_state.set(Screen::Title);
        return;
    }

    if *game_state.get() == GameState::TimeFreeze {
        next_game_state.set(GameState::Playing);
    }

    level_state.set(LevelState::NewLevel);
}
