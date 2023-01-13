pub use druid_material_icons::normal;
use druid_material_icons::IconPaths;

use druid::{kurbo::Affine, Color};

use druid::widget::prelude::*;

#[derive(Debug, Clone)]
pub struct Icon {
    paths: IconPaths,
    color: Color,
}
/**
 * A widget that displays an icon from the druid_material_icons crate.
 */
impl Icon {
    #[inline]
    pub fn new(paths: IconPaths) -> Self {
        Self {
            paths,
            color: Color::WHITE,
        }
    }

    pub fn set_color(&mut self, color: &Color) {
        self.color = color.clone();
    }
}

impl<T: Data> Widget<T> for Icon {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}
    
    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        // Try to preserve aspect ratio if possible, but if not then allow non-uniform scaling.
        bc.constrain_aspect_ratio(self.paths.size.aspect_ratio(), self.paths.size.width)
    }
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, _: &Env) {
        let Size { width, height } = ctx.size();
        let Size {
            width: icon_width,
            height: icon_height,
        } = self.paths.size;
        ctx.transform(Affine::scale_non_uniform(
            width * icon_width.recip(),
            height * icon_height.recip(),
        ));
        for path in self.paths.paths {
            ctx.fill(path, &self.color);
        }
    }
}
