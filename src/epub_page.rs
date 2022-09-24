

use druid::piet::{Text, TextLayoutBuilder, TextLayout as a};
use druid::{Data, WidgetPod, WidgetExt};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, MouseButton, PaintCtx, Point, Rect, RenderContext, Size, TextLayout, UpdateCtx,
    Widget, text::Selection,
};

use crate::application_state::EpubData;
use crate::tool::Tool;


use crate::widgets::epub_page::navbar::NavigationBar;
use crate::widgets::epub_page::textcontainer::TextContainer;

const SELECTED_TOOL: druid::Key<u64> = druid::Key::new("org.linebender.example.important-label-color");



pub struct EpubPage {
    layout: druid::TextLayout<druid::text::RichText>,
    //mark_points: Vec<(Point, Point)>,
    //pen_points: Vec<(Point, Point)>,
    font_size : u32,
    until_pos: usize,
    selection : Selection,

    text_container : WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
    navigation_bar : WidgetPod<EpubData, Box<dyn Widget<EpubData>>>,
    
}

impl EpubPage {
    pub fn new() -> Self {

        let text_container = WidgetPod::new(TextContainer::new().boxed());
        let navigation_bar = WidgetPod::new(NavigationBar::new().boxed());
        EpubPage {
            layout: druid::TextLayout::new(),
            //mark_points: Vec::new(),
            //pen_points: Vec::new(),
            font_size: 14,
            until_pos: 0,
            selection : Selection::default(),
            text_container,
            navigation_bar
        }
    }
}

impl Widget<EpubData> for EpubPage {
    
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {

        let selected_tool : Tool = env.get(SELECTED_TOOL).into();
        //println!("event: {:?}", event);
        match event {
            Event::MouseDown(_mouse) => {


                let label_size = ctx.size();
                self.until_pos += self.layout.text_position_for_point(Point::new(label_size.width, label_size.height));
                ////println!("until pos:{}.\npage_text {}", self.until_pos, data.epub_metrics.chapter_length);
                //if data.epub_metrics.position_in_chapter >= 46000 && data.epub_metrics.position_in_chapter  <= 47000 {
                //    data.search_in_book("BEOWULF", 1);
                //}
                //else { 
                //    data.search_in_book("BEOWULF", 2);
                //}
//
                //return;
                if self.until_pos >= data.epub_metrics.chapter_length {
                    self.until_pos = 0;
                    
                    data.next_chapter();
                    //let asasd = crate::application_state::rebuild_rendered_text(&*data.html_text.to_string(), self.until_pos);    
                }
                else {
                    data.next_page(self.until_pos);
                }
            }
            
            Event::MouseMove(mouse) => {
                if selected_tool.should_be_written() && mouse.buttons.contains(MouseButton::Left) {

                    let text_position = self.layout.text_position_for_point(mouse.pos);

                    self.selection.anchor = text_position;
                    ctx.request_paint();

                }
            }
            
            /*
            Event::MouseDown(e) => {

                if selected_tool.should_be_written() && e.buttons.contains(MouseButton::Left) {
                    if(self.first_text == 0) {
                        self.first_text = self.layout.text_position_for_point(Point::new(e.pos.x, e.pos.y));
                    }
                    else {
                        self.second_text = self.layout.text_position_for_point(Point::new(e.pos.x, e.pos.y));

                        
                    }
                    println!("Text position for point: {}", self.layout.text_position_for_point(Point::new(e.pos.x, e.pos.y)));
                    match selected_tool {
                        Tool::Pen | Tool::Marker => self.mark_points.push((e.pos, e.pos)),
                        _ => {}
                    }
                }
            }

            Event::MouseUp(e) => {

                if selected_tool.should_be_written() {
                    // Controllo sul unwrap;
                    let p = self.mark_points.last().unwrap();
                    if p.0 == p.1 { // if equal and not write nothing
                        self.mark_points.pop();

                    } else {
                        let l = self.mark_points.len();
                        // set last point to the current point
                        self.mark_points[l - 1].1 = e.pos;
                        ctx.request_paint();
                    }
                }
                else if selected_tool == Tool::Eraser {
                    
                    self.mark_points = self.mark_points.iter().filter(|(start, end)| {
                        let r = Rect::from_points(*start, *end);
                        println!("Pos {} Contains : {}", e.pos, r.contains(e.pos));
                        !r.contains(e.pos)
                    })
                    .map(|(p, s)| (*p, *s))
                    .collect();
                ctx.request_paint();
                }
                // Account for the padding
                let pos = e.pos;// - Vec2::new(10, 0.0);
                if let Some(link) = self.layout.link_for_pos(pos) {
                    ctx.submit_command(link.command.clone());
                }

            }
            Event::MouseMove(e) => {
                if selected_tool.should_be_written() && e.buttons.contains(MouseButton::Left) {
                    // and Mark is selected
                    //println!("{:?}", event);

                    ctx.request_paint();
                    let l = self.mark_points.len();

                    // set last point to the current point
                    self.mark_points[l - 1].1 = e.pos;
                }

                // Account for the padding
                let pos = e.pos;// - Vec2::new(LABEL_X_PADDING, 0.0);
                if self.layout.link_for_pos(pos).is_some() {
                    ctx.set_cursor(&druid::Cursor::Pointer);
                } else {
                    ctx.clear_cursor();
                }
            }
            */
            _ => {
                //println!("Unhandled event: {:?}", event);
            }
        }
        // let pre_data = data.raw.to_owned();
        // child.event(ctx, event, data, env);
        // if !data.raw.same(&pre_data) {
        //     data.rendered = rebuild_rendered_text(&data.raw);
        // }
        self.text_container.event(ctx, event, data, env);
        self.navigation_bar.event(ctx, event, data, env);



   }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                //self.layout.set_text(data.visualized_page.clone());
                self.layout.set_text_size(self.font_size as f64);
                self.layout.set_text_color(Color::BLACK);
                self.layout.set_text_alignment(druid::TextAlignment::Start);

                //self.page_num = data.page_number;
                
            }
            _ => {} 
        }
        self.text_container.lifecycle(ctx, event, data, env);
        self.navigation_bar.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) {

        //if !old_data.same(data) {
        //    self.layout.set_text(data.visualized_page.clone());
        //    ctx.request_layout();
        //}
        //if self.layout.needs_rebuild_after_update(ctx) {
        //    ctx.request_layout();
        //}
        self.text_container.update(ctx, data, env);
        self.navigation_bar.update(ctx, data, env);

    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {

        let navigation_bar_height = 50.0;
        self.text_container.layout(ctx, &BoxConstraints::tight(Size::new(bc.max().width-50., bc.max().height-50.)), data, env);
        self.text_container.set_origin(ctx, data, env, Point::new(25.0, 25.0));

        self.navigation_bar.layout(ctx, &BoxConstraints::tight(Size::new(bc.max().width - 50., 50.)), data, env);
        self.navigation_bar.set_origin(ctx, data, env, Point::new(25.0, bc.max().height  - navigation_bar_height));

        return bc.max();
        /*                        X
         ______________      X    |
        |              |  X  |    |
        |  text like   |  |  |    |
        |  this        |  |  |    |
        |              |  |  |    |
        |  New chap-   |  |  |    |
        |  ter         |  |  |    |
        |              |  |  |    |
        |              |  |  |    |    
        |              |  |  |    |   bc.max().height
        |______________|  X  |    |     
           <------->         X    |
           page_size              X
        <-------------->
      25 max - 50 * 2  25
      <-------------------->
            bc.max().width  
      
      
      */


      let label_x_padding = 125.;
                                                                        // SUBSTITUTE WITH FIXED WINDOWS SIZE!!!
        //let size = bc.constrain(Size::new( bc.max().width,  bc.max().height));//bc.max().height));
        let size = bc.max();
        self.layout.set_wrap_width(bc.max().width - label_x_padding);
        self.layout.rebuild_if_needed(ctx.text(), env);

        //let text_metrics = self.layout.layout_metrics();
        //ctx.set_baseline_offset(text_metrics.size.height - text_metrics.first_baseline);
        //let size = bc.constrain(Size::new(
        //    text_metrics.size.width + 2. * label_x_padding,
        //    text_metrics.size.height, 
        //));
        size

    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        

        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);



        self.text_container.paint(ctx, data, env);
        self.navigation_bar.paint(ctx, data, env);
        
        return;








        let origin = Point::new(0., 0.0);

        
        let size = ctx.size();
        let rect = Rect::new(50., 0., size.width-50., size.height);
        
        ctx.fill(size.to_rect(), &Color::WHITE);

        //ctx.fill(rect, &Color::WHITE); 

        
        let text_offset = druid::Vec2::new(50.+10., 0.0);

        let sel_rects = self.layout.rects_for_range(self.selection.range());
        for region in sel_rects {
            let rounded = (region + text_offset).to_rounded_rect(1.0);
            ctx.fill(rounded, &Color::YELLOW);

            //let y = region.max_y().floor();
            //let line = druid::kurbo::Line::new((region.min_x(), y), (region.max_x(), y)) + text_offset;
            //ctx.stroke(line, &Color::YELLOW, self.font_size as f64);
            
            // TESTO SOTTOLINEATO
            //let t = &(*_data.plain_text)[self.selection.range()];
            
            //slice_mut_unchecked(self.selection.anchor, self.selection.active);
            //println!("Region: {}{:?}", &sel_rects.len(), &region);


                }
        /*
        if  !self.set2 {
            self.set2 = true;

            // DRAW IMAGE 
        //let mut ep = epub::doc::EpubDoc::new("/home/syscall/Downloads/1.epub".to_string()).unwrap();
        //let image_data = druid::ImageBuf::from_data(&ep.get_cover().unwrap()).unwrap();
        //let image = image_data.to_image(ctx.render_ctx);
        //ctx.draw_image(&image, image_data.size().to_rect(), druid::piet::InterpolationMode::Bilinear);

        
        for (start_point, end_point) in &self.mark_points {
            let rect = Rect::from_origin_size(
                (start_point.x, start_point.y),
                (end_point.x - start_point.x, 15.0),
            );

            let fill_color = Color::rgba8(0x00, 0x00, 0x00, 0x7F);
            ctx.fill(rect, &fill_color);
        }
    }
        */
        let label_size = ctx.size();
    
        //let asasd = crate::application_state::rebuild_rendered_text(&*data.html_text[pos_text..].to_string());
        //println!("*data.html_text[asdad..] {:}", &*data.html_text[pos_text..].to_string());

        //self.layout.text_position_for_point(point)

        self.layout.draw(ctx, origin);

        //ctx.clip(label_size.to_rect());


    }



}
