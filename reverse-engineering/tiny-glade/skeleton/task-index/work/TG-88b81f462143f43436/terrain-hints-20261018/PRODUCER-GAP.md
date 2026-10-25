# Historical gap, now boundedly recovered

The initial inventory correctly found that ui_edit_focus.rs contained only the existing GeoSource conversion helpers and no selection producer. No helper was changed to reinterpret terrain GeoSource or emit events, and no AppModeChanged/TerrainFocusTrigger proxy was used. The missing original terrain-tool right-click branch has now been recovered incrementally in the same file; see CONTRACT.md and the native original-entry cases. Other original selection branches and production global mode-manager installation remain outside this bounded delivery.
