from pathlib import Path
import json,subprocess,os,re,time,hashlib
O=Path(__file__).resolve().parent;R=Path('D:/game');F=O/'frozen-v2'
config=json.loads((O/'precheck.json').read_text());args=config['argv']
args.insert(1,'--test')
args[args.index('-o')+1]=str(O/'v2-test-metadata.rmeta')
facade=(O/'candidate_precheck.rs').read_text(encoding='utf8')
facade=facade.replace(str(O).replace('\\','/')+'/candidate/',str(F).replace('\\','/')+'/candidate/')
facade=facade.replace('extern crate country_core as baseline;','extern crate country_core as baseline;\nextern crate self as country_core;')
facade=facade.replace('pub use baseline::systems::collision::*;','pub use baseline::systems::collision::*;\n#[path="D:/game/tiny-glade/crates/country-core/src/systems/collision/intersect_meshes.rs"] pub mod intersect_meshes;')
source=(R/'tiny-glade/crates/systems/decorator/src/apply_decorator_op.rs').read_text(encoding='utf8')
helpers=[]
for name in ['info_to_bytes','bytes_to_info']:
 start=source.index('pub(crate) fn '+name);end=source.index('{',start);depth=1;cursor=end+1
 while depth:
  if source[cursor]=='{':depth+=1
  elif source[cursor]=='}':depth-=1
  cursor+=1
 helpers.append(source[start:cursor])
facade+='\npub mod apply_decorator_op {\nuse crate::resources::walls::decorator::{DecoratorInfo,DecoratorExtraInfo,StairExtra};\n'+ '\n'.join(helpers)+'\n}\n'
facade+='#[path="'+str(F/'candidate/tiny-glade/crates/systems/decorator/src/restore_archived_decorators.rs').replace('\\','/')+'"] pub mod restore_archived_decorators;\n'
file=O/'v2_precheck.rs';file.write_text(facade,encoding='utf8')
args[args.index(str(O/'candidate_precheck.rs'))]=str(file)
# Unchanged real child source is only needed to resolve the facade path.
child=R/'tiny-glade/crates/country-core/src/resources/walls/decorator_archivist/changes.rs'
# Keep frozen snapshot immutable: use the existing candidate child directories
# via a precise path attribute in a separate facade copy of its parent module.
arch=F/'candidate/tiny-glade/crates/country-core/src/resources/walls/decorator_archivist.rs'
arch_text=arch.read_text(encoding='utf8').replace('pub mod changes;','#[path="'+str(child).replace('\\','/')+'"] pub mod changes;')
arch_text=arch_text.replace('[DecoratorId(2)].into()','HashSet::from([DecoratorId(2)])').replace('[DecoratorId(4)].into()','HashSet::from([DecoratorId(4)])')
shadow=O/'v2_archivist_check.rs';shadow.write_text(arch_text,encoding='utf8')
facade=facade.replace(str(arch).replace('\\','/'),str(shadow).replace('\\','/'));file.write_text(facade,encoding='utf8')
env=os.environ.copy();env['CARGO_MANIFEST_DIR']=str(R/'tiny-glade/crates/country-core')
t=time.perf_counter();result=subprocess.run(args,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
(O/'v2-test-metadata.log').write_bytes(result.stdout)
(O/'v2-test-metadata.json').write_text(json.dumps({'argv':args,'exit_code':result.returncode,'seconds':time.perf_counter()-t,
 'production_written':False,'cargo_run':False,'codegen_or_execution':False,'status':'author type-check only, not independent acceptance',
 'frozen_manifest_sha256':hashlib.sha256((F/'manifest.json').read_bytes()).hexdigest()},indent=2))
print(result.stdout.decode(errors='replace'));print('exit',result.returncode)
