"""Check every retained B owned procedure's original bytes and control-flow boundary.
Read-only PE/PDB evidence; no executable launch or persistent DB mutation.
Run from trusted game root with tools/venv/bin/python3 -I.
"""
from pathlib import Path
import hashlib,json,struct,re
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
ROOT=Path(__file__).resolve().parents[2];GAME=ROOT.parents[2];OUT=ROOT/'evidence/parallel-critters'
original=GAME/'ljxsj_92385/Tiny Glade/tiny-glade.rne';patched=original.with_name('tiny-glade.exe')
def digest(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def pe(path):
 raw=path.read_bytes();p=struct.unpack_from('<I',raw,60)[0];opt=p+24;sz=struct.unpack_from('<H',raw,p+20)[0];base=struct.unpack_from('<Q',raw,opt+24)[0]
 secs=[]
 for i in range(struct.unpack_from('<H',raw,p+6)[0]):
  vs,rva,n,off=struct.unpack_from('<IIII',raw,opt+sz+40*i+8);secs.append((rva,n,off,vs))
 def get(va,size):
  r=va-base
  for rva,n,off,_ in secs:
   if rva<=r<rva+n and r+size<=rva+n:return raw[off+r-rva:off+r-rva+size]
  raise ValueError('Not file backed')
 return base,get
_,oread=pe(original);_,pread=pe(patched);cs=Cs(CS_ARCH_X86,CS_MODE_64)
assert digest(original)=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
assert digest(patched)=='cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323'
procedures=json.loads((OUT/'B-procedures.json').read_text())['definition_owned_procedures'];db={x['va']:x for x in json.loads((OUT/'B-decompile/report.json').read_text())['results']};rows=[]
for p in procedures:
 a=int(p['va'],16);size=p['code_size'];b=oread(a,size);c=pread(a,size);ins=list(cs.disasm(b,a));outside=[];indirect=[]
 for i in ins:
  if i.mnemonic.startswith('j'):
   if i.op_str.startswith('0x'):
    t=int(i.op_str,16)
    if not a<=t<a+size:outside.append({'from':hex(i.address),'instruction':i.mnemonic+' '+i.op_str,'target':hex(t)})
   else:indirect.append({'from':hex(i.address),'instruction':i.mnemonic+' '+i.op_str})
 record={'va':p['va'],'name':p['name'],'pdb_module':p['module'],'pdb_code_size':size,'pdb_end_exclusive':hex(a+size),'original_range_sha256':hashlib.sha256(b).hexdigest(),'patched_range_sha256':hashlib.sha256(c).hexdigest(),'original_equals_patched_range':b==c,'linear_decoded_bytes':sum(i.size for i in ins),'linear_instruction_count':len(ins),'direct_jump_targets_outside_pdb_range':outside,'indirect_jumps':indirect,'db_body_bytes':db.get(p['va'],{}).get('body_bytes'),'decompile_success':db.get(p['va'],{}).get('decompiled'),'trusted_decompiler_boundary':False,'boundary_status':'needs_cfg_review' if outside or indirect else 'linear_no_external_jump_observed_not_reachability_proof'}
 rows.append(record)
report={'baseline':'B','original':str(original),'original_sha256':digest(original),'patched':str(patched),'patched_sha256':digest(patched),'command':'tools/venv/bin/python3 -I scripts/parallel-critters/check_boundaries.py','rows':rows,'limitations':['Linear decode is not a CFG proof and may include padding/data','External jumps may be legitimate panic/shared tails; each must be resolved','Indirect jumps/jump-table edges need original table and reachability evidence','DB body-size inequality may be noncontiguous body/padding exclusion, not necessarily wrong semantics','No row is automatically accepted as complete behavior']}
(OUT/'B-boundaries.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'procedures':len(rows),'byte_equal':sum(x['original_equals_patched_range'] for x in rows),'external_jumps':sum(bool(x['direct_jump_targets_outside_pdb_range']) for x in rows),'indirect_jumps':sum(bool(x['indirect_jumps']) for x in rows)}))
