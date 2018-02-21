//! tools for working with love projects
//!
//! supports love project folders and love `.love` packages

use version::version::Version;

use regex::Regex;
use zip;

use std::path::PathBuf;
use std::fs::File;
use std::io::{Cursor,Read};

pub fn get_required_version(path : &PathBuf) -> Result<Version,&'static str> {
  //! will look inside the `conf.lua` file for a project or `.love` file to determine what version of love should be used
  
  let conf_file = match is_love_package(path) {
    true => { get_file_contents_from_archive(&path,"conf.lua") }
    false => { get_file_contents(&path,"conf.lua") }
  };

  match conf_file {

    Err(error) => { return Err(error); }
    Ok(content) => {
      let re_version_assignment = Regex::new(r#"version *= *["|'](\d+\.\d+\.\d+)["|']"#).unwrap();
      if let Some(version) = re_version_assignment.captures(&content) {
        let captured_version = version.get(1).unwrap().as_str().to_string();
        let version = Version::from_str(&captured_version);
        match version {
          None => { return Err("Version failed to parse."); }
          Some(ver) => { return Ok(ver); }
        }
      }
    }

  }

  Err("Failed get_required_version function")
}

// PRIVATE FUNCTIONS ///////////////////////

fn is_love_package(project_path : &PathBuf) -> bool {
  //! checks if the path given is a love package or a folder
  //! essentially just checks the extension

  let string : String = project_path.display().to_string();
  let split : Vec<&str> = string.split(".").collect();

  if split[split.len()-1] == "love" { return true; }

  false
}

fn get_file_contents(project_path : &PathBuf,file_path : &str) -> Result<String,&'static str> {
  //! gets the contents of the file from a folder.

  let default_err = Err("File does not exist inside project.");

  let mut buf : String = String::new();

  // does this section if it a .love file
  if is_love_package(project_path) {
    return get_file_contents_from_archive(&project_path,&file_path);
  // if this is a directory
  } else {
    let mut project_file_path = project_path.clone();
    project_file_path.push(file_path);

    // loads the contents of the file if found, else it returns an error!!!
    if let Ok(mut project_file) = File::open(project_file_path) { project_file.read_to_string(&mut buf);
    } else { return default_err; }
  }

  if buf.len() > 0 { return Ok(buf); }

  default_err
}

fn get_file_contents_from_archive(archive_path : &PathBuf, file_path_in_archive : &str) -> Result<String,&'static str> {
  //! gets the contents of a file that is in a .love package

  let default_err = Err("File does not exist inside of archive.");

  let mut content : String = "".to_string();
  let mut buf : Vec<u8> = Vec::new();
  let zip_file = File::open(&archive_path);

  match zip_file {
    Err(error) => { return Err("Zipfile cannot be opened."); }
    Ok(mut zip_file) => { 
      match zip_file.read_to_end(&mut buf){
        Err(error) => { return Err("Failed to read zipfile buffer."); },
        Ok (_) => { }
      }
    }
  }

  let archive = zip::ZipArchive::new(Cursor::new(buf));
  match archive {
    Err(error) => { return Err("Can not reading archive's stream buffer"); }
    Ok(mut archive) => {
      match archive.by_name(&file_path_in_archive) {
        Err(_) => { return default_err; }
        Ok(mut file) => {
          file.read_to_string(&mut content);
        }
      }
    }
  }

  if content.len() > 0 { return Ok(content); }

  default_err
}