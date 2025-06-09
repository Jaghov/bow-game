use std::{f32::consts::FRAC_PI_4, fmt};

use bevy::prelude::*;

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
#[cfg(all(feature = "dev", feature = "hot"))]
mod level_maker;
mod new_level;
mod next_level;
mod timer;

const WALL_START_PLANE: f32 = GAMEPLAY_CAMERA_OFFSET + 20.;
const SPHERE_START_PLANE: f32 = GAME_PLANE - 20.;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((new_level::plugin, next_level::plugin, timer::plugin));
    app.add_sub_state::<LevelState>()
        .init_resource::<Level>()
        .insert_resource(Levels::init());
    app.add_systems(Startup, setup_wall_material)
        .add_systems(OnEnter(Screen::Gameplay), reset_level)
        .add_observer(sphere::spawn_sphere);

    #[cfg(all(feature = "dev", feature = "hot"))]
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

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Level {
    fn default() -> Self {
        Self(0)
    }
}

pub struct LevelProps {
    course_par: i32,
    walls: Vec<WallBuilder>,
    spheres: Vec<SpawnSphere>,
}

impl LevelProps {
    pub fn new(course_par: i32, walls: Vec<WallBuilder>, spheres: Vec<SpawnSphere>) -> Self {
        Self {
            course_par,
            walls,
            spheres,
        }
    }
    pub fn par(&self) -> i32 {
        self.course_par
    }
}

fn reset_level(mut level: ResMut<Level>) {
    *level = Level::default();
}

#[derive(Resource, Default)]
pub struct Levels {
    levels: Vec<LevelProps>,
}

#[allow(dead_code)]
impl Levels {
    pub fn init() -> Self {
        let mut levels = Levels::default();

        // simple first level
        levels.insert(LevelProps::new(
            1,
            vec![
                vert!(6., -5., 5.),
                horz!(6., -6., 6.),
                vert!(-6., -5., 5.),
                horz!(-6., -6., 6.),
            ],
            vec![sphere!(Normal, 5., 0.), sphere!(Normal, 10., 0.)],
        ));

        //multiplier simple
        levels.insert(LevelProps::new(
            1,
            vec![
                vert!(8., -5., 5.),
                horz!(6., -8., 8.),
                vert!(-8., -5., 5.),
                horz!(-6., -8., 8.),
            ],
            vec![
                sphere!(Multiplier, 5., 0.),
                sphere!(Normal, 10., 0.),
                sphere!(Normal, 10., 5.),
                sphere!(Normal, 10., -5.),
            ],
        ));

        // multiplier advanced
        levels.insert(LevelProps::new(
            1,
            vec![
                vert!(6., -4., 4.),
                horz!(5., -6., 6.),
                vert!(-6., -4., 4.),
                horz!(-5., -6., 6.),
                vert!(0., 1., 4.),
                vert!(0., -4., -1.),
            ],
            vec![
                sphere!(Normal, -17., -6.),
                sphere!(Multiplier, -7., 0.),
                sphere!(Multiplier, 7., 0.),
                sphere!(Normal, 18., 8.),
                sphere!(Normal, 18., 0.),
                sphere!(Normal, 18., -8.),
            ],
        ));

        //exploder introduction
        levels.insert(LevelProps::new(
            3,
            vec![
                vert!(6., -5., 5.),
                horz!(6., -6., 6.),
                vert!(-6., -5., 5.),
                horz!(-6., -6., 6.),
            ],
            vec![
                sphere!(Exploder, 24., 24.),
                sphere!(Exploder, -24., -24.),
                sphere!(Exploder, 24., -24.),
                sphere!(Exploder, -24., 24.),
                sphere!(Multiplier, 0., -7.),
                sphere!(Multiplier, 0., 7.),
                sphere!(Normal, 0., 0.),
                sphere!(Normal, 24., 31.),
                sphere!(Normal, -24., -31.),
                sphere!(Normal, -24., 31.),
                sphere!(Normal, 24., -31.),
                sphere!(Normal, 31., 24.),
                sphere!(Normal, -31., -24.),
                sphere!(Normal, -31., 24.),
                sphere!(Normal, 31., -24.),
                sphere!(Normal, 24., 17.),
                sphere!(Normal, -24., -17.),
                sphere!(Normal, 24., -17.),
                sphere!(Normal, -24., 17.),
            ],
        ));

        //timefreeze spiral
        levels.insert(LevelProps::new(
            2,
            vec![
                vert!(-8., -5., 6.),
                horz!(-6., -8., 8.),
                vert!(8., -5., 5.),
                horz!(6., -4., 8.),
                vert!(-4., -2., 5.),
                horz!(-2., -3., 4.),
                vert!(4., -1., 1.),
                horz!(2., 0., 4.),
            ],
            vec![
                sphere!(Normal, -36., 24.),
                sphere!(TimeFreeze, -36., -24.),
                sphere!(TimeFreeze, 36., -24.),
                sphere!(TimeFreeze, 36., 24.),
                sphere!(TimeFreeze, -12., 24.),
                sphere!(TimeFreeze, -12., 0.),
                sphere!(Multiplier, 10., 0.),
                sphere!(Normal, 18., 0.),
                sphere!(Normal, 18., 6.),
                sphere!(Normal, 18., -6.),
            ],
        ));

        levels.insert(LevelProps::new(
            3,
            vec![
                //right
                vert!(7., -4., 6.),
                //horz left
                horz!(5., -7., 1.),
                //left
                vert!(-7., -4., 4.),
                //bottom
                horz!(-5., -7., 7.),
                //divider top
                vert!(2., 4., 6.),
                //horz top right
                horz!(6., 3., 6.),
                //div bot
                vert!(2., -4., 2.),
                WallBuilder::block(5., 1., 26., -10.),
            ],
            vec![
                //left side
                sphere!(TimeFreeze, -18., -2.),
                sphere!(Multiplier, -18., -8.),
                //left side array
                sphere!(Exploder, 6., 6.),
                sphere!(Exploder, 6., 0.),
                sphere!(Multiplier, 3., -6.),
                sphere!(Normal, -5., -4.),
                sphere!(Normal, -2., -13.),
                sphere!(Normal, 7., -15.),
                //middle ball
                sphere!(Normal, 5., 17.),
                //right side
                sphere!(Exploder, 21., 20.),
                sphere!(Normal, 17., 14.),
                sphere!(Normal, 17., 24.),
                sphere!(TimeFreeze, 26., 21.),
                //bowling
                sphere!(Exploder, 26., -8.),
                sphere!(Multiplier, 26., -13.),
                sphere!(Normal, 26., -24.),
                sphere!(Normal, 23., -24.),
                sphere!(Normal, 20., -24.),
                sphere!(Normal, 18., -23.),
                sphere!(Normal, 29., -24.),
                sphere!(Normal, 33., -24.),
                sphere!(Normal, 35., -23.),
            ],
        ));

        //mayhem
        levels.insert(LevelProps::new(
            3,
            vec![
                horz!(4., -7., 8.),
                horz!(2., -7., 6.),
                vert!(8., -5., 3.),
                vert!(6., -3., 1.),
                WallBuilder::block_rot(3., 6., 44.5, 20.5, FRAC_PI_4),
                WallBuilder::block_rot(3., 6., 44.5, -26.5, -FRAC_PI_4),
                horz!(-3., 4., 5.),
                horz!(-5., 4., 7.),
                horz!(-6., 0., 4.),
                horz!(-2., 0., 4.),
                vert!(-1., -6., -2.),
            ],
            vec![
                sphere!(Bouncy, 0., 18.),
                sphere!(Bouncy, 6., 18.),
                sphere!(Bouncy, 12., 18.),
                sphere!(Bouncy, 18., 18.),
                sphere!(Bouncy, 24., 18.),
                sphere!(Normal, 42., 0.),
                sphere!(Bouncy, 42., 6.),
                sphere!(Bouncy, 42., -6.),
                sphere!(Multiplier, 42., -12.),
                sphere!(Bouncy, 24., -24.),
                sphere!(Bouncy, 18., -24.),
                sphere!(Multiplier, 15., -24.),
                sphere!(Bouncy, 12., -24.),
                sphere!(Normal, 9., -24.),
                sphere!(Normal, 9., -21.),
                sphere!(Normal, 9., -27.),
                sphere!(Normal, 9., -30.),
                sphere!(Normal, 9., -18.),
                sphere!(Normal, 12., -27.),
                sphere!(Normal, 12., -21.),
                sphere!(Normal, 12., -18.),
                sphere!(Normal, 12., -30.),
                sphere!(Normal, 15., -27.),
                sphere!(Normal, 15., -21.),
                sphere!(Normal, 15., -18.),
                sphere!(Normal, 15., -30.),
                sphere!(Normal, 6., -24.),
                sphere!(Normal, 6., -21.),
                sphere!(Normal, 6., -27.),
                sphere!(Normal, 6., -30.),
                sphere!(Normal, 6., -18.),
                sphere!(Normal, 3., -24.),
                sphere!(Normal, 3., -21.),
                sphere!(Normal, 3., -27.),
                sphere!(Normal, 3., -30.),
                sphere!(Normal, 3., -18.),
                sphere!(Normal, 0., -24.),
                sphere!(Normal, 0., -21.),
                sphere!(Normal, 0., -27.),
                sphere!(Normal, 0., -30.),
                sphere!(Normal, 0., -18.),
            ],
        ));

        levels.insert(LevelProps::new(
            3,
            vec![
                vert!(8., -5., 5.),
                horz!(6., -8., 8.),
                vert!(-8., -5., 5.),
                horz!(-6., -8., 8.),
            ],
            vec![
                //gravity column
                sphere!(Gravity, -40., 0.),
                sphere!(Normal, -40., 5.),
                sphere!(Multiplier, -40., -5.),
                //absorber column
                sphere!(Absorber, -30., 0.),
                sphere!(Multiplier, -30., 5.),
                sphere!(Exploder, -30., 10.),
                sphere!(Bouncy, -30., 15.),
                //exploder testing column
                sphere!(Exploder, -20., 0.),
                sphere!(Multiplier, -23., 5.),
                sphere!(Multiplier, -23., -5.),
                sphere!(Multiplier, -17., 5.),
                sphere!(Multiplier, -17., -5.),
                //others
                sphere!(TimeFreeze, -10., 0.),
                sphere!(Normal, 0., 0.),
                // exploder column
                sphere!(Exploder, 10., 0.),
                sphere!(Exploder, 10., 5.),
                sphere!(Exploder, 10., 10.),
                sphere!(Normal, 8., 8.),
                // bouncy column
                sphere!(Bouncy, 20., 0.),
                sphere!(Multiplier, 20., 5.),
                sphere!(Normal, 20., 10.),
                // bouncy timefreeze column
                sphere!(Bouncy, 30., 0.),
                sphere!(TimeFreeze, 30., 5.),
                sphere!(Multiplier, 30., 10.),
                // multiplier column
                sphere!(Multiplier, 40., 0.),
                sphere!(Multiplier, 40., 5.),
                sphere!(Multiplier, 40., 10.),
                sphere!(Normal, 40., 15.),
            ],
        ));

        levels
    }

    #[allow(dead_code)]
    pub fn mulligan_debug() -> Self {
        let mut levels = Levels::default();

        levels.insert(LevelProps::new(
            2,
            vec![
                vert!(6., -5., 5.),
                horz!(6., -6., 6.),
                vert!(-6., -5., 5.),
                horz!(-6., -6., 6.),
            ],
            vec![sphere!(Normal, 5., 0.)],
        ));
        levels.insert(LevelProps::new(
            1,
            vec![
                vert!(6., -5., 5.),
                horz!(6., -6., 6.),
                vert!(-6., -5., 5.),
                horz!(-6., -6., 6.),
            ],
            vec![sphere!(Normal, -2., 0.)],
        ));

        levels
    }

    fn insert(&mut self, props: LevelProps) {
        self.levels.push(props);
    }
    pub fn num_levels(&self) -> usize {
        self.levels.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, LevelProps> {
        self.levels.iter()
    }

    /// will get or insert a new random level based on the value
    pub fn get(&mut self, level: usize) -> Option<&LevelProps> {
        self.levels.get(level)
    }
}
