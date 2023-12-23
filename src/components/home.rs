use async_trait::async_trait;
use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame, info::Info, chat::Chat};
use crate::action::Action;


pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    info_widget: Info,
    chat_widget: Chat,
}

impl Home {
    pub fn new() -> Self {
        Self {
            command_tx: None,
            info_widget: Info::new(),
            chat_widget: Chat::new()
        }
    }
}

#[async_trait]
impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    async fn init(&mut self, area: Rect) -> Result<()> {
        self.info_widget.init(area).await?;
        self.chat_widget.init(area).await?;

        Ok(())
    }

    async fn update(&mut self, action: Action) -> Result<Option<Action>> {
        self.info_widget.update(action.clone()).await?;
        self.chat_widget.update(action.clone()).await?;

        match action {
            Action::Tick => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);

        self.info_widget.draw(f, main_layout[0])?;
        self.chat_widget.draw(f, main_layout[1])?;

        Ok(())
    }
}
