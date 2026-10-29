# M01 放置意图与墙合并恢复（2026-10-27）

## 原始身份与范围

本轮先实际查看 B 骨架 `crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs`、`merge_proposal_wall_decorators.rs`、`collision_detection.rs` 与 core `resources/walls/decorator.rs`，在现有路径修改。未新建业务目录。

- B 原 RNE SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 匹配 PDB SHA256：`2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。
- Intent 源 SHA：`1cdfe72ba7eb2b535e22eb8c995b3b4d5fd4538369cce62dc9e7d2baa23ee8dc`。
- Wall merge 源 SHA：`bffb6a4b775ed649960bd36f81d3b46e0abf227d79fb2a758f1b20190078f93f`。
- `14127cf70` / 8546B，PDB 1522/record1440；`142001d40` / 1954B，3064/1256；`142001880` / 1201B，3064/264。
- 实际 Intent 墙相交 wrapper `141fe5c20` / 848B；闭环谓词 `141fe5530` / 495B、开墙 `141fe5720` / 477B；闭环距离 `142007170`。
- 共享角度 `140833a20` / 154B；此处重用原完整 NEG_Z 四元数运算，未修改其它构造器。

7 个 bounded LLVM 原字节 pack、14 个符号点保存在 `m01-intent-evidence-*` 与各 full ASM/JSON；collector cold 本次 0.534s、8 analysis+2 version 进程、输出947850B，未宣称模拟性能提升。

## 修正的真实业务

1. 原 Intent 输出前两个 qword 是 `DecoratorStorageAddr`，不是 destination tag。旧0x98记录与恒定 valid=1已移除。新安全枚举分别保存 Accepted、UnsupportedTerrain（原tag3）和 Rejected（原tag4完整quat/position）。Accepted独立保存原rank、当前装饰ID、merge目标/合并rank、MoveDstCandidate。
2. 墙：required PublishedWall、真实曲线最近投影、真实subtype/mesh、实际ID排除相交、全墙合并、矩形0..3原角顺序/墙材质阈值、阳台门五态及原allow_merge转换门、闭环/开墙project失败，均直接使用现owner。
3. 屋顶：required wall/entity/query、原flat-only门、类型角度门、required UV、旗帜脊吸附两次调用（4.0预览/1.0吸附）、真实cached/mesh碰撞与完整roof merge。屋顶 leaf 由freehand owner独立恢复，Intent直接调用，未复制算法。
4. 地形：原支持谓词及真实XZ/yaw；不支持时保留原独立反馈变体。
5. 楼梯：按原两个节点ID查权威Requests，4轮×8样本夹逼，代价abs(cross)+沿切线残差平方；保留端点代价重用、首最小、原插值顺序、半宽、失败bounds/NaN与归一化退化分支。没有换最近线段算法。
6. 墙合并：目标subtype/mesh先于类型rank门；族是`wrapping_sub38 <8 ? x:6`（0x3f为7）；rank先宽加再比较；第二次查目标subtype+合并rank真实mesh；沿墙移动使用合并前后width差；corner保持、subtype9垂直居中仅在原分支；完整72B incoming Info仅rank被替换，保留platform/corner/bottom/compound。两个独立ID和额外slice共同排除。最近选择保首相等候选，只在实际比较NaN时panic。
7. 闭环相交按原`min(abs(a-b), abs(length-abs(a-b)))`执行，不用泛化多圈算法改变原输入语义。
8. `DecoratorRotationXZ::from_quat`之前的代数简化会改变非unit及舍入，现改原NEG_Z完整乘加；Intent支持探针也共用同一个kernel。

## 原程序差分与真实资源验证

- 独立Rosetta进程直接执行原完整 `14127cf70`，128楼梯、2地形，共130组，0 native panic。楼梯u和across与Rust128组全逐位一致。
- 同进程执行原完整 `140833a20` 96组，捕获Windows atan2f前的两个实际输入。93有限组逐位一致；3非有限组按NaN分类一致。ARM与原x86对Inf*0的NaN符号有差异，首轮失败日志保留。原Intent在进入各分支前即按原断言拒绝非有限位置/四元数；未伪称Windows CRT最终atan2输出跨平台逐位一致。
- Unicorn执行原完整 `142001d40` 352提案：176接受/176拒绝，Rust全72B Info、u/y、platform、attachment/compound、distance逐位一致。桥接仅已解析目标查询、subtype和mesh资源边界；桶迭代和两ID排除执行原指令。
- 原完整 `141fe5530/141fe5720` 160组，相交结果一致，包含闭环接缝、dy早退和ID排除。
- 原完整最小选择`142001880`额外24组覆盖顺序、重复、等距首命中、NaN单候选与实际比较失败。
- 真实World测试直接借用canonical PublicWalls/Requests/Storage/Subtype/Mesh/Cached/RaycastWorld，覆盖父Intent的墙和屋顶接线及失效门；最终收据见本目录validation JSON。CPU测试不是本机GPU输入验收。

## 可重复经验

- 共享Ghidra对主函数只有371B body却输出47KB反编译，不能把文本长度当完整CFG。先从RNE定位4项相对表142c24718，在私有worker限定完整8546B、扣除其它函数body、写JumpTable override；共享库只读。
- 原容器前缀、union和Arc allocation/data基址必须先核callsite；本轮先与Place owner核18个原参数，再改typed接口。独立ID绝不从Info.word2c猜测。
- 同一语义subtype getter有140b26970与140833ba0两个物理入口。原执行桥接漏第二个时会触发真实缺键panic，应修测试边界，不能放宽生产不变量。
- 宽度/高度必须来自原阶段的正确rank记录，不能因为现字段可拿到就复用待放置bbox。
- 真实World夹具必须走真实path/height准备；首次漏PathRasters导致required panic，保留日志并接实际from_encoded→consume_with_paths(Some)，没有填假默认。
- 先保存原失败，再确定是源错误、oracle错误、还是平台算术/CRT边界。有限位型、无效输入分类与最终数学库输出分开报告。

## 合作和验收边界

height owner负责原Place生产入口及事件，circle负责dispatcher，rectangle负责真实roof指定collider UV，root负责5-owner RaycastResources与宿主注册；本轮不改这些源。roof merge由freehand恢复并单独完成525原leaf case。原算法/原叶子通过不等于整个toolbar/GPU/历史联合验收完成，最终主链结论由中央记录。

最终收据：Intent 3/3、wall merge 3/3（含352+160+24原case）、core角度1/1，全部0失败/0忽略；own源定点格式化后两decorator filters再次通过。原程序复验共762组，且持久化Rosetta复验脚本再次运行226组活跃输出一致。`git diff --check`通过。仅本任务独占ReVa worker已正常停止保存；共享逆向库未变更。
