"""Read-only PDB/PE camera inventory; no program import or analysis."""
from pathlib import Path
import hashlib,json,re,subprocess,sys
ROOT=Path(__file__).resolve().parents[2]
GAME=ROOT.parents[2]
OUT=ROOT/'evidence/parallel-camera'
LLVM=Path('/opt/homebrew/Cellar/llvm/22.1.2/bin/llvm-pdbutil')
BASES={'A':GAME/'Tiny Glade','B':GAME/'ljxsj_92385/Tiny Glade','C':GAME/'tiny.glade/XJ07036/Tiny Glade v1.16.0'}
def legacy(n):
 z=n[3:] if n.startswith('_ZN') else n; parts=[]
 while z and z[0].isdigit():
  m=re.match(r'(\d+)',z);k=int(m[1]);z=z[len(m[1]):];parts.append(z[:k]);z=z[k:]
 return '::'.join(parts[:-1] if parts and re.fullmatch('h[a-f0-9]{16}',parts[-1]) else parts) or n
for v in (sys.argv[1:] or list('ABC')):
 b=BASES[v];binary=b/('tiny-glade.rne' if v=='B' else 'tiny-glade.exe');pdb=b/'tiny_glade.pdb'
 expected=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines'][v]
 def digest(p):
  with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
 ids={'baseline':v,'binary':str(binary),'pdb':str(pdb),'binary_sha256':digest(binary),'pdb_sha256':digest(pdb),'target_executed':False}
 assert ids['binary_sha256']==expected['binary']['sha256'];assert ids['pdb_sha256']==expected['pdb']['sha256']
 sec=expected['pe']['sections'];base=expected['pe']['image_base'];active=None;direct=[];glue=[]
 cmd=[str(LLVM),'dump','--publics',str(pdb)]
 p=subprocess.Popen(cmd,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
 for line in p.stdout:
  if 'S_PUB32' in line:
   m=re.search(r'`([^`]+)`',line);active=m[1] if m else None
  elif active and 'addr =' in line:
   name,active=active,None
   if not any(t in name for t in ['main_camera','camera_rig','MainCamera','NormalCamera','FreeCamera','WalkCamera','ButterflyCamera','CameraBody','CameraParameters']):continue
   m=re.search(r'addr = (\d+):(\d+)',line)
   if not m:continue
   sn,off=map(int,m.groups());va=base+sec[sn-1]['rva']+off
   row={'mangled':name,'name':legacy(name),'section':sn,'section_offset':off,'va':hex(va),'rva':hex(va-base),'function':'flags = function' in line,'classification':'game_candidate' if name.startswith('_ZN12country_core') else 'generic_or_derive_glue'}
   (direct if row['classification']=='game_candidate' else glue).append(row)
 rc=p.wait();err=p.stderr.read();assert rc==0,err
 sources=json.loads((ROOT/f'evidence/multi-version/{v}-sources.json').read_text())
 report={'identities':ids,'command':cmd,'source_files':[x for x in sources['files'] if '/camera' in x['path'] or 'main_camera' in x['path']],'game_candidate_public_records':direct,'glue_records':glue,'limitations':['Public inventory is not complete private/inlined inventory','Definition and caller checks still required','Shared aliases are not distinct routines','No author source text recovered']}
 OUT.mkdir(exist_ok=True);(OUT/f'{v}-inventory.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
 print(json.dumps({'baseline':v,'direct':len(direct),'unique_va':len({x['va'] for x in direct}),'glue':len(glue),'files':len(report['source_files'])}),flush=True)
