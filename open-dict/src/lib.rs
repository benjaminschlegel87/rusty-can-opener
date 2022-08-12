#![cfg_attr(test, no_std)]
//! A simple no_std implementation of the structure of the CANopen Object dictionary
//! For now only Objects of Type i32,u32 and Domain are implemented as the choosen API/Implementation is ver much
//! experimental. More Types and changes will come over Time if the Dictionary is used by a actual CANopen Service
//! Implementation
//! # Example
//! ```
//! # use crate::open_dict::dictionary::Dictionary;
//! let dict = Dictionary::<5,5,5>::new();
//! ```
//!
/// Core Struct of the object that holds:
/// - The actual value of the object
/// - The lower/upper limits
/// - and the default value
pub mod data;
/// Actual dictionary implementation as a LinearMap of Type (CANopen Index, Object)
pub mod dictionary;
/// Structure of the CANopen Index: Used to access objects in the Dictionary API
pub mod index;
/// Structure of a CANopen Object
/// A Object holds
/// - the actual data  
/// - The access rights to this object  
/// Not implemented: PDO Mapping, Stringly Names,
pub mod object;
#[cfg(test)]
mod tests {}
