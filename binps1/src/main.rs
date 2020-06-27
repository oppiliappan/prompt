#![feature(type_alias_impl_trait)]

use libps1::helpers::{cwd, git_status, prompt_char, GitStatus};
use libps1::module::Module;
use libps1::prompt;
use libps1::prompt::Prompt;
use libps1::{Color, Style};

fn main() {
    let cwd: Module = Module::new()
        .action(|| cwd(true, true))
        .style(Style::new().fg(Color::Fixed(12)))
        .padding(1, 1);

    let prompt_char: Module = Module::new()
        .action(|| Some(prompt_char('#', '$').to_string()))
        .style(Style::default())
        .padding(0, 0);

    let gst = || {
        let clean = Color::Green.normal();
        let staged = Color::Yellow.normal();
        let unstaged = Color::Red.normal();
        match git_status() {
            Some((b, GitStatus::Clean)) => Some(format!("{}", clean.paint(b))),
            Some((b, GitStatus::Staged)) => Some(format!("{}", staged.paint(b))),
            Some((b, GitStatus::Unstaged)) => Some(format!("{}", unstaged.paint(b))),
            None => None,
        }
    };

    let vcs = Module::new().action(gst).padding(1, 0);

    let prompt: Prompt = prompt!(prompt_char, vcs, cwd);

    println!("{}", prompt);
}
