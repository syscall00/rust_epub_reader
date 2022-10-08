use druid::{Widget, Color, RenderContext, WidgetPod, widget::{Scroll, List, Label, TextBox}, LayoutCtx, UpdateCtx, LifeCycle, LifeCycleCtx, Env, Size, BoxConstraints, PaintCtx, EventCtx, Event, WidgetExt, Point, piet::{Text, TextLayoutBuilder}, LensExt};

use crate::
{appstate::{ EpubData, IndexedText}, 

core::{commands::{INTERNAL_COMMAND, GO_TO_POS}, style::{BAR_COLOR, CONTENT_COLOR}}};

const ICON_SIZE : f64 = 40.;




    pub enum InternalUICommand {
    SwitchTab(TabKind),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TabKind {
    Toc = 0,
    Search = 1,
    Highlights = 2,
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
            // set cursors hand on hover 
            Event::MouseMove(_) => {
                ctx.set_cursor(&druid::Cursor::Pointer);
            }
            _ => {}
        }
        ctx.request_paint();
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) { }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        Size::new(ICON_SIZE, ICON_SIZE)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
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
    input_box: WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
    search_results: WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,

}

impl Search {
    pub fn new() -> Self {

        let input_box = TextBox::new().lens(EpubData::search_input).boxed();

        let search_results = List::new(|| {
            Label::new(|item: &IndexedText, _env: &_| item.key.clone())
            .on_click(|ctx, data: &mut IndexedText, _env | {
                ctx.submit_command(GO_TO_POS.with((*data.value).clone()));
            })

            }).lens(EpubData::search_results).boxed();


        Self {
            input_box: WidgetPod::new(input_box),
            search_results: WidgetPod::new(search_results),
        }
    }
}

impl Widget<EpubData> for Search {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            // If click middle, search
            Event::MouseDown(mouse) => {
                if mouse.button.is_right() {
                    data.search_string_in_book();
                    ctx.request_update();
                    ctx.request_layout();
                }
            }
            _ => {},
        }
        self.input_box.event(ctx, event, data, env);
        self.search_results.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        self.input_box.lifecycle(ctx, event, data, env);
        self.search_results.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) {
        self.input_box.update(ctx, data, env);
        self.search_results.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        let size = bc.max();
        let input_box_size = Size::new(size.width, 30.);
        let search_results_size = Size::new(size.width, 300.);
        self.input_box.layout(ctx, &BoxConstraints::tight(input_box_size), data, env);
        self.input_box.set_origin(ctx, data, env, Point::ORIGIN);
        self.search_results.layout(ctx, &BoxConstraints::tight(search_results_size), data, env);
        self.search_results.set_origin(ctx, data, env, Point::new(0., 30.));

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        self.input_box.paint(ctx, data, env);
        self.search_results.paint(ctx, data, env);
    }
}

pub struct Toc {
    list : WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
}

impl Toc {
    pub fn new() -> Self {

        let list = List::new(|| {
            Label::new(|item: &IndexedText, _env: &_| item.key.clone())
            .on_click(|ctx, data: &mut IndexedText, _env | {
                ctx.submit_command(GO_TO_POS.with((*data.value).clone()));
            })

            }).lens(EpubData::table_of_contents).boxed();

        Self {
            list : WidgetPod::new(list)
        }
    }
}

impl Widget<EpubData> for Toc {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {

        self.list.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &EpubData, _env: &Env) {
        self.list.lifecycle(_ctx, _event, _data, _env);
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &EpubData, _data: &EpubData, _env: &Env) {
        self.list.update(_ctx, _data, _env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        let size = self.list.layout(ctx, bc, data, env);
        self.list.set_origin(ctx, data, env, Point::new(0., 30.));
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &EpubData, _env: &Env) {
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


pub struct Hightlights {
    list : WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
}

impl Hightlights {
    pub fn new() -> Self {

        let list = List::new(|| {
            Label::new(|item: &IndexedText, _env: &_| item.key.clone())
            .on_click(|ctx, data: &mut IndexedText, _env | {
                ctx.submit_command(GO_TO_POS.with((*data.value).clone()));
            })

            }).lens(EpubData::book_highlights).boxed();

        Self {
            list : WidgetPod::new(list)
        }
    }
}

impl Widget<EpubData> for Hightlights {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {

        self.list.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &EpubData, _env: &Env) {
        self.list.lifecycle(_ctx, _event, _data, _env);
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &EpubData, _data: &EpubData, _env: &Env) {
        self.list.update(_ctx, _data, _env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        let size = self.list.layout(ctx, bc, data, env);
        self.list.set_origin(ctx, data, env, Point::new(0., 30.));
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &EpubData, _env: &Env) {
        let rect = ctx.size().to_rect();

        let text = ctx.text();
        let layout = text
            .new_text_layout("HIGHLIGHTS")
            .font(druid::FontFamily::MONOSPACE, 14.0)

            .text_color(Color::WHITE)
            .max_width(ctx.size().width-10.)
            .build()
            .unwrap();
        ctx.draw_text(&layout, (5.0, 10.0));
    
        self.list.paint(ctx, _data, _env);
    
    }
}




pub struct Sidebar {
    side_buttons : Vec<WidgetPod<EpubData, Box<dyn Widget<EpubData>>>>,
    panels: Vec<WidgetPod<EpubData, Box<dyn Widget<EpubData>>>>,

    opened_tab : Option<TabKind>,
}

impl Sidebar {
    pub fn new() -> Sidebar {

        let mut side_buttons = Vec::new();
        let mut panels = Vec::new();

        for kind in vec![TabKind::Toc, TabKind::Search, TabKind::Highlights] {

            match &kind {
                TabKind::Toc => panels.push(WidgetPod::new((Toc::new()).boxed())),
                TabKind::Search => panels.push(WidgetPod::new((Search::new()).boxed())),
                TabKind::Highlights => panels.push(WidgetPod::new((Hightlights::new()).boxed())),
                _ => {},

            }
            let other_but  = CustomButton::new(kind).boxed();
            side_buttons.push(WidgetPod::new(other_but));

        }
        Sidebar { side_buttons, panels, opened_tab: None }
    }

    pub fn get_active_panel(&mut self) -> &mut WidgetPod<EpubData, Box<dyn Widget<EpubData>>> {
        if !self.opened_tab.is_some() {
            panic!("Sidebar is not opened");
        }
        &mut self.panels[*self.opened_tab.as_ref().unwrap() as usize]
    }

}


impl Widget<EpubData> for Sidebar {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut EpubData, env: &druid::Env) {
        match event {
            Event::Command(cmd) => {
                if let Some(cmd) = cmd.get(INTERNAL_COMMAND) {
                    match cmd {
                        InternalUICommand::SwitchTab(tab) => {

                        
                            if self.opened_tab == Some(tab.clone()) {
                                self.opened_tab = None;
                            } else {
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
        for button in self.side_buttons.iter_mut() {
            button.event(ctx, event, data, env);
        }
        if event.should_propagate_to_hidden() {
            for panel in self.panels.iter_mut() {
                panel.event(ctx, event, data, env);
            }
        }
        if self.opened_tab.is_some() {
            self.get_active_panel().event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &EpubData, env: &druid::Env) {
        for button in self.side_buttons.iter_mut() {
            button.lifecycle(ctx, event, data, env);
        }
        for panel in self.panels.iter_mut() {
            panel.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &EpubData, data: &EpubData, env: &druid::Env) {
        for button in self.side_buttons.iter_mut() {
            button.update(ctx, data, env);
        }
        if self.opened_tab.is_some() {
            self.get_active_panel().update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &druid::Env) -> druid::Size {
        let max_size = bc.max();
        let closed_size = Size::new(ICON_SIZE, max_size.height);
        let mut prev_height = Point::new(0., 0.);
        for button in self.side_buttons.iter_mut() {
            button.layout(ctx, &BoxConstraints::tight(max_size), data, env);
            button.set_origin(ctx, data, env, prev_height);

            prev_height.y += button.layout_rect().height();
        }
        if self.opened_tab.is_some() {
            self.get_active_panel().layout(ctx, &BoxConstraints::tight(max_size), data, env);
            self.get_active_panel().set_origin(ctx, data, env, Point::new(ICON_SIZE, 0.));
            Size::new(ICON_SIZE*6., max_size.height)
        } else {
            closed_size
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &EpubData, env: &druid::Env) {
        
        if self.opened_tab.is_some() {
            let size = ctx.size();
            ctx.fill(size.to_rect(), &CONTENT_COLOR.unwrap());
        }

        let rect = Size::new(ICON_SIZE, ctx.size().height).to_rect();
        ctx.fill(rect, &BAR_COLOR.unwrap());
    


        for button in self.side_buttons.iter_mut() {
            button.paint(ctx, data, env);
        }

        if self.opened_tab.is_some() {
            self.get_active_panel().paint(ctx, data, env);
        
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


