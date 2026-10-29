"""运行原 continue_stairs 及其五个业务 helper；只桥接标准锁/GraphMap/IndexMap 容器。"""
from pathlib import Path
import hashlib,json,struct,itertools,subprocess
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x200ff000;u.mem_map(D,0x100000);u.mem_map(S,0x100000)
STATE=D+0x1000;QUEUE=D+0x3000;NODES=D+0x4000;EDGES=D+0x6000;EVENTS=D+0x8000
q=lambda a,v:u.mem_write(a,struct.pack('<Q',v))
w=lambda a,v:u.mem_write(a,struct.pack('<I',v))
uq=lambda a:struct.unpack('<Q',u.mem_read(a,8))[0]
uw=lambda a:struct.unpack('<I',u.mem_read(a,4))[0]
r=lambda reg:u.reg_read(reg)
def ret(value=0):
 sp=r(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,uq(sp));u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RAX,value)
def graph_memory():
 # GraphMap 的 IndexMap 顺序表，仅物化原业务 helper 读取的 key/邻接长度。
 q(STATE+0x30,NODES);q(STATE+0x38,len(nodes))
 for i,node in enumerate(nodes):
  at=NODES+i*40;w(at+0x20,node);q(at+0x10,sum(node in edge for edge in edges))
 q(STATE+0x78,EDGES);q(STATE+0x80,len(edges))
 for i,(a,b) in enumerate(edges):w(EDGES+i*16+8,a);w(EDGES+i*16+12,b)
def add_node(node):
 if node not in nodes:nodes.append(node)
 graph_memory()
def add_edge(a,b):
 add_node(a);add_node(b);edge=tuple(sorted((a,b)))
 if edge not in edges:edges.append(edge)
 graph_memory()
def hook(uc,a,size,_):
 global panic
 rcx,rdx,r8,r9=[r(x)for x in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]]
 # 这些五个业务函数只观察入口，继续执行原机器码，不在 Python 重写其条件。
 if a==0x1408ffd30:calls.append({'helper':'add_node','node':rdx&0xffffffff})
 elif a==0x1408fe890:calls.append({'helper':'endpoint','payload_hex':bytes(u.mem_read(rdx+0x10,24)).hex()})
 elif a==0x1408ffd70:calls.append({'helper':'add_edge','from':rdx&0xffffffff,'to':r8&0xffffffff})
 elif a==0x140833240:calls.append({'helper':'locator_new','addr_words':[rdx,r8],'node':r9&0xffffffff})
 elif a==0x1408fe7b0:calls.append({'helper':'try_set_endpoint','payload_hex':bytes(u.mem_read(rdx,24)).hex()})
 elif a==0x14067dd80:
  requests.append('deferred_queue_get');ret(QUEUE)
 elif a in [0x140c81c70,0x140c81e10]:ret() # 标准锁 guard/debug instrumentation。
 elif a in [0x1428cc4b0,0x1428ccf50]:
  raise AssertionError('unexpected slow read lock path')
 elif a==0x14093b0c0:
  assert rcx==STATE+0x28;add_node(rdx&0xffffffff);ret()
 elif a==0x14093ac90:
  assert rcx==STATE+0x28;add_edge(rdx&0xffffffff,r8&0xffffffff);ret()
 elif a==0x1402c8640:
  requests.append({'container':'union_find_insert','key':rdx&0xffffffff,'value':r8&0xffffffff});ret()
 elif a in [0x140106b70,0x140106840]:ret(0) # IndexMap hash，查找另以实际 key 桥接。
 elif a==0x140b4f360:
  assert rcx==STATE+0x28;node=uw(r8);found=node in nodes
  ret(1 if found else 0);u.reg_write(UC_X86_REG_RDX,nodes.index(node)if found else 0)
 elif a==0x140b4f270:
  assert rcx==STATE+0x70;key=tuple(struct.unpack('<II',u.mem_read(r8,8)))
  ret(1 if key in edges else 0);u.reg_write(UC_X86_REG_RDX,edges.index(key)if key in edges else 0)
 elif a in [0x1428d9518,0x1428d9430]:
  panic={'target':hex(a),'return_address':hex(uq(r(UC_X86_REG_RSP)))};uc.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
def locator(tag=3,node=0,word1=0,padding=0xfedcba98,tag_padding=0):
 return struct.pack('<QQII',(tag&0xffffffff)|tag_padding<<32,word1,node,padding)
def event(node=27,tag=3,tag_padding=0xdeadbeef,kind=63,flag=0,addr=(1,0x123456789abcdef0)):
 b=bytearray([0xa5]*0x88);struct.pack_into('<IIQQ',b,0,tag,tag_padding,*addr);struct.pack_into('<I',b,0x40,node);b[0x80]=flag;b[0x81]=kind;return b

def run(name,mode=4,mode_type=0,mode_padding=0,events=None,endpoint=None,initial_nodes=None,initial_edges=None):
 global calls,requests,nodes,edges,panic
 calls=[];requests=[];panic=None;nodes=list(initial_nodes or []);edges=[tuple(sorted(e))for e in(initial_edges or [])]
 u.mem_write(D,bytes(0x10000));u.mem_write(S,bytes([0x5a])*0x100000)
 graph_memory();q(STATE,0x7777777777777777);u.mem_write(STATE+0x178,b'\x01')
 endpoint=endpoint if endpoint is not None else locator();u.mem_write(STATE+0x10,endpoint)
 mode_ref=D+0x100;mode_data=D+0x200;stairs_ref=D+0x300;events_ref=D+0x400;ticks=D+0x500
 q(mode_ref,mode_data);w(mode_data,mode);w(mode_data+4,(mode_type&255)|((mode_padding&0xffffff)<<8))
 q(stairs_ref,STATE);q(stairs_ref+0x10,ticks);w(stairs_ref+0x1c,0x1234);w(ticks,0xabcd);q(events_ref,D+0x700)
 events=events or[];q(QUEUE+0x10,EVENTS);q(QUEUE+0x18,len(events))
 for i,e in enumerate(events):u.mem_write(EVENTS+i*0x88,bytes(e))
 sp=S+0xf0008;q(sp,STOP)
 for reg,value in [(UC_X86_REG_RCX,mode_ref),(UC_X86_REG_RDX,stairs_ref),(UC_X86_REG_R8,events_ref),(UC_X86_REG_RSP,sp)]:u.reg_write(reg,value)
 u.emu_start(0x14079c3a0,STOP,count=100000)
 assert panic or r(UC_X86_REG_RIP)==STOP,name
 return {'name':name,'input':{'mode':mode,'mode_type':mode_type,'mode_padding':mode_padding,'events_hex':[bytes(e).hex()for e in events],'endpoint_hex':endpoint.hex(),'nodes':initial_nodes or[],'edges':initial_edges or[]},'output':{'helpers':calls,'container_requests':requests,'endpoint_hex':bytes(u.mem_read(STATE+0x10,24)).hex(),'nodes':nodes,'edges':[list(e)for e in edges],'changed_tick':uw(ticks),'checksum_word':uq(STATE),'cached_flag':u.mem_read(STATE+0x178,1)[0],'queue_lock_word':uq(QUEUE),'queue_event_bytes':bytes(u.mem_read(EVENTS,len(events)*0x88)).hex(),'panic':panic}}
rows=[]
for mode,ty in itertools.product([0,1,2,3,4,5,6,7,10,11,12,20,21,0xffffffff],[0,63,64]):
 rows.append(run(f'mode_{mode}_type_{ty}',mode=mode,mode_type=ty,mode_padding=0xffffff,events=[event()]))
for mode,ty in [(4,0),(11,63)]:
 rows.append(run(f'empty_{mode}',mode=mode,mode_type=ty))
 for tag,padding,kind,flag in itertools.product([0,2,3,4,0xffffffff],[0,0xdeadbeef],[0,62,63,64],[0,1,255]):
  rows.append(run(f'gate_{mode}_{tag}_{padding}_{kind}_{flag}',mode=mode,mode_type=ty,events=[event(tag=tag,tag_padding=padding,kind=kind,flag=flag)]))
for flag,pad in itertools.product([0,1,255],[0,0xdeadbeef]):
 rows.append(run(f'same_endpoint_{flag}_{pad}',events=[event(flag=flag)],endpoint=locator(tag=1,node=27,tag_padding=pad),initial_nodes=[27]))
 rows.append(run(f'unset_endpoint_padding_{flag}_{pad}',events=[event(flag=flag)],endpoint=locator(tag=3,node=99,tag_padding=pad)))
 for degree in [0,1,2,3]:
  rows.append(run(f'degree_{degree}_flag_{flag}_padding_{pad}',events=[event(flag=flag)],endpoint=locator(tag=1,node=27,tag_padding=pad),initial_nodes=[27]+list(range(80,80+degree)),initial_edges=[[27,x]for x in range(80,80+degree)]))
rows.extend([
 run('same_endpoint_two_following_events',events=[event(node=27,flag=1),event(node=27,flag=1),event(node=28,flag=0)],endpoint=locator(tag=1,node=99),initial_nodes=[99]),
 run('multi_event_flag_and_branch_sequence',events=[event(node=1),event(node=2,flag=1),event(node=3),event(node=4,tag=4),event(node=5,kind=62),event(node=6)],endpoint=locator(tag=3,tag_padding=0xdeadbeef)),
 run('degrees_invalidate_same_endpoint_then_restore_with_new_node',events=[event(node=27),event(node=28)],endpoint=locator(tag=1,node=27),initial_nodes=[27,80,81],initial_edges=[[27,80],[27,81]]),
 run('address_union_and_extreme_node_bits',events=[event(node=0xffffffff,addr=(0x1234567800000000,0xfedcba9876543210))]),
])
report={'binary_sha256':sha,'entry':'0x14079c3a0','native_helpers':['0x1408ffd30','0x1408fe890','0x1408ffd70','0x140833240','0x1408fe7b0'],'bridges':{'0x14067dd80':'DeferredEvent storage getter','0x140c81c70/140c81e10':'standard lock guard instrumentation','0x14093b0c0/14093ac90':'GraphMap add_node/add_edge container semantics; serialize nodes/neighbor length/edges read by original business code','0x1402c8640':'union-find insert recorded; result unused by original chain','0x140106b70/140106840':'hash result ignored by explicit key-aware container lookup bridge','0x140b4f360/140b4f270':'IndexMap key lookup returns tag/index','panic':'stop at original panic boundary; no Windows unwind'},'endpoint_semantic_bytes':'first20 bytes when active; inactive payload/padding unspecified; final4 retained native stack scratch bytes are not canonical state','rows':rows}
Path(__file__).with_name('m01-continue-stairs-native.json').write_text(json.dumps(report,indent=2)+'\n')
def csv(values):return ','.join(map(str,values)) or '-'
def edge_csv(values):return ','.join(f'{a}:{b}'for a,b in values)or '-'
lines=['# name\tmode_u32\tmode_type_u8\tmode_padding24\tendpoint_before24_hex\tnodes_before_csv\tedges_before_a:b_csv\tevents136_hex_semicolon\tendpoint_after24_hex\tnodes_after_csv\tedges_after_a:b_csv\tchanged_tick\tchecksum_word\tcached_flag']
for row in rows:
 x,y=row['input'],row['output']
 lines.append('\t'.join(map(str,[row['name'],x['mode'],x['mode_type'],x['mode_padding'],x['endpoint_hex'],csv(x['nodes']),edge_csv(x['edges']),';'.join(x['events_hex'])or '-',y['endpoint_hex'],csv(y['nodes']),edge_csv(y['edges']),y['changed_tick'],y['checksum_word'],y['cached_flag']])))
Path(__file__).with_name('m01-continue-stairs-native.tsv').write_text('\n'.join(lines)+'\n')
for name,start,end in [('body',0x14079c3a0,0x14079c574),('endpoint',0x1408fe7b0,0x1408fe8a3),('graph-wrappers',0x1408ffd30,0x1408ffe74),('locator',0x140833240,0x14083324f)]:
 Path(__file__).with_name(f'm01-continue-stairs-{name}.asm').write_text(subprocess.check_output(['/opt/homebrew/opt/llvm/bin/llvm-objdump','-d',f'--start-address={hex(start)}',f'--stop-address={hex(end)}',str(root/'B版本内容/tiny-glade.rne')],text=True))
print(json.dumps({'cases':len(rows),'active_cases':sum(bool(r['output']['helpers'])for r in rows),'panics':sum(bool(r['output']['panic'])for r in rows),'gated_changed_empty':sum(not r['output']['helpers']and r['output']['changed_tick']==0x1234 for r in rows)}))
