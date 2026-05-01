use gpui::*;

pub struct Theme {
    pub background: Hsla,
    pub sidebar_bg: Hsla,
    pub editor_bg: Hsla,
    pub status_bg: Hsla,
    pub panel_bg: Hsla,
    pub toolbar_bg: Hsla,
    pub activity_bar_bg: Hsla,
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_accent: Hsla,
    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub selection: Hsla,
    pub cursor: Hsla,
    pub line_number: Hsla,
    pub shadow: Hsla,
    pub tab_active_bg: Hsla,
    pub tab_inactive_bg: Hsla,
    pub tab_active_border: Hsla,
    pub git_added: Hsla,
    pub git_modified: Hsla,
    pub git_deleted: Hsla,
}

impl Theme {
    pub fn zed_pro_dark() -> Self {
        Self {
            background: hsla(220.0 / 360.0, 0.12, 0.10, 1.0),      // Deep navy-gray
            sidebar_bg: hsla(220.0 / 360.0, 0.12, 0.08, 1.0),      // Slightly darker
            editor_bg: hsla(220.0 / 360.0, 0.12, 0.10, 1.0),       
            status_bg: hsla(220.0 / 360.0, 0.12, 0.06, 1.0),       
            panel_bg: hsla(220.0 / 360.0, 0.12, 0.14, 1.0),        
            toolbar_bg: hsla(220.0 / 360.0, 0.12, 0.12, 1.0),      
            activity_bar_bg: hsla(220.0 / 360.0, 0.12, 0.05, 1.0), 
            border: hsla(220.0 / 360.0, 0.10, 0.20, 1.0),          
            border_variant: hsla(220.0 / 360.0, 0.10, 0.15, 1.0),  
            border_focused: hsla(200.0 / 360.0, 0.60, 0.40, 1.0),  
            text: hsla(220.0 / 360.0, 0.10, 0.90, 1.0),            
            text_muted: hsla(220.0 / 360.0, 0.10, 0.50, 1.0),      
            text_accent: hsla(200.0 / 360.0, 0.70, 0.70, 1.0),     
            accent: hsla(200.0 / 360.0, 0.60, 0.50, 1.0),          
            accent_hover: hsla(200.0 / 360.0, 0.60, 0.60, 1.0),    
            selection: hsla(200.0 / 360.0, 0.40, 0.30, 0.3),       
            cursor: hsla(200.0 / 360.0, 0.60, 0.50, 1.0),          
            line_number: hsla(220.0 / 360.0, 0.10, 0.30, 1.0),     
            shadow: hsla(0.0, 0.0, 0.0, 0.6),                      
            tab_active_bg: hsla(220.0 / 360.0, 0.12, 0.10, 1.0),   
            tab_inactive_bg: hsla(220.0 / 360.0, 0.12, 0.07, 1.0), 
            tab_active_border: hsla(200.0 / 360.0, 0.60, 0.50, 1.0),
            git_added: hsla(150.0 / 360.0, 0.50, 0.50, 1.0),       
            git_modified: hsla(40.0 / 360.0, 0.60, 0.50, 1.0),      
            git_deleted: hsla(0.0 / 360.0, 0.60, 0.50, 1.0),       
        }
    }
}

impl Global for Theme {}
