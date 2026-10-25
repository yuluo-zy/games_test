from pathlib import Path
import subprocess,re,json,sys
ROOT=Path(__file__).resolve().parents[2];GAME=ROOT.parents[2];OUT=ROOT/'evidence/parallel-camera';LLVM='/opt/homebrew/Cellar/llvm/22.1.2/bin/llvm-pdbutil'
for v in (sys.argv[1:] or ['B']):
 root={'A':'Tiny Glade','B':'ljxsj_92385/Tiny Glade','C':'tiny.glade/XJ07036/Tiny Glade v1.16.0'}[v];pdb=GAME/root/'tiny_glade.pdb'
 p=subprocess.Popen([LLVM,'dump','--types',str(pdb)],stdout=subprocess.PIPE,text=True);blocks=[];block=[]
 for l in p.stdout:
  if re.match(r'  0x[0-9A-F]+ \|',l):
   if any(t in ''.join(block) for t in ['country_core::systems::camera_rig','country_core::systems::main_camera']):blocks.append(''.join(block))
   block=[]
  block.append(l)
 p.wait();text=''.join(blocks);(OUT/f'{v}-types-candidates.txt').write_text(text)
 ids=[]
 for b in blocks:
  if 'LF_STRUCTURE' in b or 'LF_ENUM' in b:
   if 'forward ref' not in b:
    print(b[:1500]);m=re.search(r'field list: (0x[0-9A-F]+)',b)
    if m:ids.append(m[1])
 result=subprocess.check_output([LLVM,'dump','--types',*[f'--type-index={i}' for i in dict.fromkeys(ids)],str(pdb)],text=True) if ids else ''
 (OUT/f'{v}-type-fields.txt').write_text(result);print('fields',len(ids))
