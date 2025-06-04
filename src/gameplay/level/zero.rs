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
        LevelProps::new(vec![
            vert!(2., 4., 6.),
            horz!(7., 0., 6.),
            horz!(3., 0., 6.),
        ]),
    );

    //todo
}
