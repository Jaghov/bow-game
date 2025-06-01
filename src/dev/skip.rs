use bevy::prelude::*;

use crate::Screen;
/// Goes instantly into gameplay
pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Splash),
        |mut screen: ResMut<NextState<Screen>>| screen.set(Screen::Gameplay),
    );
}
