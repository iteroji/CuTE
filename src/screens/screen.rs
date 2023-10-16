/*
    Screen Enum And Implementation
*/

use std::fmt::{Display, Formatter};

use crate::display::inputopt::InputOpt;
use crate::display::menuopts::{
    AUTHENTICATION_MENU_OPTIONS, DOWNLOAD_MENU_OPTIONS, MAIN_MENU_OPTIONS, METHOD_MENU_OPTIONS,
    MORE_FLAGS_MENU, OPTION_PADDING_MAX, OPTION_PADDING_MID, OPTION_PADDING_MIN,
    REQUEST_MENU_OPTIONS, RESPONSE_MENU_OPTIONS,
};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};

#[derive(Debug, PartialEq, Clone)]
pub enum Screen {
    Home,
    Method,
    Downloads,
    HeaderAddRemove,
    RequestMenu(String),
    InputMenu(InputOpt),
    Response(String),
    Authentication,
    Success,
    SavedKeys,
    SavedCommands,
    Error(String),
    ViewBody,
    MoreFlags,
    AlertMenu(usize),
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let screen = match self {
            Screen::Home => "Home",
            Screen::Method => "Method",
            Screen::Downloads => "Downloads",
            Screen::HeaderAddRemove => "HeaderAddRemove",
            Screen::RequestMenu(_) => "RequestMenu",
            Screen::InputMenu(_) => "InputMenu",
            Screen::Response(_) => "Response",
            Screen::Authentication => "Authentication",
            Screen::Success => "Success",
            Screen::SavedKeys => "Saved Keys",
            Screen::SavedCommands => "My Saved Commands",
            Screen::Error(_) => "Error",
            Screen::ViewBody => "ViewBody",
            Screen::MoreFlags => "MoreFlags",
            Screen::AlertMenu(_) => "AlertMenu",
        };
        write!(f, "{}", screen)
    }
}

pub fn determine_line_size(len: usize) -> &'static str {
    match len {
        len if len <= 4 => OPTION_PADDING_MAX,
        len if len < 8 => OPTION_PADDING_MID,
        _ => OPTION_PADDING_MIN,
    }
}

impl<'a> Screen {
    pub fn default() -> Self {
        Screen::Home
    }
    pub fn get_opts(&self, items: Option<Vec<String>>) -> Vec<ListItem<'a>> {
        match &self {
            Screen::Home => {
                let len = MAIN_MENU_OPTIONS.len();
                return MAIN_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, determine_line_size(len))))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::Method => {
                let len = METHOD_MENU_OPTIONS.len();
                return METHOD_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, determine_line_size(len))))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::HeaderAddRemove => {
                let len = METHOD_MENU_OPTIONS.len();
                return METHOD_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, determine_line_size(len))))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::RequestMenu(_) => {
                let len = REQUEST_MENU_OPTIONS.len();
                return REQUEST_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, determine_line_size(len))))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::SavedCommands => {
                let len = REQUEST_MENU_OPTIONS.len();
                return items
                    .unwrap_or(vec!["No Saved Commands".to_string()])
                    .iter()
                    .map(|c| ListItem::new(format!("{}{}", c, determine_line_size(len))))
                    .collect();
            }
            Screen::Response(_) => {
                let len = RESPONSE_MENU_OPTIONS.len();
                return RESPONSE_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, determine_line_size(len))))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::InputMenu(_) => {
                vec![ListItem::new("Input Menu").style(Style::default().fg(Color::Green))]
            }
            Screen::Authentication => {
                let len = AUTHENTICATION_MENU_OPTIONS.len();
                return AUTHENTICATION_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, determine_line_size(len))))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::Success => {
                vec![ListItem::new("Success!").style(Style::default().fg(Color::Green))]
            }
            Screen::Error(_) => {
                vec![ListItem::new("Error!").style(Style::default().fg(Color::Red))]
            }
            Screen::ViewBody => {
                vec![ListItem::new("View Body").style(Style::default().fg(Color::Green))]
            }
            Screen::AlertMenu(_) => {
                vec![
                    ListItem::new("Execute Command"),
                    ListItem::new("Copy to Clipboard"),
                    ListItem::new("Cancel"),
                ]
            }
            Screen::Downloads => {
                let len = DOWNLOAD_MENU_OPTIONS.len();
                return DOWNLOAD_MENU_OPTIONS
                    .iter()
                    .map(|x| String::from(format!("{}{}", x, determine_line_size(len))))
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
            }
            Screen::SavedKeys => {
                let mut len = 0;
                if items.is_some() {
                    len = items.as_ref().unwrap().len();
                }
                return items
                    .unwrap_or(vec!["No Saved Commands".to_string()])
                    .iter()
                    .map(|c| ListItem::new(format!("{}{}", c, determine_line_size(len))))
                    .collect();
            }
            Screen::MoreFlags => {
                let len = MORE_FLAGS_MENU.len();
                return MORE_FLAGS_MENU
                    .iter()
                    .map(|i| {
                        ListItem::new(format!("{}{}", i, determine_line_size(len)))
                            .style(Style::default().fg(Color::Red))
                    })
                    .collect();
            }
        }
    }

    pub fn get_list(&self, items: Option<Vec<String>>) -> List {
        List::new(self.get_opts(items))
            .block(
                Block::default()
                    .title(self.to_string().clone())
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("󱋰 ")
    }
}
