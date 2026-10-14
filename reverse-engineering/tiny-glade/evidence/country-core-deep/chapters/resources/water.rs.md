# water.rs

> 文件作用：**水内容资源模块：RippleSourceSsbo（鸭子入水波纹源 SSBO，system_critters::duck::update_duck_ripples 生产 / water_planes_draw 消费）+ WaterDrawEvent/WaterEraseEvent 事件（draw_raster/replay_raster 写入）。CPU 水位本体在 world_raster_defs::WaterRaster（WorldRaster<512>）。**

函数总计 3：业务 0 / 内联无独立符号 0 / 派生胶水 3（PDB name-only，无独立业务过程）

## 业务函数

（无——PDB 仅命名实体/SSBO 标记类型，生命周期走 AssetSsboLibrary 泛型胶水）

## 内联/无独立符号函数

（无）

## 派生胶水

`add_new`/`get_rg_buffer`/`write_storage_buffer` 等泛型胶水（不计独立业务）
