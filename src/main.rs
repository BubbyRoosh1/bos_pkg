use rargsxd::*;
use std::process;

fn main() {
    let mut parser = ArgParser::new("bos_pkg");
    parser.author("BubbyRoosh")
        .info("Package manager for the bos distribution.")
        .copyright("Copyright (C) 2021 BubbyRoosh")
        .usage("bos_pkg [flags] <packages>")
        .require_args(true)
        .args(vec!(
            Arg::new("install")
                .short('i')
                .help("Installs the specified programs.")
                .flag(false),

            // TODO: Targets array in config.yaml; remove all files in that array; ez rm
            Arg::new("remove")
                .short('r')
                .help("Removes the specified programs.")
                .flag(false),
        ))
        .parse();

    if sudo::check() != sudo::RunningAs::Root {
        println!("Please run bos_pkg as root.");
        //process::exit(1);
    }

    if parser.get_flag("install").unwrap()
        && parser.get_flag("remove").unwrap() {
        println!("You can't install and remove the same programs!");
        process::exit(1);
    } else if parser.get_flag("install").unwrap() {
        if let Err(e) = bos_pkg::install_pkgs(parser.extra) {
            eprintln!("Error installing programs: {}", e);
        }
    }
}
