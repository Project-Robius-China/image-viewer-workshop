use makepad_widgets::*;
use crate::app::State;

live_design! {
    use link::widgets::*;
    use crate::components::image_item::ImageItem;

    pub ImageRow = {{ImageRow}} {
        <PortalList> {
            height: 256,
            flow: Right,

            ImageItem = <ImageItem> {}
        }
	}
}

#[derive(Live, LiveHook, Widget)]
pub struct ImageRow {
    #[deref]
    view: View,
}

impl Widget for ImageRow {
    fn draw_walk(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        walk: Walk,
    ) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get_mut::<State>().unwrap();
                let row_idx = *scope.props.get::<usize>().unwrap();

                list.set_item_range(cx, 0, state.num_images_for_row(row_idx));
                while let Some(item_idx) = list.next_visible_item(cx) {
                    if item_idx >= state.num_images_for_row(row_idx) {
                        continue;
                    }

                    let item_widget_id = live_id!(ImageItem);
                    let item = list.item(cx, item_idx, item_widget_id);

                    let absolute_image_idx = state.first_image_idx_for_row(row_idx) + item_idx;

                    item.apply_over(cx, live!{ image_index: (absolute_image_idx) });

                    let filtered_image_idx = state.filtered_image_idxs[absolute_image_idx];

                    let image_path = &state.image_paths[filtered_image_idx];
                    let image_view = item.image(id!(image));
                    image_view.load_image_file_by_path_async(cx, &image_path).unwrap();

                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}
