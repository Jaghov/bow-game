use bevy::prelude::*;

use crate::{
    Screen,
    gameplay::{arrow::ArrowAssets, bow::BowAssets},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_items)
        .add_systems(Update, set_locations.run_if(in_state(Screen::Title)));
    //todo
}

// note these are different from the game components
#[derive(Component)]
struct Arrow;

#[derive(Component)]
struct Bow;

fn spawn_items(mut commands: Commands, bow_assets: Res<BowAssets>, arrow_assets: Res<ArrowAssets>) {
    commands.spawn((
        Bow,
        StateScoped(Screen::Title),
        SceneRoot(bow_assets.scene.clone()),
    ));
    commands.spawn((
        Arrow,
        StateScoped(Screen::Title),
        SceneRoot(arrow_assets.glowing.clone()),
    ));
    //todo
}

#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
fn set_locations(
    mut bow: Query<&mut Transform, With<Bow>>,
    mut arrow: Query<&mut Transform, (With<Arrow>, Without<Bow>)>,
) {
    let mut bow = bow.single_mut().unwrap();
    let mut arrow = arrow.single_mut().unwrap();
    //todod
}
