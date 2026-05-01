use gpui::*;
use gpui::prelude::*;
use walkdir::WalkDir;
use crate::theme::Theme;
use crate::actions::{ToggleSidebar, ToggleBottomPanel};
use crate::ui::{Sidebar, Editor, BottomPanel, SearchView, CommandPalette};
use crate::ui::icons;

pub struct Workspace {
    pub sidebar_visible: bool,
    pub bottom_panel_visible: bool,
    pub sidebar: Entity<Sidebar>,
    pub editor: Entity<Editor>,
    pub bottom_panel: Entity<BottomPanel>,
    pub search_view: Entity<SearchView>,
    pub command_palette: Option<Entity<CommandPalette>>,
}

impl Workspace {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            sidebar_visible: true,
            bottom_panel_visible: false,
            sidebar: cx.new(|_| Sidebar {
                entries: WalkDir::new(".")
                    .max_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .map(|e| e.path().to_path_buf())
                    .collect(),
            }),
            editor: cx.new(|_| Editor::new(include_str!("../main.rs"))),
            bottom_panel: cx.new(|_| BottomPanel {
                active_tab: "TERMINAL".to_string(),
            }),
            search_view: cx.new(|_| SearchView {
                search_query: "".into(),
                replace_query: "".into(),
            }),
            command_palette: None,
        }
    }

    pub fn toggle_sidebar(&mut self, _: &ToggleSidebar, _window: &mut Window, cx: &mut Context<Self>) {
        self.sidebar_visible = !self.sidebar_visible;
        cx.notify();
    }

    pub fn toggle_bottom_panel(&mut self, _: &ToggleBottomPanel, _window: &mut Window, cx: &mut Context<Self>) {
        self.bottom_panel_visible = !self.bottom_panel_visible;
        cx.notify();
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        
        div()
            .size_full()
            .bg(theme.background)
            .on_action(cx.listener(Self::toggle_sidebar))
            .on_action(cx.listener(Self::toggle_bottom_panel))
            .flex()
            .child(
                // Activity Bar (Leftmost)
                div()
                    .flex()
                    .flex_col()
                    .w(px(52.0))
                    .bg(theme.activity_bar_bg)
                    .border_r_1()
                    .border_color(theme.border_variant)
                    .items_center()
                    .py_4()
                    .gap(px(24.0))
                    .child(icons::project_icon(theme.accent))
                    .child(icons::search_icon(theme.text_muted))
                    .child(div().w(px(16.0)).h(px(16.0)).border_2().border_color(theme.text_muted).rounded_sm()) // Git dummy
                    .child(div().mt_auto().child(icons::settings_icon(theme.text_muted)))
            )
            .when(self.sidebar_visible, |s| s.child(self.sidebar.clone()))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .child(
                        // Tab Bar
                        div()
                            .flex()
                            .h(px(34.0))
                            .bg(theme.tab_inactive_bg)
                            .border_b_1()
                            .border_color(theme.border_variant)
                            .child(
                                // Active Tab
                                div()
                                    .flex()
                                    .h_full()
                                    .px_4()
                                    .bg(theme.tab_active_bg)
                                    .border_r_1()
                                    .border_color(theme.border_variant)
                                    .relative()
                                    .items_center()
                                    .gap(px(10.0))
                                    .child(
                                        // Top accent line
                                        div()
                                            .absolute()
                                            .top_0()
                                            .left_0()
                                            .w_full()
                                            .h(px(2.0))
                                            .bg(theme.tab_active_border)
                                    )
                                    .child(icons::file_icon(theme.git_modified))
                                    .child(div().text_size(px(13.0)).text_color(theme.text).child("main.rs"))
                                    .child(icons::close_icon(theme.text_muted))
                            )
                            .child(
                                // Inactive Tab
                                div()
                                    .flex()
                                    .h_full()
                                    .px_4()
                                    .items_center()
                                    .gap(px(10.0))
                                    .border_r_1()
                                    .border_color(theme.border_variant)
                                    .child(icons::file_icon(theme.text_muted))
                                    .child(div().text_size(px(13.0)).text_color(theme.text_muted).child("theme.rs"))
                            )
                    )
                    .child(self.editor.clone())
                    .when(self.bottom_panel_visible, |s| s.child(self.bottom_panel.clone()))
                    // Status Bar
                    .child(
                        div()
                            .flex()
                            .h(px(26.0))
                            .bg(theme.status_bg)
                            .border_t_1()
                            .border_color(theme.border_variant)
                            .px_3()
                            .text_size(px(11.0))
                            .text_color(theme.text_muted)
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .gap(px(16.0))
                                    .items_center()
                                    .child(div().flex().gap(px(6.0)).items_center().child(div().w(px(10.0)).h(px(1.0)).bg(theme.text_muted)).child("main"))
                                    .child(div().flex().gap(px(6.0)).items_center().child(div().size(px(6.0)).bg(theme.git_deleted).rounded_full()).child("0").child(div().size(px(6.0)).bg(theme.git_modified).rounded_full()).child("2"))
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(16.0))
                                    .child("Rust")
                                    .child("UTF-8")
                                    .child("Ln 1, Col 1")
                            )
                    )
            )
            .when_some(self.command_palette.clone(), |s, palette| s.child(palette))
    }
}
