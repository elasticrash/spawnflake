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