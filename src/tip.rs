use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::WidgetRef,
};

#[derive(Debug, Default)]
pub struct Tip<'a> {
    which: usize,
    lines: [Line<'a>; 3],
}
impl<'a> Tip<'a> {
    pub fn new() -> Self {
        let edit_list_message_line = Line::from(vec![
            Span::styled("Shift+N", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 添加hosts "),
            Span::styled("Shift+D", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 删除hosts "),
            Span::styled("Ctrl+C", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 退出 "),
             Span::styled("→", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("进入编辑"),
        ]);
        let edit_hosts_message_line = Line::from(vec![
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 退出编辑模式 "),
        ]);
        let edit_title_message_line: Line<'_> = Line::from(vec![
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 退出 "),
        ]);
        return Tip {
            which: 0,
            lines: [
                edit_list_message_line,
                edit_hosts_message_line,
                edit_title_message_line,
            ],
        };
    }

    pub fn show_line(&mut self, idx: usize) {
        self.which = idx;
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        self.lines[self.which].render_ref(area, buf);
    }
}
