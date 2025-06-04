use bevy::{platform::collections::HashMap, prelude::*};

use crate::{Screen, gameplay::level::wall::WallBuilder};

#[macro_use]
mod wall;
mod zero;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((zero::plugin));
    app.add_sub_state::<LevelState>().init_resource::<Level>();
    app.add_systems(OnEnter(LevelState::NewLevel), load_level);
    //todo
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
    walls: Vec<WallBuilder>,
}

impl LevelProps {
    pub fn new(walls: Vec<WallBuilder>) -> Self {
        Self { walls }
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

fn load_level(mut levels: ResMut<Levels>, level: Res<Level>) {
    let props = levels.get(level.0);
    //todo
}
