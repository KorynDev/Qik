use gpui::*;
use walkdir::WalkDir;
use crate::theme::Theme;
use crate::actions::{ToggleSidebar, ToggleBottomPanel};
use crate::ui::{Sidebar, Editor, BottomPanel, SearchView, CommandPalette};

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
                div()
                    .flex()
                    .flex_col()
                    .w(px(48.0))
                    .bg(theme.status_bg)
                    .border_r_1()
                    .border_color(theme.border)
                    .items_center()
                    .py_4()
                    .gap(px(16.0))
                    .child(div().text_xl().text_color(theme.accent).child(""))
                    .child(div().text_xl().text_color(theme.text_muted).child(""))
                    .child(div().text_xl().text_color(theme.text_muted).child(""))
                    .child(div().mt_auto().text_xl().text_color(theme.text_muted).child(""))
            )
            .when(self.sidebar_visible, |s| s.child(self.sidebar.clone()))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .child(
                        div()
                            .flex()
                            .h(px(40.0))
                            .bg(theme.sidebar_bg)
                            .border_b_1()
                            .border_color(theme.border)
                            .child(
                                div()
                                    .flex()
                                    .h_full()
                                    .px_4()
                                    .bg(theme.editor_bg)
                                    .border_r_1()
                                    .border_color(theme.border)
                                    .gap(px(8.0))
                                    .child(div().text_color(theme.accent).child(""))
                                    .child(div().text_sm().child("main.rs"))
                            )
                    )
                    .child(self.editor.clone())
                    .when(self.bottom_panel_visible, |s| s.child(self.bottom_panel.clone()))
                    .child(
                        div()
                            .flex()
                            .h(px(28.0))
                            .bg(theme.status_bg)
                            .px_3()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .gap(px(16.0))
                                    .child("qik-master")
                                    .child("0 errors")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(16.0))
                                    .child("Rust")
                                    .child("Ln 1, Col 1")
                            )
                    )
            )
            .when_some(self.command_palette.clone(), |s, palette| s.child(palette))
    }
}
