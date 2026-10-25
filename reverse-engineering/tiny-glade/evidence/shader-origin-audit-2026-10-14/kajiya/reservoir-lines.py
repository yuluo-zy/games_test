from pathlib import Path
import struct,json
HERE=Path(__file__).resolve().parent
p=Path('D:/game/ljxsj_92385/Tiny Glade/compiled-shaders/_deferred_grass_light_sun_sky.raster.hlsl_b903e43ffb3da915.bin')
d=p.read_bytes();at=21538;n=struct.unpack_from('<I',d,at-4)[0];w=struct.unpack_from('<'+'I'*n,d,at);i=5;strings={};names={};inst=[];line=None;defs={}
while i<n:
 wc,op=w[i]>>16,w[i]&65535;a=list(w[i+1:i+wc])
 if op in [5,7]:
  s=struct.pack('<'+'I'*(len(a)-1),*a[1:]).split(b'\0')[0].decode('utf-8','replace')
  (strings if op==7 else names)[a[0]]=s
 if op==8:line=a
 if op==317:line=None
 if op in [43,44,59,61,65,80,81,12,133,136] and len(a)>1:defs[a[1]]={'offset':at+4*i,'opcode':op,'operands':a}
 if line and strings.get(line[0],'').endswith('inc/reservoir.hlsl'):
  inst.append({'line':line[1],'column':line[2],'offset':at+4*i,'opcode':op,'operands':a})
 i+=wc
up=(HERE/'kajiya-upstream/assets/shaders/inc/reservoir.hlsl').read_text().splitlines()
def tree(t,depth=6):
 v=defs.get(t)
 if not v or depth==0:return {'id':t,'name':names.get(t)}
 args=v['operands'][2:]
 if v['opcode']==43:return {'id':t,'opcode':43,'value_bits':hex(args[0]),'float32_interpretation':struct.unpack('<f',struct.pack('<I',args[0]))[0]}
 if v['opcode']==12:args=args[2:]
 return {'id':t,'name':names.get(t),**v,'args':[tree(x,depth-1) for x in args]}
out={'file':str(p),'module_offset':at,'instructions':inst,'upstream_line_context':{str(x['line']):up[x['line']-1] if x['line']<=len(up) else None for x in inst},'finish_stream_divide_operand_tree':tree(1753)}
(HERE/'reservoir-original-instructions.json').write_text(json.dumps(out,indent=2),encoding='utf-8')
print(out['upstream_line_context'])
