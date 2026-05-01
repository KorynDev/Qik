use gpui::*;
use gpui::prelude::*;
use ropey::Rope;
use std::path::{PathBuf};
use walkdir::WalkDir;

// --- Actions ---
actions!(qik, [
    Quit, ToggleSidebar, OpenCommandPalette, SearchFiles, SaveFile,
    ToggleBottomPanel, NextTab, PrevTab, CloseTab, FocusEditor
]);

// --- Global Theme ---
pub struct Theme {
    pub background: Hsla,
    pub sidebar_bg: Hsla,
    pub editor_bg: Hsla,
    pub status_bg: Hsla,
    pub panel_bg: Hsla,
    pub border: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_accent: Hsla,
    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub selection: Hsla,
    pub cursor: Hsla,
    pub line_number: Hsla,
    pub shadow: Hsla,
    pub git_added: Hsla,
    pub git_modified: Hsla,
    pub git_deleted: Hsla,
}

impl Theme {
    pub fn qik_dark() -> Self {
        Self {
            background: hsla(220.0 / 360.0, 0.13, 0.18, 1.0),
            sidebar_bg: hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
            editor_bg: hsla(220.0 / 360.0, 0.13, 0.18, 1.0),
            status_bg: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
            panel_bg: hsla(220.0 / 360.0, 0.13, 0.22, 1.0),
            border: hsla(220.0 / 360.0, 0.13, 0.25, 1.0),
            text: hsla(220.0 / 360.0, 0.10, 0.85, 1.0),
            text_muted: hsla(220.0 / 360.0, 0.10, 0.50, 1.0),
            text_accent: hsla(200.0 / 360.0, 0.80, 0.80, 1.0),
            accent: hsla(200.0 / 360.0, 0.80, 0.60, 1.0),
            accent_hover: hsla(200.0 / 360.0, 0.80, 0.70, 1.0),
            selection: hsla(200.0 / 360.0, 0.50, 0.30, 0.5),
            cursor: hsla(200.0 / 360.0, 0.80, 0.60, 1.0),
            line_number: hsla(220.0 / 360.0, 0.10, 0.35, 1.0),
            shadow: hsla(0.0, 0.0, 0.0, 0.3),
            git_added: hsla(140.0 / 360.0, 0.60, 0.50, 1.0),
            git_modified: hsla(45.0 / 360.0, 0.80, 0.50, 1.0),
            git_deleted: hsla(0.0 / 360.0, 0.70, 0.50, 1.0),
        }
    }
}

impl Global for Theme {}

// --- Command Palette ---
pub struct CommandPalette {
    _query: String,
    results: Vec<String>,
    selected_index: usize,
}

impl CommandPalette {
    pub fn new() -> Self {
        Self {
            _query: String::new(),
            results: vec![
                "File: New".to_string(), "File: Open".to_string(), "File: Save".to_string(),
                "View: Toggle Sidebar".to_string(), "View: Toggle Bottom Panel".to_string(),
                "Editor: Format Document".to_string(), "Help: Documentation".to_string(),
                "Settings: Open Settings".to_string(), "Project: Run Tests".to_string(),
            ],
            selected_index: 0,
        }
    }
}

impl EventEmitter<CommandPaletteEvent> for CommandPalette {}
pub enum CommandPaletteEvent { Executed, Dismissed }

impl Render for CommandPalette {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .absolute().top(px(48.0)).left_1_2().ml(px(-320.0)).w(px(640.0)).bg(theme.panel_bg)
            .rounded_xl().border_1().border_color(theme.border).shadow_xl().p_2()
            .child(div().px_4().py_3().border_b_1().border_color(theme.border)
                .child(div().flex().gap(px(8.0)).child(div().text_color(theme.accent).child(">")).child(div().child("Search commands..."))))
            .child(div().flex().flex_col().mt_1().children(self.results.iter().enumerate().map(|(i, res)| {
                let is_selected = i == self.selected_index;
                div().px_4().py_2().rounded_md().when(is_selected, |s| s.bg(theme.accent).text_color(theme.background)).child(res.clone())
            })))
    }
}

// --- Search View ---
pub struct SearchView {
    _search_query: String,
    _replace_query: String,
}

impl Render for SearchView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div().flex().flex_col().p_4().bg(theme.sidebar_bg).border_b_1().border_color(theme.border).gap(px(8.0))
            .child(div().flex().gap(px(8.0))
                .child(div().flex_grow().bg(theme.background).p_1().rounded_sm().child("Find"))
                .child(div().bg(theme.accent).px_2().rounded_sm().child("All")))
            .child(div().flex().gap(px(8.0))
                .child(div().flex_grow().bg(theme.background).p_1().rounded_sm().child("Replace"))
                .child(div().bg(theme.border).px_2().rounded_sm().child("Replace All")))
    }
}

// --- Editor Engine ---
pub struct Editor {
    content: Rope,
    cursor_char: usize,
}

impl Editor {
    pub fn new(content: &str) -> Self { Self { content: Rope::from_str(content), cursor_char: 0 } }
    fn cursor_coords(&self) -> (usize, usize) {
        let line = self.content.char_to_line(self.cursor_char);
        let col = self.cursor_char - self.content.line_to_char(line);
        (line, col)
    }
}

impl Render for Editor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let (cursor_line, cursor_col) = self.cursor_coords();
        div().flex().flex_grow().bg(theme.editor_bg).text_color(theme.text).font_family("JetBrains Mono").text_size(px(14.0))
            .child(div().flex().flex_col().w(px(56.0)).h_full().bg(theme.sidebar_bg).text_color(theme.line_number).py_4().items_end().pr_4()
                .children((0..self.content.len_lines()).map(|i| div().h(px(24.0)).child((i+1).to_string()).when(i == cursor_line, |s| s.text_color(theme.text)))))
            .child(div().flex().flex_grow().relative().p_4()
                .child(div().flex().flex_col().children(self.content.lines().enumerate().map(|(i, line)| {
                    let is_active = i == cursor_line;
                    div().flex().h(px(24.0)).relative().when(is_active, |s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                        .child(div().w(px(4.0)).h_full().absolute().left(px(-16.0)).when(i % 10 == 0, |s| s.bg(theme.git_modified)))
                        .child(line.to_string().replace("\n", ""))
                        .when(is_active, |s| s.child(div().absolute().left(px(cursor_col as f32 * 8.4)).w_px().h_full().bg(theme.cursor)))
                }))))
    }
}

// --- Sidebar ---
pub struct Sidebar {
    entries: Vec<PathBuf>,
}

impl Render for Sidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div().flex().flex_col().w(px(256.0)).bg(theme.sidebar_bg).border_r_1().border_color(theme.border)
            .child(div().flex().h(px(40.0)).px_4().items_center().child(div().text_xs().font_weight(FontWeight::BOLD).text_color(theme.text_muted).child("EXPLORER")))
            .child(div().flex().flex_col().px_2().children(self.entries.iter().map(|p| {
                let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                div().px_2().py_1().rounded_sm().hover(|s| s.bg(theme.background)).child(name)
            })))
    }
}

// --- Bottom Panel ---
pub struct BottomPanel {
    _active_tab: String,
}

impl Render for BottomPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div().flex().flex_col().h(px(192.0)).bg(theme.status_bg).border_t_1().border_color(theme.border)
            .child(div().flex().h(px(36.0)).px_4().gap(px(16.0)).border_b_1().border_color(theme.border)
                .child(div().text_xs().font_weight(FontWeight::BOLD).child("TERMINAL"))
                .child(div().text_xs().text_color(theme.text_muted).child("OUTPUT")))
            .child(div().p_4().text_sm().font_family("JetBrains Mono").child("$ cargo check\n  Finished dev [unoptimized + debuginfo] target(s) in 0.05s"))
    }
}

// --- Workspace ---
pub struct Workspace {
    sidebar_visible: bool,
    bottom_panel_visible: bool,
    sidebar: Entity<Sidebar>,
    editor: Entity<Editor>,
    bottom_panel: Entity<BottomPanel>,
    search_view: Entity<SearchView>,
    command_palette: Option<Entity<CommandPalette>>,
}

impl Workspace {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            sidebar_visible: true, bottom_panel_visible: false,
            sidebar: cx.new(|_cx| Sidebar { entries: WalkDir::new(".").max_depth(1).into_iter().filter_map(|e| e.ok()).map(|e| e.path().to_path_buf()).collect() }),
            editor: cx.new(|_cx| Editor::new(include_str!("main.rs"))),
            bottom_panel: cx.new(|_cx| BottomPanel { _active_tab: "TERMINAL".to_string() }),
            search_view: cx.new(|_cx| SearchView { _search_query: "".into(), _replace_query: "".into() }),
            command_palette: None,
        }
    }
    fn toggle_sidebar(&mut self, _: &ToggleSidebar, _window: &mut Window, cx: &mut Context<Self>) { self.sidebar_visible = !self.sidebar_visible; cx.notify(); }
    fn toggle_bottom_panel(&mut self, _: &ToggleBottomPanel, _window: &mut Window, cx: &mut Context<Self>) { self.bottom_panel_visible = !self.bottom_panel_visible; cx.notify(); }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div().size_full().bg(theme.background).on_action(cx.listener(Self::toggle_sidebar)).on_action(cx.listener(Self::toggle_bottom_panel)).flex()
            .child(div().flex().flex_col().w(px(48.0)).bg(theme.status_bg).border_r_1().border_color(theme.border).items_center().py_4().gap(px(16.0))
                .child(div().text_xl().text_color(theme.accent).child(""))
                .child(div().text_xl().text_color(theme.text_muted).child(""))
                .child(div().text_xl().text_color(theme.text_muted).child(""))
                .child(div().mt_auto().text_xl().text_color(theme.text_muted).child("")))
            .when(self.sidebar_visible, |s| s.child(self.sidebar.clone()))
            .child(div().flex().flex_col().flex_grow()
                .child(div().flex().h(px(40.0)).bg(theme.sidebar_bg).border_b_1().border_color(theme.border)
                    .child(div().flex().h_full().px_4().bg(theme.editor_bg).border_r_1().border_color(theme.border).gap(px(8.0))
                        .child(div().text_color(theme.accent).child("")).child(div().text_sm().child("main.rs"))))
                .child(self.editor.clone())
                .when(self.bottom_panel_visible, |s| s.child(self.bottom_panel.clone()))
                .child(div().flex().h(px(28.0)).bg(theme.status_bg).px_3().text_xs().text_color(theme.text_muted).justify_between()
                    .child(div().flex().gap(px(16.0)).child("qik-master").child("0 errors"))
                    .child(div().flex().gap(px(16.0)).child("Rust").child("Ln 1, Col 1"))))
            .when_some(self.command_palette.clone(), |s, palette| s.child(palette))
    }
}

fn main() {
    App::new().run(|cx: &mut App| {
        cx.set_global(Theme::qik_dark());
        
        cx.open_window(WindowOptions::default(), |window, cx| {
            cx.new(|cx| Workspace::new(window, cx))
        }).expect("failed to open window");
    });
}
