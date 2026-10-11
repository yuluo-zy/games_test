"""Read only original PDB/PE; persist critter-owned inventory, no analysis/import."""
from pathlib import Path
import hashlib,json,re,struct,subprocess,sys
ROOT=Path(__file__).resolve().parents[2]
GAME=ROOT.parents[2]
OUT=ROOT/'evidence/parallel-critters'
LLVM=Path('/opt/homebrew/Cellar/llvm/22.1.2/bin/llvm-pdbutil')
BASES={'A':GAME/'Tiny Glade','B':GAME/'ljxsj_92385/Tiny Glade','C':GAME/'tiny.glade/XJ07036/Tiny Glade v1.16.0'}
def run(v):
 b=BASES[v];binary=b/('tiny-glade.rne' if v=='B' else 'tiny-glade.exe');pdb=b/'tiny_glade.pdb'
 expected=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines'][v]
 def digest(p):
  with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
 identities={'baseline':v,'binary':str(binary),'pdb':str(pdb),'binary_sha256':digest(binary),'pdb_sha256':digest(pdb),'target_executed':False}
 assert identities['binary_sha256']==expected['binary']['sha256']
 assert identities['pdb_sha256']==expected['pdb']['sha256']
 sec=expected['pe']['sections'];base=expected['pe']['image_base'];active=None;direct=[];glue=[]
 cmd=[str(LLVM),'dump','--publics',str(pdb)]
 p=subprocess.Popen(cmd,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
 for line in p.stdout:
  if 'S_PUB32' in line:
   m=re.search(r'`([^`]+)`',line);active=m[1] if m else None
  elif active and 'addr =' in line:
   name,active=active,None
   if 'system_critters' not in name:continue
   m=re.search(r'addr = (\d+):(\d+)',line)
   if not m:continue
   sn,off=map(int,m.groups());va=base+sec[sn-1]['rva']+off
   row={'mangled':name,'section':sn,'section_offset':off,'va':hex(va),'rva':hex(va-base),'function':'flags = function' in line,'classification':'game_owned' if name.startswith('_ZN15system_critters') else 'generic_or_derive_glue'}
   (direct if row['classification']=='game_owned' else glue).append(row)
 rc=p.wait();err=p.stderr.read();assert rc==0,err
 def legacy(n):
  z=n[3:] if n.startswith('_ZN') else n;parts=[]
  while z and z[0].isdigit():
   m=re.match(r'(\d+)',z);k=int(m[1]);z=z[len(m[1]):];parts.append(z[:k]);z=z[k:]
  return '::'.join(parts[:-1] if parts and re.fullmatch('h[a-f0-9]{16}',parts[-1]) else parts) or n
 for x in direct+glue:x['name']=legacy(x['mangled']);x['canonical_candidate']=re.sub('17h[a-f0-9]{16}E$','E',x['mangled'])
 sources=json.loads((ROOT/f'evidence/multi-version/{v}-sources.json').read_text())
 report={'identities':identities,'command':cmd,'source_files':[x for x in sources['files'] if x['path'].startswith('crates/systems/critters/')],'game_owned_public_records':direct,'excluded_glue_records':glue,'limitations':['Public inventory does not inventory all inlined or private procedures','Definition ownership uses direct system_critters namespace; derive/default/schema require individual business check','Shared VA aliases remain distinct records, not distinct functions','Original Rust source text not available']}
 (OUT/f'{v}-inventory.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
 print(json.dumps({'baseline':v,'direct':len(direct),'unique_va':len({x['va'] for x in direct}),'glue':len(glue),'files':len(report['source_files'])}),flush=True)
if __name__=='__main__':
 for v in (sys.argv[1:] or list('ABC')):run(v)
