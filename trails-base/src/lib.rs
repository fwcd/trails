mod constants;

pub use constants::*;

pub use indoc;
pub use log;
pub use regex;
pub use once_cell;

pub type Result<T> = anyhow::Result<T>;
pub type Error = anyhow::Error;
pub use anyhow::bail;
