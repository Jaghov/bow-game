use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

/// This is a new crate from the original creator (Alice) of leafing_input_manager.
///
/// It's a lot nicer imo
pub fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin);
}
