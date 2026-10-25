import struct, hashlib, json
from pathlib import Path
ROOT=Path('D:/game/ljxsj_92385/Tiny Glade/compiled-shaders')
OUT=Path(__file__).resolve().parent
CHECKS={0x04040404:'octant_x',0x02020202:'octant_y',0x01010101:'octant_z',0x10101010:'inner_meta_mask',0x1f1f1f1f:'child_index_mask',0x07070707:'child_bits_mask',0xff000000:'inner_hits_mask',0x00ffffff:'triangle_hits_mask'}
results=[]
for path in sorted(ROOT.glob('*.bin')):
    data=path.read_bytes(); cursor=0; modules=[]
    while True:
        at=data.find(b'\x03\x02\x23\x07',cursor)
        if at<0:break
        if at<4:break
        n=struct.unpack_from('<I',data,at-4)[0]
        if n<5 or at+4*n>len(data):cursor=at+4;continue
        raw=data[at:at+4*n];words=struct.unpack('<'+'I'*n,raw)
        i=5; names=[];constants=[]; strides=[]; name_by_id={}; runtime_arrays={}; defs={}; insts=[]
        while i<n:
            wc,op=words[i]>>16,words[i]&65535
            if wc==0 or i+wc>n:break
            a=words[i+1:i+wc]
            if op==5 and len(a)>1:
                name=struct.pack('<'+'I'*(len(a)-1),*a[1:]).split(b'\0')[0].decode('utf-8','replace')
                name_by_id[a[0]]=name
                if any(s in name.lower() for s in ['bvh','node','tlas','triangle','travers','ray']):names.append(name)
            if op==29 and len(a)==2:runtime_arrays[a[0]]=a[1]
            if op==43 and len(a)==3 and a[2] in CHECKS:constants.append({'offset':at+4*i,'id':a[1],'value':hex(a[2]),'label':CHECKS[a[2]]})
            if op==71 and len(a)>=3 and a[1]==6 and a[2]==80:strides.append({'offset':at+4*i,'target_id':a[0],'array_stride':80})
            if op in [43,194,195,196,197,198,199] and len(a)>=2:defs[a[1]]={'opcode':op,'operands':list(a),'offset':at+4*i}
            insts.append({'opcode':op,'operands':list(a),'offset':at+4*i})
            i+=wc
        for stride in strides:
            elem=runtime_arrays.get(stride['target_id'])
            stride['element_type_id']=elem
            stride['element_type_name']=name_by_id.get(elem)
        for const in constants:
            uses=[v for v in insts if v['opcode'] in [194,195,196,197,198,199] and const['id'] in v['operands'][2:]]
            const['direct_bitwise_uses']=uses[:8]
            const['operand_definitions']=[defs[v] for u in uses[:8] for v in u['operands'][2:] if v in defs and v!=const['id']]
            def tree(value,depth=4):
                d=defs.get(value)
                if not d or depth==0:return {'id':value}
                if d['opcode']==43:return {'constant':d['operands'][2],'id':value}
                return {'id':value,'opcode':{194:'OpShiftRightLogical',195:'OpShiftRightArithmetic',196:'OpShiftLeftLogical',197:'OpBitwiseOr',198:'OpBitwiseXor',199:'OpBitwiseAnd'}.get(d['opcode'],str(d['opcode'])),'offset':d['offset'],'args':[tree(x,depth-1) for x in d['operands'][2:]]}
            const['direct_bitwise_use_trees']=[tree(u['operands'][1]) for u in uses[:2]]
        if constants or strides or names:
            modules.append({'offset':at,'word_count':n,'sha256':hashlib.sha256(raw).hexdigest(),'names':names,'constants':constants,'array_stride_80':strides})
        cursor=at+4*n
    if modules:results.append({'file':path.name,'sha256':hashlib.sha256(data).hexdigest(),'modules':modules})
report={'source_root':str(ROOT),'method':'Raw shipped bytes; find original SPIR-V magic and preceding u32 word count; inspect OpName (5), OpConstant (43), OpDecorate ArrayStride (71/6). No recovered HLSL used.','files':results,'limitations':['Constant and stride presence is a structural fingerprint, not a full instruction-equivalence proof. Includes or CPU-side BVH construction are outside this scan.']}
(OUT/'raw_spirv_fingerprints.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
for file in results:
    for m in file['modules']:
        if len(m['constants'])>=3 or m['array_stride_80']:
            print(file['file'],'offset',m['offset'],'constants',[v['label'] for v in m['constants']],'stride80',len(m['array_stride_80']),'names',m['names'][:25])
