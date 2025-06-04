use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Quiver>();
}

#[derive(Resource, Default)]
pub struct Quiver {
    /// the count of arrows is infinite if this is none
    num_arrows: Option<u32>,
}

impl Quiver {
    pub fn set_arrow_count(&mut self, num_arrows: Option<u32>) {
        self.num_arrows = num_arrows;
    }
    pub fn arrow_count(&self) -> Option<u32> {
        self.num_arrows
    }

    pub fn can_fire(&self) -> bool {
        self.num_arrows.is_none_or(|arrow_count| arrow_count != 0)
    }

    /// removes an arrow if not infinite
    pub fn remove_arrow(&mut self) {
        if let Some(num_arrows) = &mut self.num_arrows {
            *num_arrows = num_arrows.saturating_sub(1);
        }
    }
}
