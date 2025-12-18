use std::io::{ Write };
use clap::{ Parser, Subcommand };

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
            println!("Parsed >> {:?}", args);
        },
        Err(err) => println!("{}", err),
    }
}

fn proccess_cmd(args: StdInput) {
    match args.command {
        Cmd::Channel { action } => {
            match action {
                channel::ChannelAction::Create { name } => todo!(),
                channel::ChannelAction::Delete { name } => todo!(),
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