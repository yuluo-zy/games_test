from pathlib import Path
R=Path('D:/game/reverse-engineering/tiny-glade');S=R/'scripts'
paths=[S/'roof-tiles'/name for name in ['verify_helpers.py','verify_circular.py','verify_corners.py','verify_context.py']]
paths += [S/'roof-ridge'/name for name in ['verify_faces.py','verify_composed_faces.py']]
paths += [S/'roof-surface/verify-caps.py',S/'verify-rectangular-edges-full.py',S/'verify-roof-pipeline.py']
for p in paths:
 s=p.read_text(encoding='utf-8');lines=s.splitlines();output=[]
 for line in lines:
  if "subprocess.run(['C:/Users/liyu/.cargo/bin/rustc.exe'" in line:
   if p.parent.name=='roof-tiles':
    prefix=line.split(';subprocess.run')[0] if ';subprocess.run' in line else ''
    var='R'
    command='build_runner(runner,exe)'
   elif p.name in ['verify_faces.py','verify_composed_faces.py']:
    prefix='';var='ROOT';command="build_runner(ROOT/'scripts/roof-ridge/faces_runner.rs',exe)"
   elif p.name=='verify-caps.py':
    prefix='';var='ROOT';command="build_runner(ROOT/'scripts/roof-surface/caps_runner.rs',binary)"
   elif p.name=='verify-rectangular-edges-full.py':
    prefix='';var='root';command="build_runner(root/'scripts/edge-runner.rs',root/'workspace/edge-runner.exe')"
   else:
    prefix='';var='root';command="build_runner(root/'scripts/roof-pipeline-runner.rs',exe)"
   if prefix:output.append(prefix)
   output.extend(["import sys",f"sys.path.insert(0,str({var}/'scripts'))","from dependency_build import build_runner",command])
  else:output.append(line)
 p.write_text('\n'.join(output)+'\n',encoding='utf-8');print(p.name)
