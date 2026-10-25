import struct,pathlib,json
p=pathlib.Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe');d=p.read_bytes();pe=struct.unpack_from('<I',d,60)[0];s=pe+24+struct.unpack_from('<H',d,pe+20)[0]
vs,rva,rs,raw=struct.unpack_from('<IIII',d,s+8); text=d[raw:raw+rs];wanted={0x140b46960,0x140b46920};i=0;out=[]
while True:
 i=text.find(b'\xe8',i)
 if i<0:break
 va=0x140000000+rva+i;target=va+5+struct.unpack_from('<i',text,i+1)[0]
 if target in wanted:out.append({'call':hex(va),'target':hex(target)})
 i+=1
pathlib.Path(__file__).with_name('next-callers.json').write_text(json.dumps(out,indent=1));print(out)
