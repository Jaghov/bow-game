use bevy::prelude::*;
use bevy_dog::settings::{DoGSettings, PassesSettings, Thresholding};

use crate::gameplay::GameSet;

use super::palette::Palette;

pub fn plugin(app: &mut App) {
    app.add_plugins((bevy_dog::plugin::DoGPlugin,))
        .add_systems(Update, (insert_post_process,))
        .add_systems(Update, sync_shader_with_palette.in_set(GameSet::UiUpdate));
}

fn insert_post_process(query: Query<Entity, Added<Camera3d>>, mut commands: Commands) {
    for camera in query {
        commands.entity(camera).insert((
            DoGSettings {
                min_color: Vec3::ZERO,
                max_color: Vec3::ONE,
                //k: 10.,
                tau: 32.,
                phi: 0.2,
                thresholding: Thresholding::Quantization as i32,
                thresholds: Vec4::from_array([12., 6., 3., 0.5]),
                ..DoGSettings::DEFAULT
            },
            PassesSettings::default(),
        ));
    }
}
/// Lerps between current timer default and 0, changing scene color
fn sync_shader_with_palette(palette: Res<Palette>, query: Query<&mut DoGSettings>) {
    for (mut setting) in query {
        setting.min_color = palette.dark.to_vec3();
    }
}
