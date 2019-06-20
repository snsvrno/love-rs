extern crate love;
extern crate version_lp;

#[test]
fn check_if_love_projects() {


    use love::project::Project;

    assert_eq!(
        love::project::get_type("tests/files/11.0.love").unwrap().unwrap(),
        Project::FileWith(version_lp::Version::new(&[11,0]))
    );

    assert_eq!(
        love::project::get_type("tests/files/0.10.2.love").unwrap().unwrap(),
        Project::FileWith(version_lp::Version::new(&[0,10,2]))
    );

    assert_eq!(
        love::project::get_type("tests/files/love_folder_0.10.2").unwrap().unwrap(),
        Project::FolderWith(version_lp::Version::new(&[0,10,2]))
        );

    assert_eq!(
        love::project::get_type("tests/files/not_a_love").unwrap(),
        None
    );

    assert_eq!(
        love::project::get_type("tests/files/love_folder_no_ver").unwrap().unwrap(),
        Project::Folder
    );

    assert_eq!(
        love::project::get_type("tests/files/love_no_ver.love").unwrap().unwrap(),
        Project::File
    );
}

#[test]
fn get_required_version_folder() {
    match love::project::get_version("tests/files/love_folder_0.10.2") {
        Err(error) => { println!("ERROR : {}", error); assert!(false); },
        Ok(Some(version)) => {
            assert_eq!(version,version_lp::Version::new(&[0,10,2]));
        },
        _ => { assert!(false); }
    }

    // this one is not a love project so it should error.
    assert!(love::project::get_version("tests/files/not_a_love").is_err());
}

#[test]
fn get_required_version_file() {
    match love::project::get_version("tests/files/0.10.2.love") {
        Err(error) => { println!("ERROR(0.10.2) : {}", error); assert!(false); },
        Ok(Some(version)) => {
            assert_eq!(version,version_lp::Version::new(&[0,10,2]));
        },
        _ => { assert!(false); }
    }

    match love::project::get_version("tests/files/11.0.love") {
        Err(error) => { println!("ERROR(11.0) : {}", error); assert!(false); },
        Ok(Some(version)) => {
            assert_eq!(version,version_lp::Version::new(&[11,0]));
        },
        _ => { assert!(false); }
    }
}