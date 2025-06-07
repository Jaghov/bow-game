use bevy::prelude::*;

use crate::{
    gameplay::level::{LevelProps, Levels},
    world::BLOCK_LEN,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, create_level);
}

fn create_level(mut levels: ResMut<Levels>) {
    //this is the debug level
    levels.insert(
        0,
        LevelProps::new(
            None,
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
        ),
    );
}
