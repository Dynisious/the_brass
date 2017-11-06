//! `file_system` defines interactions between the game and the file system. As well as
//! useful constants and statics for interacting with the file system.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/10/17

pub use std::path::Path;
use std::io::{Write, Read};
pub use std::io::Error;
use std::ffi::OsString;


/// A pointer to the String representation in memory of the working directory of this
/// instance of the game. Initially is a null pointer as no directory has been set.
static mut WORKING_DIR_STR: *const String = 0 as *const String;

/// A function to attempt to get a copy of the String pointed to `WORKING_DIR_STR`.
/// Simply returns `None` as `WORKING_DIR_STR` starts uninitialised.
static mut WORKING_DIR_GET: fn() -> Option<String> = || None;

/// An unsafe function to attempt to set the working directory used by this instance of
/// the game. This function is designed to only be called once and as such all but the
/// first call to `set_working_dir` will return `false`.
pub unsafe fn set_working_dir(mut dir: String) -> bool {
    if WORKING_DIR_STR == 0 as *const String {
        dir.shrink_to_fit();
        
        WORKING_DIR_STR = Box::into_raw(Box::new(dir));
        
        WORKING_DIR_GET = || {
            Some((*WORKING_DIR_STR).clone())
        };
        
        true
    } else {
        false
    }
}

/// A public interface to call `WORKING_DIR_GET` and return the result.
pub fn working_dir() -> Option<String> {
    unsafe {
        WORKING_DIR_GET()
    }
}

/// `read_file` attempts to read the file specified by `file_path` into `buffer` and
/// returns any Error that occours.
///
/// #Params
///
/// file_path --- The path to the file to be read in.
/// buffer --- The `String` buffer to read the file into.
pub fn read_file(file_path: &Path, buffer: &mut String) -> Result<(), Error> {
        ::std::fs::File::open(file_path)?.read_to_string(buffer)?;
        Ok(())
    }

/// `write_file` attempts to write the passed bytes to the file specified by `file_path`
/// and returns the result.
///
/// #Params
///
/// file_path --- The path to the file to be read in.
/// buffer --- The `String` buffer to read the file into.
pub fn write_file(file_path: &Path, bytes: &[u8]) -> Result<(), Error> {
    match ::std::fs::File::open(file_path) {
        Ok(mut file) => file.write_all(bytes),
        Err(e) => Err(e)
    }
}

/// A `FileInterface` is something which can be encoded as a file in the file system.
pub trait FileInterface {
    /// The type returned from a call to `from_file`.
    type Output: Sized;
    /// The type returned from a call to `from_file`.
    type Error: Sized;
    
    /// Attempts to create an instance of `Output` from the contents of a `File`.
    ///
    /// #Params
    ///
    /// file_path --- The path to the file to read in.
    fn from_file(file_path: &Path) -> ::std::result::Result<Self::Output, Self::Error>;
    /// Writes the `FileInterface` to the file specified by `file_path` and returns the `Result`.
    ///
    /// #Params
    ///
    /// file_path --- The path to the file to write too.
    fn write_file(&self, file_path: &Path) -> Result<(), Self::Error>;
}

/// An `ExternalResources` is something which is intended to be stored and retrieved from
/// a particular place in the working directory on file system.
pub trait ExternalResources {
    /// Returns all the relative directory paths from the working directory to the
    /// resources needed by this implementation.
    fn relative_dirs() -> &'static [&'static str];
    /// Creates a factory function which appends the passed `Path` to the relative
    /// directory selected by `index` from `Self::relative_dirs`.
    ///
    /// #Params
    ///
    /// index --- The index within 
    fn relative_path(index: usize) -> Option<Box<Fn(&Path) -> OsString>> {
        if index < Self::relative_dirs().len() {
            let root = Path::new(
                Self::relative_dirs()[index]
            );
            Some(
                Box::new(
                    move |path| {
                        root.join(path)
                        .into_os_string()
                    }
                )
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_working_dir() {
        assert!(working_dir().is_none(), "`test_working_dir` Failed to return `None` for uninitialised working directory.");
        
        unsafe {
            assert!(set_working_dir(String::from(".\\Home")), "`test_working_dir` Failed to set working directory.");
        
            assert!(!set_working_dir(String::from(".\\blah")), "`test_working_dir` Failed to fail on second set of working directory.");
        }
        
        assert!(working_dir() == Some(String::from(".\\Home")), "`test_working_dir` Failed to return correct value for initialised working directory.");
    }
}
