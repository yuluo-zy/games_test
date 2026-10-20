from pathlib import Path
import struct,hashlib,json,subprocess,re
R=Path('D:/game');O=Path(__file__).resolve().parent;X=R/'ljxsj_92385/Tiny Glade/tiny-glade.rne';raw=X.read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,60)[0];base=struct.unpack_from('<Q',raw,pe+48)[0];opt=struct.unpack_from('<H',raw,pe+20)[0]
secs=[struct.unpack_from('<4I',raw,pe+24+opt+i*40+8) for i in range(struct.unpack_from('<H',raw,pe+6)[0])]
def read(a,n):
 for _,rva,size,off in secs:
  if rva<=a-base and a-base+n<=rva+size:return raw[off+a-base-rva:off+a-base-rva+n]
 raise ValueError(hex(a))
a,n=0x1409d49a0,5361;b=read(a,n);p=O/'startup_critters.original.asm'
p.write_bytes(subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={a:#x}',f'--stop-address={a+n:#x}',str(X)],check=True,stdout=subprocess.PIPE).stdout)
procs={}
for line in (R/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf8').splitlines():
 f=line.split('\t');procs.setdefault(f[0],[]).append({'size':int(f[1]),'module':int(f[2]),'symbol':f[-1]})
calls=[{'va':v,'pdb':procs.get(v,[])} for v in dict.fromkeys(re.findall(r'callq\s+(0x[0-9a-f]+)',p.read_text()))]
consts={hex(v):{'hex':read(v,s).hex(),'f32':list(struct.unpack('<'+'f'*(s//4),read(v,s))),'ascii':read(v,s).decode('ascii','replace')} for v,s in [(0x142ac1928,12),(0x14295a0e8,4),(0x1429a0788,4),(0x1429cd034,4),(0x142ac20a5,0x250),(0x142a8c620,16)]}
(O/'native-manifest.json').write_text(json.dumps({'original_sha256':sha,'va':hex(a),'size':n,'range_sha256':hashlib.sha256(b).hexdigest(),'pdb':procs.get(hex(a)),'calls':calls,'constants':consts},indent=2),encoding='utf8')
job=[{'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':[{'address':hex(a),'size':n,'bytes_sha256':hashlib.sha256(b).hexdigest(),'range_basis':'PDB S_GPROC32 exact author5361B from procs.tsv; original RNE SHA checked'}]}},{'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':hex(a),'fields':'signature,decompiled_code,callees'}}]
(O/'decomp-job.json').write_text(json.dumps(job),encoding='utf8');print(json.dumps({'sha':hashlib.sha256(b).hexdigest(),'calls':calls,'constants':consts},indent=2))
