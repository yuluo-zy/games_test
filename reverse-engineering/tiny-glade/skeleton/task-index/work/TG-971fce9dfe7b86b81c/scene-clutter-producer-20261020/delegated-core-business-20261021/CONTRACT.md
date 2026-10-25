# 必要编译迁移：dormer roof tiles唯一ID读取

仅generate_dormer_roofs中derived.get(id).subtype().0改为derived.get(id).0；当前canonical图保存DecoratorSubtype(pub u8)，get仍required panic，不引入fallback/optional。原完整瓦片业务尚有已知旧壳，不用此单API迁移宣称其已还原，分支/次序全保持。复用既有子类型掩码与rank失败测试，不为单token API添加镜像测试。与TG609 canonical图配对。

原owner/委派/当前before见同目录baseline.json和live-show.json。只有root生产写入/Cargo，worker只候选。
