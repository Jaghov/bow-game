use bevy::prelude::*;

use std::{f32::consts::FRAC_PI_2, time::Duration};

use crate::{
    Screen,
    gameplay::{arrow::ArrowAssets, bow::BowAssets, sph::SphereAssets},
    world::backdrop::{BACKDROP_OFFSET, BLOCK_LEN},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_items)
        .add_systems(OnEnter(Screen::Transition), start_transition_clock)
        .add_systems(
            PreUpdate,
            update_transition_clock.run_if(in_state(Screen::Transition)),
        )
        .add_systems(
            Update,
            despawn_on_complete.run_if(in_state(Screen::Transition)),
        )
        .add_systems(OnExit(Screen::Transition), remove_transition_clock)
        .add_systems(Update, set_locations.run_if(in_state(Screen::Title)));
    //todo
}

#[derive(Resource)]
struct TransitionClock(Timer);
impl Default for TransitionClock {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(300), TimerMode::Once))
    }
}
fn start_transition_clock(mut commands: Commands) {
    commands.init_resource::<TransitionClock>();
}
fn remove_transition_clock(mut commands: Commands) {
    commands.remove_resource::<TransitionClock>();
}
fn update_transition_clock(mut clock: ResMut<TransitionClock>, time: Res<Time>) {
    clock.0.tick(time.delta());
}
fn despawn_on_complete(
    mut commands: Commands,
    clock: Res<TransitionClock>,
    props: Query<Entity, With<Prop>>,
) {
    if !clock.0.finished() {
        return;
    }
    for prop in props {
        commands.entity(prop).despawn();
    }
}

#[derive(Component)]
struct Prop;

// note these are different from the game components
#[derive(Component)]
struct Arrow(usize);

#[derive(Component)]
struct Bow;

#[derive(Component)]
struct Sphere(usize);

fn spawn_items(
    mut commands: Commands,
    bow_assets: Res<BowAssets>,
    arrow_assets: Res<ArrowAssets>,
    sphere: Res<SphereAssets>,
) {
    commands.spawn((Bow, Prop, SceneRoot(bow_assets.scene.clone())));
    for i in (0..5) {
        commands.spawn((Arrow(i), Prop, SceneRoot(arrow_assets.glowing.clone())));
    }

    let mesh = Mesh3d(sphere.mesh.clone());

    commands.spawn((
        Sphere(0),
        mesh.clone(),
        Prop,
        MeshMaterial3d(sphere.normal.clone()),
    ));
    commands.spawn((
        Sphere(1),
        Prop,
        mesh.clone(),
        MeshMaterial3d(sphere.multiplier.clone()),
    ));
    commands.spawn((
        Sphere(2),
        Prop,
        mesh,
        MeshMaterial3d(sphere.time_freeze.clone()),
    ));

    //todo
}

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn set_locations(
    mut bow: Query<&mut Transform, (With<Bow>, Without<Sphere>)>,
    mut arrows: Query<(&mut Transform, &Arrow), (Without<Bow>, Without<Sphere>)>,
    mut spheres: Query<(&mut Transform, &Sphere), (Without<Bow>, Without<Arrow>)>,
    time: Res<Time>,
) {
    let mut bow = bow.single_mut().unwrap();

    *bow = Transform::from_xyz(
        BLOCK_LEN * 7. - 2.85,
        BLOCK_LEN * 4. + 1.2,
        -1.5 - BACKDROP_OFFSET,
    )
    .with_rotation(Quat::from_euler(
        EulerRot::XYX,
        FRAC_PI_2 - 0.15,
        FRAC_PI_2 + 0.02,
        -0.2,
    ))
    .with_scale(Vec3::splat(0.5));

    for (mut arrow_trns, arrow) in &mut arrows {
        let offset = arrow.0 as f32 * 0.2;
        *arrow_trns = Transform::from_xyz(
            BLOCK_LEN * 7. - 2.83,
            BLOCK_LEN * 4. + 0.7 - offset,
            -1.65 - BACKDROP_OFFSET,
        )
        .with_rotation(Quat::from_euler(
            EulerRot::XYX,
            FRAC_PI_2 - 0.15,
            FRAC_PI_2 + 0.02,
            -0.2,
        ))
        .with_scale(Vec3::splat(0.5));
    }

    for (mut trns, sphere) in &mut spheres {
        use std::f32::consts::PI;

        let offset = sphere.0 as f32 * 1.2;

        let time = 2. * (time.elapsed_secs() + (2. * PI * (sphere.0 as f32 + 1.) / 3.));
        let z_offset = time.cos() * 0.05;

        *trns = Transform::from_xyz(
            BLOCK_LEN * 7. - 1.5 + offset,
            BLOCK_LEN * 4. + 2.5,
            -1.65 - BACKDROP_OFFSET + z_offset,
        )
        .with_rotation(Quat::from_euler(
            EulerRot::XYX,
            FRAC_PI_2 - 0.15,
            FRAC_PI_2 + 0.02,
            -0.2,
        ))
        .with_scale(Vec3::splat(0.3));
    }

    //todod
}
