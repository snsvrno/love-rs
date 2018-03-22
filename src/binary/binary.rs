//! tools for working with love binaries

use version::version::Version;
use platform::Platform;
use std::path::PathBuf;

use std::process::Command;
use ansi_term::Colour::Red;


use lpsettings;
use binary::install;

pub struct Binary {
  pub path : Option<PathBuf>,
  pub platform : Platform,
  pub version : Version
}

impl Binary {
  pub fn new(platform : &Platform, version : &Version, path : Option<PathBuf>) -> Binary {
    //! creates a new love binary object.
    
    // checking the path, will return the supplied pathbuf it exists or default directory if it already exists. 
    let new_path : Option<PathBuf> = match path {
      Some(pathbuf) => { Some(pathbuf) }
      None => {
        let ideal_path = build_path(&platform,&version);
        match ideal_path.exists() {
          true => { Some(ideal_path) }
          false => { None }
        }
      }
    };

    Binary{ path: new_path, platform : platform.clone(), version : version.clone() }
  }

  pub fn install(&mut self) -> bool {
    //! installs the binary, returns true if install successful or already exists, false if it fails.

    // checks if the path exists, then assumes that it is already installed.
    if self.path.is_some() { return true; }

    // tries to install it.
    if let Ok(path) = install::install(&self.platform,&self.version) {
      self.path = Some(path);
      return true; 
    }

    // if we are here then it couldn't install.
    false
  }

  pub fn run(&mut self, package_path : Option<PathBuf>) -> bool {
    //! runs the binary

    if self.path.is_none() { self.install(); }
    match self.path {
      Some(ref path) => {
        let mut command = Command::new(&path);
        if let Some(package_path) = package_path { command.arg(package_path); }
        match command.spawn() {
          Err(error) => { output_debug!("Error running LOVE: {}",Red.paint(error.to_string())); return false; }
          Ok(_) => { output_debug!("LOVE ran successfully"); return true; }
        }
      },
      None => { output_debug!("Error running LOVE: {}",Red.paint("no path found")); return false; }
    }
  }

}

fn build_path(platform : &Platform, version : &Version) -> PathBuf {
  //! generates the path to the binary, used for executing.

  let mut path = match lpsettings::get_settings_folder() {
    Ok(path) => { path },
    Err(_) => { PathBuf::from(".") }
  };

  path.push(lpsettings::get_value_or("run.binaries-root","bin"));
  path.push(platform.to_short_string());

  if platform == &Platform::Win32 || platform == &Platform::Win64 { 
    path.push(version.to_string());
    path.push("love.exe"); 
  } else if platform == &Platform::Nix32 || platform == &Platform::Nix64 { 
    path.push(version.to_string());
    path.push("love"); 
  } 

  path
}