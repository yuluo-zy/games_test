"""Execute original IndexCondenser::get inline prefix, std hash code unchanged."""
from pathlib import Path
import hashlib,json,random,struct,sys
sys.path.insert(0,str(Path(__file__).parent.parent))
import native_spline_rdp as o
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).parent;active=False;result=None
def hook(uc,address,size,data):
    global result
    if not active:return
    # LLVM module540 IPI0x74E3: success ends at remove body offset0xe8,
    # miss ends at0x1f4. Stop before the unrelated swap-removal transaction.
    if address==0x140a1a038:result=o.u.reg_read(UC_X86_REG_ESI);uc.emu_stop()
    elif address in [0x140a1a140,0x140a1a142]:result=None;uc.emu_stop()
o.u.hook_add(UC_HOOK_CODE,hook)
def hash_key(key):
    o.reset();state=o.INPUT;pointer=o.INPUT+0x100
    o.u.mem_write(state,bytes(16));o.u.mem_write(pointer,struct.pack('<I',key))
    o.u.reg_write(UC_X86_REG_RCX,state);o.u.reg_write(UC_X86_REG_RDX,pointer);o.run(0x1405b0ed0)
    return o.u.reg_read(UC_X86_REG_RAX)
def get(keys,key,cached):
    global active,result
    hashes=[hash_key(k) for k in keys]
    o.reset();state=o.OUTPUT;condex=o.INPUT+0x1000;vector=o.INPUT;ctrl=o.HEAP+0x1000;mask=127
    o.u.mem_write(vector,struct.pack('<'+'I'*len(keys),*keys))
    o.u.mem_write(condex,struct.pack('<II',key,cached))
    o.u.mem_write(state,struct.pack('<9Q',len(keys),vector,len(keys),ctrl,mask,128-len(keys),len(keys),0,0))
    # A valid caller-supplied raw HashMap fixture. Native std/hashbrown lookup
    # executes original instructions; no hash/probe algorithm is game source.
    control=bytearray([0xff]*144);used=set()
    for index,(k,h) in enumerate(zip(keys,hashes)):
        bucket=h&mask
        while bucket in used:bucket=(bucket+1)&mask
        used.add(bucket);control[bucket]=h>>57
        o.u.mem_write(ctrl-(bucket+1)*8,struct.pack('<II',k,index))
    control[128:]=control[:16];o.u.mem_write(ctrl,bytes(control))
    o.u.reg_write(UC_X86_REG_RCX,state);o.u.reg_write(UC_X86_REG_RDX,condex)
    active=True;result=None
    try:o.u.emu_start(0x140a19f50,o.STOP,count=500000)
    finally:active=False
    pc=o.u.reg_read(UC_X86_REG_RIP);assert pc in [0x140a1a038,0x140a1a140,0x140a1a142],hex(pc)
    return {'keys':keys,'key':key,'cached':cached,'result':result,'updated_cached':struct.unpack('<I',o.u.mem_read(condex+4,4))[0],'stop_pc':hex(pc)}
rng=random.Random(131303);fixtures=[]
for n in [0,1,2,3,16,32]:
    keys=rng.sample(range(100,10000),n)
    for _ in range(12):
        key=rng.choice(keys) if keys and rng.random()<.75 else 0xffffffff
        cached=rng.randrange(0,max(n,1)+4)
        fixtures.append(get(keys,key,cached))
(OUT/'index-get-native-fixtures.json').write_text(json.dumps({'original_sha256':hashlib.sha256(o.raw).hexdigest(),'method':'Original get prefix identified by LLVM inlinee0x74E3 executes. Captures resolved ESI or None at original branch boundaries; original std hash executes without hooks. Raw table/vector layouts supplied as validation inputs, not delivered as game implementations.','fixtures':fixtures},indent=2)+'\n',encoding='utf8')
test='''use super::*;
// Encoded original inline-prefix input/output fixture shape.
#[allow(clippy::type_complexity)]
#[test]fn original_get_cache_policy() {
    let cases:&[(&[u32],u32,u32,Option<u32>,u32)]=&[
'''
for row in fixtures:test+=f"        (&{row['keys']!r},{row['key']},{row['cached']},{'None' if row['result'] is None else 'Some('+str(row['result'])+')'},{row['updated_cached']}),\n"
test+='''    ];
    for (keys,key,cached,expected,updated) in cases {
        let mut indices=IndexCondenser::default();for &key in *keys {indices.insert(key);}
        let handle=Condex{key:*key,cached_index:Cell::new(*cached)};
        assert_eq!(indices.get(&handle),*expected);assert_eq!(handle.last_known_index(),*updated);
    }
}
#[test]fn clear_invalidates_old_keys_and_preserves_capacity() {
    let mut indices=IndexCondenser::default();let a=indices.insert(10u32);let b=indices.insert(20);
    let capacities=(indices.keys.capacity(),indices.indices.capacity());
    indices.clear();assert!(indices.keys().is_empty());assert_eq!(indices.get(&a),None);assert_eq!(indices.get(&b),None);
    assert_eq!((indices.keys.capacity(),indices.indices.capacity()),capacities);
    let c=indices.insert(30);assert_eq!(indices.get(&c),Some(0));assert_eq!(indices.get(&a),None);
}
'''
(OUT/'index_tests.rs').write_text(test,encoding='utf8')
print('original get cases',len(fixtures))
