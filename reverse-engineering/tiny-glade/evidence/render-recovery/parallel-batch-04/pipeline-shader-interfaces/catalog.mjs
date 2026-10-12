import {readFileSync, writeFileSync, readdirSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {resolve, dirname, join} from 'node:path';

const base=dirname(fileURLToPath(import.meta.url));
const root=resolve(base,'../../../..');
const shaderRoot=process.argv[2]??resolve(root,'../../ljxsj_92385/Tiny Glade/compiled-shaders');
const executable=resolve(root,'reconstruction/render/work-packages/pipeline-interfaces-r6/target/debug/reflect-shaders.exe');
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
const readJson=path=>JSON.parse(readFileSync(resolve(root,path),'utf8'));
const write=(name,value)=>writeFileSync(join(base,name),JSON.stringify(value,null,2)+'\n');
const stable=value=>JSON.stringify(value);
const stripNames=value=>Array.isArray(value)?value.map(stripNames):value&&typeof value==='object'?Object.fromEntries(Object.entries(value).filter(([key])=>key!=='name').map(([key,child])=>[key,stripNames(child)])):value;
const result=spawnSync(executable,[shaderRoot],{encoding:'utf8',maxBuffer:256*1024*1024});
writeFileSync(join(base,'reflection-stderr.log'),result.stderr);
if(result.status!==0)throw Error(result.stderr);
const files=JSON.parse(result.stdout);
const grammar=readJson('skeleton/_tools/spirv.core.grammar.json');
const enums=kind=>Object.fromEntries(grammar.operand_kinds.find(item=>item.kind===kind).enumerants.map(item=>[Number(item.value),item.enumerant]));
const decorationNames=enums('Decoration'),storageNames=enums('StorageClass'),builtinNames=enums('BuiltIn');
const decode=words=>Buffer.from(new Uint32Array(words).buffer).toString('utf8').split('\0')[0];
const rawDeclarations=(bytes,offset,count)=>{
 const words=Array.from({length:count},(_,index)=>bytes.readUInt32LE(offset+4*index));
 const names=new Map(),variables=new Map(),decorations=new Map(),memberDecorations=[];
 const capabilities=[],extensions=[],rawEntries=[];
 for(let cursor=5;cursor<count;){
  const length=words[cursor]>>>16,opcode=words[cursor]&65535;
  if(!length||cursor+length>count)throw Error('invalid SPIR-V instruction span');
  const operands=words.slice(cursor+1,cursor+length);
  if(opcode===5)names.set(operands[0],decode(operands.slice(1)));
  if(opcode===59)variables.set(operands[1],{type_id:operands[0],storage:storageNames[operands[2]]??operands[2]});
  if(opcode===71){const list=decorations.get(operands[0])??[];list.push({decoration:decorationNames[operands[1]]??operands[1],args:operands.slice(2)});decorations.set(operands[0],list);}
  if(opcode===72)memberDecorations.push({struct_id:operands[0],member:operands[1],decoration:decorationNames[operands[2]]??operands[2],args:operands.slice(3)});
  if(opcode===17)capabilities.push(operands[0]);
  if(opcode===10)extensions.push(decode(operands));
  if(opcode===15)rawEntries.push({execution_model:operands[0],function_id:operands[1],name:decode(operands.slice(2))});
  cursor+=length;
 }
 const ioDecorations=[...variables].filter(([,variable])=>['Input','Output'].includes(variable.storage)).map(([id,variable])=>({id,name:names.get(id)??null,...variable,decorations:decorations.get(id)??[]}));
 return {declared_stage_io:ioDecorations,member_builtin_decorations:memberDecorations.filter(item=>item.decoration==='BuiltIn').map(item=>({...item,builtin:builtinNames[item.args[0]]??item.args[0]})),capabilities,extensions,raw_entries:rawEntries};
};
const interfaceFamilies=new Map(),layoutFamilies=new Map(),stages={},descriptorKinds={};
const unknowns=[];
let moduleCount=0,reflectedCount=0,entryCount=0;
for(const file of files){
 const bytes=readFileSync(join(shaderRoot,file.file));
 file.sha256=hash(bytes);
 file.kind=file.file.includes('.raster.')?'raster':file.file.includes('.cs.')?'compute':'unclassified';
 if(!file.modules.length)unknowns.push({file:file.file,question:'no recognized SPIR-V payload',status:'unresolved'});
 for(const [index,module] of file.modules.entries()){
  moduleCount++;
  const payload=bytes.subarray(module.offset,module.offset+4*module.word_count);
  module.sha256=hash(payload);
  module.id=file.file+'#'+index;
  module.container_end=module.offset+payload.length;
  try{module.raw_declarations=rawDeclarations(bytes,module.offset,module.word_count);}catch(error){module.raw_declaration_error=String(error);}
  if(module.error){unknowns.push({file:file.file,module:index,question:module.error,status:'reflection_failed'});continue;}
  reflectedCount++;
  for(const entry of module.entries){
   entryCount++;stages[entry.stage]=(stages[entry.stage]??0)+1;
   for(const descriptor of entry.descriptors){const kind=descriptor.descriptor_type.split('(')[0];descriptorKinds[kind]=(descriptorKinds[kind]??0)+1;}
   const member={file:file.file,module:index,module_id:module.id,stage:entry.stage,entry_point:entry.entry_point};
   const layout={descriptors:entry.descriptors.map(descriptor=>({set:descriptor.set,binding:descriptor.binding,descriptor_type:descriptor.descriptor_type.split('(')[0],count:descriptor.count})),push_constants:entry.push_constants.map(push=>stripNames(push.type))};
   const iface={stage:entry.stage,descriptors:entry.descriptors.map(stripNames),inputs:entry.inputs.map(stripNames),outputs:entry.outputs.map(stripNames),push_constants:entry.push_constants.map(stripNames)};
   for(const [map,signature,prefix,field] of [[layoutFamilies,layout,'layout','layout_family'],[interfaceFamilies,iface,'interface','interface_family']]){
    const id=prefix+'-'+hash(stable(signature));
    if(!map.has(id))map.set(id,{id,signature,members:[]});
    map.get(id).members.push(member);entry[field]=id;
   }
  }
 }
}
const functions=readJson('evidence/render-recovery/workgraph/functions.json');
const calls=readJson('evidence/render-recovery/workgraph/calls.json');
const native=readJson('evidence/render-skeleton/native-procedures.json');
const seeds=readJson('evidence/render-architecture/symbol-seeds.json');
const pdbRhapsody=functions.filter(item=>item.names.some(name=>/rhapsody::(raw::(shader|binding|descriptor_set)|rg::binding)/.test(name)));
const genericConsumerLinks=pdbRhapsody.flatMap(item=>{
 const owners=[...new Set(item.names.flatMap(name=>[...name.matchAll(/country_core::(?:render|asset_libraries)::[A-Za-z0-9_:$]+/g)].map(match=>match[0])))].sort();
 return owners.map(owner=>({rhapsody_va:item.va,country_core_owner:owner,evidence:'PDB monomorphized generic argument names',confidence:'observed type/closure association, not runtime call trace'}));
});
const gameConsumers=seeds.filter(item=>item.va&&/^country_core::/.test(item.demangled)&&/render::(pass_builder|shader)|systems::render_loop/.test(item.demangled)).map(item=>({va:item.va,name:item.demangled,source:'PDB public symbol'}));
const ownedNative=native.filter(item=>/^country_core::/.test(item.name)&&/render::(pass_builder|shader)|systems::render_loop/.test(item.name)).map(item=>({va:item.va,name:item.name,code_size:item.code_size,pdb_type:item.pdb_type,raw_evidence:item.raw_evidence,source:'PDB native procedure'}));
const shaderStem=file=>file.replace(/^_/,'').replace(/\.(cs|raster)\.hlsl.*$/,'');
const semanticCandidates=files.map(file=>{
 const tokens=shaderStem(file.file).toLowerCase().split('_').filter(token=>token.length>=4&&!['main','draw','blit','nani'].includes(token));
 const candidates=gameConsumers.filter(consumer=>tokens.length&&tokens.every(token=>consumer.name.toLowerCase().includes(token))).map(consumer=>consumer.va);
 return {file:file.file,stem:shaderStem(file.file),consumer_candidates:[...new Set(candidates)].sort(),confidence:'inferred name-token match only; not a shader binding xref'};
});
const directEdges=pdbRhapsody.flatMap(source=>(calls.functions[source.va]?.direct??[]).filter(call=>pdbRhapsody.some(target=>target.va===call.target)).map(call=>({source:source.va,site:call.site,target:call.target,evidence:'existing native direct-call workgraph'})));
const declaredDescriptorCount=files.flatMap(file=>file.modules).flatMap(module=>module.entries??[]).reduce((sum,entry)=>sum+entry.descriptors.length,0);
const summary={schema_version:1,source:shaderRoot,source_execution:false,files:files.length,file_kinds:Object.fromEntries(['compute','raster','unclassified'].map(kind=>[kind,files.filter(file=>file.kind===kind).length])),modules:moduleCount,reflected_modules:reflectedCount,reflection_failed_modules:moduleCount-reflectedCount,entries:entryCount,stages,descriptor_variables:declaredDescriptorCount,descriptor_kinds:descriptorKinds,layout_families:layoutFamilies.size,interface_families:interfaceFamilies.size,opaque_or_failed:unknowns.length,rhapsody_pdb_functions:pdbRhapsody.length,country_core_public_consumers:gameConsumers.length,country_core_native_consumers:ownedNative.length,shader_name_candidate_matches:semanticCandidates.filter(item=>item.consumer_candidates.length).length};
write('catalog.json',{schema_version:1,reflection:{library:'spirq0.6.2',source:'https://github.com/h3r2tic/spirq-rs',commit:'4f22e6f149750401705d47ae15a60e77cd5a6905',reference_all_resources:true,combine_image_samplers:false},files});
write('interface-families.json',{schema_version:1,layout_families:[...layoutFamilies.values()].sort((left,right)=>left.id.localeCompare(right.id)),interface_families:[...interfaceFamilies.values()].sort((left,right)=>left.id.localeCompare(right.id))});
write('cross-index.json',{schema_version:1,rhapsody_pipeline_binding_descriptor_functions:pdbRhapsody,country_core_public_consumers:gameConsumers,country_core_native_consumers:ownedNative,observed_direct_edges:directEdges,observed_generic_consumer_links:genericConsumerLinks,shader_consumer_candidates:semanticCandidates,limits:['Name-token candidates are not observed native shader-use xrefs','PDB generic association does not prove runtime pass dispatch','PDB void types remain placeholders','No descriptor-to-game-field ABI attribution is inferred']});
write('summary.json',summary);
write('unknowns.json',{schema_version:1,reflection_failures:unknowns,remaining:['Runtime descriptor counts for nbind0','Actual native shader-library-to-pass selection and specialization','Built-in block member IO linkage requires Vulkan interface rules beyond location-only candidate','Descriptor range/image layout/device feature requirements','Full shader algorithms and frame/image equivalence']});
write('inventory.json',{schema_version:1,files:files.map(file=>({file:file.file,bytes:file.bytes,sha256:file.sha256})),digest:hash(stable(files.map(file=>({file:file.file,sha256:file.sha256}))))});
console.log(JSON.stringify(summary,null,2));
