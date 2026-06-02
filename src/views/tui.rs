use std::process;
use colored::Colorize;
use inquire::Select;

use crate::views::hello::{self, display_text_with_typing_effect};
use crate::agent::openrouter;
use crate::config::config;
use crate::git::cil::{self, init_commit};

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

pub async fn render_state() {
    let answer = vec!["oui", "non"];
    if !cil::check_folder_git() {
        hello::display_text_with_typing_effect(
            " Ce dossier n'est pas encore un dépôt git. \n",
        );
        match Select::new("Voulez-vous initialiser le dépôt git ?", answer).prompt() {
            Ok(choice) => {
                if choice == "non" {
                    display_text_with_typing_effect(" Opération annulée. \n");
                    process::exit(0);
                }
                 display_text_with_typing_effect(cil::git_init().as_mut_str());
                return ;
            }
            Err(_) => {
                display_text_with_typing_effect(" Opération annulée. \n");
                return ;
            }
        }
    }
    let status = cil::check_status();
    if status.is_empty() {
        display_text_with_typing_effect(
            " Le prochain commit peut changer tout le projet…\n continue de coder.\n",
        );
        return ;
    }


    if status[0] == "fatal: ni ceci ni aucun de ses répertoires parents n'est un dépôt git : .git" {
        display_text_with_typing_effect(cil::git_init().as_mut_str()
        );
        return ;
    }

    for file in status {
        let diff = cil::diff(file.clone());
        if diff.is_empty() && !cil::check_log() {
             init_commit("init commit");
             return ;
        }
        if let Err(e) = openrouter::agent(diff, file).await {
            let _ = e;
            display_text_with_typing_effect(
                " Veuillez vérifier votre clé API et votre connexion internet. \n Rentrez à nouveau votre clé API. \n-taper reconfig puis entrer.\n",
            );
            break;
        }
    }
}
