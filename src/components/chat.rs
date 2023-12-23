use async_trait::async_trait;
use crossterm::style::Stylize;
use tracing_subscriber::field::display::Messages;
use std::{sync::Arc, ops::Deref, cmp::max};

use ratatui::{Frame, layout::{Rect, Alignment}, widgets::{Block, Borders, Paragraph, Wrap}, text::{Line, Span, Text}, style::{Styled, Style, Color}};
use tokio::{sync::{mpsc::{UnboundedSender, UnboundedReceiver}, RwLock}, task::JoinHandle};
use color_eyre::eyre::Result;

use twitch_irc::{login::StaticLoginCredentials, message::{ServerMessage, PrivmsgMessage}};
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use crate::action::Action;

use super::Component;


pub struct TwitchMessageReceiver {
    incoming_messages: Arc<RwLock<UnboundedReceiver<ServerMessage>>>,
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    messages: Arc<RwLock<Vec<PrivmsgMessage>>>,
    join_handler: Option<JoinHandle<()>>
}


impl TwitchMessageReceiver {
    pub fn new() -> Self {
        let config = ClientConfig::default();
        let (incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        Self {
            incoming_messages: Arc::new(RwLock::new(incoming_messages)),
            client,
            messages: Arc::new(RwLock::new(vec![])),
            join_handler: None
        }
    }

    pub fn start(mut self) -> Self {
        let incoming_messages = self.incoming_messages.clone();
        let messages = self.messages.clone();

        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.write().await.recv().await {
                match message {
                    ServerMessage::ClearChat(msg) => {
                        messages.write().await.clear();
                    },
                    // ServerMessage::ClearMsg(_) => todo!(),
                    // ServerMessage::GlobalUserState(_) => todo!(),
                    // ServerMessage::Join(_) => todo!(),
                    // ServerMessage::Notice(_) => todo!(),
                    // ServerMessage::Part(_) => todo!(),
                    // ServerMessage::Ping(_) => todo!(),
                    // ServerMessage::Pong(_) => todo!(),
                    ServerMessage::Privmsg(msg) => {
                        messages.write().await.push(msg);
                    },
                    // ServerMessage::Reconnect(_) => todo!(),
                    // ServerMessage::RoomState(_) => todo!(),
                    // ServerMessage::UserNotice(_) => todo!(),
                    // ServerMessage::UserState(_) => todo!(),
                    // ServerMessage::Whisper(_) => todo!(),
                    _ => (),
                };
            }
        });

        self.client.join("hafmc_".to_owned()).unwrap();

        self.join_handler = Some(join_handle);

        self
    }
}


pub struct Chat {
    twitch_message_receiver: Option<TwitchMessageReceiver>,
    messages: Vec<PrivmsgMessage>,
}


impl Chat {
    pub fn new() -> Self {
        Self {
            twitch_message_receiver: None,
            messages: vec![]
        }
    }
}


#[async_trait]
impl Component for Chat {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        // self.command_tx = Some(tx);
        Ok(())
    }

    async fn init(&mut self, area: Rect) -> Result<()> {
        let twitch_message_receiver = TwitchMessageReceiver::new();

        self.twitch_message_receiver = Some(twitch_message_receiver.start());

        Ok(())
    }

    async fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                let receiver = (&self.twitch_message_receiver).as_ref().unwrap();

                self.messages = receiver
                    .messages
                    .read()
                    .await
                    .iter()
                    .map(|v| v.clone())
                    .collect();
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let chat_block = Block::default()
            .title("Chat")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);
        f.render_widget(chat_block, area);

        let text = self.messages
            .iter()
            .map(|msg| {
                let username_style = Style::new().fg(Color::Red);

                Line::from(
                    vec![
                        Span::styled(format!("@{}: ", msg.sender.login), username_style),
                        Span::styled(msg.message_text.clone(), Style::default())
                    ]
                )
            })
            .collect::<Vec<Line>>();

        let mut last_message_widget = Paragraph::new(text)
            .wrap(Wrap { trim: true });

        let line_to_render = last_message_widget.line_count(area.width - 2) as u16;

        if line_to_render > (area.height - 2).into() {
            last_message_widget = last_message_widget.scroll((line_to_render - (area.height - 2), 0));
        };

        f.render_widget(
            last_message_widget,
            Rect { x: area.x + 1, y: area.y + 1, width: area.width - 2, height: area.height - 2 }
        );

        Ok(())
    }
}
