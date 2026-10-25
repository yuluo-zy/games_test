from pathlib import Path
import json,subprocess,os,shutil,time
ROOT=Path('D:/game'); OUT=Path(__file__).resolve().parent
target=ROOT/'tiny-glade/target-round12/debug'
manifest=json.loads((target/'.fingerprint/country-core-453e83dd454432af/lib-country_core.json').read_text())
args=['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustc.exe',
      '--edition=2024','--crate-type=lib','--emit=metadata','--cfg','feature="native-physics"',
      str(OUT/'candidate_precheck.rs'),'--extern','country_core='+str(target/'deps/libcountry_core-453e83dd454432af.rmeta'),
      '-L','dependency='+str(target/'deps'),'-o',str(OUT/'candidate-precheck.rmeta')]
selected={}
for _,name,_,fingerprint in manifest['deps']:
    for folder in (target/'.fingerprint').glob(name.replace('_','-')+'-*'):
        p=folder/('lib-'+name)
        if not p.exists():continue
        if int.from_bytes(bytes.fromhex(p.read_text().strip()),'little')!=fingerprint:continue
        rlib=target/'deps'/('lib'+name+'-'+folder.name.rsplit('-',1)[-1]+'.rlib')
        if rlib.exists():selected[name]=str(rlib);break
    if name not in selected:
        for folder in (target/'.fingerprint').glob(name+'-*'):
            p=folder/('lib-'+name)
            if not p.exists():continue
            if int.from_bytes(bytes.fromhex(p.read_text().strip()),'little')!=fingerprint:continue
            rlib=target/'deps'/('lib'+name+'-'+folder.name.rsplit('-',1)[-1]+'.rlib')
            if rlib.exists():selected[name]=str(rlib);break
for name,path in selected.items():args+=['--extern',name+'='+path]
# Copy unchanged child only for resolving the candidate module tree.
source=ROOT/'tiny-glade/crates/country-core/src/resources/walls/decorator_archivist/changes.rs'
for dest in [OUT/'candidate/tiny-glade/crates/country-core/src/resources/walls/decorator_archivist/changes.rs',
             OUT/'candidate/tiny-glade/crates/country-core/src/resources/walls/changes.rs']:
    dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source,dest)
env=os.environ.copy();env['CARGO_MANIFEST_DIR']=str(ROOT/'tiny-glade/crates/country-core')
t=time.perf_counter();r=subprocess.run(args,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
(OUT/'candidate-precheck.log').write_bytes(r.stdout)
(OUT/'precheck.json').write_text(json.dumps({'argv':args,'exit_code':r.returncode,'seconds':time.perf_counter()-t,
    'production_written':False,'cargo_run':False,'dependencies':selected},indent=2))
print(r.stdout.decode(errors='replace'));print('exit',r.returncode)
