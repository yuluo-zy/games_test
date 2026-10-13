import pathlib,re,collections,json,hashlib
root=pathlib.Path(r'D:\game\ljxsj_92385\Tiny Glade')
d={}
for kind,path in [('texture',root/'assets-src/textures'),('prefab',root/'assets/prefabs')]:
 c=collections.Counter(); vals=collections.Counter(); files=[]
 for p in path.rglob('*.ron'):
  s=p.read_text(encoding='utf8'); c.update(re.findall(r'\b([A-Za-z][A-Za-z0-9_]*)\s*:',s)); vals.update(re.findall(r':\s*([A-Za-z][A-Za-z0-9_]*)',s))
  files.append({'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
 d[kind]={'fields':dict(c),'values':dict(vals),'files':files}
(pathlib.Path(__file__).parent/'config-inventory.json').write_text(json.dumps(d,indent=2))
print(json.dumps({k:{x:y for x,y in v.items() if x!='files'} for k,v in d.items()},indent=2))
