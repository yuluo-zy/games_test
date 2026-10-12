// Read-only evidence orchestration using the installed official MCP client SDK.
// Usage: node mcp-evidence-job.mjs SERVER JOB.json REPORT.json [--config CONFIG]
import {Client} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import {StdioClientTransport} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import {parse} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/smol-toml/dist/index.js';
import {readFileSync,writeFileSync,mkdirSync,renameSync,existsSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {resolve,dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';

const root=resolve(dirname(fileURLToPath(import.meta.url)),'../..');
const canonical=value=>Array.isArray(value)?value.map(canonical):value&&typeof value==='object'?Object.fromEntries(Object.keys(value).sort().map(key=>[key,canonical(value[key])])):value;
const hash=value=>createHash('sha256').update(value).digest('hex');
const digest=value=>hash(JSON.stringify(canonical(value)));
const json=path=>JSON.parse(readFileSync(path,'utf8'));
function write(path,value){mkdirSync(dirname(path),{recursive:true});const temp=path+'.'+process.pid+'.tmp';writeFileSync(temp,JSON.stringify(value,null,2)+'\n');renameSync(temp,path);}
function successful(result){
 if(!result||result.isError===true)return false;
 const envelopes=[result.structuredContent,...(result.content??[]).filter(x=>x.type==='text').map(x=>{try{return JSON.parse(x.text);}catch{return null;}})];
 return !envelopes.some(x=>x&&(x.success===false||x.ok===false||x.status==='error'||x.error||(typeof x.failed==='number'&&x.failed>0)));
}
const lifecycle=new Set(['open_database','close_database','open_binary','close_binary','wait_for_analysis']);
// Tools the server refuses to run nested inside a meta-tool (see re_mcp
// DEFAULT_BLOCKED_TOOLS); a batch may only carry tools outside this set.
const nonBatchableTools=new Set(['open_database','close_database','wait_for_analysis','list_targets','search_tools','get_schema','execute','batch','call']);
// No execution or generic execute/call wrappers, no saves, no type/comment mutation.
const readOnlyTools=new Set(['open_database','close_database','wait_for_analysis','list_databases','list_targets','get_database_info','list_functions','decompile_function','disassemble_function','list_names','get_strings','find_code_by_string','list_structures','get_structure','get_type_info','get_xrefs_to','get_xrefs_from','get_schema','batch']);
const batchableReadOnlyTools=new Set([...readOnlyTools].filter(tool=>!nonBatchableTools.has(tool)));
const maxBatchOperations=50;

/** Validate one batch step and return its nested operations.
 *
 * A batch step must declare `expands`, one entry per operation, so the batch's
 * per-item results can be written back as one report entry per operation —
 * keeping the report shape identical to a serial run.
 */
function batchOperations(step){
 const args=step.args??{};
 if(!Array.isArray(args.operations)||args.operations.length===0||args.operations.length>maxBatchOperations)throw Error('batch operations must be a non-empty list of at most '+maxBatchOperations);
 if(!Array.isArray(step.expands)||step.expands.length!==args.operations.length)throw Error('batch step must declare one expands entry per operation');
 for(const [position,operation] of args.operations.entries()){
  if(!batchableReadOnlyTools.has(operation.tool))throw Error('batch operation outside batchable read-only allowlist: '+operation.tool);
  const params=operation.params??{};
  if(params.database!==undefined&&params.database!==args.database)throw Error('batch operation database differs from its batch database');
  if(step.expands[position].tool!==operation.tool)throw Error('batch expands entry does not match operation '+position);
 }
 return args.operations;
}

/** Recover the BatchResult envelope from a batch call's result.
 *
 * On partial failure the server raises with `BatchResult.model_dump_json()` as
 * the message, so the per-item results survive as JSON text inside an isError
 * envelope rather than as structured content.
 */
function batchPayload(result){
 const envelopes=[result?.structuredContent,...(result?.content??[]).filter(item=>item.type==='text').map(item=>{try{return JSON.parse(item.text);}catch{return null;}}),result?.error];
 for(let envelope of envelopes){
  if(typeof envelope==='string'){try{envelope=JSON.parse(envelope);}catch{continue;}}
  if(envelope&&Array.isArray(envelope.results))return envelope;
  // The server may also return a normal (isError:false) envelope whose payload is
  // `{"error":"<BatchResult json>"}`, i.e. the BatchResult sits one level deeper
  // under a string `error` field. Descend once more before giving up.
  if(envelope&&typeof envelope.error==='string'){
   try{const inner=JSON.parse(envelope.error);if(inner&&Array.isArray(inner.results))return inner;}catch{}
  }
 }
 return null;
}

/** Replace a batch step's single entry with one entry per expanded operation.
 *
 * Keeps the report shape identical to a serial run: each operation gets its own
 * entry with the fields a directly-called step would have. Returns null when the
 * BatchResult envelope cannot be recovered, leaving the batch step's own entry in
 * place so the failure is still recorded.
 */
function expandBatchEntry(report,entry,step){
 const payload=batchPayload(entry.result);
 const at=report.results.indexOf(entry);
 if(payload===null||at<0)return null;
 const expanded=step.expands.map((item,position)=>{
  const inner=payload.results.find(result=>result.index===position);
  const record={index:entry.index,position,tool:item.tool,args:item.args??{},via_batch:true,started_at:entry.started_at,completed_at:new Date().toISOString(),source:entry.source};
  if(entry.cache_key!==undefined)record.cache_key=entry.cache_key;
  if(entry.cache_eligible!==undefined)record.cache_eligible=entry.cache_eligible;
  if(entry.reused_from!==undefined)record.reused_from=entry.reused_from;
  if(!inner)record.error='batch produced no result for operation '+position;
  else if(inner.error!==undefined&&inner.error!==null)record.error=String(inner.error);
  else{record.result=inner.result;record.result_sha256=digest(inner.result);}
  record.ok=record.error===undefined;
  if(/xref|reference/.test(item.tool))record.coverage={completeness:'unknown',negative_result_scope:'returned records only; not whole-program absence'};
  return record;
 });
 report.results.splice(at,1,...expanded);
 return expanded;
}

export async function run(server,job,output,config){
 const started=new Date().toISOString();
 const report={schema_version:1,started_at:started,server,actual_mcp_connection:false,target_execution:false,results:[],cleanup:[],lock_events:[],ok:false,
  evidence_limits:['Static tool outputs do not prove runtime execution.','Xref completeness is unknown with auto-analysis disabled. Empty or DATA-only references cannot establish whole-program absence.','Successful cache reuse preserves the original evidence coverage; it does not expand it.']};
 let client,transport,unsafe=false;
 const opened=new Map(),pendingCache=[];
 const flush=()=>write(output,report);
 try{
  if(!job.target?.path||!job.provider?.id)throw Error('job.target.path and job.provider.id are required');
  const targetPath=resolve(job.target.path),targetHash=hash(readFileSync(targetPath));
  if(job.target.sha256&&job.target.sha256!==targetHash)throw Error('target hash mismatch');
  const context=job.cache_context??{};
  const contextInputs=['provider_manifest','database_snapshot'].filter(role=>context[role]).map(role=>({path:context[role],role}));
  const inputs=[...(job.inputs??[]),...contextInputs].map(item=>{const path=resolve(typeof item==='string'?item:item.path),sha256=hash(readFileSync(path));if(item.sha256&&item.sha256!==sha256)throw Error('input hash mismatch: '+path);return {path,sha256,...(item.role?{role:item.role}:{})};});
  const cacheContextComplete=!!context.provider_manifest&&!!context.database_snapshot;
  report.cache_policy={enabled:cacheContextComplete,reason:cacheContextComplete?'Caller-supplied provider manifest and DB revision/snapshot hashes; caller must update snapshot after DB changes.':'Database analysis cache disabled: both provider_manifest and database_snapshot files are required.',provider_version_provenance:'job-declared; actual server self-report is recorded separately upon connection'};
  report.identity={target:{path:targetPath,sha256:targetHash},provider:job.provider,launch_sha256:digest(config),inputs};
  for(const step of job.steps??[]){
   const tool=step.tool??step.name,args=step.args??{};
   if(!readOnlyTools.has(tool))throw Error('operation outside static read-only allowlist: '+tool);
   if(tool==='close_database'&&args.save!==undefined&&args.save!==false)throw Error('close_database(save:true) is prohibited');
   if(tool==='open_database'&&(!job.database||args.run_auto_analysis!==false||resolve(args.file_path??'')!==targetPath||args.database_id!==job.database))throw Error('open_database must bind the job target/database with auto-analysis false');
   if(args.database!==undefined&&args.database!==job.database)throw Error('operation database differs from locked job database');
   if(tool==='batch')batchOperations(step);
  }
  report.job_sha256=digest(job);
  const cacheRoot=resolve(job.cache_dir??join(root,'evidence/render-recovery/tooling/cache'));
  // No cross-process session lock. A job is now expected to carry a database_id
  // unique to its lane, and each database_id gets its own worker process with
  // its own Ghidra project, so concurrent jobs cannot contend for one provider
  // session. The lock that used to serialise them only hid that contention.
  report.lock_events.push({event:'lock_removed',reason:'per-database workers own separate Ghidra projects; a shared session lock is no longer required'});flush();
  const ensureConnected=async()=>{
   if(client)return;
   transport=new StdioClientTransport({...config,cwd:config.cwd??root,env:{...process.env,...config.env},stderr:'pipe'});
   transport.stderr?.on('data',data=>writeFileSync(output+'.stderr.log',data,{flag:'a'}));
   client=new Client({name:'tiny-glade-evidence-job',version:'1.0'});
   await client.connect(transport,{timeout:job.connect_timeout_ms??120000});
   report.actual_mcp_connection=true;
   report.actual_server=client.getServerVersion();report.server_capabilities=client.getServerCapabilities();
   // Keep every catalog page and the complete schemas, rather than names only.
   report.catalog=[];let cursor;
   do{const page=await client.listTools(cursor?{cursor}:{});report.catalog.push(page);cursor=page.nextCursor;}while(cursor);
   flush();
  };
  const call=async(tool,args,timeout)=>{
   await ensureConnected();
   if(!report.catalog.some(page=>page.tools.some(item=>item.name===tool)))throw Error('tool not advertised: '+tool);
   return client.callTool({name:tool,arguments:args},{timeout:timeout??600000});
  };
  for(const [index,step] of (job.steps??[]).entries()){
   const tool=step.tool??step.name,args={...(step.args??{})};
   if(tool==='close_database'&&args.save===undefined)args.save=false;
   if(tool==='close_database'&&args.save!==false)throw Error('this evidence runner requires close_database(save:false)');
   if(tool==='open_database'&&args.run_auto_analysis!==false)throw Error('open_database requires run_auto_analysis:false');
   const keyData={identity:report.identity,operation:tool,args},key=digest(keyData),cacheFile=join(cacheRoot,key+'.json');
   const cacheable=cacheContextComplete&&step.cache!==false&&!lifecycle.has(tool),entry={index,tool,args,cache_key:key,cache_eligible:cacheable,started_at:new Date().toISOString()};
   report.results.push(entry);flush();
   let cached;
   if(cacheable&&existsSync(cacheFile)){
    try{const old=json(cacheFile);if(old.key===key&&digest(old.key_data)===key&&old.result_sha256===digest(old.result)&&successful(old.result)&&old.session_cleanup_ok===true)cached=old;}catch(error){entry.cache_rejection=String(error);}
   }
   // Reuse a successful result from an existing fully identified runner report.
   // Legacy reports without past target/input identity are deliberately rejected.
   if(!cached&&cacheable&&step.reuse){
    const prior=json(resolve(step.reuse.report)),source=prior.results[step.reuse.index];
    if(prior.ok===true&&prior.cleanup_ok===true&&digest(prior.identity)===digest(report.identity)&&source?.cache_key===key&&successful(source.result))cached={result:source.result,source_report:resolve(step.reuse.report),source_report_sha256:hash(readFileSync(resolve(step.reuse.report)))};
    else throw Error('reuse identity/success/cleanup mismatch');
   }
   try{
    // A timed-out/failed open can still have changed provider state. Attempt
    // cleanup for every open sent, and quarantine if closure cannot be proved.
    if(!cached&&tool==='open_database')opened.set(args.database_id,{tool:'close_database',args:{database:args.database_id,save:false}});
    if(!cached&&tool==='open_binary')opened.set('binary',{tool:'close_binary',args:{}});
    entry.result=cached?cached.result:await call(tool,args,step.timeout_ms);
    entry.source=cached?'retained_success':'actual_mcp_call';if(cached)entry.reused_from=cached.source_report??cacheFile;
    entry.completed_at=new Date().toISOString();
    const resultForCache=entry.result;
    let ok;
    if(step.expands){
     const expanded=expandBatchEntry(report,entry,step);
     entry.result_sha256=digest(entry.result);
     if(expanded===null){
      entry.ok=successful(entry.result);
      if(!entry.ok)entry.error='batch result envelope unavailable';
      ok=entry.ok;
     }else{
      const failed=expanded.filter(item=>!item.ok);
      ok=failed.length===0;
      if(!ok)entry.error='batch operations failed: '+failed.map(item=>item.tool).join(', ');
     }
    }else{
     entry.ok=successful(entry.result);entry.result_sha256=digest(entry.result);
     if(/xref|reference/.test(tool))entry.coverage={completeness:'unknown',negative_result_scope:'returned records only; not whole-program absence'};
     ok=entry.ok;
    }
    if(tool==='open_database'&&ok)opened.set(args.database_id,{tool:'close_database',args:{database:args.database_id,save:false}});
    if(tool==='open_binary'&&ok)opened.set('binary',{tool:'close_binary',args:{}});
    if(tool==='close_database'&&ok)opened.delete(args.database);
    if(tool==='close_binary'&&ok)opened.delete('binary');
    if(cacheable&&ok&&!cached)pendingCache.push({path:cacheFile,value:{schema_version:1,key,key_data:keyData,result:resultForCache,result_sha256:entry.result_sha256,source_report:output,session_cleanup_ok:true}});
    flush();
    if(!ok&&!step.allow_error)throw Error(entry.error??(tool+' returned an error envelope'));
   }catch(error){entry.error=String(error);entry.ok=false;flush();if(!step.allow_error)throw error;}
  }
  report.ok=true;
 }catch(error){report.error=String(error);report.ok=false;}
 finally{
  for(const close of opened.values()){
   const entry={...close,cleanup:true};report.cleanup.push(entry);
   try{entry.result=await client.callTool({name:close.tool,arguments:close.args},{timeout:120000});entry.ok=successful(entry.result);if(!entry.ok)unsafe=true;}catch(error){entry.error=String(error);entry.ok=false;unsafe=true;}
  }
  try{if(client)await client.close();else if(transport)await transport.close();}catch(error){report.transport_close_error=String(error);unsafe=true;}
  report.cleanup_ok=!unsafe;report.ok=report.ok&&!unsafe;
  // Commit only successful sessions after all cleanup completes. No error entries.
  if(report.ok&&report.cleanup_ok)for(const item of pendingCache)write(item.path,item.value);
  report.completed_at=new Date().toISOString();flush();
 }
 return report;
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
 const [server,jobFile,output,...flags]=process.argv.slice(2);
 if(!server||!jobFile||!output)throw Error('Usage: SERVER JOB.json REPORT.json [--config CONFIG]');
 const configPath=flags[0]==='--config'?flags[1]:'C:/Users/liyu/.codex/config.toml';
 const parsed=configPath.endsWith('.json')?json(configPath):parse(readFileSync(configPath,'utf8'));
 const config=parsed.mcp_servers?.[server]??parsed[server];
 if(!config)throw Error('Configured MCP server not found: '+server);
 const report=await run(server,json(resolve(jobFile)),resolve(output),config);
 console.log(JSON.stringify({ok:report.ok,actual_mcp_connection:report.actual_mcp_connection,results:report.results.length,cleanup_ok:report.cleanup_ok,error:report.error}));
 if(!report.ok)process.exitCode=1;
}
