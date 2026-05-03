use crate::hello;
// use crate::cmd;
use crate::agent;
use crate::config;
// use crate::hello;

pub fn render() {
    hello::logo();
    hello::hero();
    if !config::check_api_key() {
        hello::description();
    }
    let  _ = agent::agent();
    hello::input();
}
