from pathlib import Path
import pefile,hashlib,bisect,json
root=Path('/Users/yuluo/Desktop/game')
raw=(root/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True)
procs=[]
for line in (root/'games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text().splitlines():
 cols=line.split('\t')
 try: procs.append((int(cols[0],16),int(cols[1]),cols[4]))
 except (ValueError,IndexError): pass
procs.sort();starts=[p[0] for p in procs]
out={}
for name,value in [('E',0x874b1541877dacfd),('G',0xb65bf3141f93e938),('F',0x47c94cb78283a72e),('B',0xf0b963d163fe760f)]:
 matches=[];off=-1
 while True:
  off=raw.find(value.to_bytes(8,'little'),off+1)
  if off<0: break
  try: addr=pe.OPTIONAL_HEADER.ImageBase+pe.get_rva_from_offset(off)
  except: continue
  idx=bisect.bisect_right(starts,addr)-1
  if idx<0: continue
  start,size,sym=procs[idx]
  matches.append({'site':hex(addr),'function':hex(start),'name':sym,'in_pdb_extent':addr<start+size})
 out[name]=matches
Path(__file__).with_suffix('.json').write_text(json.dumps(out,indent=2))
for name,rows in out.items():
 print(name,len(rows))
 for r in rows:
  if any(s in r['name'] for s in ['insert_resource','resource_scope','init_resource','new_from']): print(r)
