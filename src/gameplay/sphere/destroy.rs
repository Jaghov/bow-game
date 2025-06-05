use super::Sphere;
use avian3d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_destroyable_sphere);
    //todo
}

#[derive(Event)]
pub struct DestroySphere;

fn add_destroyable_sphere(trigger: Trigger<OnAdd, Sphere>, mut commands: Commands) {
    commands.entity(trigger.target()).observe(destroy_sphere);
}

// listener should ONLY be on the Sphere component.
fn destroy_sphere(trigger: Trigger<DestroySphere>, mut commands: Commands) {
    // this will make the thing break into a million pieces.
    // TODO
    commands.entity(trigger.target()).try_despawn();
}
