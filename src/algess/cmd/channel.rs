use clap::{ Subcommand };

#[derive(Subcommand, Debug)]
pub enum ChannelAction {
    Create {
        #[arg(short, long)]
        name: String,
    },
    Delete {
        #[arg(short, long)]
        name: String,
    },
    Show { }
}