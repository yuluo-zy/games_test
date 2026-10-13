"""Run complete original RT/audio option inheritance and finalization entries."""
import importlib.util,json,random,struct
from pathlib import Path
from unicorn.x86_const import *
OUT=Path(__file__).parent;ROOT=OUT.parents[2]
prior=ROOT/'evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py'
spec=importlib.util.spec_from_file_location('original',prior);o=importlib.util.module_from_spec(spec);spec.loader.exec_module(o)
def rt(tags,mesh,semantic,start=2):
    return b''.join(struct.pack('<Ifff',tag,float(start+i),float(start+1+i),float(start+2+i)) for i,tag in enumerate(tags))+bytes([mesh,semantic])+bytes(2)
def decode_rt(data):
    fields=[{'tag':struct.unpack_from('<I',data,i*16)[0],'value_bits':list(struct.unpack_from('<3I',data,i*16+4))} for i in range(4)]
    return {'fields':fields,'mesh':data[64],'semantic':data[65]}
def run(address,child,parent):
    o.reset();o.u.mem_write(o.INPUT,child);o.u.mem_write(o.INPUT+0x1000,parent)
    o.u.reg_write(UC_X86_REG_RCX,o.INPUT);o.u.reg_write(UC_X86_REG_RDX,o.INPUT+0x1000);o.run(address)
    return bytes(o.u.mem_read(o.INPUT,len(child)))
fixtures=[];rng=random.Random(130139)
for _ in range(100):
    tags=[rng.randrange(3) for _ in range(4)];ptags=[rng.randrange(3) for _ in range(4)]
    mesh=rng.randrange(3);pmesh=rng.randrange(3);sem=rng.randrange(4);psem=rng.randrange(4)
    child=rt(tags,mesh,sem)
    parent=rt(ptags,pmesh,psem,20)
    for parent_none in [False,True]:
        if parent_none:parent=struct.pack('<I',3)+bytes(64)
        result=run(0x14268a2e0,child,parent)
        fixtures.append({'child':decode_rt(child),'parent':None if parent_none else decode_rt(parent),'result':decode_rt(result)})
audio=[]
for trigger_kind_tag in [0,1,2,3]:
    for scale_tag in [0,1,2]:
        for shape in [0,1,2]:
            child=struct.pack('<3Q',0x8000000000000000+trigger_kind_tag,0,0)+struct.pack('<IfffB3x',scale_tag,2,3,4,shape)
            parent=struct.pack('<Q',0x8000000000000004)+bytes(36)
            try:result=run(0x14268a700,child,parent)
            except RuntimeError as error:
                audio.append({'child_kind_tag':trigger_kind_tag,'child_scale_tag':scale_tag,'child_shape':shape,'status':'original_panic','exception':str(error)})
                continue
            audio.append({'child_kind_tag':trigger_kind_tag,'child_scale_tag':scale_tag,'child_shape':shape,'status':'returned','result_kind_capacity':struct.unpack_from('<Q',result,0)[0],'result_scale_tag':struct.unpack_from('<I',result,24)[0],'result_scale_bits':list(struct.unpack_from('<3I',result,28)),'result_shape':result[40]})
report={'original_sha256':o.hashlib.sha256(o.raw).hexdigest(),'method':'Complete original option inheritance entry execution; no game algorithm or field-policy hooks. Existing allocator/copy boundaries only; no concrete String values supplied in these cases. Scalar tags confirmed through child-copy/finalizer/required-value paths.','entries':{'rt':'0x14268a2e0','audio':'0x14268a700'},'rt':fixtures,'audio':audio,'schema':{'array_tags':{'inherit':0,'missing_default':1,'value':2},'shape_tags':{'inherit':0,'missing_default':1,'aabb':2},'semantic_tags':{'none':0,'canopy':1,'inherit':2,'missing_default':3},'audio_string_tags':'0x8000000000000000+0/1 = kind Inherit/MissingDefault; +2/3 = outer trigger Inherit/MissingDefault; +4 = option None'}}
(OUT/'native-prefab-fixtures.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
print('rt',len(fixtures),'audio',len(audio))
print('empty enabled RT defaults',decode_rt(run(0x14268a2e0,rt([0,0,0,0],0,2),struct.pack('<I',3)+bytes(64))))
print('empty enabled audio',audio[0])
