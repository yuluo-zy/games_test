from pathlib import Path
import json,hashlib,pefile,subprocess,concurrent.futures
OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[2]
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase
symbols=json.loads((ROOT/'evidence/render-architecture/public-symbols.json').read_text(encoding='utf-8'))
print(type(symbols).__name__)
if isinstance(symbols,dict):print(list(symbols)[:8])
addresses=[0x1407a45b0,0x1407a4700,0x140c99e30,0x14127cab0,0x14127c950,0x14127c970]
all_symbols=[]
def recurse(x):
 if isinstance(x,list):
  for r in x:recurse(r)
 elif isinstance(x,dict):
  for key in ['va','address']:
   if key in x:
    try:
     value=int(x[key],16) if isinstance(x[key],str)else x[key]
     if isinstance(value,int):all_symbols.append(value)
    except (ValueError,TypeError):pass
  for value in x.values():
   if isinstance(value,(dict,list)):recurse(value)
recurse(symbols);all_symbols=sorted(set(all_symbols))
targets=[]
for a in addresses:
 ranges=[(base+e.struct.BeginAddress,base+e.struct.EndAddress)for e in pe.DIRECTORY_ENTRY_EXCEPTION if base+e.struct.BeginAddress==a]
 if ranges: _,end=ranges[0];basis='original PE unwind contiguous range'
 else:
  end=next(x for x in all_symbols if x>a);basis='next original PDB public symbol upper bound; not original full lexical body'
 size=end-a;assert 0<size<=24000,(hex(a),size)
 targets.append({'address':hex(a),'size':size,'bytes_sha256':hashlib.sha256(pe.get_data(a-base,size)).hexdigest(),'range_basis':basis})
job=[{'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':targets}},{'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':','.join(hex(a)for a in addresses),'fields':'signature,decompiled_code,callees,body'}}]
(OUT/'bounded-job.json').write_text(json.dumps(job,indent=2)+'\n',encoding='utf-8')
def dump(m):
 result=subprocess.run(['C:/Program Files/LLVM/bin/llvm-pdbutil.exe','dump','-symbols',f'-modi={m}','D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb'],stdout=subprocess.PIPE,stderr=subprocess.PIPE,check=True)
 (OUT/f'module-{m}.txt').write_bytes(result.stdout)
 return {'module':m,'bytes':len(result.stdout),'sha256':hashlib.sha256(result.stdout).hexdigest()}
with concurrent.futures.ThreadPoolExecutor(max_workers=4)as pool:modules=list(pool.map(dump,[533,768,493,1676]))
(OUT/'module-dumps.json').write_text(json.dumps(modules,indent=2)+'\n',encoding='utf-8')
print(targets)
