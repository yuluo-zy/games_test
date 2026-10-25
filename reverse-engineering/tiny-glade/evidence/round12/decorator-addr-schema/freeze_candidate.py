from pathlib import Path
import json,hashlib,difflib,re,subprocess,shutil
O=Path(__file__).resolve().parent;F=O/'frozen-schema-v1';base=json.loads((O/'base-manifest.json').read_text())
rows=[];patch=[]
for name,meta in base.items():
 before=(O/'base'/name).read_bytes();assert hashlib.sha256(before).hexdigest()==meta['base_sha256']
 p=O/'candidate'/name;p.write_bytes(p.read_bytes().replace(b'\r\n',b'\n'))
 result=subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe','--edition','2024',
  '--config','skip_children=true,newline_style=Unix',str(p)],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
 assert result.returncode==0,result.stdout
 after=p.read_bytes();old=before.decode().replace('\r\n','\n').splitlines(True);new=after.decode().splitlines(True)
 diff=list(difflib.unified_diff(old,new,fromfile='a/'+name,tofile='b/'+name));patch.extend(diff)
 rebuilt=[];cursor=0;j=2
 while j<len(diff):
  m=re.match(r'@@ -(\d+)(?:,\d+)? \+\d+(?:,\d+)? @@',diff[j]);assert m
  start=int(m.group(1))-1;rebuilt.extend(old[cursor:start]);cursor=start;j+=1
  while j<len(diff) and not diff[j].startswith('@@ '):
   line=diff[j];j+=1
   if line.startswith(' '):assert old[cursor]==line[1:];rebuilt.append(line[1:]);cursor+=1
   elif line.startswith('-'):assert old[cursor]==line[1:];cursor+=1
   elif line.startswith('+'):rebuilt.append(line[1:])
   else:raise AssertionError(line)
 rebuilt.extend(old[cursor:]);assert ''.join(rebuilt).encode()==after
 for side,data in [('base',before),('candidate',after)]:
  dest=F/side/name;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data)
 rows.append({'path':name,**meta,'base_source_still_matches':hashlib.sha256(Path(meta['base_source']).read_bytes()).hexdigest()==meta['base_sha256'],
  'proposed_sha256':hashlib.sha256(after).hexdigest(),'patch_roundtrip_exact':True,'line_count':len(new)})
F.mkdir(exist_ok=True);(F/'schema.patch').write_bytes(''.join(patch).encode())
for name in ['REPORT.md','native-schema.json','source-consumers.json','corpus-schema.json','original-nonempty-address-state.json']:
 shutil.copyfile(O/name,F/name)
m={'revision':'frozen-schema-v1','requires':'archival-owner/frozen-v2 applied first; world unchanged',
 'production_written':False,'cargo_run':False,'codegen_or_tests_run':False,'rustfmt_parse':'passed',
 'patch_sha256':hashlib.sha256((F/'schema.patch').read_bytes()).hexdigest(),'files':rows,
 'fixture_sha256':hashlib.sha256((F/'original-nonempty-address-state.json').read_bytes()).hexdigest()}
(F/'manifest.json').write_text(json.dumps(m,indent=2));print(json.dumps(m,indent=2))
