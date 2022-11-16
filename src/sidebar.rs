use druid::{
    piet::{Text, TextLayoutBuilder},
    widget::{Button, Flex, Label, List, Scroll, Slider, TextBox, ControllerHost},
    ArcStr, BoxConstraints, Color, Data, Env, Event, EventCtx, FontFamily, LayoutCtx, LensExt,
    LifeCycle, LifeCycleCtx, PaintCtx, Point, RenderContext, Size, TextLayout, UpdateCtx, Widget,
    WidgetExt, WidgetPod,
};
use druid_material_icons::IconPaths;

use crate::{
    appstate::{EpubData, IndexedText, SidebarData},
    core::{
        commands::{GO_TO_POS},
        constants::{epub_settings::{
            MAX_FONT_SIZE, MAX_MARGIN, MAX_PARAGRAPH_SPACING, MIN_FONT_SIZE, MIN_MARGIN,
            MIN_PARAGRAPH_SPACING,
        }, commands::{InternalUICommand, INTERNAL_COMMAND}},
        style::{self, COMPLEMENTARY_DARK, PRIMARY_DARK},
    },
    widgets::{widgets::{Icon, RoundButton}, tooltip::TooltipController}, data::epub::settings::{EpubSettings, VisualizationMode},
};

const ICON_SIZE: f64 = 32.;

// Create GroupButton; a group of buttons that can be toggled on and off
pub struct GroupButton<T> {
    buttons: Vec<WidgetPod<T, RoundButton<T>>>,
    active: usize,
}

impl <T : Data> GroupButton<T> {
    pub fn new(buttons: Vec<RoundButton<T>>) -> Self {
        Self {
            buttons: buttons.into_iter().map(|button| WidgetPod::new(button)).collect(),
            active: 0,
        }
    }
    pub fn with_active(mut self, active: usize) -> Self {
        self.active = active;
        self
    }
}

impl <T : Data> Widget<T> for GroupButton<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                if mouse_event.button.is_left() {
                    let rect = ctx.size().to_rect();
                    if rect.contains(mouse_event.pos) {
                        let mut index = 0;
                        for button in &mut self.buttons {
                            let button_rect = druid::Rect::from_origin_size(Point::ORIGIN, button.layout_rect().size());
                            if button_rect.contains(mouse_event.pos) {
                                self.active = index;
                                ctx.request_paint();
                                break;
                            }
                            index += 1;
                        }
                    }
                }
            }
            _ => {}
        }
        for button in &mut self.buttons {
            button.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        for button in &mut self.buttons {
            button.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        for button in &mut self.buttons {
            button.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {

        const PADDING_BETWEEN_BUTTONS: f64 = 8.;
        // positionate buttons consecutively
        let mut x = 0.;
        let mut y = 0.;
        let mut max_height : f64 = 0.;
        for button in &mut self.buttons {
            let size = button.layout(ctx, bc, data, env);
            button.set_origin(ctx, data, env, Point::new(x, y));
            x += size.width + PADDING_BETWEEN_BUTTONS;
            max_height = max_height.max(size.height);
        }
        Size::new(x, max_height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let mut index = 0;
        for button in &mut self.buttons {
            // create a shadow for active button
            if index == self.active {
                let rect = druid::Rect::from_origin_size(Point::ORIGIN, button.layout_rect().size());
                let shadow = ctx.render_ctx.solid_brush(Color::rgb8(0, 0, 0).with_alpha(0.2));
                ctx.render_ctx.fill(rect, &shadow);
            }

            button.paint(ctx, data, env);
        }
    }
}



pub trait ButtonTrait {
    fn icon(&self) -> IconPaths;
    fn hint(&self) -> String;
    fn command(&self) -> InternalUICommand;
}
#[derive(Debug, Clone, PartialEq)]
pub enum PanelButton {
    Toc,
    Search,
    //Highlights,
    Settings,
}
impl PanelButton {
    pub fn title(&self) -> String {
        match self {
            PanelButton::Toc => "Table of Contents".to_string(),
            PanelButton::Search => "Search".to_string(),
            //PanelButton::Highlights => "Highlights".to_string(),
            PanelButton::Settings => "Settings".to_string(),
        }
    }

    pub fn to_widget(&self) -> Box<dyn Widget<EpubData>> {
        match self {
            PanelButton::Toc => Scroll::new(
                List::new(|| ClickableLabel::new())
                    .lens(EpubData::sidebar_data.then(SidebarData::table_of_contents)),
            )
            .vertical()
            .boxed(),
            PanelButton::Search => Box::new(
                Scroll::new(
                    List::new(|| ClickableLabel::new())
                        .lens(EpubData::sidebar_data.then(SidebarData::search_results)),
                )
                .vertical()
                .boxed(),
            ),

            PanelButton::Settings => Scroll::new(
                Flex::column()
                    .with_child(
                        GroupButton::new(vec![
                            RoundButton::new(druid_material_icons::normal::content::AMP_STORIES),
                                //.with_tooltip("Dark Mode")
                                //.with_command(InternalUICommand::ToggleDarkMode),
                            RoundButton::new(druid_material_icons::normal::image::AUTO_STORIES)
                                //.with_tooltip("Light Mode")
                                //.with_command(InternalUICommand::ToggleLightMode),
                        ])
                        // Create three button able to change visualization mode
                        //Flex::row()
                        //    .with_child(Button::new("Single").on_click(
                        //        |ctx, data: &mut EpubData, _env| {
                        //            data.epub_settings.visualization_mode =
                        //                VisualizationMode::SinglePage;
                        //            ctx.request_paint();
                        //        },
                        //    ))
                        //    .with_child(Button::new("Two").on_click(
                        //        |ctx, data: &mut EpubData, _env| {
                        //            data.epub_settings.visualization_mode =
                        //                VisualizationMode::TwoPage;
                        //            ctx.request_paint();
                        //        },
                        //    )),
                    )
                    .with_spacer(20.)
                    .with_child(
                        Flex::column()
                            .with_child(Label::new(|data: &EpubData, _env: &_| {
                                format!(
                                    "Font size: {number:.prec$}",
                                    prec = 2,
                                    number = data.epub_settings.font_size
                                )
                            }))
                            .with_child(
                                Slider::new()
                                    .with_range(MIN_FONT_SIZE, MAX_FONT_SIZE)
                                    .lens(EpubData::epub_settings.then(EpubSettings::font_size))
                                    .expand_width(),
                            ),
                    )
                    .with_spacer(10.)
                    .with_child(
                        Flex::column()
                            .with_child(Label::new(|data: &EpubData, _env: &_| {
                                format!(
                                    "Text margin: {number:.prec$}",
                                    prec = 2,
                                    number = data.epub_settings.margin
                                )
                            }))
                            .with_child(
                                Slider::new()
                                    .with_range(MIN_MARGIN, MAX_MARGIN)
                                    .lens(EpubData::epub_settings.then(EpubSettings::margin))
                                    .expand_width(),
                            ),
                    )
                    .with_spacer(10.)
                    .with_child(
                        Flex::column()
                            .with_child(Label::new(|data: &EpubData, _env: &_| {
                                format!(
                                    "Paragraph spacing: {number:.prec$}",
                                    prec = 2,
                                    number = data.epub_settings.paragraph_spacing
                                )
                            }))
                            .with_child(
                                Slider::new()
                                    .with_range(MIN_PARAGRAPH_SPACING, MAX_PARAGRAPH_SPACING)
                                    .lens(
                                        EpubData::epub_settings
                                            .then(EpubSettings::paragraph_spacing),
                                    )
                                    .expand_width(),
                            ),
                    ),
            )
            .vertical()
            .boxed(),
        }
    }
}
impl ButtonTrait for PanelButton {
    fn icon(&self) -> IconPaths {
        match self {
            PanelButton::Toc => druid_material_icons::normal::communication::LIST_ALT,
            PanelButton::Search => druid_material_icons::normal::action::FIND_IN_PAGE,
            //PanelButton::Highlights => druid_material_icons::normal::action::SETTINGS,
            PanelButton::Settings => druid_material_icons::normal::action::SETTINGS,
        }
    }
    fn hint(&self) -> String {
        match self {
            PanelButton::Toc => "Table of Contents".to_string(),
            PanelButton::Search => "Search".to_string(),
            //PanelButton::Highlights => "Highlights".to_string(),
            PanelButton::Settings => "Settings".to_string(),
        }
    }
    fn command(&self) -> InternalUICommand {
        InternalUICommand::SwitchTab(self.clone())
    }
}

pub enum ActionButton {
    CloseBook,
    EditBook,
    OCROpen,
}

impl ButtonTrait for ActionButton {
    fn icon(&self) -> IconPaths {
        match self {
            // Check if can rotate
            ActionButton::CloseBook => druid_material_icons::normal::action::EXIT_TO_APP,
            ActionButton::EditBook => druid_material_icons::normal::editor::EDIT_NOTE,
            ActionButton::OCROpen => druid_material_icons::normal::editor::EDIT_NOTE,
        }
    }
    fn hint(&self) -> String {
        match self {
            ActionButton::CloseBook => "Close Book".to_string(),
            ActionButton::EditBook => "Edit Book".to_string(),
            ActionButton::OCROpen => "Find using OCR".to_string(),
        }
    }
    fn command(&self) -> InternalUICommand {
        match self {
            ActionButton::CloseBook => InternalUICommand::GoToMenu,
            ActionButton::EditBook => InternalUICommand::OpenEditDialog,
            ActionButton::OCROpen => InternalUICommand::OpenOCRDialog,
        }
    }
}

pub struct CustomButton<T: ButtonTrait> {
    kind: T,
    icon: WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
    font: FontFamily,
    open: bool,
    clickable: bool,
}

impl<T: ButtonTrait> CustomButton<T> {
    pub fn new(kind: T) -> Self {

        let hint = kind.hint();
        let icon_data = kind.icon();
        Self {
            kind,
            icon: WidgetPod::new(ControllerHost::new(
                Icon::new(icon_data),
                TooltipController::new(hint),
            ).boxed()),
            font: FontFamily::default(),
            open: false,
            clickable: true,
        }
    }
}

impl<T: ButtonTrait> Widget<EpubData> for CustomButton<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if self.clickable {
                    self.open = !self.open;
                    ctx.request_paint();
                }
                ctx.submit_command(INTERNAL_COMMAND.with(self.kind.command()));
            }
            // set cursors hand on hover
            Event::MouseMove(_) => {
                ctx.set_cursor(&druid::Cursor::Pointer);
            }
            Event::WindowConnected => {}
            _ => {}
        }

        self.icon.event(ctx, event, data, env);
        ctx.request_paint();
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                // load the new font for icons
                if self.font == FontFamily::default() {
                    self.font = ctx.text().font_family("druid-epub-icons").unwrap();
                }
            }
            _ => {}
        }
        self.icon.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, _: &mut UpdateCtx, _: &EpubData, _: &EpubData, _: &Env) {}

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        _: &BoxConstraints,
        data: &EpubData,
        env: &Env,
    ) -> Size {
        self.icon.layout(
            ctx,
            &BoxConstraints::tight(Size::new(ICON_SIZE, ICON_SIZE)),
            data,
            env,
        );
        self.icon.set_origin(ctx, data, env, Point::ORIGIN);
        Size::new(ICON_SIZE, ICON_SIZE)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        // with save, rotate context, paint icon and restore context
        ctx.with_save(|ctx| {
            // rotate of 180 degree

            //ctx.transform(druid::Affine::rotate((180. as f64).sin()));
            self.icon.paint(ctx, data, env);
        });
        //self.icon.paint(ctx, data, env);
        // rotate icon
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
        ctx.clip((size-Size::new(15., 0.)).to_rect());

        self.layout.draw(ctx, (5., 0.));
    }
}

pub struct Sidebar {
    side_buttons: Vec<WidgetPod<EpubData, Box<dyn Widget<EpubData>>>>,
    action_buttons: Vec<WidgetPod<EpubData, Box<dyn Widget<EpubData>>>>,
    panels: Vec<WidgetPod<EpubData, Box<dyn Widget<EpubData>>>>,

    opened_tab: Option<PanelButton>,
}

impl Sidebar {
    pub fn new() -> Sidebar {
        let mut side_buttons = Vec::new();
        let mut action_buttons = Vec::new();

        let mut panels = Vec::new();

        for kind in vec![
            PanelButton::Toc,
            PanelButton::Search,
            //PanelButton::Highlights,
            PanelButton::Settings,
        ] {
            if let PanelButton::Search = kind {
                panels.push(WidgetPod::new(
                    (Panel::new(&&kind.title(), kind.to_widget()))
                        .with_input_widget()
                        .boxed(),
                ))
            } else {
                panels.push(WidgetPod::new(
                    (Panel::new(&kind.title(), kind.to_widget())).boxed(),
                ))
            }

            let other_but = CustomButton::new(kind).boxed();
            side_buttons.push(WidgetPod::new(other_but));
        }

        for actions in vec![ActionButton::CloseBook, ActionButton::EditBook, ActionButton::OCROpen] {
            let other_but = CustomButton::new(actions).boxed();
            action_buttons.push(WidgetPod::new(other_but));
        }

        Sidebar {
            side_buttons,
            action_buttons,
            panels,
            opened_tab: None,
        }
    }

    pub fn get_active_panel(&mut self) -> &mut WidgetPod<EpubData, Box<dyn Widget<EpubData>>> {
        if !self.opened_tab.is_some() {
            panic!("Sidebar is not opened");
        }
        &mut self.panels[self.opened_tab.clone().unwrap() as usize]
    }
}

impl Widget<EpubData> for Sidebar {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut EpubData,
        env: &druid::Env,
    ) {
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
                        _ => {}
                    }
                }
            }
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

        for button in self.action_buttons.iter_mut() {
            button.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &EpubData,
        env: &druid::Env,
    ) {
        for button in self.side_buttons.iter_mut() {
            button.lifecycle(ctx, event, data, env);
        }
        for panel in self.panels.iter_mut() {
            panel.lifecycle(ctx, event, data, env);
        }

        for button in self.action_buttons.iter_mut() {
            button.lifecycle(ctx, event, data, env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _: &EpubData,
        data: &EpubData,
        env: &druid::Env,
    ) {
        for button in self.side_buttons.iter_mut() {
            button.update(ctx, data, env);
        }
        if self.opened_tab.is_some() {
            self.get_active_panel().update(ctx, data, env);
        }

        for button in self.action_buttons.iter_mut() {
            button.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &BoxConstraints,
        data: &EpubData,
        env: &druid::Env,
    ) -> druid::Size {
        const PADDING_BETWEEN_BUTTONS: f64 = 5.0;
        const ITEM_BAR_SIZE: f64 = 40.;
        const PANEL_PADDING: f64 = 0.;
        let max_size = bc.max();
        let closed_size = Size::new(ITEM_BAR_SIZE, max_size.height);
        let mut prev_height = Point::new(5., PADDING_BETWEEN_BUTTONS);

        for button in self.side_buttons.iter_mut() {
            button.layout(ctx, &BoxConstraints::tight(max_size), data, env);
            button.set_origin(ctx, data, env, prev_height);

            prev_height.y += button.layout_rect().height() + PADDING_BETWEEN_BUTTONS;
        }
        // draw action buttons starting from the bottom
        let mut prev_height = Point::new(5., max_size.height - ICON_SIZE);
        for button in self.action_buttons.iter_mut() {
            button.layout(ctx, &BoxConstraints::tight(max_size), data, env);
            button.set_origin(ctx, data, env, prev_height);

            prev_height.y -= button.layout_rect().height() - PADDING_BETWEEN_BUTTONS;
        }

        if self.opened_tab.is_some() {
            self.get_active_panel().layout(
                ctx,
                &BoxConstraints::tight(Size::new(
                    ITEM_BAR_SIZE * 5.,
                    max_size.height - PANEL_PADDING,
                )),
                data,
                env,
            );
            self.get_active_panel()
                .set_origin(ctx, data, env, Point::new(ITEM_BAR_SIZE, 0.));
            Size::new(ITEM_BAR_SIZE * 5. + ITEM_BAR_SIZE, max_size.height)
        } else {
            closed_size
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &EpubData, env: &druid::Env) {
        let rect = Size::new(40., ctx.size().height).to_rect();
        ctx.fill(rect, &style::get_color_unchecked(PRIMARY_DARK));

        for button in self.side_buttons.iter_mut() {
            button.paint(ctx, data, env);
        }

        for button in self.action_buttons.iter_mut() {
            button.paint(ctx, data, env);
        }

        // Draw panel and side line if some is opened
        if self.opened_tab.is_some() {
            self.get_active_panel().paint(ctx, data, env);

            let mut rect = Size::new(2., ICON_SIZE).to_rect();
            let num = self.opened_tab.clone().unwrap() as usize;
            rect.y0 = num as f64 * ICON_SIZE + 7.;
            rect.y1 = rect.y0 + ICON_SIZE + 7.;
            ctx.fill(rect, &Color::WHITE);
        }
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
        let input_widget = TextBox::new()
            .lens(EpubData::sidebar_data.then(SidebarData::search_input))
            .boxed();
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
                        let pos = data.search_with_ocr_input("photo_2022-11-12_17-39-49.jpg");
                        ctx.submit_command(GO_TO_POS.with(pos));
                        ctx.request_update();
                        ctx.request_layout();
                    }
                }
                _ => {}
            }
            self.input_widget
                .as_mut()
                .unwrap()
                .event(ctx, event, data, env);
        }

        self.widget.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        if self.input_widget.is_some() {
            self.input_widget
                .as_mut()
                .unwrap()
                .lifecycle(ctx, event, data, env);
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

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &EpubData,
        env: &Env,
    ) -> Size {
        const PANEL_COMPONENT_PADDING: f64 = 7.;
        let size = bc.max();
        let mut widget_size =
            Size::new(size.width - PANEL_COMPONENT_PADDING * 2., size.height - 30.);
        let mut input_widget_size = Size::new(size.width, 0.);
        if self.input_widget.is_some() {
            input_widget_size = self.input_widget.as_mut().unwrap().layout(
                ctx,
                &BoxConstraints::tight(Size::new(size.width - 50., 25.)),
                data,
                env,
            );
            self.input_widget.as_mut().unwrap().set_origin(
                ctx,
                data,
                env,
                Point::new(PANEL_COMPONENT_PADDING, 30.),
            );
            widget_size.height -= 25.;
        }

        self.header.rebuild_if_needed(ctx.text(), env);
        self.header.layout();
        self.widget
            .layout(ctx, &BoxConstraints::tight(widget_size), data, env);
        self.widget.set_origin(
            ctx,
            data,
            env,
            Point::new(PANEL_COMPONENT_PADDING, 30. + input_widget_size.height),
        );

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        let size = ctx.size();
        ctx.fill(size.to_rect(), &style::get_color_unchecked(style::PRIMARY_LIGHT)); //&COMPLEMENTARY_DARK.unwrap());
        self.header.draw(ctx, (5., 5.));
        if self.input_widget.is_some() {
            self.input_widget.as_mut().unwrap().paint(ctx, data, env);
        }
        self.widget.paint(ctx, data, env);
    }
}
