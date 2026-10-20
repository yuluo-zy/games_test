from pathlib import Path
import hashlib, json, struct, subprocess

root=Path('D:/game')
out=Path(__file__).resolve().parent
raw=(root/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,0x3c)[0]
count=struct.unpack_from('<H',raw,pe+6)[0]
opt=pe+24
assert struct.unpack_from('<H',raw,opt)[0]==0x20b
base=struct.unpack_from('<Q',raw,opt+24)[0]
table=opt+struct.unpack_from('<H',raw,pe+20)[0]
sections=[]
for i in range(count):
    at=table+i*40
    virtual_size,va,raw_size,offset=struct.unpack_from('<IIII',raw,at+8)
    sections.append((va,max(virtual_size,raw_size),offset,raw_size,struct.unpack_from('<I',raw,at+36)[0]))
def data(rva,n):
    va,size,offset,_,_=next(s for s in sections if s[0]<=rva<s[0]+s[1])
    return raw[offset+rva-va:offset+rva-va+n]
pdata_rva,pdata_size=struct.unpack_from('<II',raw,opt+112+3*8)
pdata=data(pdata_rva,pdata_size)
manifest=[]
for addr in [0x1413036c0,0x14093d830,0x1403ac400]:
    rva=addr-base
    begin,end,_=next(struct.unpack_from('<III',pdata,i) for i in range(0,len(pdata),12) if struct.unpack_from('<I',pdata,i)[0]==rva)
    size=end-begin
    code=data(rva,size)
    asm=subprocess.check_output(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn',f'--start-address={addr}',f'--stop-address={addr+size}',str(root/'ljxsj_92385/Tiny Glade/tiny-glade.rne')],text=True)
    (out/f'{addr:x}.asm').write_text(asm,encoding='utf-8')
    manifest.append({'address':hex(addr),'size':size,'sha256':hashlib.sha256(code).hexdigest(),'range_source':'original RNE .pdata exact BeginAddress/EndAddress'})
constants={hex(a):data(a-base,n).hex() for a,n in [(0x142ab1530,64),(0x142ab20e6,32)]}
calls=[]
for va,size,offset,raw_size,flags in sections:
    if not flags&0x20000000:continue
    for i in range(offset,offset+raw_size-5):
        if raw[i]==0xe8 and base+va+i-offset+5+struct.unpack_from('<i',raw,i+1)[0]==0x1403ac400:
            calls.append(hex(base+va+i-offset))
(out/'get-monitor-call-candidates.json').write_text(json.dumps(calls,indent=2))
(out/'native-manifest.json').write_text(json.dumps({'ranges':manifest,'constants':constants},indent=2))
print(json.dumps(manifest))
