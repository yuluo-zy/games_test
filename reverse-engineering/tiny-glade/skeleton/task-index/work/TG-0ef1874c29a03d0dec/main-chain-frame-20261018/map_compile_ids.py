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
for name,value in [('0xa68561b396784eaa', 11999104206150061738), ('0x8fd9dc19c7973bca', 10365558020650777546), ('0x47c94cb78283a72e', 5172749998064117550), ('0x4da5153112b09210', 5594901412588851728), ('0xa284155e68bfeb3f', 11710508426298649407), ('0x109ca79d5cccc2d1', 1197015895282336465), ('0x8226cc786e679355', 9378408091666060117), ('0x662978458699a5ef', 7361547305920079343), ('0x162d45f443f2465a', 1598010358200682074), ('0xd38dae617fb79c7a', 15244032047496797306), ('0x35385c45b6f578e9', 3834916537198016745), ('0x1f18d9c9bdba6c60', 2240780275111586912), ('0x6831e8f98c915159', 7508038212306162009), ('0x4a142e09642375b9', 5337942076210509241), ('0xc2b8e941c91a6428', 14031221107828876328), ('0xa4c02661e67b048b', 11871530819669132427)]:
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
  if 'insert_resource' in r['name']: print(r)
