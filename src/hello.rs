use ratatui::{
    widgets::{Block, Borders, Paragraph},
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
};
pub fn hello_welcome()-> Result<(), Box<dyn std::error::Error>> {

    enable_raw_mode()?;

    let backend = ratatui::backend::CrosstermBackend::new(std::io::stdout());
    let mut terminal = ratatui::Terminal::new(backend)?;

    terminal.draw(|frame| {
        let area = frame.area();

        let input = Paragraph::new("Votre texte ici")
            .block(
                Block::default()
                    .title("Komiteo Input")
                    .borders(Borders::ALL),
            );

        frame.render_widget(input, area);
    })?;

    disable_raw_mode()?;

    Ok(())
}