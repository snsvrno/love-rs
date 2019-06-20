use std::path::{Path,PathBuf};

use failure::Error;
use std::fs::File;

pub struct PackageOptions {
	
}

pub fn package<P : AsRef<Path>>(src : P, dest : P, options : PackageOptions) -> Result<(), Error> {
	let src_path = PathBuf::from(src.as_ref());
	let dest_path = PathBuf::from(dest.as_ref());

	let mut file = File::create(dest_path)?;
	let mut zip = zip::ZipWriter::new(file);
	let zip_options = zip::write::FileOptions::default();

	load_files_into_archive(&mut zip, &zip_options, &src_path, &src_path)?;

	zip.finish()?;

	Ok(())
}

// PRIVATE

fn load_files_into_archive(
	//! loads files into the provided archive.
	//! 
	//! it will recursively call itself once it finds a folder and iterate
	//! through all subfolders to add to the archive.
	
	zip : &mut zip::ZipWriter<File>, 
	zip_options : &zip::write::FileOptions, 
	src : &PathBuf, 
	root : &PathBuf ) -> Result<(),Error> {
	
	use std::io::{ Write, Read };
	use std::fs::read_dir;

	for entry in read_dir(src)? {
		let entry = entry?;
		let path = entry.path();

		match path.is_dir() {
			true => load_files_into_archive(zip, zip_options, &path, root)?,
			false => {
				let short_file_path : &Path = path.strip_prefix(root)?;

				zip.start_file(short_file_path.display().to_string(), *zip_options)?;

				let mut file : File = File::open(path)?;
				let mut bytes : Vec<u8> = Vec::new();
				file.read_to_end(&mut bytes)?;

				zip.write(&bytes)?; 
			}
		}
	}

	Ok(())
}