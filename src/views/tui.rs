use crate::views::hello;
// use crate::cmd;
use crate::agent;
use crate::config::config;
// use crate::hello;

pub async  fn render() {
    hello::logo();
    hello::hero();
    if !config::check_api_key() {
        hello::description();
    }
   agent::run_agent_with_spinner().await;
    // hello::input();
}
