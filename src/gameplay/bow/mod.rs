use bevy::prelude::*;
use bevy_gltf_animation::prelude::*;

use crate::asset_tracking::LoadResource;

use super::{GameLoadState, GameSet, cursor::CursorPosition};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<BowAssets>()
        .load_resource::<BowAssets>();
    app.add_systems(OnEnter(GameLoadState::Loaded), spawn_bow)
        .add_systems(Update, move_bow.in_set(GameSet::Update));
}
#[derive(Resource, Asset, Reflect, Clone)]
struct BowAssets {
    #[dependency]
    model: Handle<Gltf>,
}
impl FromWorld for BowAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            model: assets.load("models/Bow.glb"),
        }
    }
}

#[derive(Component)]
pub struct Bow;

fn spawn_bow(mut commands: Commands, assets: Res<BowAssets>) {
    info!("Spawning bow");
    commands.spawn((
        Bow,
        GltfSceneRoot::new(assets.model.clone()),
        children![(
            DirectionalLight {
                illuminance: light_consts::lux::CLEAR_SUNRISE,
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(0., 0., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        )],
    ));
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
