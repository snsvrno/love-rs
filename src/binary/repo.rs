use version::version::Version;
use platform::Platform;
use std::path::PathBuf;

use std::fs::{File,metadata};
use std::io::prelude::*;
use std::collections::HashMap;

use lpsettings;

use toml;
use reqwest;
use serde_json;
use regex::Regex;

type Repo = HashMap<Version,HashMap<Platform,String>>;

static REPO_FILE : &str = "repo.toml";

static REGEX_LINUX_64 : &str = r"(nix64|x86_64).*\.[A|a]pp[I|i]mage";
static REGEX_LINUX_32 : &str = r"(nix32|i686).*\.[A|a]pp[I|i]mage";
static REGEX_WINDOWS_64 : &str = r"win64.*\.zip";
static REGEX_WINDOWS_32 : &str = r"(win32|win-x86).*\.zip";
static REGEX_VERSION_MATCH : &str = r"(\d+[-|.|_]\d+[-|.|_]\d+)";

pub static REPOSITORY : [&str;2] = [
  "https://api.bitbucket.org/2.0/repositories/rude/love/downloads"
];



pub fn get_link_to_version(platform : &Platform, version : &Version) -> Result<String,&'static str> {
  //! returns the direct like to the version of love desired.

  match update_repo(false) {
    Ok(_) => { println!("Updating repo success!"); }
    Err(error) => { println!("{}",error);}
  }

  match load_local_repo() {
    Err(error) => { return Err(error); }
    Ok(repo) => { 
      println!("{:?}",&version);
      match repo.get(&version) {
        None => { return Err("Version not found"); }
        Some(ver_hash) => {
          println!("{:?}",&platform);
          match ver_hash.get(&platform) {
            None => { return Err("Platform not found for supplied version"); }
            Some(link) => { 
              println!("{}",&link);
              return Ok(link.to_string()); 
            }
          }
        }
      }
    }
  }
}

fn load_local_repo() -> Result<Repo,&'static str> {
  //! loads the repo data from the repo.toml file into a Repo data structure
  if let Ok(mut path) = lpsettings::get_settings_folder() {
    path.push(REPO_FILE);

    match File::open(&path) {
      Err(_) => { return Err("error"); }
      Ok(mut file) => { 
        let mut buffer : String = String::new();
        match file.read_to_string(&mut buffer) {
          Err(_) => { return Err("error"); }
          Ok(_) => { 
            if let Ok(repo) = toml::from_str(&buffer) { 
              return Ok(repo);
            }
          }
        }
      }
    }
  }
  
  Err("Not implemeneted")
}

fn update_repo(forced_update : bool) -> Result<(),&'static str> {
  //! checks sources and updates the repo
  //! force_update does the update even if it has been performed in the last X hours
  //! normally it will not attempt to redownload the repo information if the time frame hasn't pased
  //! which will be set by a parameter in the config called binary.repo_update_frequency and will default to 24 hours

  let mut repositories = repo_links();
  let mut collected_repo = HashMap::new();

  loop {
    match repositories.pop() {
      None => { break; }
      Some(repo_url) => {
        println!("{}",&repo_url);
        if let Some(additional_link) = process_bitbucket(&mut collected_repo,&repo_url) { repositories.push(additional_link); }
      }
    }
  }

  // if forced_update { return save_to_file(&collected_repo); }

  if let Ok(path) = get_repo_file() {
    if let Ok(dat) = metadata(&path) {
      if let Ok(time) = dat.modified() {
        if let Ok(time_diff) = time.elapsed() {
          if time_diff.as_secs() > (60*60*24) {
            println!("Updating repository...");
            return save_to_file(&collected_repo);
          } else {
          println!("Repository current.") ;
            return Ok(()); 
          }
        }
      }
    } else { return save_to_file(&collected_repo); }
  }

  Err("general error")
}

fn get_repo_file() -> Result<PathBuf,&'static str> {
  if let Ok(mut path) = lpsettings::get_settings_folder() {
    path.push(REPO_FILE);
    Ok(path)
  } else { return Err("Error getting repo path"); }
}

fn save_to_file(repo : &Repo) -> Result<(),&'static str> {
  if let Ok(path) = get_repo_file() {
    match File::create(&path) {
      Err(_) => { return Err("Error creating file"); }
      Ok(mut file) => {
        match toml::to_string(&repo) {
          Err(_) => { return Err("Error creating toml string"); }
          Ok(toml_string) => {
            match file.write(toml_string.as_bytes()) {
              Err(_) => { return Err("Error writing toml to file"); }
              Ok(_) => { return Ok(()); }
            }
          }
        }
      }
    }
  } else { return Err("Error getting repo path"); }
}

fn process_bitbucket(mut repo_obj : &mut Repo, url : &str) -> Option<String> {
  if !url.contains("bitbucket") { return None; }
  let mut additional_page : Option<String> = None;

  if let Ok(mut resp) = reqwest::get(url) {
    if let Ok(raw_json) = resp.text() {
      let json : Result<serde_json::Value,serde_json::Error> = serde_json::from_str(&raw_json);
      if let Ok(json) = json {
        // picks the next link in on the page
        if let Some(json_next) = json["next"].as_str(){ additional_page = Some(json_next.to_string()); }
        // process the values
        if let Some(json_releases) = json["values"].as_array() {
          let re_version = Regex::new(REGEX_VERSION_MATCH).unwrap();
          for download in json_releases {
            let mut version : Option<Version> = None;
            if let Some(version_capured) = re_version.captures(download["name"].as_str().unwrap()) {
              version = Version::from_str(version_capured.get(1).unwrap().as_str());
            }
            if let Some(version) = version {
              let link = download["links"]["self"]["href"].as_str().unwrap();
              println!("{}",download["name"].as_str().unwrap());
              let platform = get_platform(download["name"].as_str().unwrap());

              if platform != Platform::None {
                println!("{:?} {:?} = {}",&version,&platform,&link);
                add_to_releases(&mut repo_obj,link.to_string(),version,platform);
              }
            }
          }
        }
      }   
    }
  }

  additional_page
}

fn get_platform(name : &str) -> Platform {
  //! all supported bianries will be here.
  //!
  //! uses the regex to analyze and check against the available binaies so it can order them into their 
  //! respective releases and platforms
  if Regex::new(REGEX_WINDOWS_64).unwrap().is_match(name) { return Platform::Win64; }
  if Regex::new(REGEX_WINDOWS_32).unwrap().is_match(name) { return Platform::Win32; }
  if Regex::new(REGEX_LINUX_64).unwrap().is_match(name) { return Platform::Nix64; }
  if Regex::new(REGEX_LINUX_32).unwrap().is_match(name) { return Platform::Nix32; }

  Platform::None
}

fn add_to_releases(repo_obj : &mut Repo, url : String, version : Version, platform : Platform) {
  // adds an empty version if doesn't already exist
  if !repo_obj.contains_key(&version) { repo_obj.insert(version.clone(),HashMap::new()); }
  let mut new_hash : HashMap<Platform,String> = HashMap::new();
  if let Some(hash) = repo_obj.get(&version) { 
    for (key,value) in hash.iter() { new_hash.insert(key.clone(),value.to_string());}
  }
  new_hash.insert(platform.clone(),url.to_string());
  repo_obj.insert(version.clone(),new_hash);
}

fn repo_links() -> Vec<String> {
  //! gets the list of links to check, (1) will use the default ones unless told not to
  //! and (2) will load additional ones if they are available.
  let mut links : Vec<String> = Vec::new();

  // loads the default links. can be disabled by setting the option *install.use_default_repos* to "false"
  if lpsettings::get_value_or("install.use_default_repos","true") == "true" {
    for default_repo in REPOSITORY.iter() { links.push(default_repo.to_string()); }
  }

  if lpsettings::has_value("install.custom_repo") {
    if let Some(values) = lpsettings::get_value("install.custom_repo") { links.push(values); } 
  }

  links
}
