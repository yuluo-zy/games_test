"""Prepare bounded original B business bodies; never infer ABI from PDB void."""
import bisect, hashlib, json
from pathlib import Path
import pefile
OUT=Path(__file__).parent
ROOT=OUT.parents[2]
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True)
pe.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_EXCEPTION']])
base=pe.OPTIONAL_HEADER.ImageBase
symbols=json.loads((ROOT/'evidence/render-architecture/public-symbols.json').read_text(encoding='utf8'))
starts=sorted({int(s['va'],16) for s in symbols if s.get('va')})
targets=[]
for a in [0x140c926b0,0x140c930f0,0x140c93970,0x140c94a10,0x141fe0b90,0x140c913e0,0x1413e4830,0x1413e4990]:
    ranges=[e.struct for e in pe.DIRECTORY_ENTRY_EXCEPTION if e.struct.BeginAddress==a-base]
    size=ranges[0].EndAddress-ranges[0].BeginAddress if ranges else starts[bisect.bisect_right(starts,a)]-a
    targets.append({'address':hex(a),'size':size,'range_basis':'original B .pdata RuntimeFunction' if ranges else 'next-public upper bound; not author body', 'bytes_sha256':hashlib.sha256(pe.get_data(a-base,size)).hexdigest()})
job=[{'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':targets}}, {'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':','.join(t['address'] for t in targets),'fields':'decompiled_code,callees,body,name'}}]
(OUT/'prepare-job.json').write_text(json.dumps(job,indent=2)+'\n',encoding='utf8')
(OUT/'targets.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'targets':targets},indent=2)+'\n',encoding='utf8')
# Prior temporary decompilation is supporting historical evidence only.
for name in ['rdp','spline-scalar','spline2d','spline2d_periodic']:
    previous=json.loads((OUT.parent/'utils-tabloid'/f'{name}-reva.json').read_text())
    body=json.loads(previous['content'][0]['text'])['decompilation']
    body=body.replace('\\r\\n','\n').replace('\\t','\t')
    (OUT/f'{name}.legacy.c').write_text(body,encoding='utf8')
print('bounded targets',len(targets))
