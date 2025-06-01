use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

use super::{GAME_PLANE, GameLoadState};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ArrowAssets>()
        .load_resource::<ArrowAssets>();

    app.add_systems(OnEnter(GameLoadState::Loaded), spawn_arrow);
    //todo
}

#[derive(Resource, Asset, Reflect, Clone)]
struct ArrowAssets {
    #[dependency]
    glowing: Handle<Scene>,
    #[dependency]
    normal: Handle<Scene>,
}
impl FromWorld for ArrowAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            glowing: assets.load("models/ArrowGlow.glb#Scene0"),
            normal: assets.load("models/Arrow.glb#Scene0"),
        }
    }
}
fn spawn_arrow(mut commands: Commands, assets: Res<ArrowAssets>) {
    commands.spawn((
        Transform::from_xyz(-5., 0., GAME_PLANE),
        SceneRoot(assets.glowing.clone()),
    ));
    commands.spawn((
        Transform::from_xyz(5., 0., GAME_PLANE),
        SceneRoot(assets.normal.clone()),
    ));
}
