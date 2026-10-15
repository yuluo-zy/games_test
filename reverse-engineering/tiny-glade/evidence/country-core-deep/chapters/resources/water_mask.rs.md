# water_mask.rs

> 文件作用：**水 mask 回读缓冲 WaterMaskSsbo：512×512 水 mask 纹理 .y 通道打包回读（>0.5 水位位 / >0.2 clutter 位，read_water_mask.cs.hlsl O）。name-only（布局 U）。**

函数总计 5：业务 0 / 内联无独立符号 0 / 派生胶水 5（PDB name-only，无独立业务过程）

## 业务函数

（无——PDB 仅命名实体/SSBO 标记类型，生命周期走 AssetSsboLibrary 泛型胶水）

## 内联/无独立符号函数

（无）

## 派生胶水

`add_new`/`get_rg_buffer`/`write_storage_buffer` 等泛型胶水（不计独立业务）

## 实现方式（算法级）

无独立业务过程（name-only）。WaterMaskSsbo = 512×512 水 mask 纹理 .y 通道打包回读（>0.5 水位位 / >0.2 clutter 位，read_water_mask.cs.hlsl O）；read_terrain_data 槽位回拷消费者。字段布局 U。
