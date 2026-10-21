# 原 replay_color 来源 → 现 core adapter 测试 fixture 最窄映射

## 授权与归属

中央 agent 于本轮明确允许live NULL TG-8608a7dd158bc5f100正常claim，限定修两项实际CPU失败的typed extra测试输入。已normal claim owner core-duck-building-20261021，claim不抢他人；回执在live-claim.json。

原文件组 `crates/systems/color/src/replay_color.rs` 映射当前已有手写core adapter `tiny-glade/crates/country-core/src/systems/history/replay_color.rs`。本轮只修改该既存文件的cfg(test) helper/三调用，不新建生产文件、不把现代adapter冒称原文件完整重建。索引目标C保持原值；当前只修现Rust测试输入对canonical owner的迁移，不宣称B/C完整业务或C验证。

## 原证与失败

实际root日志 business17 core：brightness在读取as_lantern时panic，flag/window首先在读取as_flag时panic。fixture place把disc和value写到destination的dst_payload(+10)，没有写真实extra owner+30..44，因而extra仍是Stair0。disc数值并未变；原Flag5、Window6、Lantern7由现canonical decoder/set_extra与原三完整From短叶共同支持。

独立32B短叶/当前core源码/日志只读报告先前已冻结在TGf768/core-duck-building-20261021/readonly-replay-color-fixtures-20261021，READONLY_REVIEW.md SHA456c212e9769a7dfb2a85694bfa6b332ff6ba85827184c1cc7acbd49cf1f614b。三原10/10/12B函数地址140839530/540/550分别写payload+0与niche+11=5/6/7，PDB475:71708/71840/71972；原RNE f507a6ce，未执行原target。当前Extra所在Info+30，niche为Info+41；正确接口是set_extra，不能写destination假标签。

## 候选设计与验收

测试导入FlagExtra、WindowExtra、LanternExtra；place签名改接受DecoratorExtraInfo，保原Info的decorator_id/state，其余原值不动，再调用canonical info.set_extra(extra)，最后按原真实storage.apply_decorator_place发布。

仅三处测试输入：Brightness用Lantern(.25)，Flag用Flag(0)，Window用Window(0)。实际redo/undo/timed流程、生产消费者、assert结果1.0/1/9及事件副作用全部不变。不得把match失败改成功、删断言、补默认变体来通过。

独审通过后才生成source候选；root唯一应用与Cargo，测试尚未重跑。该adapter已注明plaster/sound/SSBO等原副作用partial，本轮不扩展完成声明。
