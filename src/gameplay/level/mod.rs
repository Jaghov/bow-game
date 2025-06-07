use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    Screen,
    gameplay::{
        GAMEPLAY_CAMERA_OFFSET,
        level::{sphere::SpawnSphere, wall::WallBuilder},
    },
    world::GAME_PLANE,
};

#[macro_use]
mod wall;
pub use wall::*;
#[macro_use]
mod sphere;
#[cfg(feature = "dev")]
mod level_maker;
mod new_level;
mod next_level;
mod restart;
mod timer;
mod zero;

const WALL_START_PLANE: f32 = GAMEPLAY_CAMERA_OFFSET + 20.;
const SPHERE_START_PLANE: f32 = GAME_PLANE - 20.;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        zero::plugin,
        new_level::plugin,
        next_level::plugin,
        restart::plugin,
        timer::plugin,
    ));
    app.add_sub_state::<LevelState>()
        .init_resource::<Level>()
        .init_resource::<Levels>();
    app.add_systems(Startup, setup_wall_material)
        .add_observer(sphere::spawn_sphere);

    #[cfg(feature = "dev")]
    app.add_plugins(level_maker::plugin);
}

#[derive(Resource)]
struct WallMaterial(Handle<StandardMaterial>);

fn setup_wall_material(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    commands.insert_resource(WallMaterial(material));
}

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Copy, Default, Reflect)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub(crate) enum LevelState {
    #[default]
    NewLevel,
    Playing,
    NextLevel,
}

#[derive(Resource)]
pub struct Level(pub usize);

#[allow(clippy::derivable_impls)]
impl Default for Level {
    fn default() -> Self {
        Self(1)
    }
}

pub struct LevelProps {
    arrow_count: Option<u32>,
    walls: Vec<WallBuilder>,
    spheres: Vec<SpawnSphere>,
}

impl LevelProps {
    pub fn new(
        arrow_count: Option<u32>,
        walls: Vec<WallBuilder>,
        spheres: Vec<SpawnSphere>,
    ) -> Self {
        Self {
            arrow_count,
            walls,
            spheres,
        }
    }
}

#[derive(Resource, Default)]
pub struct Levels {
    levels: HashMap<usize, LevelProps>,
    counter: usize,
}

impl Levels {
    #[allow(dead_code)]
    pub fn insert(&mut self, level: usize, props: LevelProps) {
        self.levels.insert(self.counter, props);
        self.counter += 1;
    }
    pub fn add(
        &mut self,
        arrow_count: Option<u32>,
        walls: Vec<WallBuilder>,
        spheres: Vec<SpawnSphere>,
    ) {
        self.levels
            .insert(self.counter, LevelProps::new(arrow_count, walls, spheres));
        self.counter += 1;
    }
    /// will get or insert a new random level based on the value
    pub fn get(&mut self, level: usize) -> &LevelProps {
        if let Some(level) = self.levels.get(&level) {
            return level;
        }

        self.levels.get(&1).unwrap()
        //todo!("generate dynamic random levels")
    }
}
