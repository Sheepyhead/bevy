use bevy_ecs::prelude::{Bundle, Component};
use bevy_math::Rect;
use bevy_render::view::{ComputedVisibility, Visibility};
use bevy_transform::components::{GlobalTransform, Transform};

#[derive(Clone, Component, Default)]
pub struct Billboard {
    pub bounds: Rect<f32>,
}

#[derive(Bundle, Clone, Default)]
pub struct BillboardBundle {
    pub billboard: Billboard,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
