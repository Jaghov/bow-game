use bevy::prelude::*;

use crate::gameplay::level::{LevelProps, Levels};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, create_level);
    //todo
}

fn create_level(mut levels: ResMut<Levels>) {
    //
    //level number, quiver count, wall component

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
            vec![sphere!(Normal, 0., 0.)],
        ),
    );

    //todo
}
