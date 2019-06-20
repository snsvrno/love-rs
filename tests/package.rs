extern crate love;

#[test]
fn package_folder() {
	love::package("tests/files/love_folder_0.10.2","tests/files/game.love",love::PackageOptions { });
}