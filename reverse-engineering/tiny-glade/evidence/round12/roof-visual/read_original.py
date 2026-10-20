from pathlib import Path
import struct, subprocess, json, hashlib, re

root = Path(__file__).resolve().parent
source = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
b = source.read_bytes()
pe = struct.unpack_from('<I', b, 0x3c)[0]
n = struct.unpack_from('<H', b, pe + 6)[0]
opt = pe + 24
base = struct.unpack_from('<Q', b, opt + 24)[0]
sec = opt + struct.unpack_from('<H', b, pe + 20)[0]
def read(va, size):
    rva = va - base
    for i in range(n):
        _, vs, addr, rs, ptr = struct.unpack_from('<8sIIII', b, sec + 40*i)
        if addr <= rva < addr + max(vs, rs):
            return b[ptr+rva-addr:ptr+rva-addr+size]
    raise ValueError(hex(va))
values = {}
for va in [0x1429cd078, 0x142925904, 0x142925988, 0x1429a07d0]:
    raw = read(va,4)
    values[hex(va)] = {'hex': raw.hex(), 'f32': struct.unpack('<f', raw)[0]}
for va, size in [(0x142ff8690,0x28),(0x142ff8680,1),(0x142ff834a,0x1a),(0x142ff8662,0x1a)]:
    values[hex(va)] = {'hex': read(va,size).hex(), 'text': read(va,size).decode('utf8', errors='replace')}
for va in [0x142a0c7d8,0x142a0c7f0,0x142a0c808,0x142a0c820,0x142a0c838,0x142a0c850,0x142a0c868]:
    words=struct.unpack('<3Q',read(va,24))
    values[hex(va)]={'words':[hex(v) for v in words]}
values['source_sha256'] = hashlib.sha256(b).hexdigest()
values['functions'] = {}
for name, va, size in [('stone-floor',0x1421a6d40,839),('assemble',0x141a36700,2965),('roof-trigger',0x1421a3220,12048),('create',0x1421aaea0,1803)]:
    output = subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn',f'--start-address={hex(va)}',f'--stop-address={hex(va+size)}',str(source)], capture_output=True, check=True).stdout
    (root/f'{name}.original.asm.txt').write_bytes(output)
    values['functions'][name] = {'va':hex(va),'size':size,'sha256':hashlib.sha256(read(va,size)).hexdigest()}
symbols=json.loads((root.parents[1]/'render-architecture/public-symbols.json').read_text())
symbols={s['va']:s.get('demangled',s['symbol']) for s in symbols}
for va in [0x140ddbf90,0x140ddffb0,0x140ddf400,0x140de10b0,0x140de19b0,0x140de4d60,0x140de55d0,0x141a37900,0x1419ee830,0x140ddba10,0x140ddeec0,0x140dddf00,0x140ddda40,0x141a1e290,0x140de47d0,0x141a20450,0x140de4b30,0x141a35460,0x141a35940,0x141a36040,0x140de4980,0x141a20600,0x141a33c00,0x141a34a50,0x141a30fc0,0x141a31980,0x141a32920,0x141a32ff0]:
    output=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn',f'--start-address={hex(va)}',f'--stop-address={hex(va+1500)}',str(source)],capture_output=True,check=True).stdout
    (root/f'register-{va:x}.original.asm.txt').write_bytes(output)
    main=output.decode().split('retq',1)[0]
    names=[]
    for m in re.finditer(r'leaq[^\n]*%(?:rax|rcx)[^\n]*# (0x[0-9a-f]+)\n[^\n]*\n[^\n]*movq\s+\$(0x[0-9a-f]+)',main):
        addr,size=int(m[1],16),int(m[2],16)
        if 20<size<500:
            names.append({'address':hex(addr),'length':size,'text':read(addr,size).decode('utf8',errors='replace')})
    calls=[]
    for addr in re.findall(r'callq\s+(0x[0-9a-f]+)',main):
        calls.append({'va':addr,'symbol':symbols.get(addr)})
    values[hex(va)]={'system_names':names,'calls':calls}
(root/'original-evidence.json').write_text(json.dumps(values,indent=2),encoding='utf8')
print(json.dumps(values,indent=2))
