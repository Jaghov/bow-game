//! Code to load all of our gltfs and scene stuffs

use std::{collections::HashMap, iter};

use assets::LevelAssets;
use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::screens::Screen;

mod assets;

pub struct LevelPlugin {
    pub load_level: bool,
}

impl Default for LevelPlugin {
    fn default() -> Self {
        Self { load_level: true }
    }
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(assets::plugin);

        app.init_resource::<LevelsLoaded>()
            .init_resource::<LevelOrigins>()
            .register_type::<LevelOrigins>()
            .register_type::<Level>();

        if self.load_level {
            app.add_systems(OnEnter(Screen::SpawnLevel), spawn_world)
                .add_systems(OnExit(Screen::Gameplay), despawn_world);
        }
    }
}

/// note that levels start at 0. this is length.
const NUM_LEVELS: u8 = 2;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct LevelOrigins(HashMap<Level, Vec3>);

// because the level cannot be state scoped
#[derive(Component)]
struct WorldPart;

impl LevelOrigins {
    // panics if not found, but like we totally control this.
    pub fn get_spawn_point(&self, level: Level) -> Vec3 {
        info!("getting spawn point for {level:?}");
        self.0.get(&level).copied().unwrap()
    }
}

impl Default for LevelOrigins {
    fn default() -> Self {
        let mut map: HashMap<Level, Vec3> = HashMap::with_capacity(5);

        for i in 0..NUM_LEVELS {
            map.insert(Level(i), Vec3::new(0., 0., (i as f32) * 120.));
        }

        Self(map)
    }
}

/// Associated something with a particular level.
///
/// One of the levels will be designated as a win
#[derive(Component, Reflect, Hash, Clone, Copy, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub struct Level(pub u8);

#[derive(Resource)]
pub struct LevelsLoaded {
    loaded: Vec<bool>,
}

impl Default for LevelsLoaded {
    fn default() -> Self {
        Self {
            loaded: iter::repeat_n(false, NUM_LEVELS as usize).collect(),
        }
    }
}

impl LevelsLoaded {
    pub fn reset(&mut self) {
        self.loaded.iter_mut().for_each(|l| {
            *l = false;
        });
    }
    pub fn set_ready(&mut self, level: Level) {
        let val = &mut self.loaded[level.0 as usize];
        *val = true;
    }
    pub fn all_ready(&self) -> bool {
        self.loaded.iter().all(|l| *l)
    }
    pub fn length(&self) -> usize {
        self.loaded.len()
    }
    pub fn num_loaded(&self) -> usize {
        self.loaded.iter().filter(|b| **b).count()
    }
}

fn spawn_world(
    mut commands: Commands,
    scene_assets: Res<LevelAssets>,
    spawn_points: Res<LevelOrigins>,
) {
    for (level, scene) in &scene_assets.levels {
        commands
            .spawn((
                WorldPart,
                *level,
                SceneRoot(scene.clone()),
                Transform::from_translation(spawn_points.get_spawn_point(*level)),
            ))
            .observe(announce_ready);
    }

    commands.spawn((
        PointLight {
            intensity: 5000.,
            ..default()
        },
        Transform::default(),
    ));
}

fn announce_ready(
    trigger: Trigger<SceneInstanceReady>,
    levels: Query<&Level>,
    mut res: ResMut<LevelsLoaded>,
) {
    let scene_level = levels.get(trigger.target()).unwrap();
    info!("Level {} is ready!", scene_level.0);
    res.set_ready(*scene_level);
}

fn despawn_world(
    mut commands: Commands,
    scenes: Query<Entity, With<WorldPart>>,
    mut levels_loaded: ResMut<LevelsLoaded>,
) {
    levels_loaded.reset();
    for part in scenes {
        commands.entity(part).despawn();
    }
}
