//! Recovered save-file codec, mutable shadow and incremental write consumers.
//! The original game-thread capture and asynchronous queue are not registered here yet.
pub mod save_backup_list;
pub mod save_backup_metronome;
pub mod save_game;
pub mod save_thread;
pub mod shadow_file;
pub mod snapshot_gc;

#[cfg(test)]
mod tests;
