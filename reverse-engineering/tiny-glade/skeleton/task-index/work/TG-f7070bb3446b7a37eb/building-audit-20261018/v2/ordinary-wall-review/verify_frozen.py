from pathlib import Path
import json,hashlib,pefile,re,ast,struct
ROOT=Path('/Users/yuluo/Desktop/game');W=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work'
OUT=W/'TG-f7070bb3446b7a37eb/building-audit-20261018/v2/ordinary-wall-review'
draw=W/'TG-772e8f649ffb32fc5a/terrain-curve-20261018';ui=W/'TG-cf5ffaff5ad4e28ba1/terrain-curve-20261018';smooth=W/'TG-e4029726f0fdb1a782/terrain-curve-20261018';own=W/'TG-f7070bb3446b7a37eb/building-audit-20261018/v2/drawing-smoother-review'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
m=draw/'frozen-inputs.json';rows=json.loads(m.read_text())['files']
for r in rows:
 p=Path(r['path']);assert sha(p)==r['sha256'];assert p.stat().st_size==r['bytes']
pe=pefile.PE(str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'),fast_load=True);pe.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_IMPORT']])
res={'target':'B','native_executed':False,'cargo_executed':False,'production_changed':False,'manifest':{'path':str(m),'sha256':sha(m),'count':len(rows),'all_match':True},'packs':[]}
for p in [draw/'native-pack/draw-wall.json',ui/'native-pack/ui-draw-wall.json',ui/'native-pack/smoother.json']:
 d=json.loads(p.read_text());n=0;size=0
 for i in d['instructions']:
  raw=bytes.fromhex(i['bytes']);assert pe.get_data(int(i['rva'],0),len(raw))==raw;n+=1;size+=len(raw)
 res['packs'].append({'path':str(p),'instructions':n,'bytes':size,'all_match':True})
# Copy fidelity of fixture literals only; these are not Rust execution results.
s=(ui/'ui_draw_wall.rs').read_text();begin=s.index('= &[',s.index('let cases:'))+3;end=s.index('\n        ];',begin)+10
arr=s[begin:end];arr=re.sub(r'\bfalse\b','False',arr);arr=re.sub(r'\btrue\b','True',arr)
rust=ast.literal_eval(arr)
native=json.loads((ui/'native-ui-wall.json').read_text())['cases'];expected=[]
for d in native:
 expected.append((d['mode'],d['subtype'],d['initial_state'],d['phase'],d['inside'],d['active'],d['state'],d['nursery_len'],d['nursery_kind'],d['suppressed'],[len(d['events'][k]) for k in ['record','finish','debug','bend','outside','feedback','hide']]))
assert rust==expected
res['ui_fixture_rows']=len(rust)
s=(smooth/'drawing_input_smoother.rs').read_text();native_s=json.loads((own/'native-smoother-fixtures.json').read_text());extra=json.loads((own/'f32-start-addendum/native-f32-start-fixtures.json').read_text());fixture_counts={}
for fn,key,src in [('original_entry_fixture_bits_and_last_accepted_state','cases',native_s['cases']),('original_sequence_keeps_rejected_and_zero_tick_state','sequence',native_s['sequence']),('original_large_coordinate_interpolation_cannot_skip_none_state_loop','large',extra['cases'])]:
 begin=s.index('= &[',s.index('let cases:',s.index('fn '+fn)))+3;end=s.index('\n        ];',begin)+10
 arr=s[begin:end].replace('&','');arr=re.sub(r'Some\((\[[^\]]*\])\)',r'\1',arr)
 rust=ast.literal_eval(arr);expected=[]
 for d in src:expected.append((d['previous'],d['point'],d['ticks_60'],d.get('projection_scale',0x3f800000),d.get('height',1080),d['output']['points'],d['output']['last']))
 assert rust==expected,(fn,len(rust),len(expected))
 fixture_counts[key]=len(rust)
res['smoother_fixture_rows']=fixture_counts
for d in pe.DIRECTORY_ENTRY_IMPORT:
 for imp in d.imports:
  if imp.address==0x143193198:res['roundf_import']={'dll':d.dll.decode(),'name':imp.name.decode(),'iat_va':hex(imp.address)}
assert res['roundf_import']['name']=='roundf'
res['roundf_thunk']={'va':'0x142923fa0','bytes':pe.get_data(0x2923fa0,6).hex()}
assert res['roundf_thunk']['bytes']=='ff25f2f18600'
res['stationary_call_sites']=[{'va':hex(a),'bytes':pe.get_data(a-0x140000000,5).hex()} for a in [0x140a7d590,0x140a7d5a4]]
res['constants']=[{'va':hex(a),'bits':hex(struct.unpack('<I',pe.get_data(a-0x140000000,4))[0]),'float':struct.unpack('<f',pe.get_data(a-0x140000000,4))[0]} for a in [0x1429d18cc,0x142950d70]]
(OUT/'verification.json').write_text(json.dumps(res,indent=2)+'\n');print(json.dumps(res,indent=2))
