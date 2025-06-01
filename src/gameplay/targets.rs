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
}
impl FromWorld for TargetAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            model: assets.load(
                GltfAssetLabel::Primitive {
                    mesh: 0,
                    primitive: 0,
                }
                .from_asset("models/sphere.glb"),
            ),
        }
    }
}

fn spawn_target(
    mut commands: Commands,
    assets: Res<TargetAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Spawning Target");
    commands.spawn((
        Mesh3d(assets.model.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.7, 0.7, 1.0),
            // specular_tint: Color::from(Srgba::RED),
            reflectance: 0.3,
            specular_transmission: 0.95,
            diffuse_transmission: 1.0,
            thickness: 0.6,
            ior: 1.5,
            perceptual_roughness: 0.12,
            ..Default::default()
        })),
    ));
}
