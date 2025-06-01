use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

use super::{Level, NUM_LEVELS};

pub fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct LevelAssets {
    pub(crate) levels: Vec<(Level, Handle<Scene>)>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        let levels = (0..NUM_LEVELS)
            .map(|i| {
                (
                    Level(i),
                    assets
                        .load(GltfAssetLabel::Scene(0).from_asset(format!("levels/level{i}.glb"))),
                )
            })
            .collect();
        Self { levels }
    }
}
