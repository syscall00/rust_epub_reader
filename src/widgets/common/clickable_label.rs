use druid::{
    ArcStr, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, RenderContext, Size, TextLayout, UpdateCtx, Widget,
};

use crate::{
    core::constants::commands::{InternalUICommand, INTERNAL_COMMAND},
    data::IndexedText,
};

const CLICKABLE_LABEL_BACKGROUND : Color = Color::rgba8(190, 190, 190, 60);

/**
 * A clickable label is a widget that can be used to navigate to a specific position in the ebook.
 */
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
                    ctx.submit_command(INTERNAL_COMMAND.with(InternalUICommand::EpubGoToPos(
                        data.value().as_ref().clone(),
                    )))
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
                self.layout.set_text(data.key().clone());
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
            self.layout.set_text(data.key().clone());
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

        self.layout.rebuild_if_needed(ctx.text(), env);
        let text_metrics = self.layout.layout_metrics();
        ctx.set_baseline_offset(text_metrics.size.height - text_metrics.first_baseline);
        Size::new(bc.max().width, 23.)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _: &IndexedText, _: &Env) {
        let size = ctx.size();
        if ctx.is_hot() {
            let rect = ctx.size().to_rect();

            ctx.fill(rect, &CLICKABLE_LABEL_BACKGROUND);
        }
        ctx.clip((size - Size::new(15., 0.)).to_rect());

        self.layout.draw(ctx, (5., 0.));
    }
}
