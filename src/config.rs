/**************************** config.rs ***********************
********** Configure various options for the game *************
 **************************************************************/
use std::path::*;

 // Configure root path based on ship state
#[cfg(feature = "ship")]
pub fn root() -> Path {
    std::os::self_exe_path().unwrap()
}

#[cfg(not(feature = "ship"))]
pub fn root() -> PathBuf {
    PathBuf::from("./")
}
