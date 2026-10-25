from pathlib import Path
import json,re,collections
p=Path('D:/game/reverse-engineering/tiny-glade/evidence/render-architecture')
sy=json.loads((p/'public-symbols.json').read_text());mp=collections.defaultdict(list)
for r in sy:
 if r['va']:mp[r['va']].append(r['demangled'])
reports=[json.loads(f.read_text()) for f in [p/'mcp-report.json',p/'mcp-second-report.json'] if f.exists()];maps=[]
for r in [r for report in reports for r in report['results']]:
 if r['name']!='decompile_function':continue
 code=r['result'].get('structuredContent',{}).get('decompiled_code','')
 code='\n'.join(x for x in code.splitlines() if x.strip())+'\n'
 address=r['args']['address'];(p/(address+'.c')).write_text(code,encoding='utf8')
 found=[]
 for line in code.splitlines():
  for addr in re.findall(r'(?:FUN_|func_0x)([0-9a-f]+)\s*\(',line):
   va=hex(int(addr,16))
   if va==address:continue
   names=mp[va]
   found.append({'address':va,'aliases':names,'code':line.strip()})
 maps.append({'function':address,'names':mp[address],'references_in_pseudocode':found,'limitations':['Pseudocode textual order is not runtime order','Shared-address aliases remain ambiguous','Some resolved routines may be inlined caller work or cleanup']})
(p/'resolved-calls.json').write_text(json.dumps(maps,indent=2),encoding='utf8')
for m in maps:
 print(m['function'],m['names'])
 if m['function'] in ['0x1404ec110','0x1404e8010']:
  for c in m['references_in_pseudocode']:
   ns=[n for n in c['aliases'] if n.startswith(('country_core::passes','tiny_glade::','rhapsody::','country_core::render'))]
   if ns:print(c['address'],' | '.join(ns[:4]))
