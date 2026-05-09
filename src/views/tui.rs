
use crate::views::hello;
use crate::event;
// use crate::cmd;
use crate::agent::openrouter;
use crate::config::config;

pub async  fn render() {
    hello::logo();
    if !config::check_api_key() {
        hello::hero();
        hello::description();
    } else {
        if let Err(e) = openrouter::agent().await {
            let _ = e;
            hello::display_text_with_typing_effect(" Veuillez vérifier votre clé API et votre connexion internet. \n Rentrez à nouveau votre clé API. \n-taper reconfig puis entrer.\n" );
        }
    }
   
    let _ = event::input();
}
