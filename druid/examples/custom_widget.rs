
#![windows_subsystem = "windows"]

use druid::piet::{FontFamily};
use druid::widget::prelude::*;
use druid::{
     AppLauncher, LocalizedString, WindowDesc,
};



use tracing::{instrument, trace};

use druid::debug_state::DebugState;
use druid::kurbo::Insets;
use druid::piet::TextLayout as _;
use druid::text::{
    EditableText, ImeInvalidation, Selection, TextComponent, TextLayout, TextStorage,
};
use druid::widget::{Padding, Scroll, WidgetWrapper};
use druid::{
    theme, Color, FontDescriptor, KeyOrValue, Point, Rect, TextAlignment
};



pub struct CustomWidget<T> {

    inner: Scroll<T, TextLayout<String>>
}

impl <T : TextStorage>  CustomWidget<T> {
    /// Create a new TextBox widget.
    pub fn new() -> Self {

        let mut scroll = Scroll::new(TextLayout::default())
        .content_must_fill(true);
        scroll.set_enabled_scrollbars(druid::scroll_component::ScrollbarsEnabled::Both);
        scroll.set_horizontal_scroll_enabled(false);
        Self {
            inner: scroll,
        }
    }

}


// If this widget has any child widgets it should call its event, update and layout
// (and lifecycle) methods as well to make sure it works. Some things can be filtered,
// but a general rule is to just pass it through unless you really know you don't want it.
impl<T : TextStorage> Widget<T> for CustomWidget<T> {
    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            /*Event::MouseDown(..) => {
                // do something
                println!("MouseDown");
            }*/
            _ => {}
        }
    }
    
    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &T,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        //
        // bx.max() returns the maximum size of the widget. Be careful
        // using this, since always make sure the widget is bounded.
        // If bx.max() is used in a scrolling widget things will probably
        // not work correctly.
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);

        // We can paint with a Z index, this indicates that this code will be run
        // after the rest of the painting. Painting with z-index is done in order,
        // so first everything with z-index 1 is painted and then with z-index 2 etc.
        // As you can see this(red) curve is drawn on top of the green curve
        //    ctx.paint_with_z_index(1, move |ctx| {
        //        let mut path = BezPath::new();
        //        path.move_to((0.0, size.height));
        //        path.quad_to((40.0, 50.0), (size.width, 0.0));
        //        // Create a color
        //        let stroke_color = Color::rgb8(128, 0, 0);
        //        // Stroke the path with thickness 1.0
        //        ctx.stroke(path, &stroke_color, 5.0);
        //    });

        // Create an arbitrary bezier path
        // let mut path = BezPath::new();
        // path.move_to(Point::ORIGIN);
        // path.quad_to((40.0, 50.0), (size.width, size.height));
        // // Create a color
        // let stroke_color = Color::rgb8(0, 128, 0);
        // // Stroke the path with thickness 5.0
        // ctx.stroke(path, &stroke_color, 5.0);

        // Rectangles: the path for practical people
        // let rect = Rect::from_origin_size((10.0, 10.0), (100.0, 100.0));
        // Note the Color:rgba8 which includes an alpha channel (7F in this case)
         let fill_color = Color::rgba8(0x00, 0x00, 0x00, 0x7F);
        // ctx.fill(rect, &fill_color);

        // Text is easy; in real use TextLayout should either be stored in the
        // widget and reused, or a label child widget to manage it all.
        // This is one way of doing it, you can also use a builder-style way.
        let mut layout = TextLayout::<String>::from_text("adsdsa".to_string());
        layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(24.0));
        layout.set_text_color(fill_color);
        layout.set_wrap_width(size.width);
        layout.rebuild_if_needed(ctx.text(), env);
        layout.draw(ctx, (0.0, 40.0));

    }
}

pub fn main() {
    let mut scr = Scroll::new(CustomWidget::new())
    .content_must_fill(true);
    scr.set_enabled_scrollbars(druid::scroll_component::ScrollbarsEnabled::Vertical);
    scr.set_horizontal_scroll_enabled(false);
    

    let window = WindowDesc::new(scr).title(LocalizedString::new("Fancy Colors"));
    //let window = WindowDesc::new(TextBox::new()).title(LocalizedString::new("TextBox"));
    AppLauncher::with_window(window)
        .log_to_console()
        .launch("Lunghissimo testo molto molto motloa sdasod a0d9ohas shfhaiuedf auihfuiah fipuashdf uiashuipfhLunghissimo testo molto molto motloa sdasod a0d9ohas shfhaiuedf auihfuiah fipuashdf uiashuipfhLunghissimo testo molto molto motloa sdasod a0d9ohas shfhaiuedf auihfuiah fipuashdf uiashuipfhLunghissimo testo molto molto motloa sdasod a0d9ohas shfhaiuedf auihfuiah fipuashdf uiashuipfhLunghissimo testo molto molto motloa sdasod a0d9ohas shfhaiuedf auihfuiah fipuashdf uiashuipfhLunghissimo testo molto molto motloa sdasod a0d9ohas shfhaiuedf auihfuiah fipuashdf uiashuipfhLunghissimo testo molto molto motloa sdasod a0d9ohas shfhaiuedf auihfuiah fipuashdf uiashuipfhLunghissimo testo molto molto motloa sdasod a0d9ohas shfhaiuedf auihfuiah fipuashdf uiashuipfh asduifh asuipfh uipasfh iausd hu".to_string())
        .expect("launch failed");
}

/*
const MAC_OR_LINUX_OR_OBSD: bool = cfg!(any(
    target_os = "macos",
    target_os = "linux",
    target_os = "openbsd"
));

/// When we scroll after editing or movement, we show a little extra of the document.
const SCROLL_TO_INSETS: Insets = Insets::uniform_xy(40.0, 0.0);


pub struct TextBox<T> {

    inner: Scroll<T, Padding<T, TextComponent<T>>>,
    scroll_to_selection_after_layout: bool,
    multiline: bool,
    was_focused_from_click: bool,
    pub handles_tab_notifications: bool,
    text_pos: Point,
}

impl<T: EditableText + TextStorage> TextBox<T> {
    /// Create a new TextBox widget.
    pub fn new() -> Self {

        let mut scroll = Scroll::new(Padding::new(
            theme::TEXTBOX_INSETS,
            TextComponent::default(),
        ))
        .content_must_fill(true);
        scroll.set_enabled_scrollbars(druid::scroll_component::ScrollbarsEnabled::Both);
        scroll.set_horizontal_scroll_enabled(false);
        Self {
            inner: scroll,
            scroll_to_selection_after_layout: false,
            multiline: true,
            was_focused_from_click: false,
            handles_tab_notifications: true,
            text_pos: Point::ZERO,
        }
    }

}

impl<T> TextBox<T> {

    pub fn with_text_size(mut self, size: impl Into<KeyOrValue<f64>>) -> Self {
        self.set_text_size(size);
        self
    }


    pub fn with_text_alignment(mut self, alignment: TextAlignment) -> Self {
        self.set_text_alignment(alignment);
        self
    }


    pub fn with_font(mut self, font: impl Into<KeyOrValue<FontDescriptor>>) -> Self {
        self.set_font(font);
        self
    }


    pub fn with_text_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.set_text_color(color);
        self
    }

    pub fn set_text_size(&mut self, size: impl Into<KeyOrValue<f64>>) {
        if !self.text().can_write() {
            tracing::warn!("set_text_size called with IME lock held.");
            return;
        }

        let size = size.into();
        self.text_mut()
            .borrow_mut()
            .layout
            .set_text_size(size.clone());
    }

    pub fn set_font(&mut self, font: impl Into<KeyOrValue<FontDescriptor>>) {
        if !self.text().can_write() {
            tracing::warn!("set_font called with IME lock held.");
            return;
        }
        let font = font.into();
    }

    pub fn set_text_alignment(&mut self, alignment: TextAlignment) {
        if !self.text().can_write() {
            tracing::warn!("set_text_alignment called with IME lock held.");
            return;
        }
        self.text_mut().borrow_mut().set_text_alignment(alignment);
    }


    pub fn set_text_color(&mut self, color: impl Into<KeyOrValue<Color>>) {
        if !self.text().can_write() {
            tracing::warn!("set_text_color called with IME lock held.");
            return;
        }
        self.text_mut().borrow_mut().layout.set_text_color(color);
    }

 
    pub fn text_position(&self) -> Point {
        self.text_pos
    }
}


impl<T> TextBox<T> {

    pub fn text(&self) -> &TextComponent<T> {
        self.inner.child().wrapped()
    }

    pub fn text_mut(&mut self) -> &mut TextComponent<T> {
        self.inner.child_mut().wrapped_mut()
    }


}

impl<T: TextStorage + EditableText> TextBox<T> {
    fn rect_for_selection_end(&self) -> Rect {
        let text = self.text().borrow();
        let layout = text.layout.layout().unwrap();

        let hit = layout.hit_test_text_position(text.selection().active);
        let line = layout.line_metric(hit.line).unwrap();
        let y0 = line.y_offset;
        let y1 = y0 + line.height;
        let x = hit.point.x;

        Rect::new(x, y0, x, y1)
    }

    fn scroll_to_selection_end(&mut self) {
        let rect = self.rect_for_selection_end();
        let view_rect = self.inner.viewport_rect();
        let is_visible =
            view_rect.contains(rect.origin()) && view_rect.contains(Point::new(rect.x1, rect.y1));
        if !is_visible {
            self.inner.scroll_to(rect + SCROLL_TO_INSETS);
        }
    }

}

impl<T: TextStorage + EditableText> Widget<T> for TextBox<T> {
    #[instrument(name = "TextBox", level = "trace", skip(self, ctx, event, data, env))]
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {

        match event {
            Event::Notification(cmd) => match cmd {
                cmd if cmd.is(TextComponent::SCROLL_TO) => {
                    let after_edit = *cmd.get(TextComponent::SCROLL_TO).unwrap_or(&false);
                    if after_edit {
                        ctx.request_layout();
                        self.scroll_to_selection_after_layout = true;
                    } else {
                        self.scroll_to_selection_end();
                    }
                    ctx.set_handled();
                    ctx.request_paint();
                }

                _ => (),
            },

            Event::MouseDown(mouse) if self.text().can_write() => {
                if !ctx.is_disabled() {
                    if !mouse.focus {
                        ctx.request_focus();
                        self.was_focused_from_click = true;
                    } else {
                        ctx.set_handled();
                    }
                }
                self.inner.event(ctx, event, data, env)

            }
            Event::MouseUp(_) => {
                self.inner.event(ctx, event, data, env)

            }
            Event::MouseMove(_) => {
                self.inner.event(ctx, event, data, env)

            }

            _ => (),
        }
        

    }

    #[instrument(name = "TextBox", level = "trace", skip(self, ctx, event, data, env))]
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                ctx.register_text_input(self.text().input_handler());
            }
            LifeCycle::BuildFocusChain => {
                //TODO: make this a configurable option? maybe?
                ctx.register_for_focus();
            }
            LifeCycle::FocusChanged(true) => {
                if self.text().can_write() && !self.multiline && !self.was_focused_from_click {
                    let selection = Selection::new(0, data.len());
                    let _ = self.text_mut().borrow_mut().set_selection(selection);
                    ctx.invalidate_text_input(ImeInvalidation::SelectionChanged);
                }
                self.text_mut().has_focus = true;
                // self.reset_cursor_blink(ctx.request_timer(CURSOR_BLINK_DURATION));
                self.was_focused_from_click = false;
                ctx.request_paint();
                ctx.scroll_to_view();
            }
            LifeCycle::FocusChanged(false) => {
                if self.text().can_write() && MAC_OR_LINUX_OR_OBSD && !self.multiline {
                    let selection = self.text().borrow().selection();
                    let selection = Selection::new(selection.active, selection.active);
                    let _ = self.text_mut().borrow_mut().set_selection(selection);
                    ctx.invalidate_text_input(ImeInvalidation::SelectionChanged);
                }
                self.text_mut().has_focus = false;
                if !self.multiline {
                    self.inner.scroll_to(Rect::ZERO);
                }
                self.was_focused_from_click = false;
                ctx.request_paint();
            }
            _ => (),
        }
        self.inner.lifecycle(ctx, event, data, env);
    }

    #[instrument(name = "TextBox", level = "trace", skip(self, ctx, old, data, env))]
    fn update(&mut self, ctx: &mut UpdateCtx, old: &T, data: &T, env: &Env) {
        self.inner.update(ctx, old, data, env);

        if self.text().can_write() {
            if let Some(ime_invalidation) = self.text_mut().borrow_mut().pending_ime_invalidation()
            {
                ctx.invalidate_text_input(ime_invalidation);
            }
        }
    }

    #[instrument(name = "TextBox", level = "trace", skip(self, ctx, bc, data, env))]
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        if !self.text().can_write() {
            tracing::warn!("Widget::layout called with outstanding IME lock.");
        }
        let min_width = env.get(theme::WIDE_WIDGET_WIDTH);
        let textbox_insets = env.get(theme::TEXTBOX_INSETS);

        let min_size = bc.constrain((min_width, 0.0));
        let child_bc = BoxConstraints::new(min_size, bc.max());

        let size = self.inner.layout(ctx, &child_bc, data, env);

        let text_metrics = self.text().borrow().layout.layout_metrics();


        let layout_baseline = text_metrics.size.height - text_metrics.first_baseline;
        let baseline_off = layout_baseline
            - (self.inner.child_size().height - self.inner.viewport_rect().height())
            + textbox_insets.y1;
        ctx.set_baseline_offset(baseline_off);
        if self.scroll_to_selection_after_layout {
            self.scroll_to_selection_end();
            self.scroll_to_selection_after_layout = false;
        }

        trace!(
            "Computed layout: size={}, baseline_offset={:?}",
            size,
            baseline_off
        );
        size
    }

    #[instrument(name = "TextBox", level = "trace", skip(self, ctx, data, env))]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        if !self.text().can_read() {
            tracing::warn!("Widget::paint called with outstanding IME lock, skipping");
            return;
        }
        let size = ctx.size();
        let border_width = env.get(theme::TEXTBOX_BORDER_WIDTH);

        let is_focused = ctx.is_focused();

        let border_color = if is_focused {
            env.get(theme::PRIMARY_LIGHT)
        } else {
            env.get(theme::BORDER_DARK)
        };

        // Paint the background
        let clip_rect = size
            .to_rect()

            .inset(-border_width / 2.0)
            .to_rounded_rect(env.get(theme::TEXTBOX_BORDER_RADIUS));

        ctx.fill(clip_rect, &Color::GRAY);

        if !data.is_empty() {
            self.inner.paint(ctx, data, env);
        }

        // Paint the border
        ctx.stroke(clip_rect, &border_color, border_width);
    }

    fn debug_state(&self, data: &T) -> DebugState {
        let text = data.slice(0..data.len()).unwrap_or_default();
        DebugState {
            display_name: self.short_type_name().to_string(),
            main_value: text.to_string(),
            ..Default::default()
        }
    }
}

impl<T: TextStorage + EditableText> Default for TextBox<T> {
    fn default() -> Self {
        TextBox::new()
    }
}
*/
