use makepad_widgets::*;
use std::path::{Path, PathBuf};
use crate::components::ImageClickedAction;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use crate::components::image_grid::ImageGrid;

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
    pub(crate) image_paths: Vec<PathBuf>,
    pub(crate) filtered_image_idxs: Vec<usize>,
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
    pub(crate) fn num_images(&self) -> usize {
        self.filtered_image_idxs.len()
    }

    pub(crate) fn num_rows(&self) -> usize {
        self.num_images().div_ceil(self.max_images_per_row)
    }

    pub(crate) fn first_image_idx_for_row(&self, row_idx: usize) -> usize {
        row_idx * self.max_images_per_row
    }

    pub(crate) fn num_images_for_row(&self, row_idx: usize) -> usize {
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


impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::components::live_design(cx); // Step 20
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

        for action in actions {
            if let ImageClickedAction::Clicked(image_idx) = action.cast(){
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
