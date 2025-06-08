use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MulliganPossible>();
    //todod
}

#[derive(Resource)]
pub struct MulliganPossible(bool);

impl Default for MulliganPossible {
    fn default() -> Self {
        Self(true)
    }
}
