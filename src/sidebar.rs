use druid::{Widget, Color, RenderContext, WidgetPod, widget::{Scroll, List, Label, TextBox, Controller, Flex}, LayoutCtx, UpdateCtx, LifeCycle, LifeCycleCtx, Env, Size, BoxConstraints, PaintCtx, EventCtx, Event, WidgetExt, Point, piet::{Text, TextLayoutBuilder}, LensExt, Data, ArcStr, TextLayout};

use crate::
{appstate::{ EpubData, IndexedText, AppState}, 

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


struct ClickableLabel {
    layout: TextLayout<ArcStr>,
}
impl ClickableLabel {
    fn new() -> Self {
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
                    ctx.submit_command(GO_TO_POS.with((*data.value).clone()));
                }
            }
            Event::MouseMove(_) => {
                ctx.set_cursor(&druid::Cursor::Pointer);
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &IndexedText, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.layout.set_text(data.key.clone());
                self.layout.rebuild_if_needed(ctx.text(), env);
            }
            LifeCycle::HotChanged(_) => {
                ctx.request_paint();
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &IndexedText, data: &IndexedText, env: &Env) {
        if !(old_data.same(data)) {
            self.layout.set_text(data.key.clone());
            self.layout.rebuild_if_needed(ctx.text(), env);
            ctx.request_layout();
            
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &IndexedText, env: &Env) -> Size {
        self.layout.set_wrap_width(bc.max().width);
        self.layout.rebuild_if_needed(ctx.text(), env);
        let mut size = self.layout.size();
        size.width = bc.max().width;
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &IndexedText, env: &Env) {
        let size = ctx.size();
        if ctx.is_hot() {
        let rect = ctx.size().to_rect();

            ctx.fill(rect, &Color::BLUE);
        }
        //println!("painting : {:?}", size);
        ctx.clip(size.to_rect());

        self.layout.draw(ctx,(5., 0.));
        
    }
    
}

pub struct Panel {
    header: TextLayout<ArcStr>,
    input_widget: Option<WidgetPod<EpubData, Box<dyn Widget<EpubData>>>>,
    widget: WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,

}

impl Panel {
    pub fn new(title: &str, widget: Box<dyn Widget<EpubData>>) -> Self {
        Self {
            header: TextLayout::from_text(title.to_string()),
            input_widget: None,
            widget: WidgetPod::new(widget),
        }
    }

    pub fn with_input_widget(mut self) -> Self {
        let input_widget = TextBox::new().lens(EpubData::search_input).boxed();
        self.input_widget = Some(WidgetPod::new(input_widget));
        self
    }
}

impl Widget<EpubData> for Panel {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        if self.input_widget.is_some() {
            match event {
                Event::KeyUp(key) => {
                    if key.code == druid::Code::Enter {
                        data.search_string_in_book();
                        ctx.request_update();
                        ctx.request_layout();
                    }
                },
                _ => {},
            }
            self.input_widget.as_mut().unwrap().event(ctx, event, data, env);
        }

        self.widget.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        if self.input_widget.is_some() {
            self.input_widget.as_mut().unwrap().lifecycle(ctx, event, data, env);
        }
        self.widget.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) {
        if !old_data.same(data) {
            if self.input_widget.is_some() {
                self.input_widget.as_mut().unwrap().update(ctx, data, env);
            }
            self.widget.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        let size = bc.max();
        let mut widget_size = Size::new(size.width, size.height - 30.);
        let mut input_widget_size = Size::new(size.width, 0.);
        if self.input_widget.is_some() {
            input_widget_size = self.input_widget.as_mut().unwrap().layout(ctx, &BoxConstraints::tight(Size::new(size.width-50., 25.)), data, env);
            self.input_widget.as_mut().unwrap().set_origin(ctx, data, env, Point::new(0., 30.));
            widget_size.height -= 25.;
        }
        
        self.header.rebuild_if_needed(ctx.text(), env);
        self.header.layout();
        self.widget.layout(ctx, &BoxConstraints::tight(widget_size), data, env);
        self.widget.set_origin(ctx, data, env, Point::new(0., 30.+ input_widget_size.height));

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        let size = ctx.size();
        ctx.fill(size.to_rect(), &CONTENT_COLOR.unwrap());
        self.header.draw(ctx, (5., 5.));
        if self.input_widget.is_some() {
            self.input_widget.as_mut().unwrap().paint(ctx, data, env);
        }
        self.widget.paint(ctx, data, env);
    }
}
pub struct Toc {
    list : WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
}


impl Toc {
    pub fn new() -> Self {

        let list = List::new(|| {

            //| &IndexedText, _env: &_ | -> Widget<T> + 'static
            //let a = | item: &IndexedText, _env: &Env| -> String {
            //    item.key.clone()
            //};
            //Label::new(a)
            ClickableLabel::new()

            })
            .lens(EpubData::table_of_contents).boxed();

        Self {
            list : WidgetPod::new(Scroll::new(list).vertical().boxed())
        }
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
                TabKind::Toc => {
                    const TOC_TITLE : &str = "TABLE OF CONTENTS";
                    let widget = Scroll::new(
                        List::new(|| {
                            ClickableLabel::new()
                        }
                    )
                    .lens(EpubData::table_of_contents)).vertical().boxed();
        
                    panels.push(
                        WidgetPod::new((
                            Panel::new(TOC_TITLE, widget))
                            .boxed()
                        )
                    )

                },
                TabKind::Search => {
                    const TOC_TITLE : &str = "SEARCH";
                    let widget = Scroll::new(
                            List::new(|| {
                                ClickableLabel::new()
                    }).lens(EpubData::search_results)).vertical()
                    .boxed();
        
                    panels.push(
                        WidgetPod::new((
                            Panel::new(TOC_TITLE, widget))
                            .with_input_widget()
                            .boxed()
                        )
                    )
                },
                TabKind::Highlights => {
                    const TOC_TITLE : &str = "HIGHLIGHTS";
                    let widget = Scroll::new(
                        List::new(|| {
                            ClickableLabel::new()
                        }
                    )
                    .lens(EpubData::book_highlights)).vertical().boxed();
        
                    panels.push(
                        WidgetPod::new((
                            Panel::new(TOC_TITLE, widget))
                            .boxed()
                        )
                    )
                },
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
        const PANEL_PADDING : f64 = 0.;
        let max_size = bc.max();
        let closed_size = Size::new(ICON_SIZE, max_size.height);
        let mut prev_height = Point::new(0., 0.);
        for button in self.side_buttons.iter_mut() {
            button.layout(ctx, &BoxConstraints::tight(max_size), data, env);
            button.set_origin(ctx, data, env, prev_height);

            prev_height.y += button.layout_rect().height();
        }
        if self.opened_tab.is_some() {
            self.get_active_panel().layout(ctx, &BoxConstraints::tight(Size::new(ICON_SIZE*6., max_size.height-PANEL_PADDING)), data, env);
            self.get_active_panel().set_origin(ctx, data, env, Point::new(ICON_SIZE, 0.));
            Size::new(ICON_SIZE*6., max_size.height)
        } else {
            closed_size
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &EpubData, env: &druid::Env) {
        
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
            }
            TabKind::Search => {
                sizee.y0 = ICON_SIZE;
                sizee.y1 = ICON_SIZE*2.;
            }
            TabKind::Highlights => {
                sizee.y0 = ICON_SIZE*2.;
                sizee.y1 = ICON_SIZE*3.;

            }
        }
        ctx.fill(sizee, &Color::rgb8(255, 255, 255));

    }
    }
}


