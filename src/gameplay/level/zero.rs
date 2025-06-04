use bevy::prelude::*;

use crate::gameplay::level::{LevelProps, Levels};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, create_level);
}

fn create_level(mut levels: ResMut<Levels>) {
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
                sphere!(Normal, 0., 0.),
                sphere!(Exploder, 10., 0.),
                sphere!(TimeFreeze, -10., 0.),
                sphere!(Multiplier, 40., 0.),
                sphere!(Bouncy, 20., 0.),
                sphere!(Gravity, -40., 0.),
            ],
        ),
    );
}
