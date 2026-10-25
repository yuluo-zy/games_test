import json, pathlib
work = pathlib.Path(__file__).resolve().parent
catalog = json.loads((work/'fixtures/catalog.json').read_text(encoding='utf-8'))
oracle = json.loads((work/'native-boundaries.json').read_text(encoding='utf-8'))
disk_names = {
    'root-tmp-file':'tmp', 'root-tmp-listed':'tmp', 'uppercase-tmp':'Tmp/file',
    'known-case':'Case', 'separator-alias':'sub/file', 'dot-alias':'good',
    'self-dot-alias':None,'empty-file-name':None,'nul-file-name':None,
    'directory-known':'sub/file', 'unicode-and-surrogate':'中文🧱',
}
by_name = {row['name']:row['original'] for row in oracle['cases']}
assert oracle['mismatches'] == 0 and oracle['case_count'] == len(catalog) == 11
cases = []
for row in catalog:
    name = row['name']
    files = [[disk_names[name],'content']] if disk_names[name] else []
    if name == 'unicode-and-surrogate': files.append(['good','content'])
    cases.append({'name':name,'disk_files':files,'lone_surrogate':name=='unicode-and-surrogate',
                  'bytes':row['bytes'],'expected':by_name[name]})
contract = {'scope':oracle['scope'],'source_sha256':oracle['source_sha256'],
            'original_entry':'0x14067c3d0', 'cases':cases}
destination = pathlib.Path('D:/game/tiny-glade/crates/tg-package-integrity/fixtures/windows-boundaries.json')
destination.write_text(json.dumps(contract,ensure_ascii=True,indent=2)+'\n',encoding='utf-8')
print(destination)
