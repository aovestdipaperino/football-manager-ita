//! C64 BASIC V2 interpreter with a PETSCII screen model.
//!
//! The binary in `main.rs` drives this library with a crossterm event loop;
//! integration tests drive it headlessly and compare screen dumps against
//! fixtures captured from the VICE emulator's screen memory.

pub mod interp;
pub mod lang;
pub mod petscii;
pub mod screen;
