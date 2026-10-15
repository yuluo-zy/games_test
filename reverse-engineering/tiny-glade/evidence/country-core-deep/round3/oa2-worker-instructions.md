# O 结论抽查第二轮 —— worker 指令（round2，2026-10-15）

你是 Tiny Glade 逆向 O 结论抽查的验证 worker。任务：对分配给你的章节 O 行逐条核对机器证据，产出判定。**只验证和报告，不修改任何章节文件、样本 JSON 或 reconstruction 源码。**

## 你的输入/输出

- 输入：`D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3/oa2-chunks/<CHUNK>.json`
  （每条含 addr/rel/fn/claim/evid/pdb_exact/pdb_owner/pdb_pubsym/decomp_files/evid_txt）
- 输出：`D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3/oa2-results/<WORKER>.json`
  每条：`{"addr": "0x...", "verdict": "一致|不一致|证据不足", "detail": "...", "evidence": "..."}`。
  detail 用中文，不一致时写清"代码写 X，证据为 Y"，并给 fix 建议（该行应改为什么）。
  用 `json.dump(..., ensure_ascii=False, indent=1)` + `open(..., "w", encoding="utf-8")` 写文件；不要向 stdout 打印中文。

## 章节行定位

行在 `evidence/country-core-deep/chapters/<rel>`（rel 就是 chunk 里的 rel，去掉 .md 尾缀补上即可）。
表格式：`| fn | addr | line | 作用 | 逆向结论 | 证据 |`。claim 字段已从行中取出，供你核对。

## 验证链（按序，够用即停）

1. **PDB 身份**：`pdb_exact` = 该地址的 PDB 精确符号（若有）。若与行内 fn 明显不同（如 0x14003bb80 实为
   `bevy_app::plugin::Plugin::finish<MainCameraPlugin>` 却被贴成游戏函数）→ 重点嫌疑，看机器码裁决。
   `pdb_owner` = 包含该地址的函数（ICF 折叠时有用）。
2. **现有 decomp**：`decomp_files` 是绝对路径列表。**先检查文件内容不是** `{"error":"No function at..."}`/
   `{"error":"NotFound"}`——是则证据无效（第一轮系统性错误模式），改用 rne-read/MCP。
   大文件（>8KB）先 `head -c 4000` 看结构，再 grep `FUN_|UNK_|mov|cmp` 核对行内断言（callee 地址、常量、
   偏移、字符串）。同一行断言里的具体数字（浮点常量、偏移量、判别值、串内容）必须逐项核对。
3. **rne-read 原字节/串**（比 MCP 快，短函数首选）：
   - `PYTHONIOENCODING=utf-8 python -I D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py --hex <va> <n>`（n=16..48 足够手解小函数）
   - `PYTHONIOENCODING=utf-8 python -I D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py <va>`（NUL 结尾 C 串）
   - `--fat <va>`（胖指针 {ptr,len} → 串）；`--unks <decomp.txt>`（解析反编译里 UNK_ 引用）
   - 机器码常识：`cmp byte[rcx+8],2; sete al; ret` = 判别==2；`xor eax,eax; ret` = 返回 0；`lea rax,[rcx+0x18]; ret` = 返回字段指针；`mov eax,[rcx]; ret` = 读 u32。
4. **MCP 反编译（最后手段）**：decomp 缺失/无效 且 rne-read 不足以裁决行为断言时。
   job 模板（每函数一 job，open→wait→decompile→close）：
   ```json
   [{"tool":"open_database","args":{"file_path":"D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe","run_auto_analysis":false,"database_id":"<DBID>","keep_open":true},"timeout_ms":300000},
    {"tool":"wait_for_analysis","args":{"database":"<DBID>"},"timeout_ms":300000},
    {"tool":"decompile_function","args":{"database":"<DBID>","address":"<VA>"},"timeout_ms":180000,"allow_error":true},
    {"tool":"close_database","args":{"database":"<DBID>","save":false},"timeout_ms":120000}]
   ```
   运行：`node D:/game/reverse-engineering/tiny-glade/scripts/mcp-job.mjs re_mcp_ghidra <job.json> <out.json>`。
   job 写到 `round3/oa2-jobs/`、out 写到 `round3/oa2-out/`（父目录先 mkdir）。Decompile 失败
   （NotFound）时看 result 里的 error 字段，改用 rne-read。

## 判定规则

- **一致**：行内断言（身份+行为）与证据吻合。胶水行（serde derive / register_required_components /
  SystemParam 胶水 / 依赖转发）只要 PDB 精确符号与行内 fn 吻合且机器码是典型胶水形态，
  判"一致（胶水，按规则不计业务）"——O 身份成立。
- **不一致**：行为不符、常量/偏移/判别值错、方向反（>= vs <=）、地址错位（PDB 精确符号与行内 fn 不同
  且机器码不支持行内断言）、NotFound 文件被当证据。
- **证据不足**：手头证据无法裁决（例如大函数且 decomp 缺失、MCP 也 NotFound、行为断言依赖未验证的
  调用方语义）。

## 已确认的系统性雷区（第一轮发现，重点警惕）

- `0x14003bb80` 是 `Plugin::finish<MainCameraPlugin>` 1B `ret` no-op（ICF 折叠区）；0x140002370 是
  `bevy_ecs schedule_system::is_exclusive`。低地址 0x140002xxx/0x140003xxx 区多为 Bevy 泛型折叠——
  多符号贴同一地址时以机器码+PDB 唯一符号为准。
- NotFound 错误文件 ≠ 证据；decomp 内容为 error 时该行证据链断裂，需补 rne-read/MCP。
- 依赖库（bevy/rapier/hashbrown 等）实例行为也要机器码确认，不能只凭库语义。
- 未证实的数量断言（"N callers"、"路径长"）不标 O，除非有 xrefs 证据。

## 效率

- 先批量处理身份类条目（PDB 一致 + 短函数）：rne-read --hex 一次过。
- 大函数 decomp 只 grep 关键点，不整读。
- 每 worker 至多开 3 个 MCP 会话（并行 2 路以内）；绝大多数条目不需要 MCP。
- 同一文件内多行同地址（如 0x14003bb80 三行）只核一次，合并成一条结果记录（detail 里注明覆盖行）。

完成后：结果文件写满全部条目；然后 SendMessage 给 main，10 行内总结（判定分布 + 最可疑 5 条）。
