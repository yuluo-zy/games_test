import json, pathlib, subprocess
work = pathlib.Path(__file__).resolve().parent
example = work/'same-source/target/debug/examples/inspect_package.exe'
contract = json.loads(pathlib.Path('D:/game/tiny-glade/crates/tg-package-integrity/fixtures/manifest-contract.json').read_text(encoding='utf-8'))
by_name = {row['name']:row['bytes'] for row in contract['cases']}
cases = [('read',None),('checksum','not json'),('parse',by_name['parse-after-gate']),
         ('version',by_name['version-two']),('success',by_name['success'])]
results = []
for name,bytes_text in cases:
    root = work/'example-fixtures'/name
    root.mkdir(parents=True,exist_ok=True)
    if bytes_text is not None:
        (root/'build-info').mkdir(exist_ok=True)
        (root/'build-info/manifest.json').write_bytes(bytes_text.encode('utf-8'))
    p = subprocess.run([str(example),str(root)],capture_output=True,text=True,encoding='utf-8',timeout=10)
    assert p.returncode == 0, p.stderr
    data = json.loads(p.stdout)
    assert data['result'].get('error') == (None if name == 'success' else name), data
    assert data['result']['ok'] == (name == 'success'), data
    if name in ('read','checksum','parse'): assert data['version'] is None and data['file_entries'] is None
    results.append({'case':name,'exit_code':p.returncode,'output':data,'stderr':p.stderr})
(work/'example-results.json').write_text(json.dumps({'case_count':len(results),'passed':len(results),'cases':results},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'cases':len(results),'passed':len(results)}))
