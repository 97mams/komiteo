use colored::Colorize;

use crate::views::hello;
use crate::agent::openrouter;
use crate::config::config;
use crate::git::cil;

pub async fn render() {
    hello::logo();
    if !config::check_api_key() {
        hello::hero();
        hello::description();
    } else {
        render_state().await;
        println!(" Tapez {} si vous avez besoin d'aide.."," help ".on_bright_yellow());
    }
}

//popup

pub async fn render_state() {
    let status = cil::check_status();
    if status.is_empty() {
        hello::display_text_with_typing_effect(
            " Le prochain commit peut changer tout le projet…\n continue de coder.\n",
        );
        return ;
    }

    if status[0] == "fatal: ni ceci ni aucun de ses répertoires parents n'est un dépôt git : .git" {        
        hello::display_text_with_typing_effect(cil::git_init().as_mut_str()
        );
        return ;
    }

    for file in status {
        let diff = cil::diff(file.clone());
        if let Err(e) = openrouter::agent(diff, file).await {
            let _ = e;
            hello::display_text_with_typing_effect(
                " Veuillez vérifier votre clé API et votre connexion internet. \n Rentrez à nouveau votre clé API. \n-taper reconfig puis entrer.\n",
            );
            break;
        }
    }
}
