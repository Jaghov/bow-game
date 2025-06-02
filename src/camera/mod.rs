use bevy::{prelude::*, render::view::RenderLayers};
use bitflags::bitflags;

mod ui;

mod world;
pub use world::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ui::plugin, world::plugin));
}

impl From<CameraOrder> for isize {
    fn from(order: CameraOrder) -> Self {
        order as isize
    }
}

/// This enum is converted to an `isize` to be used as a camera's order.
/// Since we have three cameras, we use three enum variants.
/// This ordering here mean UI > ViewModel > World.
pub enum CameraOrder {
    World,
    Ui,
}

bitflags! {
    pub struct RenderLayer: u32 {
        /// Used implicitly by all entities without a `RenderLayers` component.
        /// Our world model camera and all objects other than the player are on this layer.
        /// The light source belongs to both layers.
        const DEFAULT = 0b00000001;
        /// Used by the view model camera and the player's arm.
        /// The light source belongs to both layers.
        const VIEW_MODEL = 0b00000010;
        /// Since we use multiple cameras, we need to be explicit about
        /// which one is allowed to render particles.
        const PARTICLES = 0b00000100;
        /// Skip interaction with lights
        const TRANSLUCENT = 0b00001000;

        /// 3D gizmos. These need to be rendered only by a 3D camera, otherwise the UI camera will render them in a buggy way.
        /// Specifically, the UI camera is a 2D camera, which by default is placed at a far away Z position,
        /// so it will effectively render a very zoomed out view of the scene in the center of the screen.
        const GIZMO3 = 0b0001000;

        /// Used by meshes that appear UI render targets
        const UI_MESH = 0b10000000;
    }
}

impl From<RenderLayer> for RenderLayers {
    fn from(layer: RenderLayer) -> Self {
        // Bevy's default render layer is 0, so we need to subtract 1 from our bitfalgs to get the correct value.
        RenderLayers::from_iter(layer.iter().map(|l| l.bits() as usize - 1))
    }
}
