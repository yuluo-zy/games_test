from pathlib import Path
import pefile,struct,json,hashlib
out=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
def read(va,n):return p.get_data(va-base,n)
strings=[(0x142a99570,15),(0x142a995b3,16),(0x142a995c3,25),(0x142a995e8,19),(0x142a9948c,19)]
result={'original_sha256':hashlib.sha256(raw).hexdigest(),'strings':{hex(a):read(a,n).decode() for a,n in strings},'jump_targets':[hex(0x142a9abc0+struct.unpack('<i',read(0x142a9abc0+4*i,4))[0]) for i in range(5)],'default_quaternion':struct.unpack('<4f',read(0x14293e0c0,16))}
(out/'original-constants.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
