use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

use super::GameLoadState;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TargetAssets>()
        .load_resource::<TargetAssets>();
    app.add_systems(OnEnter(GameLoadState::Loaded), spawn_target);
}
#[derive(Resource, Asset, Reflect, Clone)]
struct TargetAssets {
    #[dependency]
    model: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}
impl FromWorld for TargetAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let model = assets.load(
            GltfAssetLabel::Primitive {
                mesh: 0,
                primitive: 0,
            }
            .from_asset("models/sphere.glb"),
        );
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        let material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            reflectance: 0.3,
            specular_transmission: 0.9,
            diffuse_transmission: 1.0,
            thickness: 1.6,
            ior: 1.5,
            perceptual_roughness: 0.12,
            ..Default::default()
        });

        Self { model, material }
    }
}

fn spawn_target(mut commands: Commands, assets: Res<TargetAssets>) {
    info!("Spawning Target");
    commands.spawn((
        Mesh3d(assets.model.clone()),
        MeshMaterial3d(assets.material.clone()),
    ));
}
