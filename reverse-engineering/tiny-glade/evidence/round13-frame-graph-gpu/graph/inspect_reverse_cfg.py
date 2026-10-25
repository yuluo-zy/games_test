from pathlib import Path
import sys,struct,json,hashlib,subprocess
R=Path('D:/game/reverse-engineering/tiny-glade');O=R/'evidence/round13-frame-graph-gpu/graph'
sys.path.insert(0,str(R/'tools/render-passes'));import spirv_parse
G=json.loads((R/'skeleton/_tools/spirv.core.grammar.json').read_text());ops={o['opcode']:o for o in G['instructions']}
p=R/'evidence/render-passes/shaders/oracle-bin/_blur_pyramid_rev_blur.cs.hlsl_cs_main_0.bin'
off,payload=next(spirv_parse.iter_modules(p.read_bytes()));ws=struct.unpack('<'+'I'*(len(payload)//4),payload);ins=[];names={};i=5
while i<len(ws):
 wc=ws[i]>>16;op=ws[i]&65535;a=list(ws[i+1:i+wc]);on=ops.get(op,{}).get('opname',str(op));ins.append((i,on,a))
 if op==5:names[a[0]]=struct.pack('<'+'I'*(len(a)-1),*a[1:]).split(b'\0')[0].decode(errors='replace')
 i+=wc
lines=[f'Original module offset={off} sha256={hashlib.sha256(payload).hexdigest()}',repr(names)]
for pos,on,a in ins:
 if on in ['OpString','OpSource','OpSourceContinued','OpLine','OpNoLine']:continue
 lines.append(f'{pos:04d} {on} '+','.join(f'{n}({names[n]})' if n in names else str(n) for n in a))
(O/'reverse-blur-original-spirv.txt').write_text('\n'.join(lines),encoding='utf8')
p=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn','--start-address=0x1409ed860','--stop-address=0x1409ee1d7',str(R/'workspace/input/tiny-glade.exe')],capture_output=True,text=True,check=True)
(O/'bidirectional-blur-original.asm').write_text(p.stdout,encoding='utf8')
print('\n'.join(lines[-95:]))
