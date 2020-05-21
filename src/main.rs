
use structopt::StructOpt;

mod installer;

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
  Local(LocalOpts),
  #[structopt(name = "install")]
  Install(InstallOpts)
}

#[derive(Debug, StructOpt)]
struct LocalOpts {
  version: u8
}

#[derive(Debug, StructOpt)]
struct InstallOpts {
  version: Option<u8>
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

      Command::Install(cfg) => {
        installer::Installer::install();
        println!("handle Install: {:?}", cfg);
      }
    }
  }
}
