use ratatui::widgets::Paragraph;
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
    let title = Text::from("komiteo 0.0.1").fg(Color::LightYellow).bold();
    let paragraph = Paragraph::new(title);
    frame.render_widget(paragraph, layout[0]);
}