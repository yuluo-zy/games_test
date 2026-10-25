# 旧树存档缺字段策略的最窄修复合同

范围仅 TG-ab197f381ed7615ce6 当前 canonical tree_edit.rs 的 TreeEditData Deserialize 两属性及同源真实反例测试。live owner boot-leaf-20261018 保持，root/lead本轮明确必要委派；不改依赖visitor实现、历史总数断言、TreeStorageIndex/variant身份、外层Place/Move/Delete格式或六字段Serialize。

原 shipped full/25-swamp/history.json Version18，/History/edit_chain/25/edits/0/Tree 是 Place {index:0,tree:{id:5,position:[7.286689758300781,0,-2.4316482543945312],rotation_degrees:359.0250549316406,scale:0.7239845991134644}}；同文件13个Place/Delete的tree缺prefab_name和is_preplaced。当前derive把两字段required，导致进入History反序列化前就missing prefab_name，实际回放没有机会执行。不是键alias，不应修改真实语料或假选择Oak等prefab。

原B PDB1630:219812 的 serde_json::value::de::visit_object<TreeData Visitor> 完整3402B（1415b6a30），PDB534:26072 missing_prefab110B（1409e6210），及外层TreeEdit Deserialize1766B均原PE逐指令连续字节核对；见NATIVE_BYTES.json。PDB树source内missing_prefab的inline site直接归属该3402B visitor。外层1766B只说明Place/Move/Delete真实分派，不把它错误当成inner字段策略的唯一证据。

字段结束门1415b6f56..6f8b按顺序检查position/scale/rotation_degrees/id存在，缺项分别跳1415b6ffc/703e/7058/7072，均原missing_field错误。之后prefab Option未提供（cap i64::MIN）在1415b6fa4..6fe3分配并复制原142ac4708的24字节MISSING_TREE_PREFAB_NAME；显式String则1415b70fb沿真实值。is_preplaced原临时Option<bool>初始化2，已提供走bool decode1415b6f32、存1415b6f44..6f4b；结束1415b7112..7119读byte并&1，未提供2→false，显式true/false保持。显式null或错误类型仍走对应decoder错误，不应用缺省掩盖。

重复字段保原duplicate_field分支（prefab1415b7246、is_preplaced1415b72c5、其它required字段相邻分支）；未知字段使用原serde ignore策略。现代继续复用serde derive，因此只在prefab_name添加 #[serde(default="missing_prefab")]，is_preplaced添加 #[serde(default)]。没有改required四字段、deny_unknown_fields或全struct默认。

测试：1直接复制真实Version18树片段解TreeEdit并核outer index0与variant5独立、原sentinel/false、保存仍六字段；2分别缺两个字段以及显式值保持；3四required字段缺失、null/错误bool与重复字段仍失败，未知字段保持可忽略。不是伪造prefab，不统计为新游戏业务。root唯一Cargo/原33语料回归/GPU/生产writer，本候选未运行。
