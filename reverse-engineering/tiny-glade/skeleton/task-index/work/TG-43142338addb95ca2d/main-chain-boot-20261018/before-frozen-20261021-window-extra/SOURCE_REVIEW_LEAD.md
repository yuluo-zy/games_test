# 窗口额外数据窄接口复核

复核者：core_business_lead。已审当前 `DecoratorInfo` 字段、`extra_style`、既有 `WindowExtra` 与候选 patch，并合读原 as_window/caller 合同。批准 root 整合此窄叶；不代表窗口 producer、catalog 或整条编辑链已完成。

候选只检查 canonical Info+0x41 判别为6，读取 canonical Info+0x30 的低32位，返回现有 `WindowExtra`。未误用 destination、框颜色或高32位；required 失败与原 as_window 同类。generic extra/extra_disc/set_extra、字段和目的地 codec 未改，不依据旧报告推测当前实现。两项测试验证 poison 与非6必失败，尚未执行。

before=`6be43c9cc5257c47947095e0a691b0ce673a88be1ae6a26a500d1f0e15c1810d`；after=`8c2473a78348d85c05e0bcff74bc718b25c3fc62545570bb44b03efd8afd9104`。实际生产写入与 Cargo/原生验证由 root 执行。
