// // use futures_util::StreamExt;
// // use openrouter_rs::{OpenRouterClient, api::chat::*, types::Role};
// // use crossterm::{
// //     event::{self, Event, KeyCode, KeyEventKind},
// //     execute,
// //     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// // };
// // use std::io;

// // mod hello;
// // mod config;
// // mod cil;

// use color_eyre::eyre::{Ok, Result};
// use ratatui::{
//     DefaultTerminal, Frame,
//     crossterm::event::{self, Event},
//     layout::Alignment,
//     style::{Color, Style},
//     widgets::BorderType,
//     widgets::{Block, Borders, Paragraph},
// };
// use image::ImageReader;




// // #[tokio::main]
// fn main() -> Result<()> {
//     // let config = config::get_api_key().api_key;
//     // let key:&str = &config;
//     // let diff = cil::diff();
//     // let client = OpenRouterClient::builder()
//     //     .api_key(key)
//     //     .build()?;

//     // let request = ChatCompletionRequest::builder()
//     //     .model("arcee-ai/trinity-large-preview:free")
//     //     .messages(vec![Message::new(Role::User, format!("Generate a short Git commit message in English based on this git diff.

//     // Format:
//     // <type>: <body>

//     // Rules:
//     // - Use conventional types (feat, fix, refactor, chore, docs, test)
//     // - Present tense
//     // - One line only

//     // Git diff:
//     // {}", diff))])
//     //     .build()?;

//     // let mut stream = client.chat().stream(&request).await?;

//     // let mut commit_message = String::new();

//     // while let Some(result) = stream.next().await {
//     //     if let Ok(response) = result {
//     //         if let Some(content) = response.choices[0].content() {
//     //             commit_message.push_str(content);
//     //             print!("{}", content);
//     //         }
//     //     }
//     // }
//     // cil::commit(&commit_message);
//     // cil::push();
//     // println!();
//     // Ok(())
//     color_eyre::install()?;

//     let terminal = ratatui::init();

//     let result = run(terminal);
//     ratatui::restore();

//     return result;
// }

// fn run(mut terminal: DefaultTerminal) -> Result<()> {
//     loop {
//         terminal.draw(render)?;

//         if let Event::Key(key) = event::read()? {
//             match key.code {
//                 event::KeyCode::Char('q') => break,
//                 _ => {}
//             }
//         }
//     }

//     Ok(())
// }

// fn render(frame: &mut Frame) {
//     let dyn_img = ImageReader::open("./assets/images/logo.png")
//         .decode()?;
//     println!("{:?}", dyn_img);

//     let p = Paragraph::new("Welcome to Komiteo!")
//         .alignment(Alignment::Center)
//         .style(Style::default().fg(Color::Yellow))
//         .block(
//             Block::default()
//                 .borders(Borders::ALL)
//                 .title("Komiteo")
//                 .border_type(BorderType::Rounded), 
//         );
//     frame.render_widget(p, frame.area());
// }
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    widgets::{Block, Borders},
    Frame,
};
use ratatui_image::{picker::Picker, protocol::Protocol, Image};
use image::io::Reader as ImageReader;
use std::rc::Rc;

// --- Structure pour stocker l'état de ton image ---
struct App {
    image_state: Box<dyn Protocol>,
}

fn ui(f: &mut Frame, app: &mut App) {
    let area = f.size();

    // 1. On crée une boîte centrée
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Mon Image dans une Box ");

    // 2. On récupère la zone à l'intérieur de la box (pour ne pas dessiner sur les bords)
    let inner_area = chunks[1].inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });

    // 3. On affiche la box
    f.render_widget(block, chunks[1]);

    // 4. On affiche l'image à l'intérieur
    // L'image va automatiquement s'adapter à la taille de inner_area
    let image_widget = Image::new(app.image_state.as_ref());
    f.render_widget(image_widget, inner_area);
}

// --- Dans ton main (Initialisation) ---
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise le picker SANS chafa (utilisera les Halfblocks par défaut)
    let mut picker = Picker::from_termios()?;
    
    // Charge ton image
    let img = ImageReader::open("assets/images/logo.png")?.decode()?;
    let image_state = picker.new_protocol(img);
    
    let mut app = App { image_state };
    
    // ... reste de ta boucle Ratatui classique ...
    Ok(())
}