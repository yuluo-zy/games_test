# Shared menu widget contract

Only evidence/staged source changes in this batch. Root owns integration and Cargo.

Original MenuButtonContent vtable `142baf6d0`, 0x260 bytes, alignment8.
See `menu-button-vtable.json` and its full bounded `menu-button-vtable-{slot}.asm`.

* TextFragment600 bytes at0; color u32 at258; `always_white` bool at25c; digital action at25d (28 absent).
* Constructor `get_content_mut_impl`140f9dba0,731B including cleanup, in `menu-button-content-init.asm`: TextFragment::new role1, metrics constant142a16e40, weight700; colorffffffa0, always_white=false, action28. Inspect the metric's four original bytes, don't assume48.
* interactive vtable40 leaf14003bb70 returns1. Input50 leaf140002370 returns0. update48 leaf14003bb80 ret.
* pre_update14067ea20 45B: if action!=28 and MainActionSet.just_pressed(action), OR state20.
* prepare14067e510 73B is **the same physical original LabelContent prepare**. Preserve the existing owning TextFragment and exact glyph layout path. Do not use a separate fake text renderer.
* draw14067ea50 95B: color white when state&1 or always_white, otherwise owncolor; scale by opacity and TextFragment render. Full draw/prepare evidence is in vtable68/60.

`NodeContext::menu_button`14068bfc0 670B, `menu-button.asm`: parameter text `&str` (rdx parent; r8/r9 text), not separate invented name/image. ID derives text hash + full native TypeId, whose highword is615bfcfe4597ab8f; childID from parent. Set TextFragment text, flags OR20, set_sounds(0,2). Native setter passes both halves into Rust TypeId::hash; prior highword-only hash implementation is the established source adapter and needs consistency review, not an independent new scheme.

Load UI wants `slot.menu_button(&localized_text)` returning retained NodeContext, exposing `MenuButtonContent::{text,color,always_white,action}` and pending-text staging compatible with renderer-owned set_text. Consumers modify color7b5a51a0; theme current colorc7b7a7ff. Action18 used by cancel/No.

Also wants general `NodeContext::pill_button(name,image)` (native140fa0130 950B), preserving current PillButtonContent all functions. `back_button` is currently hardcoded menu/back; contextual callers need delete_button/menu/delete, switch_glade/menu/switch_glade, view_backups/menu/glade_backups. Strength.6; size65/60 depending parenthover. Existing PillButtonContent is already full producer/prepare/draw/pre_update; don't duplicate it.

Load also wants `NodeContext::label(&str)` with the existing concrete LabelContent identity; backup pagination dynamic text uses Label, size native142a16e40 and weight700, color7b5a51a0. The caller already has child_with_content for named labels, but dynamic String original has TypeId distinct from &str. Native label child derives (hash(text),type highwordce69dbf0aee13da2), then parent. Pagination full native1126B is `backup-pagination.asm`.

All bounded bytes are original RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. No production source applied, no Cargo run.
