# Mute guard V2

保持 V1 冻结，只将新增测试中三个 Events::write 改为实际 pin Bevy0.16 的 Events::send；真实系统 EventWriter::write 不改，入口 read.count 不改。stairs 与 V1 逐字一致，仅配对导航更新。完整原证/业务范围/未运行边界同 V1。两个 actual-system tests 未 Cargo 执行；不挂旧完整audio，单位 Events 注册由root唯一App负责。
