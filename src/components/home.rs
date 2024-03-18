use std::{collections::HashMap, time::Duration};

use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use crossterm::event::{KeyCode, KeyEvent};
use futures::SinkExt;
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{action::Action, config::Config, service::postgres::PostgresClient};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
  #[default]
  Normal,
  Insert,
}
pub struct Home {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
  client: PostgresClient,
  last_event: Vec<KeyEvent>,
  mode: Mode,
}

impl Home {
  pub async fn new() -> Self {
    let client = match PostgresClient::new(None).await {
      Ok(c) => c,
      Err(_) => panic!("Client failed to initlize"),
    };
    Self { command_tx: None, config: Config::default(), client, mode: Mode::default(), last_event: Vec::new() }
  }
}

impl Component for Home {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    Ok(())
  }

  fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    self.last_event.push(key);
    Ok(None)
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => {},
      _ => {},
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let db_names = self.client.database_names.clone();
    let layout = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
      .split(area);
    let mut state = ListState::default().with_selected(Some(0));
    let list = List::new(db_names.clone())
      .highlight_symbol(">>")
      .direction(ListDirection::TopToBottom)
      .block(Block::default().title("Databases").borders(Borders::ALL));
    let list2 = List::new(db_names.clone()).block(Block::default().title("Tabble").borders(Borders::ALL));
    f.render_stateful_widget(list, layout[0], &mut state);
    f.render_stateful_widget(list2, layout[1], &mut state);

    Ok(())
  }
}
