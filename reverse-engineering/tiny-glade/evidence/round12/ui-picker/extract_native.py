from pathlib import Path
import pefile,capstone,hashlib,json,struct
root=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
functions=[('hover_intersection',0x1408383a0,3887),('picker_closure2',0x140ac3f50,3455),('ui_update',0x140b2de60,7577),('get_ui_volume',0x140b2dc60,503),('add_ui',0x140945c60,613),('picker_occlusion_retain',0x1412f6680,509),('sort_canvas_comparator',0x141339440,245),('sort_distance_comparator',0x141339a30,429),('is_inside_bbx',0x14123ffc0,44)]
manifest=[]
for name,va,size in functions:
 b=pe.get_data(va-base,size);(root/(name+'.bin')).write_bytes(b)
 (root/(name+'.asm')).write_text('\n'.join(f'{i.address:016x}: {i.bytes.hex():24s} {i.mnemonic:10s} {i.op_str}' for i in cs.disasm(b,va)),encoding='utf8')
 manifest.append(dict(name=name,va=hex(va),size=size,sha256=hashlib.sha256(b).hexdigest()))
(root/'native-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')
