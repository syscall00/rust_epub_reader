use druid::{widget::Either, Widget, WidgetExt, WidgetPod};

use crate::appstate::{EpubData, OcrData};

pub struct OcrWidget {
    image_selector: WidgetPod<OcrData, Box<dyn Widget<OcrData>>>,
    image_viewer: WidgetPod<OcrData, Box<dyn Widget<OcrData>>>,
    recognized_page_navigator: WidgetPod<OcrData, Box<dyn Widget<OcrData>>>,
}

impl OcrWidget {
    pub fn new() -> Self {
        OcrWidget {
            image_selector: WidgetPod::new(
                druid::widget::Button::new("Select Image")
                    .on_click(|ctx, data, _env| {
                        println!("Select Image");
                        ctx.request_update();
                    })
                    .boxed(),
            ),
            image_viewer: WidgetPod::new(
                Either::new(
                    |data: &OcrData, _env| data.image_to_position.is_some(),
                    druid::widget::Label::new("Image Viewer").boxed(),
                    druid::widget::Label::new("No Image Selected").boxed(),
                )
                .boxed(),
            ),
            recognized_page_navigator: WidgetPod::new(
                druid::widget::Button::new("Select Image")
                    .on_click(|ctx, data: &mut OcrData, _env| {
                        println!("Select Image");
                        ctx.request_update();
                    })
                    .boxed(),
            ),
        }
    }
}


//impl Widget<OcrData> for OcrWidget {
//    fn event(
//}