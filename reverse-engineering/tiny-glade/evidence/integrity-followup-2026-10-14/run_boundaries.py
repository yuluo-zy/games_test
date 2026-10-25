"""Identity-bound Windows original-code probes; never starts the shipped EXE."""
import hashlib, json, pathlib, struct, subprocess, sys

work = pathlib.Path(__file__).resolve().parent
original = pathlib.Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
previous = work.parent / 'small-libs-development-2026-10-13/tg-package-integrity'
container = previous / 'original-verify-container.dll'
identity = json.loads((previous / 'native-container-identity.json').read_text(encoding='utf-8'))
source = original.read_bytes()
image = container.read_bytes()
assert hashlib.sha256(source).hexdigest() == identity['original_sha256']
assert hashlib.sha256(image).hexdigest() == identity['container_sha256']
pe = struct.unpack_from('<I', source, 0x3c)[0]
opt = pe + 24
sections = [struct.unpack_from('<8sIIII', source, opt + struct.unpack_from('<H', source, pe+20)[0]+40*i)
            for i in range(struct.unpack_from('<H', source, pe+6)[0])]
def offset(rva):
    for _, vs, va, rs, ro in sections:
        if va <= rva < va+max(vs, rs): return ro+rva-va
    raise ValueError(hex(rva))

pdata = next(s for s in sections if s[0].rstrip(b'\0') == b'.pdata')
runtime_functions = list(struct.iter_unpack('<III', source[pdata[4]:pdata[4]+pdata[3]-pdata[3]%12]))
ranges = []
for address in ['0x14067c3d0','0x140f77f30','0x140f79650']:
    rva = int(address,16)-0x140000000
    begin,end,_ = next(row for row in runtime_functions if row[0] <= rva < row[1])
    assert begin == rva
    start, length = offset(begin), end-begin
    before, after = source[start:start+length], image[start:start+length]
    assert before == after
    ranges.append({'address':address,'size':length,'range_basis':'original RNE PE RUNTIME_FUNCTION begin/end',
                   'sha256':hashlib.sha256(before).hexdigest(),'unchanged':True})

driver = work / 'same-source/target/debug/fixture-driver.exe'
fixtures = work / 'fixtures'
created = subprocess.run([str(driver),'fixtures',str(fixtures)],text=True,encoding='utf-8',capture_output=True,timeout=60)
assert created.returncode == 0, created.stderr
catalog = json.loads((fixtures / 'catalog.json').read_text(encoding='utf-8'))
records = []
for row in catalog:
    process = subprocess.run([sys.executable,str(work/'native_verify.py'),'inspect',str(fixtures/row['name'])],
                             text=True,encoding='utf-8',capture_output=True,timeout=30)
    assert process.returncode == 0, process.stderr
    outputs = [json.loads(line) for line in process.stdout.splitlines() if line.strip()]
    observed = outputs[-1]
    actual = {key:observed[key] for key in ['kind','message','version','lists'] if key in observed}
    expected = row['candidate']
    # std::read_dir ordering is not a game sort contract; compare extra as sets.
    if actual['kind']=='files':
        actual['lists'][2].sort()
        expected['lists'][2].sort()
    records.append({'name':row['name'],'original':actual,'candidate':expected,'equal':actual==expected,
                    'raw_original_stdout':process.stdout,'stderr':process.stderr})
report = {'scope':'Windows x64 stable-filesystem original verify in retained DLL loader container, not shipped EXE process',
          'source':str(original),'source_sha256':identity['original_sha256'],
          'container':str(container),'container_sha256':identity['container_sha256'],
          'patches':identity['patches'],'import_descriptors_removed':identity['import_descriptors_removed'],
          'original_ranges':ranges,'candidate_source':str(pathlib.Path('D:/game/tiny-glade/crates/tg-package-integrity/src/lib.rs')),
          'candidate_source_sha256':hashlib.sha256(pathlib.Path('D:/game/tiny-glade/crates/tg-package-integrity/src/lib.rs').read_bytes()).hexdigest(),
          'driver_sha256':hashlib.sha256(driver.read_bytes()).hexdigest(),
          'case_count':len(records),'mismatches':sum(not row['equal'] for row in records),'cases':records}
(work / 'native-boundaries.json').write_text(json.dumps(report,ensure_ascii=True,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'cases':len(records),'mismatches':report['mismatches']}))
assert report['mismatches'] == 0
