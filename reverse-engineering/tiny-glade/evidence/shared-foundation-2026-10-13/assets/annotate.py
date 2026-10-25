exec(open(__file__.replace('annotate.py','disassemble.py')).read().split('rows=json')[0])
rows=json.loads((p/'functions.json').read_text()); names={r['va'][2:]:r['name'].split('<')[0].replace('::','__') for r in rows}
for f in p.glob('decomp-*.c'):
 s=f.read_text(); s=re.sub(r'\n\s*\n','\n',s)
 s=re.sub(r'FUN_([0-9a-f]+)',lambda m:names.get(m[1],m[0]),s)
 (p/f.name.replace('decomp','readable')).write_text(s,encoding='utf8')
