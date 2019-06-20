//! tools for working with love projects
//!
//! supports love project folders and love `.love` packages

use failure::{ Error, format_err };
use version_lp::Version;
use zip;
use regex::Regex;

use std::path::{Path,PathBuf};
use std::fs::File;
use std::io::{Cursor,Read};


/// enum containing all the variants of projects that exist
#[derive(Debug,Eq,PartialEq)]
pub enum Project {

    FileWith(Version),
    FolderWith(Version),

    // used when we can't determine the version to use
    File,
    Folder
}

pub fn get_version<P : AsRef<Path>>(project_path : P) -> Result<Option<Version>, Error> {
    //! returns the actual version of the love project. will return Option::None`
    //! if no version can be determined.
    
    let path = PathBuf::from(project_path.as_ref());

    match get_type(project_path)? {
        Some(Project::FileWith(version)) |
        Some(Project::FolderWith(version)) => Ok(Some(version)),
        Some(Project::File) |
        Some(Project::Folder) => Ok(None),
        _ => Err(format_err!("{:?} is not a love project",path)),
    }

}

pub fn get_type<P : AsRef<Path>>(project_path : P) -> Result<Option<Project>,Error> {
    //! returns the type of project, along with the version if it can be 
    //! determined. returns `Option::None` if the given path is actually
    //! not a love project.
    
    let path = PathBuf::from(project_path.as_ref());

    if is_love_package(&path) {
        let content = get_file_contents_from_archive(&path,"conf.lua")?;
        match get_version_from_file(&content) {
            Some(version) => return Ok(Some(Project::FileWith(version))),
            None => return Ok(Some(Project::File))
        }
    }

    if is_love_project_folder(&path) {
        let content = get_file_contents(&path,"conf.lua")?;

        match get_version_from_file(&content) {
            Some(version) => return Ok(Some(Project::FolderWith(version))),
            None => return Ok(Some(Project::Folder))
        }
    }

    Ok(None)
}

// PRIVATE FUNCTIONS ///////////////////////

fn get_version_from_file(file_content : &str) -> Option<Version> {
    let re_version_assignment = Regex::new(r#"version *= *["|'](.*)["|']"#).unwrap();

    if let Some(version) = re_version_assignment.captures(file_content) {
        let captured_version = version.get(1).unwrap().as_str().to_string();
        if let Some(version) = Version::from_str(&captured_version) {
            return Some(version);
        }
    }

    None
}

fn is_love_package(project_path : &PathBuf) -> bool {
    //! checks if the path given is a love package or a folder
    //! essentially just checks the extension

    let string : String = project_path.display().to_string();
    let split : Vec<&str> = string.split(".").collect();

    if split[split.len()-1] == "love" { 
        true
    } else {
        false
    }
}

fn is_love_project_folder(project_path : &PathBuf) -> bool {
    //! checks if the given folder is a love project. it determines
    //! this if it has a 'main.lua' in the root (all that it can really 
    //! check for)
    
    let mut main_lua_path = project_path.clone();
    main_lua_path.push("main.lua");

    main_lua_path.exists()
}

fn get_file_contents(project_path : &PathBuf,file_path : &str) -> Result<String,Error> {
    //! gets the contents of the file from a folder.

    let mut buf : String = String::new();

    // does this section if it a .love file
    if is_love_package(project_path) {
        return get_file_contents_from_archive(&project_path,&file_path);
    // if this is a directory
    } else {
        let mut project_file_path = project_path.clone();
        project_file_path.push(file_path);

        // loads the contents of the file if found, else it returns an error!!!
        let mut project_file = File::open(project_file_path)?;
        project_file.read_to_string(&mut buf)?;
    }

    if buf.len() > 0 { 
        Ok(buf)
    } else {
        Err(format_err!("Failed to read file's contents ({} inside of {:?}", file_path, project_path))
    }
}

fn get_file_contents_from_archive(archive_path : &PathBuf, file_path_in_archive : &str) -> Result<String,Error> {
    //! gets the contents of a file that is in a .love package

    let mut content : String = "".to_string();
    let mut buf : Vec<u8> = Vec::new();

    let mut zip_file = File::open(&archive_path)?;
    zip_file.read_to_end(&mut buf)?;

    let mut archive = zip::ZipArchive::new(Cursor::new(buf))?;
    let mut file = archive.by_name(&file_path_in_archive)?;
    file.read_to_string(&mut content)?;

    if content.len() > 0 {
        Ok(content)
    } else {
        Err(format_err!("Failed to read file's contents"))
    }
}