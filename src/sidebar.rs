use druid::{Widget, Color, RenderContext, WidgetPod, widget::Scroll, LayoutCtx, UpdateCtx, LifeCycle, LifeCycleCtx, Env, Size, BoxConstraints, PaintCtx, EventCtx, Event, WidgetExt, Point, Selector, piet::{Text, TextLayoutBuilder}};

use crate::application_state::{AppState, EpubData};

const ICON_SIZE : f64 = 40.;

pub const INTERNAL_COMMAND: Selector<InternalUICommand> =
    Selector::new("epub_reader.ui_command");



    const BAR_COLOR : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#7EA0B7");
    const _PRIMARY_LIGHT : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#637391");
    const CONTENT_COLOR : Result<Color, druid::piet::ColorParseError> = Color::from_hex_str("#597081");
    pub enum InternalUICommand {
    SwitchTab(TabKind),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TabKind {
    Toc,
    Highlights,
    Search,
    Notes,
}

pub struct CustomButton {
    kind: TabKind
}

impl CustomButton {
    pub fn new(kind : TabKind) -> Self {
        Self {
            kind
        }
    }
}

impl Widget<EpubData> for CustomButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.submit_command(INTERNAL_COMMAND.with(InternalUICommand::SwitchTab(self.kind.clone())));
            },
            _ => {}
        }
        ctx.request_paint();
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        //println!("Lifecycle: {:?}", event);
        ctx.request_paint();
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) {
        //println!("Update: {:?}", data);
        ctx.request_paint();
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        //println!("Layout: {:?}", bc.max());
        //ctx.request_paint();
        
        Size::new(ICON_SIZE, ICON_SIZE)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        //println!("Paint: {:?}", data);
        let half_width = ctx.size().width / 2.;
        let half_height = ctx.size().height / 2.-3.;
        match self.kind {
            TabKind::Toc => {
                let text = ctx.text();
                let layout = text
                    .new_text_layout("T")
                    .text_color(Color::WHITE)
                    .build()
                    .unwrap();

                ctx.draw_text(&layout, (half_width, half_height));
            }
            TabKind::Highlights => {
                let text = ctx.text();
                let layout = text
                    .new_text_layout("H")
                    .text_color(Color::WHITE)
                    .build()
                    .unwrap();
                ctx.draw_text(&layout, (half_width, half_height));
            }
            TabKind::Search => {
                let text = ctx.text();
                let layout = text
                    .new_text_layout("S")
                    .text_color(Color::WHITE)
                    .build()
                    .unwrap();
                ctx.draw_text(&layout, (half_width, half_height));
            }
            TabKind::Notes => {
                let text = ctx.text();
                let layout = text
                    .new_text_layout("N")
                    .text_color(Color::WHITE)
                    .build()
                    .unwrap();
                ctx.draw_text(&layout, (half_width, half_height));
            }
        
        }
    }
}




pub struct Toc {
    //list : WidgetPod<AppState, Box<dyn Widget<AppState>>>,
}

impl Toc {
    pub fn new() -> Self {
        Self {
            //list : WidgetPod::new(List::new(|| {
            //    Label::new(|item: &AppState, _env: &_| item.clone())
            //        .padding(5.)
            //        .expand_width()
            //        .center()
            //        .boxed()
            //})).lens(AppState::epub_data.then(EpubData::table_of_contents))
        }
    }
}

impl Widget<AppState> for Toc {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {
        //println!("Event: {:?}", event);
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppState, _env: &Env) {
        //println!("Lifecycle: {:?}", event);
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        //println!("Update");
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &AppState, _env: &Env) -> Size {
       //println!("Layout: {:?}", bc.max());
        
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &AppState, _env: &Env) {
        //println!("Paint");
        let rect = ctx.size().to_rect();

        let text = ctx.text();
        let layout = text
            .new_text_layout("TABLE OF CONTENTS")
            .font(druid::FontFamily::MONOSPACE, 14.0)

            .text_color(Color::WHITE)
            .max_width(ctx.size().width-10.)
            .build()
            .unwrap();
        ctx.draw_text(&layout, (5.0, 10.0));
    }
}


pub struct Panel {
    content : WidgetPod<AppState, Box<dyn Widget<AppState>>>,
}

impl Panel {
    pub fn new(kind : TabKind) -> Self {
        let widget = match kind {
            TabKind::Toc => {
                Scroll::new(Toc::new()).vertical()
            }
            _ => { 
                Scroll::new(Toc::new()).vertical()
                //Scroll::new(List::new(|| {
                //    Label::new(|data: &crate::application_state::TableOfContentsItem, _env: &Env| {
                //        data.title.clone()
                //    })
                //    .with_text_color(Color::WHITE)
                //    .with_text_size(16.0)
                //    .padding(5.0)
                //})).lens(AppState::epub_data.then(EpubData::table_of_contents))
            }
        };
        Self {
            content: WidgetPod::new(widget.boxed()),
        }
    }
}

impl Widget<AppState> for Panel {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
            self.content.event(ctx, event, data, env);
        match event {

            _ => {}
        }
        

        
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        self.content.lifecycle(ctx, event, data, env);
            
        
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        self.content.update(ctx, data, env);
            
        
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
        self.content.layout(ctx, bc, data, env);
        self.content.set_origin(ctx, data, env, Point::new(0., 0.));

        
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        // draw a background                
        let rect = ctx.size().to_rect();
        ctx.fill(rect, &CONTENT_COLOR.unwrap());
        self.content.paint(ctx, data, env);
    }
}



pub struct Sidebar {
    asd : Vec<WidgetPod<AppState, Box<dyn Widget<AppState>>>>,

    panel: WidgetPod<AppState, Box<dyn Widget<AppState>>>,

    opened : bool,

    opened_tab : Option<TabKind>,
}

impl Sidebar {
    pub fn new() -> Sidebar {

        let mut buttons = Vec::new();
        for kind in vec![TabKind::Toc, TabKind::Highlights, TabKind::Search, TabKind::Notes] {
            let other_but  = CustomButton::new(kind).lens(AppState::epub_data).boxed();
            buttons.push(WidgetPod::new(other_but));
        }

        let panel = WidgetPod::new(Panel::new(TabKind::Toc).boxed());



        Sidebar { asd: buttons, panel,  opened: false, opened_tab: None }
    }

}


impl Widget<AppState> for Sidebar {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut AppState, env: &druid::Env) {
        match event {
            Event::Command(cmd) => {
                if let Some(cmd) = cmd.get(INTERNAL_COMMAND) {
                    match cmd {
                        InternalUICommand::SwitchTab(tab) => {

                            if self.opened_tab == Some(tab.clone()) {
                                self.opened = false;
                                self.opened_tab = None;
                            } else {
                                self.opened = true;
                                self.opened_tab = Some(tab.clone());
                            }

                            ctx.request_layout();

                        }
                    }
                }
            },
            _ => {}
        }
        for button in self.asd.iter_mut() {
            button.event(ctx, event, data, env);
        }
        if event.should_propagate_to_hidden() {
            self.panel.event(ctx, event, data, env);
        }
        else if self.opened {
            self.panel.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &AppState, env: &druid::Env) {
        for button in self.asd.iter_mut() {
            button.lifecycle(ctx, event, data, env);
        }
        
        self.panel.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &AppState, data: &AppState, env: &druid::Env) {
        for button in self.asd.iter_mut() {
            button.update(ctx, data, env);
        }
        
        self.panel.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &AppState, env: &druid::Env) -> druid::Size {
        let max_size = bc.max();
        let closed_size = Size::new(ICON_SIZE, max_size.height);
        let mut prev_height = Point::new(0., 0.);
        for button in self.asd.iter_mut() {
            button.layout(ctx, &BoxConstraints::tight(max_size), data, env);
            button.set_origin(ctx, data, env, prev_height);

            prev_height.y += button.layout_rect().height();
        }
        if self.opened {
            self.panel.layout(ctx, &BoxConstraints::tight(max_size), data, env);
            self.panel.set_origin(ctx, data, env, Point::new(ICON_SIZE, 0.));
            max_size
        } else {
            closed_size
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &druid::Env) {
        
        let rect = Size::new(ICON_SIZE, ctx.size().height).to_rect();
        ctx.fill(rect, &BAR_COLOR.unwrap());



        for button in self.asd.iter_mut() {
            button.paint(ctx, data, env);
        }

        if self.opened {
            self.panel.paint(ctx, data, env);
        
        let mut sizee = Size::new(2., ICON_SIZE).to_rect();
        match self.opened_tab.as_ref().unwrap() {

            TabKind::Toc => {
                sizee.y0 = 0.;
                ctx.fill(sizee, &Color::rgb8(255, 255, 255));
            }
            TabKind::Highlights => {
                sizee.y0 = ICON_SIZE;
                sizee.y1 = ICON_SIZE*2.;
                ctx.fill(sizee, &Color::rgb8(255, 255, 255));
            }
            TabKind::Search => {
                sizee.y0 = ICON_SIZE*2.;
                sizee.y1 = ICON_SIZE*3.;

                ctx.fill(sizee, &Color::rgb8(255, 255, 255));
            }
            TabKind::Notes => {
                sizee.y0 = ICON_SIZE*3.;
                sizee.y1 = ICON_SIZE*4.;

                ctx.fill(sizee, &Color::rgb8(255, 255, 255));
            }
        }
    }
    }
}


