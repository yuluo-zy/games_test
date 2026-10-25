"""Read bounded original bytes. No database or production mutation."""
from pathlib import Path
import hashlib,json,struct,subprocess
ROOT=Path('D:/game')
OUT=Path(__file__).parent
EXE=ROOT/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
raw=EXE.read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
nt=struct.unpack_from('<I',raw,0x3c)[0]
sections=[]
for i in range(struct.unpack_from('<H',raw,nt+6)[0]):
    p=nt+24+struct.unpack_from('<H',raw,nt+20)[0]+i*40
    vs,va,rs,rp=struct.unpack_from('<IIII',raw,p+8)
    sections.append((va,rs,rp))
def at(a,n):
    rva=a-0x140000000
    for va,rs,rp in sections:
        if va<=rva and rva+n<=va+rs:return raw[rp+rva-va:rp+rva-va+n]
    raise ValueError(hex(a))
def disasm(a,n,name):
    result=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn','--start-address='+hex(a),'--stop-address='+hex(a+n),str(EXE)],capture_output=True,text=True,check=True)
    (OUT/(name+'.asm')).write_text(result.stdout)
if __name__=='__main__':
    symbols={}
    for line in (ROOT/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text().splitlines():
        p=line.split('\t')
        try:symbols[int(p[0],16)]=(int(p[1]),p[-1])
        except ValueError:pass
    vtable=struct.unpack('<14Q',at(0x142baf6d0,14*8))
    output=[]
    for i,a in enumerate(vtable):
        symbol=symbols.get(a)
        output.append(dict(slot=hex(i*8),address=hex(a),symbol=symbol))
        if symbol:disasm(a,symbol[0],'menu-button-vtable-'+hex(i*8)[2:])
    (OUT/'menu-button-vtable.json').write_text(json.dumps(output,indent=2))
    table=struct.unpack('<17i',at(0x143053f50,17*4))
    output=[]
    for tag in [0,2,3,8,9,11]:
        target=0x143053f50+table[(tag-2)&255 if 0<=tag-2<17 else 5]
        output.append(dict(tag=tag,target=hex(target)))
    (OUT/'format-component-dispatch.json').write_text(json.dumps(output,indent=2))
    print(json.dumps(dict(vtable=vtable,dispatch=output),indent=2))
