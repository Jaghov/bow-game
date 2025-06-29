use bevy::prelude::*;
use bevy_mod_outline::OutlineVolume;

use std::{f32::consts::FRAC_PI_2, time::Duration};

use crate::{
    Screen,
    asset_tracking::LoadResource,
    gameplay::{
        arrow::ArrowAssets,
        bow::BowAssets,
        sphere::{
            Absorber, Bouncy, Exploder, GravitySphere, Multiplier, Normal, Sphere, SphereAssets,
            TimeFreeze,
        },
    },
    world::{BACKDROP_OFFSET, BLOCK_LEN},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<BolfTitle>()
        .load_resource::<BolfTitle>();

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
        .add_systems(
            Update,
            update_sphere_transforms.run_if(in_state(Screen::Title)),
        );
}

#[derive(Asset, Resource, Reflect, Clone)]
pub struct BolfTitle {
    #[dependency]
    model: Handle<Scene>,
}

impl FromWorld for BolfTitle {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            model: assets.load("models/thumbnail2.glb#Scene0"),
        }
    }
}

#[derive(Resource)]
struct TransitionClock(Timer);
impl Default for TransitionClock {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(600), TimerMode::Once))
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

#[derive(Component)]
struct SphereCount(usize);

fn spawn_items(
    mut commands: Commands,
    bow_assets: Res<BowAssets>,
    arrow_assets: Res<ArrowAssets>,
    sphere: Res<SphereAssets>,
    title: Res<BolfTitle>,
) {
    commands.spawn((
        Prop,
        Transform::from_xyz(
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
        .with_scale(Vec3::splat(0.5)),
        SceneRoot(bow_assets.scene.clone()),
    ));
    commands.spawn((
        Prop,
        Transform::from_xyz(BLOCK_LEN * 7. - 3.2, BLOCK_LEN * 4., 1. - BACKDROP_OFFSET)
            .with_rotation(Quat::from_euler(EulerRot::XYX, FRAC_PI_2, FRAC_PI_2, 0.0)),
        SceneRoot(title.model.clone()),
    ));

    for i in (0..5) {
        let offset = i as f32 * 0.2;
        let transform = Transform::from_xyz(
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

        commands.spawn((Prop, transform, SceneRoot(arrow_assets.glowing.clone())));
    }

    let mesh = Mesh3d(sphere.mesh.clone());
    // need to add initial transforms because these things will collide.
    commands.spawn((
        Name::new("Title Screen Normal Sphere"),
        SphereCount(0),
        Transform::from_xyz(0., 0., -100.),
        Sphere,
        mesh.clone(),
        Prop,
        Normal,
        MeshMaterial3d(sphere.normal.clone()),
    ));
    commands.spawn((
        Name::new("Title Screen Multiplier Sphere"),
        SphereCount(1),
        Sphere,
        Transform::from_xyz(0., 0., -200.),
        Multiplier,
        Prop,
        mesh.clone(),
        MeshMaterial3d(sphere.multiplier.clone()),
    ));
    commands.spawn((
        Name::new("Title Screen TimeFreeze Sphere"),
        SphereCount(2),
        Transform::from_xyz(0., 0., -300.),
        Sphere,
        TimeFreeze,
        Prop,
        mesh.clone(),
        MeshMaterial3d(sphere.time_freeze.clone()),
    ));
    commands.spawn((
        Name::new("Title Screen Absorber Sphere"),
        SphereCount(3),
        Transform::from_xyz(0., 0., -400.),
        Sphere,
        Absorber,
        Prop,
        mesh.clone(),
        MeshMaterial3d(sphere.absorber.clone()),
    ));
    commands.spawn((
        Name::new("Title Screen Gravity Sphere"),
        SphereCount(4),
        Transform::from_xyz(0., 0., -500.),
        Sphere,
        GravitySphere,
        Prop,
        mesh.clone(),
        MeshMaterial3d(sphere.gravity.clone()),
        OutlineVolume {
            visible: true,
            colour: Color::srgb(0.0, 0.0, 0.0),
            width: 25.0,
        },
    ));
    commands.spawn((
        Name::new("Title Screen Bouncy Sphere"),
        SphereCount(5),
        Transform::from_xyz(0., 0., -600.),
        Sphere,
        Bouncy,
        Prop,
        mesh.clone(),
        MeshMaterial3d(sphere.bouncy.clone()),
    ));
    commands.spawn((
        Name::new("Title Screen Exploder Sphere"),
        SphereCount(6),
        Sphere,
        Transform::from_xyz(0., 0., -700.),
        Prop,
        Exploder,
        mesh.clone(),
        MeshMaterial3d(sphere.exploder.clone()),
    ));
}

fn update_sphere_transforms(mut spheres: Query<(&mut Transform, &SphereCount)>, time: Res<Time>) {
    for (mut trns, sphere) in &mut spheres {
        use std::f32::consts::PI;

        let mut x_offset = (sphere.0 % 4) as f32 * 1.2;
        if sphere.0 / 4 == 1 {
            x_offset += 0.6;
        }

        let y_offset = -((sphere.0 / 4) as f32 * 1.);

        let time = 2. * (time.elapsed_secs() + (2. * PI * (sphere.0 as f32 + 1.) / 7.));
        let z_offset = time.cos() * 0.05;

        *trns = Transform::from_xyz(
            BLOCK_LEN * 7. - 1.5 + x_offset,
            BLOCK_LEN * 4. + 2.5 + y_offset,
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
}
