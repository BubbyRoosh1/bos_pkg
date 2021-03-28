use std::{env, fs};
use std::error::Error;
use std::process::Command;
use yaml_rust::YamlLoader;
use yaml_rust::yaml::Yaml;

/// Packages are named by category/name, so if just name is given, category needs to be figured out
pub fn parse_name(name: &mut String) {
    // TODO
    if name.contains('/') {}
    else {

    }
}

pub fn install_pkgs(names: Vec<String>) -> Result<(), Box<dyn Error>> {
    for name in names {
        install_pkg(name)?;
    }
    Ok(())
}

/// Worked the first time with vim :v
pub fn install_pkg(mut name: String) -> Result<(), Box<dyn Error>> {
    parse_name(&mut name);

    println!("Installing {}...", name);
    let mut installed = fs::read_to_string("/var/db/bos_pkg/installed")?;
    // Skip if already installed
    if installed.contains(&name) {
        println!("{} is already installed.", name);
        return Ok(());
    }

    // Read config
    let file = fs::read_to_string(format!("/var/db/bos_pkg/pkgs/{}/config.yaml", name))?;
    let yaml = &YamlLoader::load_from_str(&file)?[0];
    // Install dependencies.
    if let Yaml::Array(arr) = &yaml["depends"] {
        for dependency in arr {
            if let Yaml::String(d) = dependency {
                // Recursion :V
                install_pkg(d.to_owned())?;
            }
        }
    }

    let mut made_dir = false;
    // Prepare for build
    if let Yaml::String(git) = &yaml["git"] {
        fs::create_dir_all(format!("/tmp/{}", name))?;
        env::set_current_dir(format!("/tmp/{}", name))?;
        git2::Repository::clone(git, format!("/tmp/{}", name))?;
        made_dir = true;
    }

    // TODO: Don't finalize if this is cancelled lol
    // Run build
    Command::new("sh").arg(format!("/var/db/bos_pkg/pkgs/{}/build.sh", name)).spawn()?.wait()?;

    // Finalize
    let mut insname = name.clone();
    insname.push('\n');
    installed.push_str(&insname);
    fs::write("/var/db/bos_pkg/installed", installed)?;

    // Clean up
    if made_dir {
        fs::remove_dir_all(format!("/tmp/{}", name))?;
    }

    Ok(())
}

pub fn remove_pkgs(names: Vec<String>) -> Result<(), Box<dyn Error>> {
    for name in names {
        remove_pkg(name)?;
    }
    Ok(())
}

pub fn remove_pkg(_name: String) -> Result<(), Box<dyn Error>> {
    // TODO
    Ok(())
}
