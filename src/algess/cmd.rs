use std::io::{ Write };
use clap::{ Parser, Subcommand };

use crate::algess::channels::{self, Channel};

pub fn start_recv_stdin() {
    loop {
        let mut input = String::new();
        std::io::stdout().flush().expect("Failed to flush.");

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                parse_cmd(trimmed);
            },
            Err(err) => {
                panic!("{}", err);
            },
        }
    }
}

fn parse_cmd(command: &str) {
    let cmd = std::iter::once("");
    let cmd: Vec<&str> = cmd.chain(command.split_whitespace()).collect();

    match StdInput::try_parse_from(cmd) {
        Ok(args) => {
            handle_cmd(args);
        },
        Err(err) => println!("{}", err),
    }
}

fn handle_cmd(args: StdInput) {
    match args.command {
        Cmd::Channel { action } => {
            match action {
                channel::ChannelAction::Create { name } => {
                    let mut chm = channels::get_channel_manager().lock().unwrap();
                    chm.create_channel(name);
                },
                channel::ChannelAction::Delete { name } => {
                    let mut chm = channels::get_channel_manager().lock().unwrap();
                    chm.delete_channel(name);
                },
                channel::ChannelAction::Show { count } => {
                    let channels = channels::get_channel_manager().lock().unwrap().get_channels();
                    channels.iter().for_each(|ch: &Channel| {
                        println!("{} {}", ch.name, count);
                    });
                },
            };
        },
        Cmd::Quit {  } => todo!(),
    }
}

#[derive(Parser, Debug)]
#[command(name = "")]
// #[command(about = None)]
struct StdInput {
    #[command(subcommand)]
    command: Cmd,
}

mod channel;

#[derive(Subcommand, Debug)]
enum Cmd {
    Channel {
        #[command(subcommand)]
        action: channel::ChannelAction,
    },
    Quit { }
}