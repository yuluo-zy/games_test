# systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs

> 文件作用：常春藤实例绘制：`ivy_leaf_shader`（共享实现）、`draw_instanced_ivy_branch_shader`、`draw_instanced_ivy_leaf_shader`、`draw_instanced_ivy_flower_shader`（后两者为带 hash 常量的薄 thunk）。

函数总计 4：业务 4 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…instanced_ivy::draw_instanced_ivy_branch_shader` | 0x1408dbc50 | 19 | 常春藤枝实例绘制：DrawBuilder draw_instanced（ivy branch；draw_lists 分派目标 0x1408dbc50） | O（分派目标） | render-loop\decomp\decomp-0x1408dbc50.txt；wp5 draw-passes §1 |
| `…instanced_ivy::draw_instanced_ivy_leaf_shader` | 0x1408dc180 | 50 | 常春藤叶实例绘制（NotFound 手解）：`mov rdx,r8; movabs r9,0xe6d1a603f18a02b7; xor r8d,r8d; jmp +0x2b` = 向共享 ivy_leaf_shader 传 name_hash + flag=0 的 thunk | O | 手解：`rne-read.py --hex 0x1408dc180 40`：`4c 89 c2 49 b9 b7 02 8a f1 03 a6 d1 e6 45 31 c0 e9 2b 00 00 00 …` |
| `…instanced_ivy::draw_instanced_ivy_flower_shader` | 0x1408dc1a0 | 60 | 常春藤花实例绘制（NotFound 手解）：`mov rdx,r8; movabs r9,0x0c8893888148dab3; mov r8b,1; jmp +0xb` = 传另一 name_hash + flag=1 的 thunk | O | 手解：`rne-read.py --hex 0x1408dc1a0 40`：`4c 89 c2 49 b9 b3 da e6 48 81 93 88 0c 41 b0 01 e9 0b 00 00 00 …` |
| `…instanced_ivy::ivy_leaf_shader` | 0x1408dc1c0 | 70 | 常春藤叶共享实现：`(param_1, param_2, param_3=char flag, param_4=hash)`；两 thunk 的公共目标（DrawBuilder draw + name_hash 绑定） | O | render-loop\decomp\decomp-0x1408dc1c0.txt |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `ivy_leaf_shader`（0x1408dc1c0）共享实现签名 `(builder, &params, char flag, u64 name_hash)`；两 thunk（0x1408dc180 / 0x1408dc1a0）传 `name_hash + flag`：
  - ivy_leaf：hash `0xe6d1a603f18a02b7`、flag=0；
  - ivy_flower：hash `0x0c8893888148dab3`、flag=1。
- 回调（d_cb_a §3 #5/#6）：`ivy_leaf_cb`/`ivy_flower_cb`（21B 薄壳）= `MainCamera.eye` + `AssetSsboLibrary`（LibView）→ draw_instanced_ivy_leaf/flower_shader。
- 数据流：`ivy_send_to_gpu_leaf`（IvyLeafSsbo/IvyFlowerSsbo from_iter + FrozenMap::get<SsboId> + update_buffer_bytes，O 抽查一致）→ 实例缓冲 → thunk → 共享实现。
- name_hash 常量 = ivy_leaf/ivy_flower 的 shader 名哈希（audio-misc 域 name_hash 模式，xxh3_64）；shader body 不挖。

## 关键事实

- name_hash 常量（0xe6d1a603f18a02b7 / 0x0c8893888148dab3）= ivy_leaf / ivy_flower 的 shader 名哈希（audio-misc 域 name_hash 模式）。
- shader body 不挖（GPU 后置）。
