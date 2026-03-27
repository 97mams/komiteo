use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> color_eyre::Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> color_eyre::Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let container = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(80), Constraint::Percentage(20)]).split(frame.area());

        let title =  "Bienvenue sur Komiteo !".light_yellow().bold();
        let content = Text::from(vec![
            Line::from(""),
            Line::from("💫 Le CLI qui automatise votre flux Git avec l'intelligence d'OpenRouter.").alignment(Alignment::Center),
             Line::from(""),
            Line::from("👋 Bonjour ! Bienvenue dans l'aventure KOMITEO.").italic(),
             Line::from(""),
            Line::from("Avant de pouvoir générer des messages de commit parfaits et de pusher votre code
en un clin d'œil, nous devons configurer un petit quelque chose.").italic(),
            Line::from("KOMITEO utilise la puissance de l'IA d'OpenRouter pour comprendre vos changements
et écrire des messages de commit clairs, concis et standardisés.").italic()
            ]);
        let paragraph = Paragraph::new(content).block(Block::default().title(title).border_style(Color::LightYellow ).borders(Borders::ALL)).wrap(Wrap { trim: (true) });
        frame.render_widget(paragraph, container[0]);
        Ok(())
    }
}
