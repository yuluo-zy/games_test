from pathlib import Path
import pefile,mmap,bisect,csv,json
w=Path('games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-c459c7f73345e24709/main-chain-boot-20261018/world-terrain-20261018')
p='ljxsj_92385/Tiny Glade/tiny-glade.rne';pe=pefile.PE(p,fast_load=True)
procs=[]
with open('games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv') as f:
 for r in csv.reader(f,delimiter='\t'):
  try:procs.append((int(r[0],16),int(r[1]),r[-1]))
  except (ValueError,IndexError):pass
procs.sort();starts=[p[0] for p in procs]
ids={'W3_strokes':0x1e38c16726d0fb8d,'W3_W10_event_counts':0xcda6af2f6afb130e,'W4_rock':0xfa44538b95ae76c6,'W8_suppress':0x48b321ef779dc301,'W9_mode':0xc2b8e941c91a6428,'W11_buildings':0x9b126e795fb22adf}
out={}
with open(p,'rb') as f:
 raw=mmap.mmap(f.fileno(),0,access=mmap.ACCESS_READ)
 for key,value in ids.items():
  matches=[];off=-1
  while True:
   off=raw.find(value.to_bytes(8,'little'),off+1)
   if off<0:break
   try:addr=pe.OPTIONAL_HEADER.ImageBase+pe.get_rva_from_offset(off)
   except:continue
   idx=bisect.bisect_right(starts,addr)-1
   if idx<0:continue
   start,size,symbol=procs[idx]
   if addr<start+size:matches.append({'site':hex(addr),'function':hex(start),'symbol':symbol})
  out[key]=matches
(w/'resource-id-producer-mapping.json').write_text(json.dumps(out,indent=2))
for key,vals in out.items():
 print(key)
 for v in vals:
  if any(x in v['symbol'] for x in ['insert_resource<','init_state<','init_resource<','register_event<']):print(v)
