"""Bounded original RNE evidence for Load UI; never modifies production source."""
from pathlib import Path
import hashlib,json,re,struct,subprocess
ROOT=Path('D:/game')
OUT=Path('D:/game/reverse-engineering/tiny-glade/evidence/round12/menu-interaction')
EXE=ROOT/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
raw=EXE.read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
nt=struct.unpack_from('<I',raw,0x3c)[0]
count=struct.unpack_from('<H',raw,nt+6)[0]
optsz=struct.unpack_from('<H',raw,nt+20)[0]
sections=[]
for i in range(count):
    a=nt+24+optsz+40*i
    name=raw[a:a+8].split(b'\0')[0].decode()
    vs,va,rs,rp=struct.unpack_from('<IIII',raw,a+8)
    sections.append((name,va,vs,rp,rs))
def at(addr,size):
    rva=addr-0x140000000
    for _,va,vs,rp,rs in sections:
        if va<=rva and rva+size<=va+rs:return raw[rp+rva-va:rp+rva-va+size]
    raise ValueError(hex(addr))
functions={0x140f9b5c0:(1126,'backup-pagination'),0x140c96690:(537,'filename-date'),0x141e1fe80:(15632,'load-ui'),0x141e23b90:(110,'display-error'),0x141e23c00:(273,'load-meta'),0x141e23d20:(451,'contextual'),0x140fc3840:(11861,'backup-list-slot'),0x140fc66a0:(844,'backup-footer-slot'),0x140fc69f0:(484,'display-error-slot'),0x140fc6be0:(2267,'contextual-slot'),0x141e11780:(2245,'load-system'),0x140fb8600:(1858,'back-slot'),0x140fb9a80:(1679,'readiness-warning-slot'),0x140fbe1a0:(1679,'demo-warning-slot'),0x140fbb410:(1068,'backup-header-slot'),0x140fbc5b0:(1321,'error-title-slot'),0x140fe41e0:(203,'backup-sort-comparator')}
symbols={}
procs=ROOT/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv'
for l in procs.read_text(encoding='utf-8').splitlines():
    p=l.split('\t')
    try: addr=int(p[0],16)
    except ValueError:continue
    symbols[addr]=p[-1]
calls={}
rip_refs={}
for address,(size,name) in functions.items():
    p=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn','--start-address='+hex(address),'--stop-address='+hex(address+size),str(EXE)],capture_output=True,text=True,check=True)
    (OUT/(name+'.asm')).write_text(p.stdout,encoding='utf-8')
    targets=[]
    for line in p.stdout.splitlines():
        m=re.search(r'^\s*([a-f0-9]+):\s+(?:callq?|jmp)\s+0x([a-f0-9]+)',line)
        if m:
            target=int(m[2],16)
            targets.append(dict(site='0x'+m[1],target=hex(target),symbol=symbols.get(target)))
    calls[name]=targets
    for target in re.findall(r'# (0x[0-9a-f]+)',p.stdout):
        addr=int(target,16)
        if 0x142925000<=addr<0x143600000:
            try:b=at(addr,128)
            except ValueError:continue
            rip_refs[target]=dict(bytes=b.hex(),printable=''.join(chr(c) if 32<=c<127 else '.' for c in b))
(OUT/'call-targets.json').write_text(json.dumps(calls,indent=2),encoding='utf-8')
(OUT/'rip-data-windows.json').write_text(json.dumps(rip_refs,indent=2),encoding='utf-8')
(OUT/'date-format-table.bin').write_bytes(at(0x142bb3f08,11*24))
strings={}
for address in [0x142f881d8,0x142f88210,0x142f88260,0x142f88276,0x142f882b9,0x142f88350,0x142f8839a,0x142f883f4]:
    strings[hex(address)]=at(address,160).decode('utf-8',errors='backslashreplace')
(OUT/'strings-windows.json').write_text(json.dumps(strings,indent=2),encoding='utf-8')
(OUT/'identity.json').write_text(json.dumps(dict(exe=str(EXE),sha256=sha,image_base='0x140000000',source='bounded original RNE bytes + LLVM disassembly; no Ghidra overlay',functions={hex(a):dict(size=s,name=n,bytes_sha256=hashlib.sha256(at(a,s)).hexdigest(),symbol=symbols.get(a)) for a,(s,n) in functions.items()}),indent=2),encoding='utf-8')
print(json.dumps({'functions':len(functions),'sha256':sha,'strings':strings},indent=2))
