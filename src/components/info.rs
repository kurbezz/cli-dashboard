use async_trait::async_trait;

use ratatui::{Frame, layout::{Rect, Alignment}, widgets::{Block, Borders}};
use tokio::sync::mpsc::UnboundedSender;
use color_eyre::eyre::Result;

use crate::action::Action;

use super::Component;

#[derive(Default)]
pub struct Info {}

impl Info {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Component for Info {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        // self.command_tx = Some(tx);
        Ok(())
    }

    async fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let info_block = Block::default()
            .title("Info")
            .title_alignment(Alignment::Center)
            .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM);

        f.render_widget(info_block, area);

        Ok(())
    }
}
