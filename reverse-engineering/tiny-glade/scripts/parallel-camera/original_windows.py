from pathlib import Path
import json,struct,hashlib
import pefile,capstone
ROOT=Path(__file__).resolve().parents[2];GAME=ROOT.parents[2];OUT=ROOT/'evidence/parallel-camera/B-original';OUT.mkdir(exist_ok=True)
b=GAME/'ljxsj_92385/Tiny Glade/tiny-glade.rne';blob=b.read_bytes();pe=pefile.PE(data=blob);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image();cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
inv=json.loads((ROOT/'evidence/parallel-camera/B-inventory.json').read_text());entries={int(x['va'],16):x['name'] for x in inv['game_candidate_public_records']+inv['glue_records'] if x['function'] and ('::systems::' in x['name'] or x['name'].startswith('_$LT$country_core..systems..camera_rig..'))}
rows=[]
pdb_ranges={int(x['va'],16):x['size'] for x in json.loads((ROOT/'evidence/parallel-camera/B-pdb-ranges.json').read_text())}
for a,n in entries.items():
 fn=next((x.struct for x in pe.DIRECTORY_ENTRY_EXCEPTION if x.struct.BeginAddress<=a-base<x.struct.EndAddress),None)
 if fn is None:continue
 lo,hi=a,a+pdb_ranges.get(a,fn.EndAddress-(a-base))
 text='\n'.join(f'{i.address:x} {i.mnemonic} {i.op_str}' for i in cs.disasm(image[lo-base:hi-base],lo))+'\n'
 (OUT/(f'{a:x}.asm.txt')).write_text(text);rows.append({'va':hex(a),'name':n,'begin':hex(lo),'end':hex(hi),'bytes':hi-lo,'sha256':hashlib.sha256(image[lo-base:hi-base]).hexdigest()})
for a,count in [(0x142aabc70,6),(0x142aabc88,4),(0x142aabc98,4),(0x142aabcc0,5),(0x142aabcd4,6),(0x142aabd04,7),(0x142aabd18,6),(0x142aabd30,6),(0x142aabd48,6),(0x142aabd60,6),(0x142aabd78,6),(0x142aabd90,6)]:
 vals=struct.unpack_from('<'+'i'*count,image,a-base);print(hex(a),[hex(a+x) for x in vals])
(OUT/'ranges.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(blob).hexdigest(),'target_executed':False,'ranges':rows},indent=2)+'\n');print('ranges',len(rows))
