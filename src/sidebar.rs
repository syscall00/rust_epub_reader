use druid::{Widget, Color, RenderContext, WidgetPod, widget::{Scroll, List, Label, TextBox}, LayoutCtx, UpdateCtx, LifeCycle, LifeCycleCtx, Env, Size, BoxConstraints, PaintCtx, EventCtx, Event, WidgetExt, Point, Selector, piet::{Text, TextLayoutBuilder}, LensExt};

use crate::
{appstate::{AppState, EpubData, TocItems, PagePosition, SearchResult}, 

core::commands::{INTERNAL_COMMAND, SEARCH, GO_TO_POS}};

const ICON_SIZE : f64 = 40.;




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
                ctx.set_handled();
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

        
        }
    }
}


pub struct Search {
    input_box: WidgetPod<AppState, Box<dyn Widget<AppState>>>,
    search_results: WidgetPod<AppState, Box<dyn Widget<AppState>>>,

}

impl Search {
    pub fn new() -> Self {

        let input_box = TextBox::new().lens(AppState::search_input).boxed();

        let search_results = List::new(|| {
            Label::new(|item: &SearchResult, _env: &_| item.key.clone())
            .on_click(|ctx, data: &mut SearchResult, _env | {
                ctx.submit_command(GO_TO_POS.with((*data.value).clone()));
            })

            }).lens(AppState::epub_data.then(EpubData::search_results)).boxed();


        Self {
            input_box: WidgetPod::new(input_box),
            search_results: WidgetPod::new(search_results),
        }
    }
}

impl Widget<AppState> for Search {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            // If click middle, search
            Event::MouseDown(mouse) => {
                if mouse.button.is_middle() {
                    data.epub_data.search_string_in_book(&data.search_input);
                    ctx.request_update();
                    ctx.request_layout();
                    //ctx.submit_command(SEARCH.with(data.search_input.clone()));
                }
            }
            _ => {},
        }
        self.input_box.event(ctx, event, data, env);
        self.search_results.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        self.input_box.lifecycle(ctx, event, data, env);
        self.search_results.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        self.input_box.update(ctx, data, env);
        self.search_results.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
        let size = bc.max();
        let input_box_size = Size::new(size.width, 30.);
        let search_results_size = Size::new(size.width, 300.);
        self.input_box.layout(ctx, &BoxConstraints::tight(input_box_size), data, env);
        self.input_box.set_origin(ctx, data, env, Point::ORIGIN);
        self.search_results.layout(ctx, &BoxConstraints::tight(search_results_size), data, env);
        self.search_results.set_origin(ctx, data, env, Point::new(0., 30.));

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.input_box.paint(ctx, data, env);
        self.search_results.paint(ctx, data, env);
    }
}

pub struct Toc {
    list : WidgetPod<AppState, Box<dyn Widget<AppState>>>,
}

impl Toc {
    pub fn new() -> Self {

        let list = List::new(|| {
            Label::new(|item: &TocItems, _env: &_| item.key.clone())
            .on_click(|ctx, data: &mut TocItems, _env | {
                ctx.submit_command(GO_TO_POS.with((*data.value).clone()));
            })

            }).lens(AppState::epub_data.then(EpubData::table_of_contents)).boxed();

        Self {
            list : WidgetPod::new(list)
        }
    }
}

impl Widget<AppState> for Toc {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {

        self.list.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppState, _env: &Env) {
        self.list.lifecycle(_ctx, _event, _data, _env);
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        self.list.update(_ctx, _data, _env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
       //println!("Layout: {:?}", bc.max());
        let size = self.list.layout(ctx, bc, data, env);
        self.list.set_origin(ctx, data, env, Point::new(0., 30.));
        //size
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
    
        self.list.paint(ctx, _data, _env);
    
    }
}


pub struct Panel {
    content : WidgetPod<AppState, Box<dyn Widget<AppState>>>,
}

impl Panel {
    pub fn new(kind : TabKind) -> Self {
        let widget = match kind {
            TabKind::Toc => {
                Scroll::new(Search::new()).vertical()
            }
            TabKind::Search => { 
                Scroll::new(Search::new()).vertical()
            }
            TabKind::Highlights => {
                Scroll::new(Search::new()).vertical()
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
        for kind in vec![TabKind::Toc, TabKind::Highlights, TabKind::Search] {
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
                            ctx.set_handled();

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
        }
    }
    }
}


