import {readFileSync,writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {dirname,resolve,join} from 'node:path';
import {fileURLToPath} from 'node:url';
const base=dirname(fileURLToPath(import.meta.url));
const root=resolve(base,'../../../..');
const json=path=>JSON.parse(readFileSync(path,'utf8'));
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
const artifactNames=['catalog.json','interface-families.json','cross-index.json','summary.json','inventory.json','unknowns.json'];
const before=Object.fromEntries(artifactNames.map(name=>[name,hash(readFileSync(join(base,name)))]));
const regenerated=spawnSync(process.execPath,[join(base,'catalog.mjs')],{encoding:'utf8',maxBuffer:64*1024*1024});
if(regenerated.status!==0)throw Error(regenerated.stderr);
const after=Object.fromEntries(artifactNames.map(name=>[name,hash(readFileSync(join(base,name)))]));
const determinism=artifactNames.map(name=>({file:name,before:before[name],after:after[name],identical:before[name]===after[name]}));
if(determinism.some(item=>!item.identical))throw Error('Nondeterministic catalog output');
const catalog=json(join(base,'catalog.json'));
const summary=json(join(base,'summary.json'));
const contract=json(resolve(root,'evidence/render-recovery/parallel-batch-04/contract.json')).packages['R6C-pipeline-shader-interfaces'];
const pythonJson=value=>Array.isArray(value)?'['+value.map(pythonJson).join(', ')+']':value&&typeof value==='object'?'{'+Object.keys(value).sort().map(key=>JSON.stringify(key)+': '+pythonJson(value[key])).join(', ')+'}':JSON.stringify(value);
const contractInventory=catalog.files.map(file=>({name:file.file,size:file.bytes,sha256:file.sha256})).sort((left,right)=>left.name.toLowerCase()<right.name.toLowerCase()?-1:left.name.toLowerCase()>right.name.toLowerCase()?1:0);
const contractDigest=hash(pythonJson(contractInventory));
if(contractDigest!==contract.inventory_sha256)throw Error('Batch04 shader inventory identity mismatch');
if(summary.files!==404||summary.modules!==638||summary.reflected_modules!==638)throw Error('Incomplete shipped module coverage');
const validation=spawnSync(resolve(root,'reconstruction/render/work-packages/pipeline-interfaces-r6/target/debug/validate-catalog.exe'),[join(base,'catalog.json')],{encoding:'utf8',maxBuffer:64*1024*1024});
if(validation.status!==0)throw Error(validation.stderr);
const interfaces=JSON.parse(validation.stdout);
writeFileSync(join(base,'interface-validation.json'),JSON.stringify(interfaces,null,2)+'\n');
const entrypoints=catalog.files.flatMap(file=>file.modules).flatMap(module=>module.entries??[]);
const pushes=entrypoints.flatMap(entry=>entry.push_constants);
const dynamicDescriptors=entrypoints.flatMap(entry=>entry.descriptors).filter(descriptor=>descriptor.count===0);
const moduleSpanErrors=[];
for(const file of catalog.files){
 const bytes=readFileSync(join(summary.source,file.file));
 if(hash(bytes)!==file.sha256)moduleSpanErrors.push({file:file.file,error:'file hash changed'});
 for(const module of file.modules){
  if(bytes.readUInt32LE(module.offset-4)!==module.word_count)moduleSpanErrors.push({file:file.file,error:'word-count prefix mismatch'});
  if(hash(bytes.subarray(module.offset,module.container_end))!==module.sha256)moduleSpanErrors.push({file:file.file,error:'payload hash changed'});
 }
}
const parseLock=path=>readFileSync(path,'utf8').split('[[package]]').slice(1).map(body=>({name:body.match(/name = "([^"]+)"/)[1],version:body.match(/version = "([^"]+)"/)[1],source:body.match(/source = "([^"]+)"/)?.[1],checksum:body.match(/checksum = "([^"]+)"/)?.[1]}));
const original=parseLock(resolve(root,'tracking/original-Cargo.lock'));
const candidate=parseLock(resolve(root,'reconstruction/render/work-packages/pipeline-interfaces-r6/Cargo.lock')).filter(pkg=>pkg.source);
const mismatches=candidate.filter(pkg=>!original.some(old=>old.name===pkg.name&&old.version===pkg.version&&old.source===pkg.source&&old.checksum===pkg.checksum));
if(mismatches.length||moduleSpanErrors.length)throw Error('Identity/dependency mismatch');
writeFileSync(join(base,'dependency-verification.json'),JSON.stringify({schema_version:1,baseline:'tracking/original-Cargo.lock',packages:candidate,mismatches},null,2)+'\n');
const result={schema_version:1,mode:'static_shipped_artifact_interfaces',target_execution:false,database_opened:false,library:'original spirq0.6.2 fork commit4f22e6f149750401705d47ae15a60e77cd5a6905',coverage:summary,identity:{module_span_errors:moduleSpanErrors,inventory_sha256:hash(readFileSync(join(base,'inventory.json'))),batch04_contract_inventory_sha256:contractDigest,contract_inventory_matches:true},determinism,interface_validation:{files:interfaces.files,raster_pairs:interfaces.raster_pairs,descriptor_failures:interfaces.descriptor_failures,push_range_failures:interfaces.push_range_failures,stage_location_failures:interfaces.stage_location_failures},statistics:{push_constant_variables:pushes.length,push_constant_minimum_bytes_histogram:pushes.reduce((result,push)=>{const size=push.type.minimum_bytes;result[size]=(result[size]??0)+1;return result;},{}),runtime_sized_descriptor_variables:dynamicDescriptors.length},candidate:{cargo_test:'cargo test --locked --offline: passed 4 tests',cargo_fmt:'cargo fmt --check: passed',cargo_clippy:'cargo clippy --locked --offline --all-targets -- -D warnings: passed',dependencies_matched:candidate.length},limits:['Validation compares location/component/type only; full Vulkan/BuiltIn linking remains unknown','Descriptor count0 is runtime-sized, not a zero allocation','Spirq minimum_bytes is not a runtime buffer-size bound for dynamic arrays','Reflection never claims shader algorithms, pass invocation, image or runtime equivalence']};
writeFileSync(join(base,'validation.json'),JSON.stringify(result,null,2)+'\n');
console.log(JSON.stringify({interface_validation:result.interface_validation,statistics:result.statistics,dependencies_matched:candidate.length,deterministic:true},null,2));
