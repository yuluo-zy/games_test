import json,re,tomllib,pathlib
root=pathlib.Path(r'D:/game/reverse-engineering/tiny-glade');work=pathlib.Path(__file__).resolve().parent
raw=(root/'reconstruction/shared-foundation/Cargo.lock').read_text();data=tomllib.loads(raw);packages=data['package'];nodes={}
for p in packages:nodes.setdefault(p['name'],[]).append(p)
seen={}
def walk(p):
 key=(p['name'],p['version'],p.get('source',''))
 if key in seen:return
 seen[key]=p
 for dep in p.get('dependencies',[]):
  a=dep.split();name=a[0];version=a[1] if len(a)>1 and not a[1].startswith('(') else None
  options=[n for n in nodes[name] if version is None or n['version']==version];assert len(options)==1,(dep,options);walk(options[0])
walk(nodes['tg-package-integrity'][0]);blocks=re.split(r'(?m)^\[\[package\]\]\s*$',raw)
out=blocks[0]
for block in blocks[1:]:
 p=tomllib.loads('[[package]]\n'+block)['package'][0]
 if (p['name'],p['version'],p.get('source','')) in seen:out+='[[package]]\n'+block
(work/'same-source-release/Cargo.lock').write_text(out)
original=tomllib.loads(pathlib.Path(r'D:/game/ljxsj_92385/Tiny Glade/build-info/Cargo.lock').read_text());ident={(p['name'],p['version'],p.get('source','')):p for p in original['package']}
checks=[]
for key,p in seen.items():
 if p.get('source'):
  o=ident.get(key);checks.append({'name':p['name'],'version':p['version'],'source':p['source'],'checksum_match':o is not None and p.get('checksum')==o.get('checksum')});assert checks[-1]['checksum_match'],p
(work/'release-dependency-identity.json').write_text(json.dumps({'public_packages':checks,'all_match_original':True},indent=2));print('closed nodes',len(seen),'exact original public deps',len(checks))
