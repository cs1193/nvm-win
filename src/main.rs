
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "nvm", about = "nvim")]
struct Opt {
  #[structopt(subcommand)]
  commands: Option<Command>
}

#[derive(Debug, StructOpt)]
#[structopt(name = "command", about = "command")]
enum Command {
  #[structopt(name = "local")]
  Local(LocalOpts)
}

#[derive(Debug, StructOpt)]
struct LocalOpts {
  version: u8
}

fn main() {
  let opt = Opt::from_args();
  println!("{:?}", opt);
  handle_subcommand(opt);
}

fn handle_subcommand(opt: Opt) {
  if let Some(subcommand) = opt.commands{
    match subcommand {
      Command::Local(cfg) => {
        println!("handle Local: {:?}", cfg);
      }
    }
  }
}
