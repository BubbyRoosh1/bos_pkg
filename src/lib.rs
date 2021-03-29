use std::{env, fs};
use std::error::Error;
use std::process::Command;
use yaml_rust::YamlLoader;
use yaml_rust::yaml::Yaml;

fn get_config(name: &String) -> Result<Yaml, Box<dyn Error>> {
    let file = fs::read_to_string(format!("/var/db/bos_pkg/pkgs/{}/config.yaml", name))?;
    Ok(YamlLoader::load_from_str(&file)?[0].clone())
}

/// Packages are named by category/name, so if just name is given, category needs to be figured out
pub fn parse_name(mut name: String) -> Result<String, Box<dyn Error>> {
    // Assume names with '/' are correct.
    if !name.contains('/') {
        let package_index = fs::read_to_string("/var/db/bos_pkg/index")?;
        for pkg in package_index.lines() {
            if pkg.split("/").collect::<Vec<_>>().get(1).unwrap() == &name {
                name = pkg.to_owned();
            }
        }
    }
    Ok(name)
}

pub fn query_pkg(name: String) -> Result<(), Box<dyn Error>> {
    let name = parse_name(name)?;
    let package_index = fs::read_to_string("/var/db/bos_pkg/index")?;
    for pkg in package_index.lines() {
        if pkg.split("/").collect::<Vec<_>>().get(1).unwrap().contains(&name) {
            println!("{}", pkg);
        }
    }
    Ok(())
}

pub fn install_pkgs(names: Vec<String>) -> Result<(), Box<dyn Error>> {
    for name in names {
        install_pkg(name)?;
    }
    Ok(())
}

/// Worked the first time with vim :v
pub fn install_pkg(name: String) -> Result<(), Box<dyn Error>> {
    let name = parse_name(name)?;

    println!("Installing {}...", name);
    let mut installed = fs::read_to_string("/var/db/bos_pkg/installed")?;
    // Skip if already installed
    for line in installed.lines() {
        if line.starts_with(&name) {
            println!("{} is already installed.", name);
            return Ok(());
        }
    }

    // Read config
    let yaml = get_config(&name)?;

    // Install dependencies.
    if let Yaml::Array(arr) = &yaml["depends"] {
        for dependency in arr {
            if let Yaml::String(d) = dependency {
                // Recursion :V
                install_pkg(d.to_owned())?;
                installed.push_str(d);
                installed.push('\n');
            }
        }
    }

    let mut made_dir = false;
    // Prepare for build
    if let Yaml::String(git) = &yaml["git"] {
        let dir = format!("/tmp/{}", name);
        fs::create_dir_all(&dir)?;
        env::set_current_dir(&dir)?;
        git2::Repository::clone(git, dir)?;
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

// TODO: Find a better way to do this.
pub fn remove_pkg(name: String) -> Result<(), Box<dyn Error>> {
    let name = parse_name(name)?;
    let installed = fs::read_to_string("/var/db/bos_pkg/installed")?;

    let mut index = -1;
    for (idx, line) in installed.lines().enumerate() {
        if line.starts_with(&name) {
            index = idx as i32;
        }
    }

    // Any other way to do this..? That makes a decent amount of sense.
    if index == -1 {
        println!("{} is not installed.", name);
        return Ok(());
    }

    let yaml = get_config(&name)?;
    if let Yaml::Array(arr) = &yaml["files"] {
        for entry in arr {
            if let Yaml::String(file) = entry {
                let path = std::path::Path::new(file);
                if path.is_dir() {
                    println!("Removing dir {}", file);
                    fs::remove_dir_all(file)?;
                } else if path.is_file() {
                    println!("Removing file {}", file);
                    fs::remove_file(file)?;
                }
            }
        }

        let mut new_installed = String::new();
        for (idx, line) in installed.lines().enumerate() {
            if idx != index as usize {
                new_installed.push_str(line);
                new_installed.push('\n');
            }
        }
        fs::write("/var/db/bos_pkg/installed", new_installed)?;
    }
    Ok(())
}
