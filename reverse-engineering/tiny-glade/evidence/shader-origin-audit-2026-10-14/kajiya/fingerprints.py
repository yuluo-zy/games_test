from pathlib import Path
import struct,json,re,hashlib
HERE=Path(__file__).resolve().parent
ROOT=Path('D:/game/ljxsj_92385/Tiny Glade/compiled-shaders')
vals=[-.006,-.021,-.033,-.009,.14,.114,.111,.1005,.069,.0135,-.045,-.075,-.075,-.03,.006,.006]
bits=[struct.unpack('<I',struct.pack('<f',v))[0] for v in vals]
out=[]
for path in ROOT.glob('*.bin'):
    data=path.read_bytes();cur=0
    while True:
        at=data.find(b'\x03\x02\x23\x07',cur)
        if at<4:break
        n=struct.unpack_from('<I',data,at-4)[0]
        if n<5 or at+4*n>len(data):cur=at+4;continue
        w=struct.unpack_from('<'+'I'*n,data,at);i=5;constants={};names={};members={};types={};comps=[]
        while i<n:
            wc,op=w[i]>>16,w[i]&65535
            if wc==0 or i+wc>n:break
            a=w[i+1:i+wc]
            if op in [5,6]:
                skip=1 if op==5 else 2
                s=struct.pack('<'+'I'*(len(a)-skip),*a[skip:]).split(b'\0')[0].decode('utf-8','replace')
                if op==5:names[a[0]]=s
                else:members.setdefault(a[0],{})[a[1]]=s
            if op==30:types[a[0]]=list(a[1:])
            if op==43 and len(a)==3:constants[a[1]]=a[2]
            if op==44:comps.append({'offset':at+4*i,'id':a[1],'elements':list(a[2:])})
            i+=wc
        reservoir=[]
        for tid,name in names.items():
            if name in ['Reservoir1spp','Reservoir1sppStreamState','DepthRayMarch','HybridRootFinder','DepthRayMarchResult','DistanceWithPenetration']:
                reservoir.append({'type_id':tid,'name':name,'members':members.get(tid),'types':types.get(tid)})
        table=[{'offset':c['offset'],'result_id':c['id'],'float_values':vals,'bits':[hex(x) for x in bits]} for c in comps if [constants.get(v) for v in c['elements']]==bits]
        if reservoir or table:out.append({'file':path.name,'module_offset':at,'module_sha256':hashlib.sha256(data[at:at+4*n]).hexdigest(),'types':reservoir,'exact_hk_custom_g0_table':table})
        cur=at+4*n
(HERE/'unique-fingerprints.json').write_text(json.dumps(out,indent=2),encoding='utf-8')
for m in out:
    if m['exact_hk_custom_g0_table'] or m['types']:print(m['file'],m['module_offset'],'HKtable',len(m['exact_hk_custom_g0_table']),[(t['name'],t['members']) for t in m['types']])
