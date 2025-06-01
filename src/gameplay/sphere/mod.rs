use bevy::{color::palettes::css::RED, prelude::*};

use crate::asset_tracking::LoadResource;

use super::{GAME_PLANE, GameLoadState};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<SphereAssets>()
        .load_resource::<SphereAssets>();

    app.add_systems(OnEnter(GameLoadState::Loaded), spawn_debug_sphere);
}

#[derive(Resource, Asset, Reflect, Clone)]
struct SphereAssets {
    #[dependency]
    model: Handle<Scene>,
    #[dependency]
    mesh: Handle<Mesh>,
    normal: Handle<StandardMaterial>,
}

impl FromWorld for SphereAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let model = assets.load("models/Ball.glb#Scene0");
        let mesh = assets.load("models/Ball.glb#Mesh0/Primitive0");
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        let normal = materials.add(StandardMaterial {
            base_color: RED.into(),
            ..default()
        });

        Self {
            model,
            mesh,
            normal,
        }
    }
}

fn spawn_debug_sphere(mut commands: Commands, assets: Res<SphereAssets>) {
    // commands.spawn((
    //     Name::new("Debug Sphere"),
    //     Transform::from_xyz(10., 0., GAME_PLANE),
    //     Mesh3d(assets.mesh.clone()),
    //     MeshMaterial3d(assets.normal.clone()),
    // ));
}
