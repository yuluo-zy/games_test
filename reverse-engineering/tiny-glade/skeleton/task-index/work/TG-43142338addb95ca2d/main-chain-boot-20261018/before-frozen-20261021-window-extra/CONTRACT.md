# Info::window_extra 窄接口合同（候选前独立审查）

本任务沿TG-43142338addb95ca2d旧claim main-chain-boot-20261018，仅获root窄委派。当前源已冻结decorator.rs.before，不改其他Info业务、不迁移generic extra/set_extra、目的地codec或字段。

原证复用TG-52f7fc4f94ad125530/cloth-visual-state-20261020/window-extra-dependency-v1/CONTRACT_MANIFEST_V1.json。已重新校验原as_window105B汇编、PDB模块、wall/corner窗口及其完整caller文件哈希，见retained-evidence-check.json。身份B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。

观察：1408393b9比较Extra指针+0x11字节是否6；相等返回原指针，不等进入格式化panic。wall1407ed1e3/1ee与corner1407f7a2b/36传Info+0x30，再读取返回指针首u32，因此所读tag是Info+0x41，载荷是Info+0x30低u32。Info+0x28另属frame color，不得充当window glass color；目的地+0x10不参与。

当前源码DecoratorInfo字段u30/u40已保有原偏移，extra_style()正确取u40 byte1，WindowExtra(pub u32)已是唯一canonical类型。计划只增加`pub fn window_extra(&self)->WindowExtra`，要求extra_style()==6，否则与原as_window同类panic；返回WindowExtra(self.u30 as u32)。不增加默认值、Option或平行enum，保留payload全部32位，不解释/限制颜色索引。

现源generic extra()/extra_disc()/set_extra()与旧TG52报告叙述不同：它们现在已经读u30..u40 canonical区域。旧报告的“dst_payload高位方言”结论不能套当前源。本候选无论其历史方言如何都完全不改这些方法，避免跨范围迁移。

消费者API已和/root/decorator_chain确认：wall/corner生成器要求`DecoratorInfo::window_extra(&self)->WindowExtra`，返回的u32喂glassColorId。该agent只写其TG52/22/73候选，不写Info。UI/history/load是否所有分支都正确写+30/+41仍为完整producer缺口，不以本leaf补齐宣布闭包完成。

验证义务：Poison目的地和u30高32位，确认只取低32位；所有非6的u8标签必须panic，不返回default、不误读dst tag。为这一required failure写最窄现文件内测试即可；不运行Cargo/rustc/GPU/原执行oracle。原所有权、通知、删除、历史均不在此只读getter中改变。B行为静态可证，C运行时/ABI未验证，不升级版本等价。
