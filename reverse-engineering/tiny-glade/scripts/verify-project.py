"""编译实际 Cargo 工程并验证当前 CLI 接线；原作差分单独执行与保存。"""
from pathlib import Path
import subprocess, json
root = Path('D:/game/reverse-engineering/tiny-glade')
cargo = 'C:/Users/liyu/.cargo/bin/cargo.exe'
build = subprocess.run([cargo,'build','--release','--locked'],cwd=root/'reconstruction',capture_output=True,text=True)
if build.returncode:
    raise RuntimeError(build.stderr)
binary = root/'reconstruction/target/release/tg-reconstruction.exe'
checks = []
for args,expected in [(['identity'],{'source_sha256':'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'}),
                     (['aspect','1920','1080'],{'f32_bits':'3fe38e39'}),
                     (['roof-state','0.5','1.0'],{'ty_byte':1,'is_gable':True}),
                     (['roof-state','NaN','1.0'],{'ty_byte':1,'is_gable':True}),
                     (['roof-state','0.5','NaN'],{'ty_byte':1,'is_gable':False})]:
    output = subprocess.run([str(binary),*args],capture_output=True,text=True,check=True)
    actual = json.loads(output.stdout)
    checks.append(dict(args=args,expected=expected,actual=actual,ok=all(actual.get(k)==v for k,v in expected.items())))
report = {'ok':all(c['ok'] for c in checks),'locked_release_build':True,'checks':checks,
          'scope':'Cargo 实际二进制 CLI 接线验证；独立原作参照见各算法机器码差分报告','full_game_implemented':False}
reference=json.loads((root/'evidence/roof-ridge/rectangular-reference.json').read_text(encoding='utf-8'))
packed=root/'workspace/cargo-roof-records.bin'
result=subprocess.run([str(binary),'roof-tiles',reference['roof_bytes'],reference['rectangle_bytes'],'0',str(packed)],capture_output=True,text=True,check=True)
actual=json.loads(result.stdout)
expected=bytes.fromhex(''.join(reference['tile_records']))
checks.append(dict(args=['roof-tiles','original default Rectangle/Roof bytes','packed output'],actual=actual,
                   expected_records=len(reference['tile_records']),ok=packed.read_bytes()==expected))
report['ok']=all(c['ok'] for c in checks)
(root/'evidence/project-build-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'ok':report['ok'],'checks':len(checks)}))
if not report['ok']:
    raise SystemExit(1)
