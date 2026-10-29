# M01 clutter blueprint 第五参数独立复核（2026-10-27）

结论：现有 `generate_decorator_clutter` 的 `insert_for(id, addr, 0)` **符合 B 原行为**。不应因其它装饰物沿用 `Info.state`，就把此常量改成 `info.state()`。本轮生产代码只补说明该特殊合同的中文注释，未扩 M11 clutter 算法。

## 原始归属与证据

实际先查阅 B `skeleton/B/crates/systems/decorator/src/decorator_visual/decorator_clutter.rs`：原函数 `generate_decorator_clutter` **1407e3370**，PDB module439，原7组参数顺序与当前既有系统一致。PDB `void()` 仍是调试占位，不可据此创造签名。

原调用块：

```asm
1407e3ba3 mov r8,[rbp+1a0]    ; 原 StorageAddr 两 word
1407e3baa mov r9,[rbp+1a8]
1407e3bb1 mov byte [rsp+20],0 ; 原第五参，字面零
1407e3bb6 mov rcx,[rbp+140]   ; blueprints
1407e3bbd mov edx,edi         ; decorator id
1407e3bbf call 1408258b0      ; DecoratorBlueprints::insert
```

真实 Win64 call 进入时第五参在 `[callee rsp+28]`。保留 decompiler 的末参 `& ...ffffff00` 很容易误读为上层 flags，原指令明确字面零。当前 `insert_for` 第三语义参数（原第五 ABI 参数）应仍传零；中央负责其正确写入 blueprint `u138`，与 `interaction_source_flags` 原 +139 区分。

原 C 在 `reverse-engineering/tiny-glade/evidence/country-core-deep/systems-recovery/groupD/decorator_visual/decomp/decomp-0x1407e3370.txt`，本轮保存 `m01-clutter-insert-state-call.asm`。

## 验证

`m01_clutter_insert_state_native.py` 校验原 RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03` 后执行真实 **1407e3b94..1407e3bbf** 调用块，在真实 insert 入口记录参数。

原栈第五参数先污染为0/1/2/255，搭配3个 id 和3组原始 opaque address word，共36组；都被原指令覆盖为0，id/address及blueprint owner、tick写入保持原输入。这里的污染字节是调用前栈值，**不冒称执行了完整 native Info/state 过滤、容器或 clutter 几何算法**。

既有 `tests/terrain_clutter_visual_chain.rs` 复用真实资源/CPU边界 fixture，新回归实际运行：

`cache_decorator_transforms → clear_blueprints → generate_decorator_clutter → instantiate_blueprints → copy_blueprints_to_prev`

将非空地形 clutter 的权威 `Info.state` 依次设为 **0→1→0**，分别核权威状态仍在、blueprint `u138=0`、交互来源flag=0、Nani实体/双collider有值、跨帧旧实体/blueprint回收，再 remove 验证实体、current/previous蓝图和双collider释放。fixture 直接调用实际帧尾 tracking reset，不冒称已运行完整 validator 或真实 UI/GPU。

命令：

```sh
/Users/yuluo/Desktop/game/tools/venv/bin/python reports/module-recovery-2026-10-23/m01_clutter_insert_state_native.py
CARGO_INCREMENTAL=0 cargo test -p system-decorator --test terrain_clutter_visual_chain --offline clutter_original_insert
```

原调用块36组已通过；真实 ECS 定向回归 **1/1通过**，既有 terrain_clutter_visual_chain 全部 **5/5通过**。日志为 `m01-clutter-insert-state-chain-tests.log`。首跑被并发 dormer owner 的 `assemble_gothic_dormer_tiles` 调用过渡（2参→4参）E0061 阻断；由原 owner 补接后重跑通过，没有修改其文件。保留失败 `m01-clutter-insert-state-tests-v1.log` 和成功 `m01-clutter-insert-state-tests.log`。

## 可复用经验

- 所有装饰物共享 insert 不代表上层 producer 的 flags来源相同；必须逐 callsite 查原寄存器/栈写入。原字面常量就是算法，不能为了看起来统一而传播 Info.state。
- 原参数序号和当前 Rust语义参数序号不同，先核 Win64 return-address位移，再建立强类型接口；不要把第五参数误读成第六参数。
- 参数级原指令证据、真实 generator→consumer多帧验收、模块完整验收分开列；36个调用块输入不是36个完整原版业务场景。
