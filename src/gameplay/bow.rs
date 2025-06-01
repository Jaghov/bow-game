use bevy::prelude::*;
use bevy_gltf_animation::prelude::*;

use crate::asset_tracking::LoadResource;

use super::GameLoadState;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<BowAssets>()
        .load_resource::<BowAssets>();
    app.add_systems(OnEnter(GameLoadState::Loaded), spawn_bow);
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

fn spawn_bow(mut commands: Commands, assets: Res<BowAssets>) {
    info!("Spawning bow");
    commands.spawn(GltfSceneRoot::new(assets.model.clone()));
}
