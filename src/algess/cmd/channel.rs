use clap::{ Subcommand };

#[derive(Subcommand, Debug)]
pub enum ChannelAction {
    Create {
        name: String,
    },
    Delete {
        name: String,
    },
    Show {
        #[arg(short='c', long)]
        count: bool
    }
}