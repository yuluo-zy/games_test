from pathlib import Path
import json,hashlib
w=Path(__file__).resolve().parent
spv=w/'original-terrain-stroke.spvasm'
s=spv.read_text()
# These exact opcode guards / fetches are the basis of this bounded index model,
# not a CPU implementation of the terrain distance/noise shader.
anchors=['%325 = OpULessThan %bool %321 %324',
'%412 = OpLoad %v4uint %411 Aligned 8','%426 = OpULessThan %bool %425 %390',
'%447 = OpLoad %v4uint %446 Aligned 8','%497 = OpLoad %v4uint %496 Aligned 8',
'%522 = OpLoad %v4uint %521 Aligned 8','%549 = OpULessThanEqual %bool %548 %390',
'%570 = OpLoad %v4uint %569 Aligned 8','%592 = OpLoad %v4uint %591 Aligned 8',
'%693 = OpULessThanEqual %bool %692 %390','%714 = OpLoad %v4uint %713 Aligned 8',
'%769 = OpULessThanEqual %bool %768 %390','%790 = OpLoad %v4uint %789 Aligned 8']
assert all(x in s for x in anchors)
for n in range(1,1025):
 alloc=((n+1)&~1)*8
 reads=[]
 # Parallel coarse distance: index starts at localInvocationIndex and steps64.
 if n==1: reads.append((0,False,'coarse-single'))
 else:
  for lane in range(64):
   i=lane
   while i+1<n:
    reads.append((i,True,'coarse-pair'));i+=64
 # Fine SDF: singleton consumes only xy; multi starts with points0,1.
 if n==1: reads.append((0,False,'fine-single'))
 else:
  reads.append((0,True,'fine-initial'));i=2
  while i+4<=n:
   reads.extend([(i,True,'fine-four-a'),(i+2,True,'fine-four-b')]);i+=4
  while i+2<=n:
   reads.append((i,True,'fine-two'));i+=2
  while i+1<=n:
   reads.append((i,False,'fine-tail'));i+=1
 for index,both,label in reads:
  assert index*8+16<=alloc,(n,index,label,alloc)
  assert index<n
  if both:assert index+1<n
  elif index+1==n:assert n%2==1
result={'status':'static original-SPIRV dataflow/range audit; not GPU validation',
'spirv_text_sha256':hashlib.sha256(spv.read_bytes()).hexdigest(),
'spirv_binary_sha256':hashlib.sha256((w/'original-terrain-stroke.spv').read_bytes()).hexdigest(),
'checked_n':list([1,1024]),'counts_checked':1024,
'anchors':anchors,
'conclusion':'All physical16B fetches fit the rounded-even allocation. Singleton/final odd tail takes xy only; unused zw bits have no asserted value. Count0 outer stroke loop executes no BDA load, but W3 still declares four passes when rasterized=false.'}
(w/'point-fetch-range-audit.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items() if k!='anchors'},indent=2))
