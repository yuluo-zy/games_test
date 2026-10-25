# utils/dirty_tiles.rs

> 文件作用：**<待填>**

函数总计 39：业务 27 / 内联无独立符号 11 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `utils::dirty_tiles::DirtyTiles::clear` | 0x140960c20 | 0 | <待填> | U | — |
| `utils::dirty_tiles::DirtyTiles::extend` | 0x140960c30 | 0 | <待填> | U | — |
| `utils::dirty_tiles::impl$2::apply_to` | 0x140960ef0 | 0 | <待填> | U | — |
| `utils::dirty_tiles::DirtyTiles::new_clear` | 0x140960ac0 | 26 | <待填> | U | — |
| `resources::raster_stroke::impl$2::iter_points_xz::closure_env$0> >` | 0x140ade620 | 32 | <待填> | U | — |
| `resources::raster_stroke::impl$3::iter_points_xz::closure_env$0> >` | 0x140ade620 | 32 | <待填> | U | — |
| `systems::history::replay_path::Cobblestone1PathFilter> > > >` | 0x140ade620 | 32 | <待填> | U | — |
| `systems::history::replay_path::Cobblestone2PathFilter> > > >` | 0x140ade620 | 32 | <待填> | U | — |
| `systems::history::replay_path::Cobblestone3PathFilter> > > >` | 0x140ade620 | 32 | <待填> | U | — |
| `systems::history::replay_garden::replay_garden::closure_env$2> > >` | 0x140ade620 | 32 | <待填> | U | — |
| `systems::history::replay_path::DirtPathFilter> > > >` | 0x140ade620 | 32 | <待填> | U | — |
| `systems::history::replay_water::replay_water::closure_env$2> > >` | 0x140ade620 | 32 | <待填> | U | — |
| `utils::dirty_tiles::impl$0::intersects_world_space_rect::closure_env$0>` | 0x140adec50 | 87 | <待填> | U | — |
| `utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure_env$0>` | 0x140ade8e0 | 87 | <待填> | U | — |
| `utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::impl$1::add_region_to_lazy_grid::closure_env$0>` | 0x141cf3c00 | 87 | <待填> | U | — |
| `utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::impl$1::remove_region_from_lazy_grid::closure_env$0>` | 0x141cf3540 | 87 | <待填> | U | — |
| `utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::read_bush_positions_from_gpu::impl$0::bushes_near_point::closure_env$0<system_clutter::autoclutter::populate_autoclutter_regions::filter_clutter_object::closure_env$0> >` | 0x141cf3900 | 87 | <待填> | U | — |
| `utils::dirty_tiles::DirtyTiles::iter_world_space_rect_cells<system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure_env$1>` | 0x141cf3ee0 | 87 | <待填> | U | — |
| `utils::dirty_tiles::DirtyTiles::rasterize_world_space_rect` | 0x140960ae0 | 119 | <待填> | U | — |
| `utils::dirty_tiles::DirtyTiles::intersects_world_space_rect` | 0x140960af0 | 127 | <待填> | U | — |
| `utils::dirty_tiles::DirtyTiles::cell_from_world_space` | 0x140960b20 | 141 | <待填> | U | — |
| `systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>` | 0x140adeea0 | 160 | <待填> | U | — |
| `systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>` | 0x140adf4b0 | 160 | <待填> | U | — |
| `utils::dirty_tiles::DirtyTiles::iter_tiles<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assembly_propagate_dirty_flags::closure_env$2>` | 0x1412945c0 | 160 | <待填> | U | — |
| `resources::world_raster::impl$2::copy_to_staging::closure_env$0<512> >` | 0x140adfcb0 | 172 | <待填> | U | — |
| `systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>` | 0x141458b20 | 320 | <待填> | U | — |
| `systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>` | 0x141458a50 | 320 | <待填> | U | — |

## 内联函数名称推断

- `utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure$0` (L37, inline_instance, inlinee_declaration)
- `utils::dirty_tiles::DirtyTiles::iter_world_space_stroke_cells` (L43, inline_instance, inlinee_declaration)
- `utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure$0` (L69, inline_instance, inlinee_declaration)
- `utils::dirty_tiles::impl$0::rasterize_world_space_rect::closure$0` (L120, inline_instance, inlinee_declaration)
- `utils::dirty_tiles::impl$0::intersects_world_space_rect::closure$0` (L129, inline_instance, inlinee_declaration)
- `utils::dirty_tiles::DirtyTiles::iter_tiles` (L160, inline_instance, inlinee_declaration)
- `utils::dirty_tiles::DirtyTiles::set` (L204, inline_instance, inlinee_declaration)
- `utils::dirty_tiles::DirtyTiles::is_set` (L215, inline_instance, inlinee_declaration)
- `utils::dirty_tiles::bitmap_set` (L221, inline_instance, inlinee_declaration)
- `utils::dirty_tiles::bitmap_intersects` (L238, inline_instance, inlinee_declaration)
- `utils::dirty_tiles::dilate` (L243, inlinee_declaration)

## 派生胶水分组

- `utils::dirty_tiles::impl$4::clone` (L13)
