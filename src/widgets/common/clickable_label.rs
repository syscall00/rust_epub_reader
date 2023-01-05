use druid::{ArcStr, TextLayout, Widget, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, Color, Data, RenderContext};

use crate::{core::constants::commands::{INTERNAL_COMMAND, InternalUICommand}, data::IndexedText};

pub struct ClickableLabel {
    layout: TextLayout<ArcStr>,
}
impl ClickableLabel {
    pub fn new() -> Self {
        Self {
            layout: TextLayout::new(),
        }
    }
}
impl Widget<IndexedText> for ClickableLabel {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut IndexedText, _env: &Env) {
        match event {
            Event::MouseDown(mouse) => {
                if mouse.button.is_left() {
                    ctx.submit_command(INTERNAL_COMMAND.with(InternalUICommand::EpubGoToPos((*data.value).clone())))
                }
            }
            Event::MouseMove(_) => {
                ctx.set_cursor(&druid::Cursor::Pointer);
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &IndexedText,
        env: &Env,
    ) {
        match event {
            LifeCycle::WidgetAdded => {
                self.layout.set_text(data.key.clone());
                self.layout.set_text_size(13.);
                self.layout.rebuild_if_needed(ctx.text(), env);
            }
            LifeCycle::HotChanged(_) => {
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &IndexedText,
        data: &IndexedText,
        env: &Env,
    ) {
        if !(old_data.same(data)) {
            self.layout.set_text(data.key.clone());
            self.layout.rebuild_if_needed(ctx.text(), env);
            ctx.request_layout();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _: &IndexedText,
        env: &Env,
    ) -> Size {
        //self.layout.set_wrap_width(bc.max().width);
        self.layout.rebuild_if_needed(ctx.text(), env);
        //self.layout.set_wrap_width(f64::INFINITY);
        let text_metrics = self.layout.layout_metrics();
        ctx.set_baseline_offset(text_metrics.size.height - text_metrics.first_baseline);
        //bc.constrain(size)

        Size::new(bc.max().width, 23.)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _: &IndexedText, _: &Env) {
        let size = ctx.size();
        if ctx.is_hot() {
            let rect = ctx.size().to_rect();

            ctx.fill(rect, &Color::BLUE);
        }
        //println!("painting : {:?}", size);
        ctx.clip((size - Size::new(15., 0.)).to_rect());

        self.layout.draw(ctx, (5., 0.));
    }
}
