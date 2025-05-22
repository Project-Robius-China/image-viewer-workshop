use makepad_widgets::*;
use std::path::{Path, PathBuf};

live_design! {
    use link::widgets::*;

    PLACEHOLDER = dep("crate://self/resources/Rust.jpg");
    LEFT_ARROW = dep("crate://self/resources/left_arrow.svg");
    RIGHT_ARROW = dep("crate://self/resources/right_arrow.svg");
    LOOKING_GLASS = dep("crate://self/resources/looking_glass.svg");

    // Step 18: 增加模态弹窗
    AlertDialog = <Modal> {
        width: Fill
        height: Fill

        bg_view: <View> {
            width: Fill
            height: Fill
            show_bg: true
            draw_bg: {
                // Alpha 值的范围是 0.0 (完全透明) 到 1.0 (完全不透明)。
                // 十六进制的 cc 转换为十进制是 204。
                // Alpha 的计算方式是 十进制值 / 255。
                // 所以 204 / 255 ≈ 0.8。
                // 如果你想让它更透明，你需要一个更小的 alpha 值。
                // 例如，如果你想要大约 50% 的透明度：
                // 255 * 0.5 = 127.5。最接近的十六进制值是 80 (十进制 128)。
                // 所以颜色会是 #00000080。
                // 如果你想要大约 30% 的透明度：
                // 255 * 0.3 ≈ 76.5。最接近的十六进制值是 4D (十进制 77) 或 4C (十进制 76)。
                // 所以颜色会是 #0000004D 或 #0000004C。
                fn pixel(self) -> vec4 {
                    return #00000080;
                }
            }
        }

        content: <View> {
            width: Fit
            height: Fit
            align: {x: 0.5, y: 0.5}  // 直接在 content 层居中

            draw_bg: {
                color: #333
            }

            width: 300
            height: 150
            padding: 20
            flow: Down

            dialog = <RoundedView> {
                width: 300
                height: 150
                align: {x: 0.5, y: 0.5}
                draw_bg: {
                    color: #333
                    border_color: #555
                    border_size: 1.0
                    border_radius: 4.0
                }
                padding: 20

                message = <Label> {
                    width: Fill
                    height: Fit
                    align: {x: 0.5}
                    margin: {bottom: 20}
                    draw_text: {
                        text_style: { font_size: 12.0 }
                        color: #fff
                    }
                    text: "默认消息"
                }
            }
        }
    }

    ImageItem = {{ImageItem}} {
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


    ImageRow = {{ImageRow}} {
        <PortalList> {
            height: 256,
            flow: Right,

            ImageItem = <ImageItem> {}
        }
	}

    SearchBox = <View> {
        width: 150,
        height: Fit,
        align: { y: 0.5 }
        margin: { left: 75 }

        <Icon> {
            icon_walk: { width: 12.0 }
            draw_icon: {
                color: #8,
                svg_file: (LOOKING_GLASS)
            }
        }

        query = <TextInput> {
            empty_text: "Search",
            draw_text: {
                text_style: { font_size: 10 },
                color: #8
            }
        }
    }

    MenuBar = <View> {
        width: Fill,
        height: Fit,

        <SearchBox> {}
        <Filler> {}
        slideshow_button = <Button> {
            text: "Slideshow"
        }
    }

    ImageGrid = {{ImageGrid}} {
        <PortalList> {
            flow: Down,
            ImageRow = <ImageRow> {}
        }
    }

    ImageBrowser = <View> {
        flow: Down,

        <MenuBar> {}
        <ImageGrid> {}
    }

    SlideshowNavigateButton = <Button> {
        width: 50,
        height: Fill,
        draw_bg: {
            color: #fff0,
            color_down: #fff2,
        }
        icon_walk: { width: 9 },
        text: "",
        grab_key_focus: false,
    }

    SlideshowOverlay = <View> {
        height: Fill,
        width: Fill,
        cursor: Arrow,
        capture_overload: true,

        navigate_left = <SlideshowNavigateButton> {
            draw_icon: { svg_file: (LEFT_ARROW) }
        }
        <Filler> {}
        navigate_right = <SlideshowNavigateButton> {
            draw_icon: { svg_file: (RIGHT_ARROW) }
        }
    }

    Slideshow = <View> {
        flow: Overlay,

        image = <Image> {
            width: Fill,
            height: Fill,
            fit: Biggest,
            source: (PLACEHOLDER)
        }
        overlay = <SlideshowOverlay> {}
    }

    App = {{App}} {
        placeholder: (PLACEHOLDER),

        ui: <Root> {
            <Window> {
                body = <View> {
                    flow: Overlay,

                    page_flip = <PageFlip> {
                        active_page: image_browser,

                        image_browser = <ImageBrowser> {}
                        slideshow = <Slideshow> {}
                    }

                    alert_dialog = <AlertDialog> {}
                }
            }
        }
    }
}


#[derive(Debug)]
pub struct State {
    image_paths: Vec<PathBuf>,
    filtered_image_idxs: Vec<usize>,
    max_images_per_row: usize,
    current_image_idx: Option<usize>,
    show_alert: bool,
    alert_message: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            image_paths: Vec::new(),
            filtered_image_idxs: Vec::new(),
            max_images_per_row: 4,
            current_image_idx: None,
            show_alert: false,
            alert_message: String::new(),
        }
    }
}

impl State {
    fn num_images(&self) -> usize {
        self.filtered_image_idxs.len()
    }

    fn num_rows(&self) -> usize {
        self.num_images().div_ceil(self.max_images_per_row)
    }

    fn first_image_idx_for_row(&self, row_idx: usize) -> usize {
        row_idx * self.max_images_per_row
    }

    fn num_images_for_row(&self, row_idx: usize) -> usize {
        let first_image_idx = self.first_image_idx_for_row(row_idx);
        let num_remaining_images = self.num_images() - first_image_idx;
        self.max_images_per_row.min(num_remaining_images)
    }
}


#[derive(Live)]
pub struct App {
    #[live]
    placeholder: LiveDependency,
    #[live]
    ui: WidgetRef,
    #[rust]
    state: State,
}

#[derive(Clone, Debug, Eq, Hash, Copy, PartialEq)]
pub enum ImageClickedAction{
    None,
    Clicked(usize),
}

impl Default for ImageClickedAction {
    fn default() -> Self {
        ImageClickedAction::None
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

                    // Set the image_index for the ImageItem instance
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
        // let uid = self.widget_uid();
        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(_pd) => {
                Cx::post_action(ImageClickedAction::Clicked(self.image_index));
                // 两种方式
                // cx.widget_action(
                //     uid,
                //     &scope.path, // 使用从父级传来的 scope 的 path
                //     ImageClickedAction::Clicked(self.image_index)
                // );
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // The actual image loading and display is handled by ImageRow calling:
        //   let image_view = item.image(id!(image));
        //   image_view.load_image_file_by_path_async(cx, &image_path).unwrap();
        // So, ImageItem just needs to ensure its view is drawn.
        // The `image_index` property is primarily for event handling.
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ImageGrid {
    #[deref]
    view: View,
}

impl Widget for ImageGrid {
    fn draw_walk(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        walk: Walk,
    ) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get_mut::<State>().unwrap();

                list.set_item_range(cx, 0, state.num_rows());
                while let Some(row_idx) = list.next_visible_item(cx) {
                    if row_idx >= state.num_rows() {
                        continue;
                    }

                    let row = list.item(cx, row_idx, live_id!(ImageRow));
                    let mut scope = Scope::with_data_props(state, &row_idx);
                    row.draw_all(cx, &mut scope);
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}



impl App {
    fn set_current_image(&mut self, cx: &mut Cx, image_idx: Option<usize>) {
        self.state.current_image_idx = image_idx;

        let image = self.ui.image(id!(slideshow.image));
        if let Some(image_idx) = self.state.current_image_idx {
            let filtered_image_idx = self.state.filtered_image_idxs[image_idx];
            let image_path = &self.state.image_paths[filtered_image_idx];
            image
                .load_image_file_by_path_async(cx, &image_path)
                .unwrap();
        } else {
            image
                .load_image_dep_by_path(cx, self.placeholder.as_str())
                .unwrap();
        }
        self.ui.redraw(cx);
    }

    fn load_image_paths(&mut self, cx: &mut Cx, path: &Path) {
        self.state.image_paths.clear();
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            self.state.image_paths.push(path);
        }

        let query = self.ui.text_input(id!(query)).text();
        self.filter_image_paths(cx, &query);
    }

    // step 18: 向左导航，如果已经是第一张图片则显示提示弹窗
    pub fn navigate_left(&mut self, cx: &mut Cx) {
        if let Some(image_idx) = self.state.current_image_idx {
            if image_idx > 0 {
                self.set_current_image(cx, Some(image_idx - 1));
            } else {
                self.show_alert(cx, "已经是第一张图片了");
            }
        }
    }

    // step 18: 向右导航，如果已经是最后一张图片则显示提示弹窗
    fn navigate_right(&mut self, cx: &mut Cx) {
        if let Some(image_idx) = self.state.current_image_idx {
            if image_idx + 1 < self.state.num_images() {
                // 还有下一张图片，正常导航
                self.set_current_image(cx, Some(image_idx + 1));
            } else {
                // 已经是最后一张图片，显示边界提示弹窗
                self.show_alert(cx, "已经是最后一张图片了");
            }
        }
    }

    // step 18: 显示边界提示弹窗
    fn show_alert(&mut self, cx: &mut Cx, message: &str) {
        // 更新状态
        self.state.show_alert = true;
        self.state.alert_message = message.to_string();

        // 获取Modal引用
        let modal_ref = self.ui.modal(id!(alert_dialog));

        // 设置消息文本
        let message_lable = modal_ref.label(id!(message));
        message_lable.set_text(cx, message);

        // 显示弹窗
        modal_ref.open(cx);
    }

    // step 18: 关闭边界提示弹窗
    fn close_alert(&mut self, cx: &mut Cx) {
        // 更新状态
        self.state.show_alert = false;
        // 关闭Modal弹窗
        self.ui.modal(id!(alert_dialog)).close(cx);
    }

    pub fn filter_image_paths(&mut self, cx: &mut Cx, query: &str) {
        self.state.filtered_image_idxs.clear();
        for (image_idx, image_path) in self.state.image_paths.iter().enumerate()
        {
            if image_path.to_str().unwrap().contains(&query) {
                self.state.filtered_image_idxs.push(image_idx);
            }
        }
        if self.state.filtered_image_idxs.is_empty() {
            self.set_current_image(cx, None);
        } else {
            self.set_current_image(cx, Some(0));
        }
    }
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        let path = "resources/images";
        self.load_image_paths(cx, path.as_ref());
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui
            .handle_event(cx, event, &mut Scope::with_data(&mut self.state));
    }
}


impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {

        for action_in_event in actions {
            if let ImageClickedAction::Clicked(image_idx) = action_in_event.cast(){
                self.set_current_image(cx, Some(image_idx));
                self.ui.page_flip(id!(page_flip)).set_active_page(cx, live_id!(slideshow));
                self.ui.view(id!(slideshow.overlay)).set_key_focus(cx);
            } else {
                // log!("App::handle_actions - Action was not a WidgetAction: {:?}", action_in_event);
            }
        }

        // 两种方式
        // for action_in_event in actions {
        //     // log!("App::handle_actions - processing action: {:?}", action_in_event); // 可以保留这个日志用于调试
        //     if let Some(widget_action) = action_in_event.as_widget_action() {
        //         match widget_action.cast::<ImageClickedAction>() {
        //             ImageClickedAction::Clicked(image_idx) => {
        //                 log!("App::handle_actions - Matched ImageClickedAction::Clicked({}) from UID {}", image_idx, widget_action.widget_uid.0);
        //                 self.set_current_image(cx, Some(image_idx));
        //                 self.ui.page_flip(id!(page_flip)).set_active_page(cx, live_id!(slideshow));
        //                 self.ui.view(id!(slideshow.overlay)).set_key_focus(cx);
        //             }
        //             ImageClickedAction::None => {
        //                 log!("App::handle_actions - WidgetAction (UID {}) was not ImageClickedAction", widget_action.widget_uid.0);
        //             }
        //         }
        //     } else {
        //         // log!("App::handle_actions - Action was not a WidgetAction: {:?}", action_in_event);
        //     }
        // }

        if self.ui.button(id!(slideshow_button)).clicked(&actions) {
            self.ui
                .page_flip(id!(page_flip))
                .set_active_page(cx, live_id!(slideshow));
            self.ui.view(id!(slideshow.overlay)).set_key_focus(cx);
        }

        if self.ui.button(id!(navigate_left)).clicked(&actions) {
            self.navigate_left(cx);
        }
        if self.ui.button(id!(navigate_right)).clicked(&actions) {
            self.navigate_right(cx);
        }

        if let Some(event) =
            self.ui.view(id!(slideshow.overlay)).key_down(&actions)
        {
            match event.key_code {
                KeyCode::Escape => self
                    .ui
                    .page_flip(id!(page_flip))
                    .set_active_page(cx, live_id!(image_browser)),
                KeyCode::ArrowLeft => self.navigate_left(cx),
                KeyCode::ArrowRight => self.navigate_right(cx),
                _ => {}
            }
        }

        if let Some(query) = self.ui.text_input(id!(query)).changed(&actions) {
            self.filter_image_paths(cx, &query);
        }

        // step 18: 处理弹窗背景点击或Esc按键事件
        if self.ui.modal(id!(alert_dialog)).dismissed(actions) {
            self.close_alert(cx);  // 点击背景或按Esc键关闭弹窗
        }
    }
}

app_main!(App);
