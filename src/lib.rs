//! # Spawnflake
//!
//! Spawnflake is a schema agnostic, random and/or patterns based data generator, for MySQL databases.
//!
//! This library works by providing a configuration file (config.json). The configuration is divided into two sections.
//! * mysql configuration (mysql_configuration)
//! * type patterns (types)
//!
//! The types that can be currently defined are string, integer and float
//!
//! A type is defined as follows:
//!
//! ```json
//! {
//!     "name": "column_name",
//!     "rules": []
//! }
//! ```
//!
//! A lack of a pattern type will result in a random value being generated.
//!
//! See the example configuration file for more information

/// This module allows you to configure your generators
/// * The optional reader
/// * The model for setting up your project configuration
pub mod configuration;

/// Database functionality
pub mod datastores;
/// generates name based on patterns
pub mod name_generator;

/// generates random numbers
pub mod number_generator;

/// generates random strings
pub mod string_generator;

/// generates random dates from 1970 to now
pub mod date_generator;

/// generates bytes
pub mod byte_generator;
