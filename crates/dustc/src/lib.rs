//! Dust compiler library.
//!
//! The binary (`dustc`) is a thin wrapper around this crate so that integration
//! tests can exercise the lexer, parser, type checker, and backend directly.
#![allow(dead_code)] // front-end stages are added incrementally; silence early.

pub mod cli;
pub mod diagnostics;
pub mod driver;
pub mod source;
