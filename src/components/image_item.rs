use makepad_widgets::*;


live_design! {
    use link::widgets::*;

    PLACEHOLDER = dep("crate://self/resources/Rust.jpg");

    pub ImageItem = {{ImageItem}} {
        width: 256,
        height: 256,
        image_index: 0,

        image = <Image> {
            width: Fill,
            height: Fill,
            fit: Biggest,
            source: (PLACEHOLDER)
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ImageClickedAction{
    None,
    Clicked(usize),
}

#[derive(Live, LiveHook, Widget)]
pub struct ImageItem {
    #[deref]
    view: View,

    #[live] // image_index will be set by the parent ImageRow
    image_index: usize,
}

impl Widget for ImageItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(_) => {
                cx.action(ImageClickedAction::Clicked(self.image_index));
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
