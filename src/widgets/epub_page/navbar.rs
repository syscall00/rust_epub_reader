
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx,
    Widget, Rect, Point, WidgetPod, piet::{Text, TextLayoutBuilder, TextLayout}, Selector,
};

use crate::{appstate::EpubData, core::commands::CHANGE_PAGE};


pub struct NavigationBar {
    navigation_buttons : Vec<WidgetPod<EpubData, NavigationButton>>,



    height : f64,
    //chapter_slider : WidgetPod<EpubData, Slider>,
}

impl NavigationBar {
    pub fn new() -> Self {
        let mut navigation_buttons = Vec::new();
        navigation_buttons.push(WidgetPod::new(NavigationButton::new(false)));
        navigation_buttons.push(WidgetPod::new(NavigationButton::new(true)));

        //let chapter_slider = WidgetPod::new(Slider::new());
        
        Self {navigation_buttons, height: 0. }
    }

    pub fn with_height(mut self, height : f64 ) -> Self  {
        self.height = height;
        self
    }
}

impl Widget<EpubData> for NavigationBar {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            Event::MouseMove(_) => {
                ctx.set_active(true);
                if ctx.is_active() {
                    ctx.request_paint();
                }
            }
            _ => {}
        }
        for button in self.navigation_buttons.iter_mut() {
            button.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {

        for button in self.navigation_buttons.iter_mut() {
            button.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &EpubData, data: &EpubData, env: &Env) {

        for button in self.navigation_buttons.iter_mut() {
            button.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {

        for (i, button) in self.navigation_buttons.iter_mut().enumerate() {
            button.layout(ctx, bc, data, env);

            let origin = if i == 0 {
                Point::new(0.0, 0.0)
            } else {
                Point::new(bc.max().width-50., 0.0)
            };

            button.set_origin(ctx, data, env, origin)
        }
        if self.height == 0. {
            bc.max()
        }
        else {
            Size::new(bc.max().width, self.height)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        let COLOR_DARK_BACKG = Color::from_hex_str("#36494e").unwrap();
        let mut is_hot = false;
        for button in self.navigation_buttons.iter_mut() {
            button.paint(ctx, data, env);
            is_hot = is_hot || button.is_hot();
        }

        if is_hot || ctx.is_hot() {
            let size = ctx.size();
            ctx.fill(size.to_rect(), &COLOR_DARK_BACKG.with_alpha(0.3));

            let vec = vec!["Reading percentage", "Page number", "Page pos"];

            for (i, s) in vec.iter().enumerate() {
                let text = ctx.text();
                let mut t = String::new();
                t.push_str(s);
                if i == 0 {
                t.push_str(&data.epub_metrics.current_chapter.to_string());
                }
                else if i == 1{
                    t.push_str(&data.epub_metrics.BOOK_POSITION.to_string());
                }

                let layout = text
                .new_text_layout(t)
                .text_color(Color::BLACK)
                .build()
                .unwrap();
    
                let text_size = layout.size();
    
                let slot = ((i as f64)/3. + ((i+1) as f64)/3.) / 2.;
                let x = slot * ctx.size().width;
    
                let y = size.height - text_size.height;
                let text_pos = Point::new(x, y);
                ctx.draw_text(&layout, text_pos);
            }

        }


    }
}



struct NavigationButton {
    direction : bool // true = next, false = previous
}

impl NavigationButton {
    pub fn new(direction : bool) -> Self {
        Self {
            direction
        }
    }
}

impl Widget<EpubData> for NavigationButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut EpubData, _env: &Env) {
        match event {
            Event::MouseUp(_) => {
                ctx.set_handled();
                ctx.submit_command(CHANGE_PAGE.with(self.direction));

            }
            _ => { } 
        }
    }

    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &EpubData, _: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) { }

    fn layout(&mut self, _: &mut LayoutCtx, _: &BoxConstraints, _: &EpubData, _: &Env) -> Size {
        Size::new(50., 50.)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        
        // Draw arrow here!!!
        let r = Rect::from_origin_size(Point::new(0., 0.), ctx.size());
        ctx.fill(r, &Color::BLACK);
    }
}