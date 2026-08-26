use std::sync::OnceLock;

use crate::helpers::Config;

pub mod common;
pub mod entities;
pub mod helpers;
pub mod route;
pub mod schema;

pub static CONFIG: OnceLock<Config> = OnceLock::new();
