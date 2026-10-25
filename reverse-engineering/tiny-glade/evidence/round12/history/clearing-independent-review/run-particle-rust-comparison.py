from pathlib import Path
import subprocess,json,hashlib
ROOT=Path('D:/game');OUT=Path(__file__).resolve().parent
RUSTC='C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustc.exe'
GLAM=ROOT/'tiny-glade/target/debug/deps/libglam-6103396c150735c2.rlib'
probe=(OUT/'particle_extra_comparison.rs').read_text()
rows=[]
for name,source in [('production',ROOT/'tiny-glade/crates/country-core/src/startup/native_placement_policy.rs'),('axis_angle_candidate',OUT/'native_placement_policy.axis-angle-candidate.rs')]:
    rs=OUT/(name+'_particle_probe.rs');exe=OUT/(name+'_particle_probe.exe')
    rs.write_text(probe.replace('mod native_placement_policy;',f'#[path="{source.as_posix()}"] mod native_placement_policy;'),encoding='utf8')
    command=[RUSTC,'--edition=2021','-C','opt-level=0',str(rs),'--extern',f'glam={GLAM}','-L',f'dependency={GLAM.parent}','-o',str(exe)]
    build=subprocess.run(command,capture_output=True,text=True)
    result={'name':name,'source':str(source),'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'compile_exit':build.returncode,'compile_stderr':build.stderr,'command':command}
    if build.returncode==0:
        run=subprocess.run([str(exe)],capture_output=True,text=True)
        result.update(exit_code=run.returncode,stdout=run.stdout,stderr=run.stderr,executable_sha256=hashlib.sha256(exe.read_bytes()).hexdigest())
    rows.append(result)
(OUT/'particle-rust-comparison-results.json').write_text(json.dumps(rows,indent=2),encoding='utf8')
for r in rows:print(json.dumps({k:r[k] for k in ['name','source_sha256','compile_exit','exit_code','stdout','stderr'] if k in r},indent=2))
