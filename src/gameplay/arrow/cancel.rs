use std::time::Duration;

use avian3d::prelude::GravityScale;
use bevy::{platform::time::Instant, prelude::*};

use super::{Arrow, Fired};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(cancel_arrow);

    app.add_systems(Update, remove_canceled_arrows);
}

const REMOVE_IN: Duration = Duration::from_secs(5);

#[derive(Event)]
pub struct CancelArrow;

#[derive(Component)]
pub(super) struct Canceled(pub Instant);

fn cancel_arrow(
    _: Trigger<CancelArrow>,
    mut commands: Commands,
    arrows: Query<Entity, (With<Arrow>, Without<Fired>)>,
) {
    for arrow in arrows {
        commands
            .entity(arrow)
            .insert((Fired, Canceled(Instant::now()), GravityScale(1.)));
    }
}

fn remove_canceled_arrows(mut commands: Commands, arrows: Query<(Entity, &Canceled)>) {
    for (arrow, canceled_at) in arrows {
        if canceled_at.0.elapsed() > REMOVE_IN {
            commands.entity(arrow).despawn();
        }
    }
}
