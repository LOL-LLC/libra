use std::{path::PathBuf, process::exit};
use anyhow::Result;

use gumdrop::Options;
use ol_genesis_tools::fork_genesis::make_recovery_genesis;


#[tokio::main]
async fn main() -> Result<()> {
    #[derive(Debug, Options)]
    struct Args {
        #[options(help = "what epoch to restore from archive")]
        epoch: Option<u64>,
        #[options(help = "path to snapshot dir to read")]
        snapshot: Option<PathBuf>,
        #[options(help = "write genesis from recovery file")]
        genesis: Option<PathBuf>,
        #[options(help = "write recovery file from snapshot")]
        recover: Option<PathBuf>,
        #[options(help = "live fork mode")]
        daemon: bool,
        #[options(help = "swarm simulation mode")]
        swarm: bool,
    }

    let opts = Args::parse_args_default_or_exit();

    if let Some(g_path) = opts.genesis {
      if let Some(s_path) = opts.snapshot {
        // create a genesis file from archive file
        make_recovery_genesis(g_path, s_path).await?;
        return Ok(())
      } else {
        println!("ERROR: must provide a path with --snapshot, exiting.");
        exit(1);
      }


    } else if let Some(_a_path) = opts.recover {
      // just create recovery file
        
        return Ok(())
    } else if opts.daemon {
        // start the live fork daemon

        return Ok(())
    } else if opts.swarm {
        // Write swarm genesis from snapshot, for CI and simulation
        if let Some(_archive_path) = opts.snapshot {
            // TODO: block on this future
            // archive_into_writeset(archive_path);
        }
        return Ok(())
    } else {
        println!("ERROR: no options provided, exiting.");
        exit(1);
    }
}
