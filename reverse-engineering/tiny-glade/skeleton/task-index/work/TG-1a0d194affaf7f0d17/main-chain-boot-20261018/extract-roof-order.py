from pathlib import Path
import pefile, capstone, json
p=Path('ljxsj_92385/Tiny Glade/tiny-glade.rne')
pe=pefile.PE(str(p),fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True
out=Path('games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-1a0d194affaf7f0d17/main-chain-boot-20261018')
start=0x141302d60;size=2000
lines=[];constants=[]
for i in md.disasm(pe.get_data(start-base,size),start):
 lines.append(f'{i.address:x}: {i.mnemonic} {i.op_str}')
 for op in i.operands:
  if op.type==capstone.x86.X86_OP_MEM and op.mem.base==capstone.x86.X86_REG_RIP:
   addr=i.address+i.size+op.mem.disp
   constants.append({'instruction':hex(i.address),'va':hex(addr),'bytes16':pe.get_data(addr-base,16).hex()})
(out/'141302d60-roof-initial-map-full-rne.asm').write_text('\n'.join(lines)+'\n')
# Snapshot the masks as well as pi/half/one; retain actual original bytes.
items=[]
for va in [0x14129f5fe,0x14129f606,0x14129f60e,0x14129f616,0x14129f61e,0x14129f627]:
 i=next(md.disasm(pe.get_data(va-base,16),va))
 op=next(op for op in i.operands if op.type==capstone.x86.X86_OP_MEM)
 addr=i.address+i.size+op.mem.disp
 items.append({'instruction':hex(va),'text':f'{i.mnemonic} {i.op_str}','constant_va':hex(addr),'bytes16':pe.get_data(addr-base,16).hex()})
pe.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_IMPORT']])
i=next(md.disasm(pe.get_data(0x142923fd0-base,16),0x142923fd0))
addr=i.address+i.size+i.operands[0].mem.disp
imports=[{'dll':d.dll.decode(),'name':imp.name.decode() if imp.name else None,'iat_va':hex(imp.address)} for d in pe.DIRECTORY_ENTRY_IMPORT for imp in d.imports if imp.address==addr]
(out/'roof-operation-constants.json').write_text(json.dumps({'initial_map_constants':constants,'postprocess_constants':items,'transcendental_thunk':{'text':f'{i.mnemonic} {i.op_str}','iat_va':hex(addr),'import':imports}},indent=2))
print(json.dumps({'postprocess_constants':items,'cos_thunk_import':imports},indent=2))
