"""Run an already linked acceptance binary; record immutable artifact identity."""
import hashlib, json, os, subprocess, sys, time
from pathlib import Path
base=Path(__file__).resolve().parent
label, executable, *arguments=sys.argv[1:]
exe=Path(executable)
identity={'executable':str(exe),'sha256':hashlib.sha256(exe.read_bytes()).hexdigest(),'arguments':arguments,'compiled_source_manifest':'validation-v20-source-start.json','scope':'already linked executable; current workspace may differ'}
env=os.environ.copy()
env.update(RHAPSODY_REQUIRE_VULKAN='1', RHAPSODY_VULKAN_LIBRARY='vulkan-1.dll', TG_INSTALL_ROOT='D:/game/ljxsj_92385/Tiny Glade', TG_COMPILED_SHADERS='D:/game/ljxsj_92385/Tiny Glade/compiled-shaders')
env['PATH']=env['TG_INSTALL_ROOT']+';'+env.get('PATH','')
(base/(label+'-identity.json')).write_text(json.dumps(identity,indent=2),encoding='utf-8')
start=time.time()
with (base/(label+'.log')).open('wb') as out:
    process=subprocess.Popen([str(exe)]+arguments,cwd='D:/game/tiny-glade',env=env,stdout=out,stderr=subprocess.STDOUT)
    print('test_pid',process.pid,flush=True)
    result=process.wait()
(base/(label+'-result.json')).write_text(json.dumps(dict(identity,exit_code=result,duration_seconds=time.time()-start),indent=2),encoding='utf-8')
print('test_exit',result,flush=True)
raise SystemExit(result)
