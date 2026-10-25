from pathlib import Path
import hashlib,json
out=Path(__file__).parent;root=Path('D:/game')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
bases={x['path']:x['base_sha256'] for x in json.loads((out/'candidate-bases.json').read_text())}
sources=[]
for path in sorted((out/'proposed').rglob('*.rs')):
 rel=path.relative_to(out/'proposed').as_posix();original=root/rel
 sources.append(dict(path=rel,base_sha256=bases.get(rel,sha(original) if original.exists() else None),candidate_sha256=sha(path),new_file=not original.exists()))
report=dict(status='candidate frozen; central compile and independent chain verification pending',production_modified=False,cargo_run=False,llvm_codegen_started=False,
 original_rne_sha256='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',sources=sources,
 native_fixtures=[dict(path=p.name,sha256=sha(p)) for p in sorted(out.glob('native-*-fixtures.json'))],
 validation=dict(native_extraction='succeeded',rust_comparison='not run',true_input_history_chain='not run',gpu='not run'),
 dependencies=['ui-picker','ui-update + PlayUiSound24B fragment + activeArc recording seam','archival-owner native-physics feature','signifier-builder-full','NotifyHintSystemRoofShape byte payload','central scheduling/registration'],excluded=['T13'])
(out/'delivery-manifest.json').write_text(json.dumps(report,indent=2),encoding='utf8')
(out/'candidate-bases.json').write_text(json.dumps(sources,indent=2),encoding='utf8')
print(json.dumps(report,indent=2))
