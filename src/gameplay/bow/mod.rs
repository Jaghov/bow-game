use std::collections::VecDeque;

use bevy::prelude::*;
use pull::PullStrength;

use crate::asset_tracking::LoadResource;

use super::{GameLoadState, GameSet, cursor::CursorPosition};

mod animation;
mod pull;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<BowAssets>()
        .load_resource::<BowAssets>();

    app.add_plugins((pull::plugin, animation::plugin));

    app.add_systems(OnEnter(GameLoadState::Loaded), spawn_bow)
        .add_systems(Update, update_bow_transform.in_set(GameSet::Update));
}
#[derive(Resource, Asset, Reflect, Clone)]
struct BowAssets {
    #[dependency]
    scene: Handle<Scene>,
    #[dependency]
    pull_string: Handle<AnimationClip>,
}
impl FromWorld for BowAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            scene: assets.load("models/BowFix.glb#Scene0"),
            pull_string: assets.load("models/BowFix.glb#Animation0"),
        }
    }
}

#[derive(Component)]
#[require(PullStrength)]
pub struct Bow;

fn spawn_bow(mut commands: Commands, assets: Res<BowAssets>) {
    info!("Spawning bow");
    commands
        .spawn((
            Bow,
            SceneRoot(assets.scene.clone()),
            children![(
                DirectionalLight {
                    illuminance: light_consts::lux::CLEAR_SUNRISE,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_xyz(0., 0., 5.).looking_at(Vec3::ZERO, Vec3::Y),
            )],
        ))
        .observe(animation::setup_animations);
}

const EPS: f32 = 1e-3;
fn update_bow_transform(
    cursor: Res<CursorPosition>,
    mut bow: Query<&mut Transform, With<Bow>>,
    mut last_positions: Local<VecDeque<Vec3>>,
    mut bow_should_rotation: Local<Quat>,
) {
    //number of positions to keep track of
    const RECORD: usize = 5;
    let Some(position) = cursor.current() else {
        return;
    };
    /*
    if the number of positions is < 5, push regardless.
    if positions == 5, determine if
    */
    let mut adjust_should_rot = false;
    if last_positions.len() < 5 {
        last_positions.push_back(position);
        adjust_should_rot = true;
    } else if last_positions
        .back()
        .is_some_and(|lp| (lp.x - position.x).abs() > EPS && (lp.y - position.y) > EPS)
    {
        last_positions.pop_front();
        last_positions.push_back(position);
        adjust_should_rot = true;
    }

    if adjust_should_rot {

        //*bow_should_rotation = Quat::from_rotation_z(angle)
    }

    let Ok(mut bow) = bow.single_mut() else {
        return;
    };

    bow.translation = position;
}
