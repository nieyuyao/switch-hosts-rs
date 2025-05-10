use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    style::{Modifier, Style},
    widgets::Widget,
};

#[derive(Debug, Default)]
pub struct Tip<'a> {
    which: usize,
    lines: [Line<'a>; 3]
}
impl<'a> Tip<'a> {
    pub fn new() -> Self {
        let edit_list_message_line = Line::from(vec![
            Span::styled("Ctrl+N", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 添加hosts "),
            Span::styled("Ctrl+D", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 删除hosts "),
            Span::styled("Ctrl+C", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 退出 "),
        ]);
        let edit_hosts_message_line = Line::from(vec![
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 退出编辑模式 "),
        ]);
        let edit_title_message_line = Line::from(vec![
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 确定添加 "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" 退出 "),
        ]);
        return Tip {
            which: 0,
            lines: [edit_list_message_line, edit_hosts_message_line, edit_title_message_line]
        }
    }

    pub fn show_line(&mut self, idx: usize) {
        self.which = idx;
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        self.lines[self.which].clone().render(area, buf);
    }
}
