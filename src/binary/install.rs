use version::version::Version;
use std::fs::create_dir_all;
use platform::Platform;
use std::path::PathBuf;

use lpsettings;
use archives;
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
          match install_raw_file(&path.0,&platform,&version) {
            Err(error) => { return Err(error); }
            Ok(installed_dir_path) => { return Ok(installed_dir_path); }
          }
        }
      }
    }
  }
}

fn install_raw_file(path : &PathBuf, platform : &Platform, version : &Version) -> Result<PathBuf,&'static str> {
  //! does the OS specific install instructions for the different types of OS binaries, exes, folder, appimages, etc ...
  //! returns the path where it installed it to.
  output_debug!("Processing {}",&path.display().to_string());

  match platform {
    &Platform::Nix64 | &Platform::Nix32 => { return install_unix(&path,&platform,&version); },
    &Platform::Win64 | &Platform::Win32 => { return install_windows(&path,&platform,&version); },
    _ => { output_debug!("Processing not implemented for platform {}",&platform.display())}
  }

  Err("Not implemeneted")
}

fn install_windows(path : &PathBuf, platform : &Platform, version : &Version) -> Result<PathBuf,&'static str>  {
  Err("not implemneted")
}

fn install_unix(path : &PathBuf, platform : &Platform, version : &Version) -> Result<PathBuf,&'static str>  {
  if let Ok(mut dest_path) = lpsettings::get_settings_folder() {
    dest_path.push(lpsettings::get_value_or("bin.install_path","bin"));
    dest_path.push(platform.as_short_str());
    dest_path.push(version.to_string());

    //if dest_path.exists() {
    //  output_debug!("Extract path already exist, skipping extracting.");
    //  return(Ok(dest_path.clone()));
    //} else {
    //  output_debug!("Extract path doesn't exists, creating.");
    //  create_dir_all(&dest_path);
    //}

    match archives::extract_to(path,&dest_path) {
      Err(_) => { output_error!("Can't extract archive"); }
      Ok(dest_path) => { output_println!("need to set permissions here"); }
    }
  }

  Err("not implemneted")
}