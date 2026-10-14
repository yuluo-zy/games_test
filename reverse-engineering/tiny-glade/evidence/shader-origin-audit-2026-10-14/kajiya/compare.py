from pathlib import Path
import json, re, struct, hashlib
HERE=Path(__file__).resolve().parent
BASE=HERE.parent
orig=json.loads((BASE/'original-source-dependencies.json').read_text(encoding='utf-8-sig'))
UP=HERE/'kajiya-upstream/assets/shaders'
def norm(s):
    return re.sub(r'\.(cs|raster|inc)\.hlsl$', '.hlsl', s.replace('assets/',''))
ups={norm('shaders/'+str(p.relative_to(UP)).replace('\\','/')):p for p in UP.rglob('*') if p.is_file()}
matches=[]
for o in orig:
    p=ups.get(norm(o['source']))
    if p:
        b=p.read_bytes(); text=b.decode('utf-8-sig')
        # lexical identifiers >=7 chars are preliminary only; final evidence from SPV below.
        ids=set(re.findall(r'\b[A-Za-z_]\w{6,}\b',text))
        matches.append({'original':o['source'],'upstream':str(p.relative_to(UP)).replace('\\','/'),'original_lf':o['source_linefeed_counts'],'upstream_lf':b.count(b'\n'),'upstream_sha256':hashlib.sha256(b).hexdigest(),'ids':sorted(ids),'original_bins':o['files']})
bybin={}
for p in Path('D:/game/ljxsj_92385/Tiny Glade/compiled-shaders').glob('*.bin'):
    data=p.read_bytes(); cur=0; ms=[]
    while True:
        at=data.find(b'\x03\x02\x23\x07',cur)
        if at<4:break
        n=struct.unpack_from('<I',data,at-4)[0]
        if n<5 or at+n*4>len(data):cur=at+4;continue
        w=struct.unpack_from('<'+'I'*n,data,at); i=5; names=[];members=[];const=[]
        while i<n:
            wc,op=w[i]>>16,w[i]&65535
            if wc==0 or i+wc>n:break
            a=w[i+1:i+wc]
            if op in [5,6]:
                skip=1 if op==5 else 2
                s=struct.pack('<'+'I'*(len(a)-skip),*a[skip:]).split(b'\0')[0].decode('utf-8','replace')
                (names if op==5 else members).append(s)
            if op==43 and len(a)==3:const.append(a[2])
            i+=wc
        ms.append({'offset':at,'names':names,'members':members,'u32_constants':const})
        cur=at+n*4
    bybin['compiled-shaders/'+p.name]=ms
for m in matches:
    ids=set(m.pop('ids')); hits=[]
    for bn in m['original_bins']:
        for mod in bybin[bn]:
            found=sorted(set(mod['names']+mod['members'])&ids)
            if found:hits.append({'bin':bn,'offset':mod['offset'],'shared_identifiers':found})
    m['original_spv_shared_identifiers']=hits
(HERE/'path-and-identifiers-comparison.json').write_text(json.dumps({'upstream':'https://github.com/EmbarkStudios/kajiya','commit':'9fdec0f074f9f822d395f9cfd041b58749e32303','original_source_count':len(orig),'upstream_shader_count':len(ups),'normalized_path_matches':matches},indent=2),encoding='utf-8')
(HERE/'original-spv-names-members-constants.json').write_text(json.dumps(bybin,indent=2),encoding='utf-8')
for m in matches:
    print(m['original'],m['original_lf'],'upstreamLF',m['upstream_lf'],'shared_names',sorted(set(v for h in m['original_spv_shared_identifiers'] for v in h['shared_identifiers']))[:30])
