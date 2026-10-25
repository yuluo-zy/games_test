"""Log a bounded Cargo validation stage and bind it to actual task inputs."""
from pathlib import Path
import hashlib,json,subprocess,sys,time
OUT=Path(__file__).parent;ROOT=OUT.parents[3];WS=ROOT/'tiny-glade'
name=sys.argv[1]
commands={
 'debug':['cargo','test','--locked','--offline','-p','utils','--all-targets'],
 'release':['cargo','test','--locked','--offline','-p','utils','--all-targets','--release'],
 'clippy':['cargo','clippy','--locked','--offline','-p','utils','-p','country-core','--all-targets','--','-D','warnings'],
}
paths=['Cargo.toml','Cargo.lock','crates/utils/Cargo.toml','crates/country-core/Cargo.toml',
 'crates/utils/src/index_condenser.rs','crates/utils/src/float_grid.rs',
 'crates/utils/src/oracle/index_producer_tests.rs','crates/utils/src/oracle/grid_tests.rs',
 'crates/utils/tests/fixtures/index-producer.json',
 'crates/utils/tests/oracle/shared-foundation-2026-10-13/geometry-curves/boundary_tests.rs',
 'crates/country-core/src/utils/index_condenser.rs','crates/country-core/src/utils/float_grid.rs',
 'crates/country-core/src/render/nani/gpu_world.rs']
def hashes(): return {p:hashlib.sha256((WS/p).read_bytes()).hexdigest() for p in paths}
before=hashes();start=time.monotonic()
with (OUT/f'{name}.log').open('w',encoding='utf8') as f:
    result=subprocess.run(commands[name],cwd=WS,stdout=f,stderr=subprocess.STDOUT)
after=hashes()
report={'stage':name,'command':commands[name],'cwd':str(WS),'platform':'Windows x86_64',
 'exit_code':result.returncode,'seconds':time.monotonic()-start,'inputs':before,
 'input_changes_during_stage':{k:after[k] for k in before if before[k]!=after[k]},'log':name+'.log'}
(OUT/f'{name}-validation.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
print(json.dumps({'stage':name,'exit_code':result.returncode,'input_changes':report['input_changes_during_stage'],'seconds':report['seconds']}),flush=True)
if result.returncode: print((OUT/f'{name}.log').read_text(encoding='utf8')[-6000:],flush=True)
sys.exit(result.returncode)
