import hashlib,json,pathlib
OUT=pathlib.Path(__file__).resolve().parent;ROOT=OUT.parents[2]
cache=pathlib.Path('C:/Users/liyu/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f/easy-parallel-3.2.0.crate')
expected='6907e25393cdcc1f4f3f513d9aac1e840eb1cc341a0fccb01171f7d14d10b946'
assert hashlib.sha256(cache.read_bytes()).hexdigest()==expected
dep=pathlib.Path('C:/Users/liyu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/easy-parallel-3.2.0/src/lib.rs')
source=ROOT/'reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs'
assert 'easy_parallel::Parallel::new()' in source.read_text()
report={'schema_version':1,'baseline':'B','package':'easy-parallel','version':'3.2.0','checksum':expected,
 'original_call_sites':{'each':'0x1417b7fc3 → 0x1420dd5e0','run':'0x1417b7fd3 → 0x1420dc1a0'},
 'original_evidence':'../assets/asm-0x1417b7790.txt; LLVM generic signature each iterator=(previous meta option,name,descriptor), then run()',
 'strategy':'Parallel::new().each(descriptors, per_asset_closure).run(); original APIs take no threadcount/chunk parameter',
 'library_semantics':['one closure per descriptor','all but final closure run on library-created scoped threads; final closure runs on calling thread',
     'join all workers before exposing result vector','results retain insertion order','resume panic on calling thread only after joining'],
 'private_policy':['Result errors processed only after run returned','first Result error aborts metadata/list persistence','successful per-asset output files can exist despite another job error/panic'],
 'implemented_original_library_reuse':True,'custom_thread_pool_or_scheduler':False,
 'dependency_source_sha256':hashlib.sha256(dep.read_bytes()).hexdigest(),
 'candidate_source':source.relative_to(ROOT).as_posix(),'candidate_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
 'verification':{'crate_tests':14,'pipeline_contracts':10,'strict_clippy':True,'log':'cargo-test.log',
     'new_tests':['failed_job_joins_other_tasks_and_leaves_metadata_and_list_unchanged',
                  'panicking_job_joins_other_tasks_before_unwinding_and_does_not_persist_indices']},
 'limits':['original outer print/log/CLI flags remain unmodeled','no complete original compile_texture_assets native entry differential']}
(OUT/'parallel-policy.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print(json.dumps({'original_dependency_verified':True,'candidate_sha256':report['candidate_sha256']}))
