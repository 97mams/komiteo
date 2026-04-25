use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use tui_big_text::{BigText, PixelSize};

// use crate::config;
// use crate::hello;

pub fn render() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let title = BigText::builder()
        .pixel_size(PixelSize::Octant)
        .style(Style::new().light_blue())
        .lines(vec!["komiteo".light_yellow().into()])
        .build();

    let mut input = String::new();
    let mut messages: Vec<String> = vec![format!("{:?}", title)];

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // layout vertical
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),   // messages
                    Constraint::Length(3) // input
                ])
                .split(size);

            // messages (join avec newline)
            let text = messages.join("\n");

            let messages_widget = Paragraph::new(text)
                .block(Block::default())
                .wrap(Wrap { trim: true });

            f.render_widget(messages_widget, chunks[0]);

            // input box
            let input_widget = Paragraph::new(input.as_str())
                .block(
                    Block::default()
                        .borders(Borders::TOP | Borders::BOTTOM)
                        .title("Message"),
                );

            f.render_widget(input_widget, chunks[1]);

            // curseur dans l’input
            f.set_cursor(
                chunks[1].x + input.len() as u16 + 1,
                chunks[1].y + 1,
            );
        })?;

        // input clavier
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => input.push(c),

                KeyCode::Backspace => {
                    input.pop();
                }

                KeyCode::Enter => {
                    if !input.trim().is_empty() {
                        messages.push(format!("> {}", input));
                    }
                    input.clear();
                }

                KeyCode::Esc => break,

                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}