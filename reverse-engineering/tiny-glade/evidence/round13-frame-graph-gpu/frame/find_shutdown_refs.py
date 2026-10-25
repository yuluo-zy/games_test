from pathlib import Path
import re,struct,json,subprocess
ROOT=Path('D:/game'); OUT=Path(__file__).parent/'zero-extent'
blob=(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
pe=struct.unpack_from('<I',blob,0x3c)[0]
num=struct.unpack_from('<H',blob,pe+6)[0]
opt=struct.unpack_from('<H',blob,pe+20)[0]
base=struct.unpack_from('<Q',blob,pe+48)[0]
found=[];target=0x140b2ba40
for i in range(num):
    off=pe+24+opt+40*i
    vsize,rva,size,raw=struct.unpack_from('<IIII',blob,off+8)
    flags=struct.unpack_from('<I',blob,off+36)[0]
    if not flags&0x20000000:continue
    code=blob[raw:raw+size];va=base+rva
    for match in re.finditer(rb'\xe8[\x00-\xff]{4}|[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d][\x00-\xff]{4}',code):
        ins=match.group();dest=va+match.end()+struct.unpack_from('<i',ins,len(ins)-4)[0]
        if dest==target:found.append({'va':hex(va+match.start()),'bytes':ins.hex(),'kind':'call' if ins[0]==0xe8 else 'lea'})
(OUT/'shutdown-native-refs.json').write_text(json.dumps({'target':hex(target),'refs':found,'scope':'Raw direct E8 and RIP-relative LEA only; indirect edges not enumerated.'},indent=2)+'\n')
for ref in found:
    va=int(ref['va'],16)
    result=subprocess.check_output(['C:/Program Files/LLVM/bin/llvm-objdump.exe','--disassemble',f'--start-address={va}',f'--stop-address={va+150}',str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne')],text=True)
    (OUT/(ref['va']+'-shutdown-ref.asm')).write_text(result)
print(json.dumps(found))
