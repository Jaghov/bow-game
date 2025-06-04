use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    Screen,
    gameplay::level::{sphere::SpawnSphere, wall::WallBuilder},
};

#[macro_use]
mod wall;
#[macro_use]
mod sphere;
mod zero;

mod new_level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((zero::plugin, new_level::plugin));
    app.add_sub_state::<LevelState>()
        .init_resource::<Level>()
        .init_resource::<Levels>();
    app.add_systems(Startup, setup_wall_material);
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
pub enum LevelState {
    #[default]
    NewLevel,
    Playing,
    NextLevel,
}

#[derive(Resource, Default)]
pub struct Level(pub usize);

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
}

impl Levels {
    pub fn insert(&mut self, level: usize, props: LevelProps) {
        self.levels.insert(level, props);
    }
    /// will get or insert a new random level based on the value
    pub fn get(&mut self, level: usize) -> &LevelProps {
        if let Some(level) = self.levels.get(&level) {
            return level;
        }

        todo!("generate dynamic random levels")
    }
}
