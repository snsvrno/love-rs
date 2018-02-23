//! rust library for working with love and love projects

extern crate regex;
extern crate zip;
extern crate reqwest;
extern crate serde_json;
extern crate toml;
#[macro_use]
extern crate output;
extern crate ansi_term;

extern crate download;
extern crate platform;
extern crate version;
extern crate lpsettings;
extern crate archives;

pub mod project;
pub mod binary;