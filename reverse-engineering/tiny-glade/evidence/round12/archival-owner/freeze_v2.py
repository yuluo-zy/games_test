from pathlib import Path
import difflib,hashlib,json,re,subprocess,shutil
O=Path(__file__).resolve().parent;R=Path('D:/game');F=O/'frozen-v2'
base=json.loads((O/'baseline.json').read_text());rb=json.loads((O/'restore-base.json').read_text());base[rb['path']]=rb['base_sha256']
files=[];patch=[]
for name,expected in base.items():
 side='restore-' if '/systems/decorator/' in name else ''
 before=(O/(side+'base')/name).read_bytes();assert hashlib.sha256(before).hexdigest()==expected
 src=O/(side+'candidate')/name
 raw=src.read_bytes();src.write_bytes(raw.replace(b'\r\n',b'\n'))
 result=subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe','--edition','2024',
  '--config','skip_children=true,newline_style=Unix',str(src)],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
 assert result.returncode==0,result.stdout
 after=src.read_bytes();assert b'\r' not in after
 old=before.decode().replace('\r\n','\n').splitlines(True);new=after.decode().splitlines(True)
 diff=list(difflib.unified_diff(old,new,fromfile='a/'+name,tofile='b/'+name));patch.extend(diff)
 # Apply this exact diff to the declared base and compare reconstructed bytes.
 resultlines=[];cursor=0;j=2
 while j<len(diff):
  m=re.match(r'@@ -(\d+)(?:,\d+)? \+\d+(?:,\d+)? @@',diff[j]);assert m,diff[j]
  start=int(m.group(1))-1;resultlines.extend(old[cursor:start]);cursor=start;j+=1
  while j<len(diff) and not diff[j].startswith('@@ '):
   line=diff[j];j+=1
   if line.startswith(' '):assert old[cursor]==line[1:];resultlines.append(line[1:]);cursor+=1
   elif line.startswith('-'):assert old[cursor]==line[1:];cursor+=1
   elif line.startswith('+'):resultlines.append(line[1:])
   else:raise AssertionError(line)
 resultlines.extend(old[cursor:]);assert ''.join(resultlines).encode()==after
 dest=F/'candidate'/name;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(after)
 bd=F/'base'/name;bd.parent.mkdir(parents=True,exist_ok=True);bd.write_bytes(before)
 files.append({'path':name,'base_sha256':expected,'proposed_sha256':hashlib.sha256(after).hexdigest(),
  'production_still_at_base':hashlib.sha256((R/name).read_bytes()).hexdigest()==expected,
  'line_count':len(new),'patch_roundtrip_exact':True})
F.mkdir(exist_ok=True);(F/'all-three.patch').write_bytes(''.join(patch).encode())
for name in ['RESTORE-INTEGRATION.md','INTEGRATION.md','extract-diff-original.json','restore-callees.json','slotmap-ffi.json']:
 shutil.copyfile(O/name,F/name)
manifest={'revision':'frozen-v2','production_written':False,'cargo_run':False,'v2_compile_or_tests_run':False,
 'format_parse':'rustfmt edition2024 skip_children=true newline_style=Unix passed for all3',
 'patch_sha256':hashlib.sha256((F/'all-three.patch').read_bytes()).hexdigest(),'files':files}
(F/'manifest.json').write_text(json.dumps(manifest,indent=2))
print(json.dumps(manifest,indent=2))
