extern crate love;
extern crate version_lp;

#[test]
fn check_if_love_projects() {
    use love::project::Project;

    assert_eq!(
        love::project::project_type("tests/files/11.0.love").unwrap(),
        Project::File(version_lp::Version::new(&[11,0]))
    );

    assert_eq!(
        love::project::project_type("tests/files/0.10.2.love").unwrap(),
        Project::File(version_lp::Version::new(&[0,10,2]))
    );

    assert_eq!(
        love::project::project_type("tests/files/love_folder_0.10.2").unwrap(),
        Project::Folder(version_lp::Version::new(&[0,10,2]))
        );

    assert_eq!(
        love::project::project_type("tests/files/not_a_love").unwrap(),
        Project::None
    );
}

#[test]
fn get_required_version_folder() {
    match love::project::get_required_version("tests/files/love_folder_0.10.2") {
        Err(error) => { println!("ERROR : {}", error); assert!(false); },
        Ok(version) => {
            assert_eq!(version,version_lp::Version::new(&[0,10,2]));
        }
    }
}

#[test]
fn get_required_version_file() {
    match love::project::get_required_version("tests/files/0.10.2.love") {
        Err(error) => { println!("ERROR(0.10.2) : {}", error); assert!(false); },
        Ok(version) => {
            assert_eq!(version,version_lp::Version::new(&[0,10,2]));
        }
    }

    match love::project::get_required_version("tests/files/11.0.love") {
        Err(error) => { println!("ERROR(11.0) : {}", error); assert!(false); },
        Ok(version) => {
            assert_eq!(version,version_lp::Version::new(&[11,0]));
        }
    }
}