use version::version::Version;
use platform::Platform;
use std::path::PathBuf;

use download;
use binary::repo;

pub fn install (platform : &Platform, version : &Version) -> Result<PathBuf,&'static str> {
  //! installs the version of LOVE

  match repo::get_link_to_version(&platform,&version) {
    Err(error) => { return Err(error); }
    Ok(link) => { 
      match download::download(link) {
        Err(_) => { return Err("Could not download"); }
        Ok(path) => { 
          match install_raw_file(&path.0) {
            Err(error) => { return Err(error); }
            Ok(installed_dir_path) => { return Ok(installed_dir_path); }
          }
        }
      }
    }
  }
}

fn install_raw_file(path : &PathBuf) -> Result<PathBuf,&'static str> {
  //! does the OS specific install instructions for the different types of OS binaries, exes, folder, appimages, etc ...
  //! returns the path where it installed it to.

  Err("Not implemeneted")
}