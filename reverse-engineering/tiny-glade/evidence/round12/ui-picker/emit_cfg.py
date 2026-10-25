from pathlib import Path
import capstone,json,struct
root=Path(__file__).resolve().parent
cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);cs.detail=True
manifest=json.loads((root/'native-manifest.json').read_text());result=[]
for f in manifest:
 if f['name']not in ['hover_intersection','picker_closure2','picker_occlusion_retain','sort_canvas_comparator','sort_distance_comparator']:continue
 va=int(f['va'],16);instructions=list(cs.disasm((root/(f['name']+'.bin')).read_bytes(),va));edges=[];calls=[]
 for i in instructions:
  if i.mnemonic.startswith('j'):
   imm=i.operands[0].type==capstone.x86.X86_OP_IMM
   edges.append(dict(address=hex(i.address),instruction=i.mnemonic+' '+i.op_str,target=hex(i.operands[0].imm)if imm else None,fallthrough=hex(i.address+i.size)if i.mnemonic!='jmp'else None))
  elif i.mnemonic=='call':calls.append(dict(address=hex(i.address),target=i.op_str))
 result.append(dict(**f,conditional_and_jump_edges=edges,calls=calls))
result.append(dict(table_va='0x142adfb28',width=4,format='signed offset from table base',targets=['0x140ac414e','0x140ac4463','0x140ac4333','0x140ac4391','0x140ac429c','0x140ac4597','0x140ac45ba','0x140ac40ff'],tags=['Sphere','SphereCloud','TriangleMesh','ScreenAabb','WorldCanvas','TerrainCanvas','PlaneDisc','Empty']))
(root/'native-cfg.json').write_text(json.dumps(result,indent=2))
