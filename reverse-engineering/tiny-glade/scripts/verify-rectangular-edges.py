from pathlib import Path
import json,subprocess
root=Path('D:/game/reverse-engineering/tiny-glade')
reference=json.loads((root/'evidence/rectangular-edge-reference.json').read_text(encoding='utf-8'))
frames=''.join(c['origin_xz']+c['delta_xz'] for c in reference['contexts'])
output=subprocess.run([str(root/'workspace/edge-runner.exe')],input=reference['roof']+' '+frames+'\n',capture_output=True,text=True,check=True).stdout.split()
actual=[output[1][i:i+128] for i in range(0,len(output[1]),128)]
expected=reference['edge_records']
failures=[]
for i,(a,b) in enumerate(zip(expected,actual)):
    if a!=b:
        ba,bb=bytes.fromhex(a),bytes.fromhex(b)
        failures.append(dict(tile=i,byte_offsets=[j for j,(x,y) in enumerate(zip(ba,bb)) if x!=y],original=a,candidate=b))
report=dict(ok=len(actual)==len(expected) and not failures,source_sha256=reference['source_sha256'],
  input_frames='prepared original frames captured without replacing geometry',expected_tiles=len(expected),actual_tiles=len(actual),failures=failures,
  core_range='0x1421af190..0x1421afbfb',candidate_rng_state=output[0],full_game_executed=False)
(root/'evidence/rectangular-edge-verification.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
print(json.dumps({'ok':report['ok'],'expected':len(expected),'actual':len(actual),'mismatch_count':len(failures),'first':failures[:2]}))
