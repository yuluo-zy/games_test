import json,pathlib
s=json.loads(pathlib.Path('reverse-engineering/tiny-glade/evidence/render-architecture/public-symbols.json').read_text());targets=['140c169f0','140c17420','14167fb50','140c17740','141528e40','140b77930','140c177f0','14172ed70','140888890','140892dc0']
for t in targets:
 q=[q for q in s if q.get('va')=='0x'+t];print(t,[(x.get('demangled')) for x in q])
