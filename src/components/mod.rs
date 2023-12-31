use async_trait::async_trait;

use color_eyre::eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use futures::Future;
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
  action::Action,
  tui::{Event, Frame},
};

pub mod home;
pub mod info;
pub mod chat;


#[async_trait]
pub trait Component {
  /// Register an action handler that can send actions for processing if necessary.
  ///
  /// # Arguments
  ///
  /// * `tx` - An unbounded sender that can send actions.
  ///
  /// # Returns
  ///
  /// * `Result<()>` - An Ok result or an error.
  #[allow(unused_variables)]
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    Ok(())
  }

  /// Initialize the component with a specified area if necessary.
  ///
  /// # Arguments
  ///
  /// * `area` - Rectangular area to initialize the component within.
  ///
  /// # Returns
  ///
  /// * `Result<()>` - An Ok result or an error.
  async fn init(&mut self, area: Rect) -> Result<()> {
    Ok(())
  }

  /// Handle incoming events and produce actions if necessary.
  ///
  /// # Arguments
  ///
  /// * `event` - An optional event to be processed.
  ///
  /// # Returns
  ///
  /// * `Result<Option<Action>>` - An action to be processed or none.
  fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>> {
    let r = match event {
      Some(Event::Key(key_event)) => self.handle_key_events(key_event)?,
      Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event)?,
      _ => None,
    };
    Ok(r)
  }

  /// Handle key events and produce actions if necessary.
  ///
  /// # Arguments
  ///
  /// * `key` - A key event to be processed.
  ///
  /// # Returns
  ///
  /// * `Result<Option<Action>>` - An action to be processed or none.
  #[allow(unused_variables)]
  fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    Ok(None)
  }

  /// Handle mouse events and produce actions if necessary.
  ///
  /// # Arguments
  ///
  /// * `mouse` - A mouse event to be processed.
  ///
  /// # Returns
  ///
  /// * `Result<Option<Action>>` - An action to be processed or none.
  #[allow(unused_variables)]
  fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Result<Option<Action>> {
    Ok(None)
  }

  /// Update the state of the component based on a received action. (REQUIRED)
  ///
  /// # Arguments
  ///
  /// * `action` - An action that may modify the state of the component.
  ///
  /// # Returns
  ///
  /// * `Result<Option<Action>>` - An action to be processed or none.
  #[allow(unused_variables)]
  async fn update(&mut self, action: Action) -> Result<Option<Action>> {
    Ok(None)
  }

  /// Render the component on the screen. (REQUIRED)
  ///
  /// # Arguments
  ///
  /// * `f` - A frame used for rendering.
  /// * `area` - The area in which the component should be drawn.
  ///
  /// # Returns
  ///
  /// * `Result<()>` - An Ok result or an error.
  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()>;
}
