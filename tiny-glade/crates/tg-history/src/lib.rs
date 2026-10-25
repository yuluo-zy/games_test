//! Read-only access to Tiny Glade history JSON and supported roof projections.
//!
//! Parsing validates the document structure once. Borrowed views preserve unknown
//! data and integer IDs; typed roof decoding is an explicit, fallible operation.
//! This crate does not implement world mutation, undo, redo, or replay.
#![doc = include_str!("../README.md")]

mod document;
mod error;
mod roof;
mod summary;

pub use document::{
    HistoryCommandKind, HistoryDocument, HistoryEntryView, HistorySnapshotReference,
    LocatedHistoryCommand,
};
pub use error::{DecodeError, DecodeErrorKind};
pub use roof::{
    LocatedRoofCommand, RidgeDirection, RoofCommand, RoofDirectionEdit, RoofParameters,
    RoofPresence, RoofVisualEdit,
};
pub use summary::{HistorySummary, summarize_json};

#[cfg(test)]
mod tests;
