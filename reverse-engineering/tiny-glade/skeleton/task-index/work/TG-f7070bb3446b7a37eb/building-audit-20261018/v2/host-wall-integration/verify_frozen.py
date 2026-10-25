from pathlib import Path
import hashlib,json,pefile
P=Path(__file__).resolve().parent;ROOT=Path('/Users/yuluo/Desktop/game');rne=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';pe=pefile.PE(str(rne),fast_load=True)
report=json.loads((P/'native-ranges.json').read_text());assert hashlib.sha256(rne.read_bytes()).hexdigest()==report['B_rne_sha256'];total=0
for r in report['ranges']:
 va=int(r['va'],16);b=(P/(f'{va:x}'+'.bin')).read_bytes();assert len(b)==r['bytes'];assert hashlib.sha256(b).hexdigest()==r['sha256'];assert b==pe.get_data(va-0x140000000,len(b));total+=len(b)
for r in json.loads((P/'native-name-links.json').read_text()):assert pe.get_data(int(r['data'],16)-0x140000000,r['length']).hex()==r['bytes_hex']
for r in json.loads((P/'set-vtable-identity.json').read_text()):assert pe.get_data(int(r['va'],16)-0x140000000,len(bytes.fromhex(r['bytes_hex']))).hex()==r['bytes_hex']
manifest=P/'frozen-inputs.json'
if manifest.exists():
 for r in json.loads(manifest.read_text())['inputs']:
  b=Path(r['path']).read_bytes();assert len(b)==r['bytes'];assert hashlib.sha256(b).hexdigest()==r['sha256']
print(json.dumps({'status':'passed','ranges':len(report['ranges']),'native_bytes':total,'names':len(json.loads((P/'native-name-links.json').read_text())),'native_executed':False,'cargo_executed':False}))
