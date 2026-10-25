from pathlib import Path
import json, subprocess, re
root = Path('D:/game/reverse-engineering/tiny-glade')
report = json.loads((root/'evidence/native/roof-state-slice.json').read_text(encoding='utf-8'))
calls = []
for function in report['functions']:
    for line in function['instructions']:
        match = re.search(r'^(\w+): CALL (0x[0-9a-f]+)$',line)
        if match:
            calls.append({'function':function['name'],'callsite':'0x'+match[1],'destination':match[2],'aliases':[]})
targets = {int(c['destination'],16) for c in calls}
process = subprocess.Popen(['C:/Program Files/LLVM/bin/llvm-pdbutil.exe','dump','--publics','D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb'],stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,encoding='utf-8',errors='replace')
active = None
for line in process.stdout:
    if 'S_PUB32' in line:
        active = line.strip()
    elif active and 'addr =' in line:
        m = re.search(r'addr = (\d+):(\d+)',line)
        if m and int(m[1])==1:
            va = 0x140001000+int(m[2])
            if va in targets:
                for call in calls:
                    if int(call['destination'],16)==va:
                        call['aliases'].append(active)
        active = None
process.wait()
if process.returncode:
    raise RuntimeError(process.stderr.read())
for c in calls:
    aliases = c['aliases']
    aliases.sort(key=lambda x:0 if any(k in x for k in ['country_core','system_roof']) else 1)
    c['alias_count']=len(aliases)
    c['aliases']=aliases[:8]
(root/'evidence/native/roof-state-calls.json').write_text(json.dumps(calls,ensure_ascii=False,indent=2),encoding='utf-8')
for c in calls:
    print(c['destination'], c['aliases'][:2])
