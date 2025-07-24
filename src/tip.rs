use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
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
        let strong_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::LightGreen);
        let edit_list_message_line = Line::from(vec![
            Span::styled("Shift+N", strong_style),
            Span::raw(" 添加hosts "),
            Span::styled("Shift+D", strong_style),
            Span::raw(" 删除hosts "),
            Span::styled("Shift+M", strong_style),
            Span::raw(" 修改标题 "),
            Span::styled("→", strong_style),
            Span::raw("进入编辑"),
            Span::styled("Ctrl+C", strong_style),
            Span::raw(" 退出 "),
        ]);
        let edit_hosts_message_line = Line::from(vec![
            Span::styled("Esc", strong_style),
            Span::raw(" 退出编辑模式 "),
        ]);
        let edit_title_message_line: Line<'_> =
            Line::from(vec![Span::styled("Esc", strong_style), Span::raw(" 退出 ")]);
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
