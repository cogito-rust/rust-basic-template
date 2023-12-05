use std::path::PathBuf;

use log::{debug, error, info};
use structopt::{clap::AppSettings, StructOpt};

use my_lib::Module;

#[derive(StructOpt)]
#[structopt(
  name = "wasm-runner",
  about = "Sample project to run local wasm files",
  global_settings(&[
    AppSettings::ColoredHelp
  ]),
)]
struct CliOptions {
  /// the WebAssembly file to load
  #[structopt(parse(from_os_str))]
  pub(crate) file_path: PathBuf,
}

fn main() {
  env_logger::init();
  debug!("Initialized logger");

  let options = CliOptions::from_args();

  match Module::from_file(&options.file_path) {
    Ok(_) => {
      info!("Module loaded")
    }
    Err(e) => {
      error!("Module failed to load: {}", e);
    }
  }
}
