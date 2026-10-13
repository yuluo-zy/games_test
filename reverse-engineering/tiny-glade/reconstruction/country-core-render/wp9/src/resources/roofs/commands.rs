//! commands.rs — 屋顶命令（WP9）。
//!
//! PDB 骨架 `resources/roofs/commands.rs`。命名实体：`CreateRoofCmd` / `DeleteRoofCmd` /
//! `EditRoofCmd` / `HighlightRoofCmd` / `RecalculateRoofCmd` / `SetFlatRoofRimStyleCmd` /
//! `SwitchRoofTypeCmd`（+ `fmt` impls）。值布局 U（与 system_roof 命令/事件桥接）。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

#[derive(Clone, Debug)]
pub struct CreateRoofCmd(pub Gap);
#[derive(Clone, Debug)]
pub struct DeleteRoofCmd(pub Gap);
#[derive(Clone, Debug)]
pub struct EditRoofCmd(pub Gap);
#[derive(Clone, Debug)]
pub struct HighlightRoofCmd(pub Gap);
#[derive(Clone, Debug)]
pub struct RecalculateRoofCmd(pub Gap);
#[derive(Clone, Debug)]
pub struct SetFlatRoofRimStyleCmd(pub Gap);
#[derive(Clone, Debug)]
pub struct SwitchRoofTypeCmd(pub Gap);
