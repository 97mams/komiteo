use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen
    },
};

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph}
};

struct App {
    api_key: String,
}

pub fn hello_welcome() -> Result<(), Box<dyn std::error::Error>> {

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App {
        api_key: String::new(),
    };

    loop {

        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {

                match key.code {

                    KeyCode::Char(c) => {
                        app.api_key.push(c);
                    }

                    KeyCode::Backspace => {
                        app.api_key.pop();
                    }

                    KeyCode::Enter => {
                        break;
                    }

                    KeyCode::Esc => {
                        break;
                    }

                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    println!("Collez votre clé API pour continuer.

Appuyez sur ENTRÉE pour enregistrer
Appuyez sur ÉCHAP pour quitter");


    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(1)
        ])
        .split(frame.size());

    let header = Paragraph::new(
        "KOMITEO\nGénérateur automatique de message de commit Git"
    )
    .block(
        Block::default()
            .title("Bienvenue")
            .borders(Borders::ALL)
    );

    frame.render_widget(header, layout[0]);

    let info = Paragraph::new(
        "Fournisseur LLM : OpenRouter\nSeule l'API OpenRouter est supportée."
    )
    .block(
        Block::default()
            .title("Configuration LLM")
            .borders(Borders::ALL)
    );

    frame.render_widget(info, layout[1]);

    let input = Paragraph::new(app.api_key.as_str())
        .block(
            Block::default()
                .title("Clé API OpenRouter")
                .borders(Borders::ALL)
        );

    frame.render_widget(input, layout[2]);
   
   let footer = Paragraph::new(
        "Collez votre clé API pour continuer.\n\nAppuyez sur ENTRÉE pour enregistrer\nAppuyez sur ÉCHAP pour quitter"
    )
    .block(
        Block::default()
    );

    frame.render_widget(footer, layout[3]);


}