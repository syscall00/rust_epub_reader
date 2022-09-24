
use druid::piet::{Text, TextLayoutBuilder};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, RenderContext, Size, UpdateCtx,
    Widget,
};

use crate::application_state::EpubData;


pub struct TextContainer {
    // text: TextLayout,
    // selection: Selection,
    // tool: Tool,
    
}

impl TextContainer {
    pub fn new() -> Self {
        Self {
            // text: TextLayout::new(""),
            // selection: Selection::new(),
            // tool: Tool::Select,
        }
    }
}

impl Widget<EpubData> for TextContainer {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EpubData, env: &Env) {
        match event {
            Event::MouseDown(_mouse) => {


                //let label_size = ctx.size();
                //self.until_pos += self.layout.text_position_for_point(Point::new(label_size.width, label_size.height));
                //////println!("until pos:{}.\npage_text {}", self.until_pos, data.epub_metrics.chapter_length);
                ////if data.epub_metrics.position_in_chapter >= 46000 && data.epub_metrics.position_in_chapter  <= 47000 {
                ////    data.search_in_book("BEOWULF", 1);
                ////}
                ////else { 
                ////    data.search_in_book("BEOWULF", 2);
                ////}
////
                ////return;
                //if self.until_pos >= data.epub_metrics.chapter_length {
                //    self.until_pos = 0;
                //    
                //    data.next_chapter();
                //    //let asasd = crate::application_state::rebuild_rendered_text(&*data.html_text.to_string(), self.until_pos);    
                //}
                //else {
                    //data.next_chapter();
                //}
            }
            _ => { } 
        }

        
        // self.text.event(ctx, event, data, env);
        // self.selection.event(ctx, event, data, env);
        // self.tool.event(ctx, event, data, env);
        // if let Event::MouseDown(mouse_event) = event {
        //     if mouse_event.button == MouseButton::Left {
        //         let point = mouse_event.pos;
        //         let offset = self.text.hit_test_text_position(point);
        //         self.selection.set_primary(offset);
        //         ctx.request_paint();
        //     }
        // }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EpubData, env: &Env) {
        // self.text.lifecycle(ctx, event, data, env);
        // self.selection.lifecycle(ctx, event, data, env);
        // self.tool.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EpubData, data: &EpubData, env: &Env) {
        // self.text.update(ctx, old_data, data, env);
        // self.selection.update(ctx, old_data, data, env);
        // self.tool.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EpubData, env: &Env) -> Size {
        // let size = self.text.layout(bc, data, env);
        // self.text.set_origin(ctx, data, env, Point::ORIGIN);
        // size
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EpubData, env: &Env) {
        // self.text.paint(ctx, data, env);
        // self.selection.paint(ctx, data, env);
        // self.tool.paint(ctx, data, env);
        let text = ctx.text();
        let layout = text
        .new_text_layout(data.visualized_page.clone())
        .text_color(Color::BLACK)
        .max_width(ctx.size().width)        
        .build()
        .unwrap();
        
        ctx.draw_text(&layout, Point::ORIGIN);
        let size = ctx.size();
        ctx.clip(size.to_rect());
    }
}

