//! `country_core::startup` — 启动期生产者。只放已经对着 EXE 核对过的纯函数部分；
//! 载入网格/着色器/实体生成的系统（`load_prefabs`、`startup_terrain` 本体等）依赖 ECS 与资产库，尚未做。

pub mod init_photomode_state;
pub mod load_glade_audio;
pub mod load_textures;
pub mod startup_session;
pub mod startup_terrain;
