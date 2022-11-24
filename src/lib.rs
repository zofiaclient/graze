//! `graze` is a zero-boilerplate configuration library.
//!
//! `graze` itself does not use [serde](https://crates.io/crates/serde) as a dependency, but can
//! easily be used alongside the `serde` ecosystem.
//!
//! # Functions
//!
//! - [load_from_path]
//! - [load_or_default]
//! - [load_or_write_default]
//!
//! # Examples
//!
//! ## Load a configuration using the [toml](https://crates.io/crates/toml) crate
//!
//! ```
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     message: String
//! }
//!
//! let config: Config = graze::load_from_path("Config.toml", |c| toml::from_str(c))
//!     .expect("Could not load configuration");
//!
//! println!("{}", config.message);
//! ```

use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use std::{fmt, fs, io};

use thiserror::Error;

#[cfg(test)]
mod tests;

/// The error type returned by functions which return a [Result].
#[derive(Error)]
pub enum ConfigurationError<E> {
    /// An IO error occurred.
    Io(#[from] io::Error),

    /// The deserializer returned an error.
    Deserialize(E),
}

impl<E> Display for ConfigurationError<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => {
                write!(
                    f,
                    "An error occurred while opening the configuration file: {err}"
                )
            }
            Self::Deserialize(err) => {
                write!(f, "Configuration file is incorrect: {err}")
            }
        }
    }
}

impl<E> Debug for ConfigurationError<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "Io({err})"),
            Self::Deserialize(err) => write!(f, "Deserialize({err}"),
        }
    }
}

pub type Result<T, E> = std::result::Result<T, ConfigurationError<E>>;

/// Load a configuration from the file at the given path.
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config {
///     message: String
/// }
///
///  let config: Config = graze::load_from_path("Config.toml", |c| toml::from_str(c))
///     .expect("Could not load configuration");
///
///  println!("{}", config.message);
/// ```
pub fn load_from_path<P, T, E, D>(path: P, deserializer: D) -> Result<T, E>
where
    P: AsRef<Path>,
    D: FnOnce(&str) -> std::result::Result<T, E>,
{
    let content = fs::read_to_string(path)?;
    deserializer(&content).map_err(|e| ConfigurationError::Deserialize(e))
}

/// Load a configuration from the file at the given path, or use the default value if the file does
/// not exist.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Default)]
/// struct Config {
///     message: String
/// }
///
/// let config = graze::load_or_default("Config.toml", |s| toml::from_str(s), Config::default);
/// ```
pub fn load_or_default<P, T, E, D, F>(path: P, deserializer: D, default: F) -> Result<T, E>
where
    P: AsRef<Path>,
    D: FnOnce(&str) -> std::result::Result<T, E>,
    F: FnOnce() -> T,
{
    let path = path.as_ref();

    if path.exists() {
        return load_from_path(path, deserializer);
    }
    Ok(default())
}

/// Load a configuration from the file at the given path, or use the default value if the file does
/// not exist.
///
/// If the file does not exist, the default value will be written to the file at the given path.
///
/// # Examples
///
/// ```
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct Config {
///     message: String
/// }
///
/// impl Default for Config {
///     fn default() -> Self {
///         Self { message: "Hello, world!".to_string() }
///     }
/// }
///
/// // If Config.toml exists, deserialize the content using toml::from_str.
/// //
/// // If the file does not exist, use the value returned from Config::default(), serialize the
/// // value, and save the serialized data to Config.toml.
/// let config = graze::load_or_write_default("Config.toml",
///     |s| toml::from_str(s),
///     |c| toml::to_string(&c).unwrap(),
///     Config::default
/// );
/// ```
pub fn load_or_write_default<P, T, E, D, S, F, B>(
    path: P,
    deserializer: D,
    serializer: S,
    default: F,
) -> Result<T, E>
where
    P: AsRef<Path>,
    D: FnOnce(&str) -> std::result::Result<T, E>,
    S: FnOnce(&T) -> B,
    B: AsRef<[u8]>,
    F: FnOnce() -> T,
{
    let path = path.as_ref();

    if path.exists() {
        return load_from_path(path, deserializer);
    }

    let data = default();
    fs::write(path, serializer(&data))?;

    Ok(data)
}
