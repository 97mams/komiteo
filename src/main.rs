use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use ratatui::prelude::*;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    ratatui::run(app)?;

    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let layout= Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
                .split(frame.area());
    let title = "Komiteo 0.0.1";
    let text1 = Text::from("Le CLI qui automatise votre flux Git avec l'intelligence d'OpenRouter.");
    let paragraph = Paragraph::new(text1)
    .style(Style::default().fg(Color::Yellow))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_type(BorderType::Rounded)
    );
    frame.render_widget(paragraph, layout[0]);
}