use bevy::prelude::*;

use crate::gameplay::level::{LevelProps, Levels, wall::WallBuilder};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, create_level);
}

fn create_level(mut levels: ResMut<Levels>) {
    //this is the debug level
    #[cfg(feature = "dev")]
    levels.insert(LevelProps::new(
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
    ));

    levels.insert(LevelProps::new(
        Some(1),
        vec![
            vert!(6., -5., 5.),
            horz!(6., -6., 6.),
            vert!(-6., -5., 5.),
            horz!(-6., -6., 6.),
        ],
        vec![sphere!(Normal, 5., 0.)],
    ));
    levels.insert(LevelProps::new(
        Some(1),
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
        Some(1),
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
        Some(1),
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
        None,
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
}
