"""原B materializer→原caller高度替换/第三数组写回→原支持梁；读取真实三Vec段，不以候选为oracle。"""
from pathlib import Path
import json,sys,hashlib,struct
p=Path(__file__).resolve();root=p.parents[3];re=root/'reverse-engineering/tiny-glade'
# 旧baseline仅保存已迁移的原路径；限定该精确旧路径读取重定向到相同SHA原RNE，不改共享metadata。
old='/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne';actual=root/'B版本内容/tiny-glade.rne'
original_read=Path.read_bytes
Path.read_bytes=lambda path:original_read(actual if str(path)==old else path)
assert hashlib.sha256(actual.read_bytes()).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
sys.path.insert(0,str(re/'evidence/building-structures'));import native_structures as native
allcases=json.loads((re/'evidence/building-structures/B-lintel-chain-verification.json').read_text())['input_cases'];cases=[c for c in allcases if 'source'in c]
rows=[]
for case in cases:
 source=case['source'];native.materialize(**source);views=native.last_materialized_views[:]
 prepared=[native.prepare_current_lintel(v,source['points'],case['public_284'],case['public_288'])for v in views]
 third=native.current_third_accumulation(views,case['initial_third'])if views else case['initial_third']
 segments=[]
 for v in views:
  out={}
  for key,offset,width in [('heights',0,1),('positions',24,3),('points_u',48,1)]:
   _,ptr,n=struct.unpack('<3Q',native.cpu.mem_read(v+offset,24));out[key]=list(struct.unpack('<'+'I'*(n*width),native.cpu.mem_read(ptr,n*width*4)))
  out['source_indices']=list(struct.unpack('<2Q',native.cpu.mem_read(v+72,16)));out['range']=list(struct.unpack('<3i',native.cpu.mem_read(v+88,12)));segments.append(out)
 records=native.produce_support_batch([(*v,i)for i,v in enumerate(prepared,1)],case['wall_id'],source['closed'])
 rows.append(dict(input=case,segments=segments,third=[native.bits(v)for v in third],planks=[x.hex()for x in records]))
output=p.with_suffix('.json')
with output.open('x')as f:json.dump(dict(binary_sha256=hashlib.sha256(actual.read_bytes()).hexdigest(),scope=__doc__,bridges='existing original harness allocator/CRT/profiler/deadlock observers, original materializer/prepare/third/support/RDP/walker/SparseWriter untouched',rows=rows),f,indent=2)
print(json.dumps(dict(cases=len(rows),segments=sum(len(x['segments'])for x in rows),planks=sum(len(x['planks'])for x in rows))))
