use druid::{widget::Controller, Event, Env, Widget, Selector};
use druid_widget_nursery::navigator::{Navigator, ViewController};

use crate::application_state::AppState;

use super::uiview::UiView;

const POP_VIEW: Selector<()> = Selector::new("navigator.pop-view");

// this controller will handle commands like POP_VIEW whenever a child widget does not
// have access to AppState
pub struct NavigatorController;

/*
impl Controller<AppState, Navigator<AppState, UiView>> for NavigatorController {
    fn event(
        &mut self,
        child: &mut Navigator<AppState, UiView>,
        ctx: &mut druid::EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(selector) if selector.is(POP_VIEW) => {
                data.pop_view();
            }
            _ => (),
        }
        child.event(ctx, event, data, env)
    }
}
*/