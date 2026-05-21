use colored::Colorize;
use tokio::time::{interval, Duration};

use crate::views::hello::{self, display_text_with_typing_effect};
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

    let mut interval = interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        println!(" Vérification des changements toutes les 1 secondes... ");
    }

}

pub async fn render_state() {
    if !cil::check_folder_git() {
        hello::display_text_with_typing_effect(
            " Ce dossier n'est pas encore un dépôt git. \n Initialisation du dépôt git en cours...\n",
        );
        display_text_with_typing_effect(cil::git_init().as_mut_str());
        return ;
    }
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
