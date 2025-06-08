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
#[cfg(all(feature = "dev", feature = "hot"))]
mod level_maker;
mod new_level;
mod next_level;
mod restart;
mod timer;

const WALL_START_PLANE: f32 = GAMEPLAY_CAMERA_OFFSET + 20.;
const SPHERE_START_PLANE: f32 = GAME_PLANE - 20.;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        new_level::plugin,
        next_level::plugin,
        restart::plugin,
        timer::plugin,
    ));
    app.add_sub_state::<LevelState>()
        .init_resource::<Level>()
        .insert_resource(Levels::init());
    app.add_systems(Startup, setup_wall_material)
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
}

#[derive(Resource, Default)]
pub struct Levels {
    levels: Vec<LevelProps>,
}

impl Levels {
    pub fn init() -> Self {
        let mut levels = Levels::default();
        //this is the debug level
        #[cfg(feature = "dev")]
        levels.insert(LevelProps::new(
            9999,
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

        levels.insert(LevelProps::new(
            1,
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
                // sphere!(Normal, 10., 5.),
                // sphere!(Normal, 10., -5.),
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

        levels
    }

    fn insert(&mut self, props: LevelProps) {
        self.levels.push(props);
    }
    pub fn num_levels(&self) -> usize {
        self.levels.len()
    }

    /// will get or insert a new random level based on the value
    pub fn get(&mut self, level: usize) -> Option<&LevelProps> {
        self.levels.get(level)
    }
}
