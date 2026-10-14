import hashlib, json, pathlib, tomllib
work = pathlib.Path(__file__).resolve().parent
original_lock = pathlib.Path('D:/game/ljxsj_92385/Tiny Glade/build-info/Cargo.lock')
candidate_lock = work/'same-source/Cargo.lock'
original = tomllib.loads(original_lock.read_text(encoding='utf-8'))['package']
candidate = tomllib.loads(candidate_lock.read_text(encoding='utf-8'))['package']
rows = []
for package in candidate:
    if 'source' not in package: continue
    matches = [other for other in original if all(other.get(k)==package.get(k) for k in ('name','version','source','checksum'))]
    assert matches, package
    rows.append({k:package[k] for k in ('name','version','source','checksum')})
(work/'dependency-identity.json').write_text(json.dumps({'matched':len(rows),'mismatches':0,
    'original_lock_sha256':hashlib.sha256(original_lock.read_bytes()).hexdigest(),
    'candidate_lock_sha256':hashlib.sha256(candidate_lock.read_bytes()).hexdigest(),
    'packages':rows},indent=2)+'\n',encoding='utf-8')
crate = pathlib.Path('D:/game/tiny-glade/crates/tg-package-integrity')
files = ['src/lib.rs','fixtures/windows-boundaries.json','examples/inspect_package.rs','README.md']
checks = {name:json.loads((work/(name+'.json')).read_text(encoding='utf-8-sig')) for name in ['clippy','build']}
assert checks['clippy']['exit_code']==checks['build']['exit_code']==0
native = json.loads((work/'native-boundaries.json').read_text(encoding='utf-8'))
assert native['mismatches']==0 and native['case_count']==11
assert native['candidate_source_sha256']==hashlib.sha256((crate/'src/lib.rs').read_bytes()).hexdigest()
examples = json.loads((work/'example-results.json').read_text(encoding='utf-8'))
assert examples['passed']==examples['case_count']==5
summary = {'scope':'B Windows x64 package-integrity followup; library production algorithm unchanged',
    'native_cases':11,'native_mismatches':0,'example_cases':5,'example_passed':5,
    'rust_tests':{'passed':8,'failed':0,'ignored':0,'observed_unified_exec_session':24701,
                  'command':'cargo test --offline --locked --manifest-path same-source/Cargo.toml --all-targets'},
    'strict_clippy':checks['clippy'],'build':checks['build'], 'dependencies_matched':len(rows),
    'source_files':[{'path':str(crate/name),'sha256':hashlib.sha256((crate/name).read_bytes()).hexdigest()} for name in files],
    'remaining':['shipped-process startup/report/crash consumers','full OS/error/reparse categories',
                 'concurrent mutation and thread-spawn/panic failure timing','A/C and 32-bit/big-endian targets'],
    'algorithm_change':False,'dependency_change':False,'new_mcp_session':False,
    'original_code_scope':'unchanged verify function inside retained patched loader container; loader headers/imports/TLS differ'}
(work/'summary.json').write_text(json.dumps(summary,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'native_cases':11,'rust_tests':8,'example_cases':5,'dependencies':len(rows)}))
