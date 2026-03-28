use ratatui::{prelude::*, widgets::*};

pub fn hello(frame: &mut Frame)->color_eyre::Result<()> {
    let container = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(frame.area());

        let title =  "Bienvenue sur Komiteo !".light_yellow().bold();
        let content = Text::from(vec![
            Line::from(""),
            Line::from("💫 Le CLI qui automatise votre flux Git avec l'intelligence d'OpenRouter.").alignment(Alignment::Center),
             Line::from(""),
            Line::from("👋 Bonjour ! Bienvenue dans l'aventure KOMITEO.").italic(),
             Line::from(""),
            Line::from("Avant de pouvoir générer des messages de commit parfaits et de pusher votre code en un clin d'œil, nous devons configurer un petit quelque chose.").italic(),
            Line::from("KOMITEO utilise la puissance de l'IA d'OpenRouter pour comprendre vos changements et écrire des messages de commit clairs, concis et standardisés.").italic()
            ]);
        let paragraph = Paragraph::new(content).block(Block::default().title(title).border_style(Color::LightYellow ).borders(Borders::ALL)).wrap(Wrap { trim: (true) });
        frame.render_widget(paragraph, container[0]);

        let seconde_title = "CONFIGURATION DE L'API OPENROUTER".light_yellow().bold();
        let seconde_content = Text::from(vec![
            Line::from(""),
            Line::from("💡 KOMITEO a besoin de votre clé API OpenRouter pour fonctionner."),
            Line::from(""),
            Line::from("Si vous n'en avez pas encore, ne vous inquiétez pas ! C'est rapide et facile.").italic(),
            Line::from(""),
            Line::from("1️ Rendez-vous sur https://openrouter.ai/ et créez un compte si ce n'est pas déjà fait.").italic(),
            Line::from("2️ Une fois connecté, allez dans votre tableau de bord et trouvez la section 'API Keys'.").italic(),
            Line::from("3️ Cliquez sur 'Create API Key', donnez-lui un nom (par exemple 'Komiteo') et copiez la clé générée.").italic()]);
        let seconde_paragraph = Paragraph::new(seconde_content).block(Block::default().title(seconde_title).border_style(Color::LightYellow ).borders(Borders::ALL)).wrap(Wrap { trim: (true) });
        frame.render_widget(seconde_paragraph, container[1]);

        Ok(())
} 