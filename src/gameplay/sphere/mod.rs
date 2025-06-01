use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<SphereAssets>()
        .load_resource::<SphereAssets>();
}

#[derive(Resource, Asset, Reflect, Clone)]
struct SphereAssets {
    #[dependency]
    model: Handle<Scene>,
}

impl FromWorld for SphereAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            model: assets.load("models/Ball.glb#Scene0"),
        }
    }
}
