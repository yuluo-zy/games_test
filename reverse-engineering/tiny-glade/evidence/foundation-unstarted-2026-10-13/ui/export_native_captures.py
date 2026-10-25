from pathlib import Path
import json,zlib,struct,hashlib
root=Path(__file__).resolve().parent
def chunk(kind,body):
    return struct.pack('>I',len(body))+kind+body+struct.pack('>I',zlib.crc32(kind+body)&0xffffffff)
results=[]
for name in ['dialog','integrity','gpu','vulkan','crash']:
    p=root/(name+'.rgba')
    meta=json.loads(p.with_suffix('.json').read_text())
    b=p.read_bytes();w=meta['width'];h=meta['height']
    assert len(b)==w*h*4 and w>0 and h>0,(name,meta,len(b))
    first=b[:4]; different=sum(b[i:i+4]!=first for i in range(0,len(b),4))
    assert different>100,(name,'empty native capture',different)
    rows=b''.join(b'\0'+b[i:i+w*4] for i in range(0,len(b),w*4))
    png=b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('>IIBBBBB',w,h,8,6,0,0,0))+chunk(b'IDAT',zlib.compress(rows,9))+chunk(b'IEND',b'')
    p.with_suffix('.png').write_bytes(png)
    results.append(dict(case=name,width=w,height=h,rgba_bytes=len(b),pixels_distinct_from_corner=different,rgba_sha256=hashlib.sha256(b).hexdigest(),png_sha256=hashlib.sha256(png).hexdigest(),png=str(p.with_suffix('.png'))))
(root/'native-captures.json').write_text(json.dumps(results,indent=2))
print(json.dumps(results,indent=2))
