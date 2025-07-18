use crate::{data::read_item_data, state::State};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Cell, Row, Table, Widget},
};

pub struct SearchResult {
    list: Vec<FilterResult>,
    selected_index: usize,
}

pub struct FilterResult {
    row_content: String,
    filter_input: String,
    title: String,
    row: usize,
}

#[derive(Debug)]
pub struct SearchDetail(usize, String);

impl PartialEq for SearchDetail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

pub fn search_hosts<T: AsRef<str>, U: AsRef<str>>(
    filter_input: T,
    hosts_content: U,
) -> Vec<SearchDetail> {
    let result: Vec<FilterResult> = vec![];
    let lines = hosts_content.as_ref().split("\n");
    lines
        .enumerate()
        .filter(|l| l.1.contains(filter_input.as_ref()))
        .map(|l| SearchDetail(l.0, l.1.trim().to_string()))
        .collect::<Vec<SearchDetail>>()
}

impl SearchResult {
    pub fn new() -> Self {
        SearchResult {
            list: vec![],
            selected_index: 0,
        }
    }

    pub fn update(&mut self, state: &State) {
        let State { filter_input, .. } = state;
        let mut list: Vec<FilterResult> = vec![];
        for item in &state.all_hosts_item_list {
            if let Ok(content) = read_item_data(item.id()) {
                let searched = search_hosts(filter_input, content);
                let mut filter_results = searched
                    .iter()
                    .map(|r| FilterResult {
                        row: r.0 + 1,
                        row_content: r.1.clone(),
                        title: item.title().clone(),
                        filter_input: state.filter_input.clone(),
                    })
                    .collect::<Vec<FilterResult>>();
                list.append(&mut filter_results);
            }
        }
        self.list = list;
        self.selected_index = 0
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let rows = self
            .list
            .iter()
            .enumerate()
            .map(|(i, fr)| {
                let index = fr.row_content.find(fr.filter_input.as_str()).unwrap();
                let front = &fr.row_content[..index];
                let behind_start = index + fr.filter_input.len();
                let behind = &fr.row_content[behind_start..];
                let matched = Line::from(vec![
                    Span::from(front),
                    Span::from(fr.filter_input.clone()).style(Style::default().fg(Color::Green)),
                    Span::from(behind),
                ]);
                let row_style = if self.selected_index == i {
                    Style::default().fg(Color::Gray)
                } else {
                    Style::default()
                };
                let span = Span::from("123").style(Style::default().fg(Color::Green));
                let l = Line::from(vec![span]);
                let c = Cell::from(l);
                Row::new(vec![
                    Cell::from(matched).style(row_style),
                    Cell::from(fr.title.as_str()).style(row_style),
                    Cell::from(fr.row.to_string()).style(row_style),
                ])
            })
            .collect::<Vec<_>>();
        let table = Table::new(
            rows,
            [
                Constraint::Fill(1),
                Constraint::Length(10),
                Constraint::Length(3),
            ],
        )
        .header(Row::new(vec!["匹配", "标题", "行号"]).height(1));
        Widget::render(table, area, buf);
    }
}

#[cfg(test)]
mod tests {
    use crate::search_result::SearchDetail;

    use super::search_hosts;

    #[test]
    pub fn test_search_hosts() {
        let hosts_content = r#"
            127.0.0.1 localhost
            127.0.0.1 dev.cn
            192.167.0.1 dev2.cn
        "#;
        let result = search_hosts("127.0.0.1", hosts_content);
        assert_eq!(result, vec![
            SearchDetail(1, "127.0.0.1 localhost".to_string()),
            SearchDetail(2, "127.0.0.1 dev.cn".to_string())
        ]);
    }
}
