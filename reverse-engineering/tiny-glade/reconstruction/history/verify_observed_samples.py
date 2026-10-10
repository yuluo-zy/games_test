"""Validate recovered observations against original JSON; no simulation or engine."""
import hashlib, json
from pathlib import Path

OUT=Path(__file__).resolve().parent
schema=json.loads((OUT/'observed-history-schema.json').read_text(encoding='utf-8'))
ROOT=Path(schema['root']); docs={}
for f in schema['history_files']:
    p=ROOT/f['file']; data=p.read_bytes()
    assert hashlib.sha256(data).hexdigest()==f['sha256'],f['file']
    doc=json.loads(data); docs[f['file']]=doc
    assert json.loads(json.dumps(doc,ensure_ascii=False))==doc, 'lossless integer/numeric and structure roundtrip'

def resolve(doc,pointer):
    if not pointer: return doc
    for token in pointer.lstrip('/').split('/'):
        token=token.replace('~1','/').replace('~0','~')
        doc=doc[int(token)] if isinstance(doc,list) else doc[token]
    return doc

source_count=0
for node in schema['path_schema'].values():
    for source in node['sources']:
        resolve(docs[source['file']],source['pointer']); source_count+=1
for variants in schema['nested_tag_observations'].values():
    for tag,source in variants['sources'].items():
        v=resolve(docs[source['file']],source['pointer']); assert list(v)==[tag]

# Direct ground-truth fixtures detect lost nested operations or wrong tuple assumptions.
doc=docs['assets/starting-builds/demo/00-demo2/history.json']
wall=doc['History']['edit_chain'][0]['edits'][0]['WallNewSystem']
assert wall[0][0]['CreateWall']['wall_index']==25
assert wall[1][0]['WallChanged']['wall_index']==25
assert wall[0][0]['CreateWall']['after']==wall[1][0]['WallChanged']['before']
doc=docs['assets/starting-builds/full/04-small-bridge/history.json']
curve=doc['History']['edit_chain'][28]['edits'][0]['Wall'][0][0]['MorphFreehandWall']['before']
assert len(curve['points'][0])==2, '2D freehand morph must not be coerced into Vec3'
doc=docs['assets/starting-builds/demo/01-demo3/history.json']
entry=doc['History']['edit_chain'][7]
assert entry['edits']==[{'DecoratorV5':{'Place':{}}}]
target,state=entry['decorators']['diffs'][0]
assert target=={'Wall':2} and state['before']=={} and state['after']['0']['ty']=='CottageWindow'
doc=docs['assets/starting-builds/full/03-halftimber-roof/history.json']
stairs=doc['History']['edit_chain'][53]['stairs']
assert set(stairs)=={'before','after','hash_before','hash_after'}
assert all(len(edge)==2 for edge in stairs['after'])
assert doc['Version']==27 and doc['BackwardCompatibility']==27
assert sum(schema['command_counts'].values())==2774
assert sum(f['chain_length'] for f in schema['history_files'])==2120
assert schema['wall_group_observations']['WallNewSystem']['group_length_counts']['5']==143
assert all(x['check']=='snapshot reference file exists' for x in schema['structural_check_issues'])
result={'evidence_level':'VERIFIED_STATIC_SAMPLES','target_executed':False,'result':'passed',
 'fixtures':len(docs),'source_pointers_resolved':source_count,
 'checks':['All 33 source file SHA-256 identities match extraction','Lossless JSON roundtrip including large integer IDs/hashes',
    'Every recorded example pointer exists','Every recorded nested tag pointer matches its singleton tag',
    'CreateWall.after equals same-command WallChanged.before for wall_index 25 fixture',
    '2D freehand morph preserved separately from 3D geometry curves','Decorator Place empty marker and owner scoped delta validated',
    'Stairs edge/hash sidecar structurally distinct from decorator diffs','Version/BackwardCompatibility metadata observed',
    '2774 outer commands / 2120 history entries, grouped wall operations retain all phases'],
 'coverage_limits':['No binary snapshot decoding','22 of 54 snapshot references lack the same-directory binary file',
    'No runtime replay, undo semantics, mutation ordering, geometry reconstruction, or rendering verification']}
(OUT/'sample-verification.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps(result,ensure_ascii=False))
