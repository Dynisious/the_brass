//! `properties` defines `Properties`, which are pairs of `String`s, their construction
//! and modification.
//!
//! #Last Modified
//!
//! Author: Daniel Bechaz</br>
//! Date: 2017/09/22

use std::iter::FromIterator;
use std::collections::HashMap;
use game::file_system::{self, FileInterface, Error, Path};

pub type Property = (String, String);
pub type PropertyPairs = HashMap<String, String>;

/// A `Properties` is a collection of String, String pairs which can be read from or
/// written too a file.
pub struct Properties {
    /// The collection of `String`, `String` pairs of the `Properties`.
    pub props: PropertyPairs
}

impl Properties {
    /// Reads properties in from a properly formated text file with key value pairs on
    /// each line seperated by " : ".
    ///
    /// #Params
    ///
    /// file_path --- The path to the text file to read in.
    pub fn read_file(&mut self, file_path: &Path) -> Result<(), Error> {
        let mut props = Self::from_file(file_path)?.props;
        self.absorb(
            props.drain()
        );
        Ok(())
    }
    /// Adds all properties from the passed iterator to the `Properties` instance,
    /// overwritting existing mappings for properties.
    ///
    /// #Params
    ///
    /// iter --- The iterator to get properties from.
    pub fn absorb<T>(&mut self, iter: T)
        where T: Iterator<Item = Property> {
        for (key, value) in iter {
            self.props.insert(key, value);
        }
    }
}

impl FileInterface for Properties {
    type Output = Self;
    type Error = Error;
    
    fn from_file(file_path: &Path) -> Result<Self::Output, Self::Error> {
        let mut buffer = String::new();
        
        file_system::read_file(file_path, &mut buffer)?;
        
        Ok(Self::from(buffer))
    }
    /// Writes the `Properties` to the text file specified by `file_path` and returns the
    /// result.
    ///
    /// #Params
    ///
    /// file_path --- The path to the text file to write too.
    fn write_file(&self, file_path: &Path) -> Result<(), Self::Error> {
        let mut buffer = String::new();
        
        for (key, value) in self.props.iter() {
            buffer += key;
            buffer.push_str(" : ");
            buffer += value;
            buffer.push_str("\n");
        }
        
        file_system::write_file(file_path, buffer.as_bytes())
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self::from(
            HashMap::default()
        )
    }
}

impl From<String> for Properties {
    fn from(props: String) -> Self {
        props.lines()
        .filter_map(
            |line| {
                let parts: Vec<&str> = line.split(" : ").collect();
                
                if parts.len() != 0 {
                    Some(
                        if parts.len() == 1 {
                            (String::from(parts[0]), String::from(""))
                        } else {
                            (String::from(parts[0]), String::from(parts[1]))
                        }
                    )
                } else {
                    None
                }
            }
        ).collect()
    }
}

impl FromIterator<(String, String)> for Properties {
    fn from_iter<T>(props: T) -> Self
        where T: IntoIterator<Item = (String, String)> {
        Self::from(
            props.into_iter().collect::<PropertyPairs>()
        )
    }
}

impl From<PropertyPairs> for Properties {
    fn from(props: PropertyPairs) -> Self {
        Self {
            props
        }
    }
}

impl Into<PropertyPairs> for Properties {
    fn into(self) -> PropertyPairs {
        self.props
    }
}

impl AsRef<Properties> for PropertyPairs {
    fn as_ref(&self) -> &Properties {
        unsafe {
            &*(self as *const PropertyPairs as *const Properties)
        }
    }
}

impl AsRef<PropertyPairs> for Properties {
    fn as_ref(&self) -> &PropertyPairs {
        &self.props
    }
}
