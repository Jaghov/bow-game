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
        .add_systems(Update, move_bow.in_set(GameSet::Update));
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
            scene: assets.load("models/Bow.glb#Scene0"),
            pull_string: assets.load("models/Bow.glb#Animation0"),
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
fn move_bow(cursor: Res<CursorPosition>, mut bow: Query<&mut Transform, With<Bow>>) {
    let Ok(mut bow) = bow.single_mut() else {
        return;
    };
    let Some(position) = cursor.current() else {
        return;
    };

    bow.translation = position;
}
