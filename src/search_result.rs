use crate::data::{read_item_data, ConfigItem};
use crossterm::event::{KeyCode, KeyEvent};

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
    viewport_start: usize,
}

#[derive(Debug, Clone)]
pub struct FilterResult {
    index: usize,
    row_content: String,
    item_id: String,
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
            viewport_start: 0,
        }
    }

    pub fn handle_event<F: FnMut(&String, &usize)>(&mut self, event: KeyEvent, mut callback: F) {
        match event.code {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_index < self.list.len() - 1 {
                    self.selected_index += 1;
                }
            }
            KeyCode::Right => {
                let FilterResult { item_id, row,.. } = &self.list[self.selected_index];
                callback(item_id, row);
            }
            _ => {}
        }
    }

    pub fn clear(&mut self) {
        self.selected_index = 0;
        self.viewport_start = 0;
        self.list.clear();
    }

    pub fn update(&mut self, all_hosts_item_list: &Vec<ConfigItem>, filter_input: String) {
        let mut list: Vec<FilterResult> = vec![];
        let mut index: usize = 0;
        for item in all_hosts_item_list {
            let id = item.id();
            if id == "system" {
                continue;
            }
            if let Ok(content) = read_item_data(id) {
                let searched = search_hosts(&filter_input, content);
                let mut filter_results = searched
                    .iter()
                    .map(|r| {
                        index += 1;
                        FilterResult {
                            index,
                            row: r.0 + 1,
                            item_id: id.clone(),
                            row_content: r.1.clone(),
                            title: item.title().clone(),
                            filter_input: filter_input.clone(),
                        }
                    })
                    .collect::<Vec<FilterResult>>();
                list.append(&mut filter_results);
            }
        }
        self.list = list;
        self.selected_index = 0;
        self.viewport_start = 0;
    }

    pub fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let visible_rows = area.height.saturating_sub(1) as usize;
        if self.selected_index > self.viewport_start + visible_rows - 1 {
            self.viewport_start += 1;
        } else if self.selected_index < self.viewport_start {
            self.viewport_start = self.selected_index;
        }
        let rows = self
            .list
            .iter()
            .enumerate()
            .skip(self.viewport_start)
            .take(visible_rows as usize)
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
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };
                Row::new(vec![
                    Cell::from(fr.index.to_string()),
                    Cell::from(matched),
                    Cell::from(fr.title.as_str()),
                    Cell::from(fr.row.to_string()),
                ])
                .style(row_style)
            })
            .collect::<Vec<_>>();
        let table = Table::new(
            rows.clone(),
            [
                Constraint::Length(10),
                Constraint::Fill(1),
                Constraint::Length(20),
                Constraint::Length(10),
            ],
        )
        .header(Row::new(vec!["Index", "匹配", "标题", "行号"]).height(1));

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
        assert_eq!(
            result,
            vec![
                SearchDetail(1, "127.0.0.1 localhost".to_string()),
                SearchDetail(2, "127.0.0.1 dev.cn".to_string())
            ]
        );
    }

    #[test]
    pub fn test_take() {
        let a = vec![2, 3, 4, 5];

        let b = a.iter().enumerate().skip(2).take(2).map(|(index, num)| {
            return index
        }).collect::<Vec<_>>();

        println!("{:#?}", b);
    }
}
