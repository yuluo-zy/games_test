"""Record current selected source SHA without replacing historical import IDs."""
from pathlib import Path
import hashlib,json,re
OUT=Path(__file__).parent;ROOT=OUT.parents[3];PKG=ROOT/'tiny-glade';RE=ROOT/'reverse-engineering/tiny-glade'
def read(p):return json.loads(p.read_text(encoding='utf-8-sig'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
paths=['crates/utils/src/natural_cubic.rs','crates/utils/src/swizzlers.rs','crates/utils/src/curve/mod.rs',
       'crates/utils/tests/deep_consumers.rs','crates/utils/tests/fixtures/deep-consumers.json',
       'crates/utils/tests/fixtures/deep-spline3d.json','crates/utils/README.md']
files=[{'path':p,'sha256':sha(PKG/p),'kind':'original_evidenced_reconstruction_with_documented_adapters' if '/src/' in p else 'fixture_or_documentation'} for p in paths]
gates=read(OUT/'gate-results.json')
for gate in gates:
    assert gate['exit_code']==0 and not gate['changes_during_stage'],gate['name']
    gate['test_results']=re.findall(r'test result: (.+)',(OUT/gate['log']).read_text(encoding='utf8'))
    gate['log_sha256']=sha(OUT/gate['log'])
# Check our raw C13 parser independently against LLVM's module 761 formatting.
lines=read(OUT/'source-line-inventory.json')
expected=[(str(e['line']),int(e['section_offset'],16)) for r in lines['source_ranges'] if r['module']==761 for e in r['entries']]
dump=(OUT/'module761-lines.txt').read_text(encoding='utf-8-sig')
actual=[(m[0],int(m[1],16)) for m in re.findall(r'(?<![\w])([0-9]+)\s+([0-9A-F]{8})\s+!',dump)]
assert expected==actual,(len(expected),len(actual))
native=read(OUT/'consumer-native-fixtures.json')
reopened=read(OUT/'reopen-result.artifacts/000.stdout.txt')['functions'][0]
assert reopened['name']=='dense_segment_B' and reopened['signature_source']=='USER_DEFINED' and reopened['decompiler_completed']
val={'baseline':'B','date':'2026-10-14','platform':'Windows x86_64','files':files,'gates':gates,
     'native':{'dense_rows':len(native['dense']),'bounds_panics':sum(v['panic'] for v in native['dense']),
               'projection_arrays':len(native['projection']),'projection_points':sum(len(v['input']) for v in native['projection']),
               'strict_3d_constructor_rows':20,'strict_3d_query_rows':120},
     'line_parser_llvm_module761_matches':len(actual),'full_original_module_accepted':False,
     'overlay_reopen':{k:reopened[k] for k in ['name','signature','signature_source','body_bytes','prototype_is_not_verified_Rust_ABI']},
     'user_scope':'Only B; A/C not requested',
     'evidence_sha256':{str(p.relative_to(OUT)):sha(p) for p in OUT.iterdir() if p.is_file() and p.suffix in ('.json','.py','.rs','.txt','.md') and p.name not in ['validation.json']}}
write(OUT/'validation.json',val)
audit_rel='provenance/utils-deep-audit.json'
audit={'schema_version':1,'date':'2026-10-14','baseline':'B','status':'partial',
       'report':'reverse-engineering/tiny-glade/evidence/utils-deep-followup-2026-10-14/REPORT.md',
       'files':files,'validation':val,'historical_import_audit':'provenance/foundation-audit.json',
       'boundaries':['Dense API represents original system_decorator consumer, not original Spline2D global query',
                     'Owned Curve XZ mapping proven; borrowed as_vec2 receiver/return/axes remain unknown',
                     'All-module absence is not proof of dead source or algorithm semantics']}
write(PKG/audit_rel,audit)
by_path={v['path']:v for v in files}
for name in ['sources.json','foundation-audit.json']:
    p=PKG/'provenance'/name;d=read(p)
    if name=='sources.json' and audit_rel not in d['audits']:d['audits'].append(audit_rel)
    if name=='foundation-audit.json':d.setdefault('current_followup_audits',[]).append(audit_rel) if audit_rel not in d.setdefault('current_followup_audits',[]) else None
    seen=set()
    for f in d['files']:
        path=f['path']
        if path not in by_path:continue
        seen.add(path);new=by_path[path]['sha256']
        if f.get('sha256')!=new:
            f.setdefault('revision_history',[]).append({'date':'2026-10-14','sha256':f.get('sha256'),'audit':f.get('audit','provenance/foundation-audit.json'),'scope':'Historical selected source, superseded by B consumer follow-up'})
        f['sha256']=new;f['audit']=audit_rel
        f['current_revision']='2026-10-14 B consumer follow-up (working tree; original import commit preserved)'
        evidence_sources={'crates/utils/tests/deep_consumers.rs':'deep-consumers-tests.rs','crates/utils/tests/fixtures/deep-consumers.json':'consumer-native-fixtures.json','crates/utils/tests/fixtures/deep-spline3d.json':'spline3d-original-fixtures.json'}
        if f.get('source_commit') is None and path in evidence_sources:
            source=OUT/evidence_sources[path];f['source_path']=str(source);f['source_sha256']=sha(source)
    if name=='sources.json':
        for path,f in by_path.items():
            if path not in seen:
                source=OUT/({'crates/utils/tests/deep_consumers.rs':'deep-consumers-tests.rs','crates/utils/tests/fixtures/deep-consumers.json':'consumer-native-fixtures.json','crates/utils/tests/fixtures/deep-spline3d.json':'spline3d-original-fixtures.json'}.get(path,'REPORT.md'))
                d['files'].append({**f,'source_path':str(source),'source_sha256':sha(source),'source_commit':None,'audit':audit_rel})
    write(p,d)
tracking={'schema_version':1,'date':'2026-10-14','baseline':'B','user_scope':'Only B','status':'partial',
          'report':'evidence/utils-deep-followup-2026-10-14/REPORT.md','audit':'D:/game/tiny-glade/'+audit_rel,
          'closed':['Original 2D dense-segment consumer','Scalar RHS/coefficient evaluation order','Owned Curve XZ axes and 12/8 stride'],
          'remaining':['WallSpaceCoord three method bodies','Spline2D global query bodies','Original borrowed as_vec2 type/axes/stride','Sampling flag writer and full consumer process'],
          'validation':'evidence/utils-deep-followup-2026-10-14/validation.json','full_module_accepted':False}
write(RE/'tracking/utils-deep-followup.json',tracking)
p=RE/'tracking/project.json';d=read(p)
d['utils_deep_followup']={k:tracking[k] for k in ['baseline','status','report','validation','user_scope']}
write(p,d)
p=RE/'tracking/utils-gap-recovery.json';d=read(p)
d['historical_snapshot']=True;d['current_followup']='tracking/utils-deep-followup.json'
d['scope_correction']='User requested B only; historical A/C remainder is out of current task scope. Owned Curve projection now evidenced; borrowed as_vec2 type remains unknown.'
write(p,d)
print(json.dumps({'utils_tests':93,'country_tests':1142,'native':val['native'],'llvm_matched_line_entries':len(actual),'audit':audit_rel}))
