//! includecargotomldatamod.rs

use crate::utilsmod::*;

use ansi_term::Colour::{Green, Yellow};
use chrono::{Datelike, Utc};
use glob::glob;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use unwrap::unwrap;

struct CargoTomlData {
    version: String,
    authors: String,
    description: String,
}

pub fn increment_patch() {
    println!("pub fn increment_patch");
}

pub fn increment_minor() {
    println!("pub fn increment_patch");
}
