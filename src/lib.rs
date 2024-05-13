//! # rk-utils
//! 
//! A collection of utility functions and data structures.
//! 
//! ## Features
//! 
//! - `str`: String utilities
//! - `topo_sort`: Topological sorting
//! - `trie`: Trie data structure for longest matching a path of nodes
//! 
//! ## Usage
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! rk-utils = "0.1"
//! ```
//! 
//! ## Examples
//! 
//! ```rust
//! use rk_utils::StringUtil;
//! 
//! let s = "'Hello, World!'";
//! assert_eq!(s.is_quoted(), true);
//! 
//! let s = "'Hello, World!'";
//! assert_eq!(s.unquote(true, None), "Hello, World!");
//! 
//! let s = "'Hello, World!'";
//! assert_eq!(s.substring(1, -1), "Hello, World!");
//! ```
//!  
//! ## License
//! 
//! MIT

mod str;
mod topo_sort;
mod trie;

pub use crate::str::StringUtil;
pub use crate::topo_sort::{ topo_sort, DepGraph };
pub use crate::trie::Trie;

#[macro_export]
macro_rules! to_matches {
    // to_matches!(Enum::variant(val) if val > 0)    
    ($pattern:pat $(if $guard:expr)? $(,)?) => {
        |ttt| match ttt {
            $pattern $(if $guard)? => true,
            _ => false
        }
    };
}

#[macro_export]
macro_rules! e_value {
    // e_value!(enum_value, Enum::variant)
    ($enum:expr, $variant:pat) => {
        match $enum {
            $variant(val) => Some(val),
            _ => None,
        }
    };
    // e_value!(enum_value, Enum::variant if val > 0)
    ($enum:expr, $variant:pat $(if $guard:expr)? $(,)?) => {
        match $enum {
            $variant(val) $(if $guard)? => Some(val),
            _ => None,
        }
    };
}

#[cfg(test)]
mod tests;