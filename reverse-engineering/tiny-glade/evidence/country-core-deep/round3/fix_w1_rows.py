# -*- coding: utf-8 -*-
import io, os

CH = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"

# 1) draw_builder: 3 rows' evid fix (0x14162c290 is a function, WallColorIds is a type-name blob substring)
p1 = os.path.join(CH, "render/pass_builder/draw_builder.rs.md")
txt = io.open(p1, encoding="utf-8").read()
old = "（0x141629e90 系；WALL_COLOR_ID_SLOT_NAME=\"WallColorIds\" 0x14162c290）"
new = "（0x141629e90 系；round3 round2 修正：0x14162c290 实为函数非串址；\"WallColorIds\" 为类型名串段 @0x14293ff6b 区）"
n = txt.count(old)
txt = txt.replace(old, new)
io.open(p1, "w", encoding="utf-8", newline="").write(txt)
print("draw_builder evid rows fixed:", n)

# 2) shader_uri: 4 source_hash rows' guess column wording
p2 = os.path.join(CH, "render/pass_builder/shader_uri.rs.md")
txt = io.open(p2, encoding="utf-8").read()
old2 = "source_hash = calculate_hash(&(\"<path>:cs_main\", &i32))（SipHash-1-3 零密钥；串+0xff+4 字节）"
new2 = "source_hash = calculate_hash(&(\"<path>:cs_main\", &self))（round3 round2 修正：第二元为 &self 非 &i32；SipHash-1-3 零密钥；串+0xff+4 字节）"
n2 = txt.count(old2)
txt = txt.replace(old2, new2)
io.open(p2, "w", encoding="utf-8", newline="").write(txt)
print("shader_uri source_hash guess rows fixed:", n2)
