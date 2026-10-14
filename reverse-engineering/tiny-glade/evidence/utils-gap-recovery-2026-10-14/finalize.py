"""Attach current scoped evidence; preserve historical import provenance."""
from pathlib import Path
import hashlib,json,re
OUT=Path(__file__).parent;ROOT=OUT.parents[3];WS=ROOT/'tiny-glade';TG=ROOT/'reverse-engineering/tiny-glade'
def read(p):return json.loads(p.read_text(encoding='utf8'))
def write(p,r):p.write_text(json.dumps(r,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
relative='evidence/utils-gap-recovery-2026-10-14'
paths=['crates/utils/src/index_condenser.rs','crates/utils/src/float_grid.rs',
 'crates/utils/src/oracle/index_producer_tests.rs','crates/utils/src/oracle/grid_tests.rs',
 'crates/utils/tests/fixtures/index-producer.json',
 'crates/utils/tests/oracle/shared-foundation-2026-10-13/geometry-curves/boundary_tests.rs',
 'crates/country-core/src/utils/index_condenser.rs','crates/country-core/src/utils/float_grid.rs',
 'crates/country-core/src/render/nani/gpu_world.rs','crates/utils/README.md','crates/country-core/README.md']
stages={n:read(OUT/f'{n}-validation.json') for n in ['debug','release','clippy']}
for n,r in stages.items():
    r['test_results']=re.findall(r'test result: (.*)',(OUT/f'{n}.log').read_text(encoding='utf8'))
validation={'date':'2026-10-14','baseline':'B','stages':stages,
 'country_core_tests':{'command':'cargo test --locked --offline -p country-core --lib','exit_code':0,'passed':1142,'failed':0,'ignored':0,'evidence':'Unified exec session 84321 completion; after production merge and rustfmt, before test-only runtime ingress fix; no production changes since.'},
 'native':{'insert_cases':52,'full_remove_cases':78,'grid_cases':100,'candidate_fixture_tests_passed':stages['debug']['exit_code']==0},
 'prior_failures':{'invalid_grid_flag_input':'grid-invalid-pointer-fixtures.json','release_constant_input':'release-before-runtime-input-validation.json'},
 'source_hashes':{p:sha(WS/p) for p in paths},'original_full_game_accepted':False,'full_module_accepted':False}
write(OUT/'validation.json',validation)
audit={'schema_version':1,'date':'2026-10-14','baseline':'B','report':str((OUT/'REPORT.md').relative_to(ROOT)).replace('\\','/'),
 'historical_import_audit':'provenance/foundation-audit.json',
 'files':[{'path':p,'sha256':sha(WS/p),'kind':'native_evidenced_business_reconstruction' if '/src/' in p else 'fixture_or_documentation'} for p in paths],
 'validation':validation,'cargo_lock_sha256':sha(WS/'Cargo.lock'),
 'exact_author_rust_generic_abi_recovered':False,'original_full_module_accepted':False,
 'supersedes':'Earlier insert exclusion is superseded only for the evidenced duplicate/new-ID policy. 2D global queries and projected slice API remain excluded.'}
write(WS/'provenance/utils-gap-audit.json',audit)

# Current source hashes plus explicit revision history, without rewriting the
# original source_commit/source_sha256 or the historical fixture proofs.
for p in [WS/'provenance/sources.json',WS/'provenance/foundation-audit.json']:
    r=read(p);rows={x['path']:x for x in r['files']}
    for path in paths:
        if path not in rows:
            r['files'].append({'path':path,'sha256':sha(WS/path),'kind':'native_evidenced_followup','audit':'provenance/utils-gap-audit.json'})
            continue
        row=rows[path];previous=row.get('sha256')
        if previous!=sha(WS/path):
            row.setdefault('revision_history',[]).append({'sha256':previous,'superseded_by':'provenance/utils-gap-audit.json','date':'2026-10-14'})
            row['sha256']=sha(WS/path)
        row['current_revision_audit']='provenance/utils-gap-audit.json'
    if 'audits' in r and 'provenance/utils-gap-audit.json' not in r['audits']:r['audits'].append('provenance/utils-gap-audit.json')
    if 'excluded_unrecovered_apis' in r:
        for x in r['excluded_unrecovered_apis']:
            if x['source'].endswith('index_condenser.rs:insert'):
                x['historical_exclusion']=True;x['current_excluded']=False;x['superseded_by']='provenance/utils-gap-audit.json'
        r['validation']['utils_gap_recovery']='provenance/utils-gap-audit.json'
    write(p,r)

ledger={'schema_version':1,'date':'2026-10-14','baseline':'B','status':'partial',
 'source_root':'D:/game/tiny-glade','report':relative+'/REPORT.md','audit':'D:/game/tiny-glade/provenance/utils-gap-audit.json',
 'closed':['insert duplicate/new key policy','complete remove index/cache/map projection','shared IndexCondenser CPU consumer wiring','512x512 byte-grid both-mode numerical oracle','shared FloatGridCells/RasterBufferRef consumer wiring'],
 'remaining':['WallSpaceCoord three compiled bodies not located','Spline2D global-query bodies not located','Vec3SliceView2 projection axes/stride unconfirmed','sampling flag producer and other grid variants','original full producer/GPU process','A/C and universal cross-platform exact floats'],
 'full_module_accepted':False,'validation':validation}
write(TG/'tracking/utils-gap-recovery.json',ledger)
r=read(TG/'tracking/project.json')
r['utils_gap_recovery']={'status':'partial','report':relative+'/REPORT.md','tracking':'tracking/utils-gap-recovery.json','source':'D:/game/tiny-glade/crates/utils','full_original_module_accepted':False}
write(TG/'tracking/project.json',r)
r=read(TG/'tracking/foundation-business-functions.json');lib=next(x for x in r['libraries'] if x['name']=='utils')
notes={
 'utils:09':'512x512 byte-grid 两模式/边界100原指令样例逐位通过，FloatGridCells/RasterBufferRef已接共享入口；flag写入者、其他单态化及完整原运行未验收',
 'utils:11':'insert重复拒绝/新key追加52原样例、完整remove78原样例通过；GPUWorld恢复源码已委托共享入口；原Rust泛型/ABI、分配失败、超大索引及完整原GPU进程未验收',
 'utils:17':'2D构造/局部求值和3D真实查询保持；全B PDB扫描未找到2D全局查询过程/内联宿主，未加入镜像实现；实际编辑消费者未全验收',
 'utils:23':'as_vec2 pointer/length传输保持；全B PDB未定位投影轴/stride消费者，不加入猜测投影接口',
 'utils:27':'全4894模块PDB扫描+MCP符号复核未定位bound/is_bounded/to_affine3编译体；仅声明不能证明未编译，保持未实现'}
for f in lib['features']:
    if f['id'] in notes:
        f['remaining']=notes[f['id']];f['review_evidence']=relative+'/REPORT.md'
        f['current_source']='D:/game/tiny-glade/crates/utils';f['full_feature_accepted']=False
write(TG/'tracking/foundation-business-functions.json',r)
print(json.dumps({'stage_exit_codes':{n:r['exit_code'] for n,r in stages.items()},'source_files':len(paths),'full_module_accepted':False}))
