import json, hashlib, struct, bisect
from pathlib import Path
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64, CS_OP_MEM
from capstone.x86_const import X86_REG_RIP
root=Path('D:/game/reverse-engineering/tiny-glade')
out=root/'evidence/shared-foundation-2026-10-13/utils-tabloid'
binary=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw=binary.read_bytes(); pe=pefile.PE(data=raw,fast_load=True)
pe.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_EXCEPTION']])
base=pe.OPTIONAL_HEADER.ImageBase
symbols=json.loads((out/'B-symbols.json').read_text())
full=json.loads((root/'evidence/render-architecture/public-symbols.json').read_text())
starts=sorted({int(s['va'],16) for s in full if s['va']})
md=Cs(CS_ARCH_X86,CS_MODE_64);md.detail=True
manifest={'binary':str(binary),'sha256':hashlib.sha256(raw).hexdigest(),'functions':[]}
symbols += [s for s in full if s['va'] in ['0x141bb0250','0x1412f4d70']][:2]
symbols += [s for s in full if any('utils..'+n in s['demangled'] for n in ['halver','comparative_space','float_grid..FloatGridSampler','selective_deserialize']) and not s['demangled'].startswith('core::')]
for s in symbols:
 if not s['va']: continue
 if any(t in s['demangled'] for t in ['utils::geometry','utils::curve','calculate_hash','serde','SystemParam','utils::components']):continue
 a=int(s['va'],16); size=next((x-a for x in starts if x>a),256)
 for entry in pe.DIRECTORY_ENTRY_EXCEPTION:
  if entry.struct.BeginAddress==a-base:
   size=entry.struct.EndAddress-entry.struct.BeginAddress;break
 size=min(size,20000)
 data=pe.get_data(a-base,size)
 lines=[]
 for i in md.disasm(data,a):
  note=''
  for op in i.operands:
   if op.type==CS_OP_MEM and op.mem.base==X86_REG_RIP:
    target=i.address+i.size+op.mem.disp;data4=pe.get_data(target-base,4)
    if len(data4)==4:note+=f' ; [{target:#x}]={data4.hex()} f32={struct.unpack("<f",data4)[0]}'
  lines.append(f'{i.address:#x}: {i.mnemonic} {i.op_str}{note}')
 text='\n'.join(lines)
 name=f'{a:x}.asm.txt'; (out/name).write_text(s['demangled']+'\n'+text,encoding='utf-8')
 manifest['functions'].append({'name':s['demangled'],'address':s['va'],'size_to_next_symbol':size,'assembly':name})
(out/'native-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf-8')
print(manifest['sha256'],len(manifest['functions']))
