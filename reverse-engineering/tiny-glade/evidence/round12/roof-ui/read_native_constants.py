from pathlib import Path
import pefile,struct,json
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
def read(a,n):return raw[p.get_offset_from_rva(a-base):][:n]
constants={}
for a in [0x142925970,0x1429cd2d4,0x142abcb20,0x1429258d0,0x142aa5d60,0x142aa5d64,0x142aa5d68,0x1429cd0c0,0x142a8cc80,0x142a7baa0,0x142abaa00,0x142a81c60,0x142a80ce0,0x1429cd128]:
 b=read(a,16);constants[hex(a)]={'hex':b.hex(),'floats':struct.unpack('<4f',b)}
for a in [0x142ff7c18,0x142ff7c28,0x142ff7c48,0x142ff7cb8,0x142ff7d10,0x142eaf5b8]:
 b=read(a,128);constants[hex(a)]={'hex':b.hex(),'text':b.decode('utf8',errors='replace')}
Path(__file__).with_name('native-constants.json').write_text(json.dumps(constants,indent=2,ensure_ascii=False),encoding='utf8')
print(json.dumps(constants,indent=2,ensure_ascii=True))
