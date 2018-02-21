//! tools for working with love binaries

use version::version::Version;
use platform::Platform;
use std::path::PathBuf;

use std::process::Command;

use lpsettings;
use binary::install;

pub struct Binary {
  pub path : Option<PathBuf>,
  pub platform : Platform,
  pub version : Version
}

impl Binary {
  pub fn new(platform : &Platform, version : &Version, path : Option<PathBuf>) -> Binary {
    
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

  pub fn install(&mut self) -> Result<(),&'static str> {
    if self.path.is_some() { return Ok( () ); }

    match install::install(&self.platform,&self.version) {
      Ok(path) => { 
        self.path = Some(path);
        return Ok( () ); 
      }
      Err(error) => { Err(error) }
    }
  }

  pub fn run(&mut self) -> Result<(),&'static str> {
    if self.path.is_none() { self.install(); }
    match self.path {
      Some(ref path) => {
        let mut command = Command::new(&path);
        // for slice in args { command.arg(slice); }

        match command.spawn() {
          Err(_) => { return Err("Error running LOVE"); }
          Ok(_) => {  return Ok( () ); }
        }
      }
      None => { Err("Can't run LOVE") }
    }
  }

}

fn build_path(platform : &Platform, version : &Version) -> PathBuf {
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
    path.push(format!("{}.appimage",version.to_string())); 
  } 

  path
}