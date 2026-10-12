// Parallel lane runner for evidence jobs.
//
// One job body is fanned out over N lanes. Each lane carries its own
// database_id, which RE-MCP turns into its own worker process and therefore its
// own Ghidra project, so every lane analyzes the same binary concurrently
// instead of queueing behind one provider session. Within a lane the body steps
// are sent as one server-side `batch` per chunk of 50, which cuts round-trips
// while keeping one report entry per step.
//
// Usage: node run-lanes.mjs JOB.json REPORT.json [--config CONFIG] [--lanes N]
import {readFileSync,writeFileSync,mkdirSync,renameSync} from 'node:fs';
import {parse} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/smol-toml/dist/index.js';
import {resolve,dirname,join} from 'node:path';
import {run} from './mcp-evidence-job.mjs';

const [jobFile,output,...flags]=process.argv.slice(2);
if(!jobFile||!output)throw new Error('Usage: JOB.json REPORT.json [--config CONFIG] [--lanes N]');
let configPath='C:/Users/liyu/.codex/config.toml',lanes=0;
for(let i=0;i<flags.length;i+=2){
 if(flags[i]==='--config')configPath=flags[i+1];
 if(flags[i]==='--lanes')lanes=Number(flags[i+1]);
}
const parsed=configPath.endsWith('.json')?JSON.parse(readFileSync(configPath,'utf8')):parse(readFileSync(configPath,'utf8'));
const server='re_mcp_ghidra';
const config=parsed.mcp_servers?.[server]??parsed[server];
if(!config)throw new Error('Configured MCP server not found: '+server);

const job=JSON.parse(readFileSync(jobFile,'utf8'));
const laneCount=lanes||job.lanes||8;
if(!Number.isInteger(laneCount)||laneCount<1)throw new Error('lane count must be a positive integer');
const batchLimit=50;
const lifecycleTools=new Set(['open_database','close_database','wait_for_analysis']);
const databaseId=/^[a-z][a-z0-9_]{0,31}$/;

function write(path,value){mkdirSync(dirname(path),{recursive:true});const temp=path+'.'+process.pid+'.tmp';writeFileSync(temp,JSON.stringify(value,null,2)+'\n');renameSync(temp,path);}

/** Split the job body into the steps that belong to a lane.
 *
 * Lifecycle steps are dropped: each lane emits its own open/wait/close around
 * the body, bound to that lane's database_id.
 */
function laneBody(steps){
 const body=(steps??[]).filter(step=>!lifecycleTools.has(step.tool??step.name));
 if(body.length===0)throw new Error('job has no steps left once lifecycle steps are removed');
 return body;
}

/** Group body steps into one `batch` step per chunk, declaring `expands` so the
 * runner can write one report entry per operation. The batch's own `database`
 * argument is injected server-side into each operation, so per-operation
 * `database` is stripped rather than repeated. */
function batchSteps(body,laneId){
 const steps=[];
 for(let at=0;at<body.length;at+=batchLimit){
  const chunk=body.slice(at,at+batchLimit);
  steps.push({
   tool:'batch',
   args:{database:laneId,stop_on_error:false,operations:chunk.map(step=>{
    const params={...(step.args??{})};
    delete params.database;
    return {tool:step.tool??step.name,params};
   })},
   expands:chunk.map(step=>({tool:step.tool??step.name,args:{...(step.args??{})}})),
   timeout_ms:Math.max(...chunk.map(step=>step.timeout_ms??600000)),
  });
 }
 return steps;
}

function laneJob(laneId){
 const {session_key,lanes,...base}=job;
 return {...base,
  database:laneId,
  steps:[
   {tool:'open_database',args:{file_path:job.target.path,run_auto_analysis:false,keep_open:true,database_id:laneId}},
   {tool:'wait_for_analysis',args:{database:laneId}},
   ...batchSteps(laneBody(job.steps),laneId),
   {tool:'close_database',args:{database:laneId,save:false}},
  ]};
}

const stem=String(job.database??resolve(job.target.path).replace(/\.[^.]*$/,'').split(/[\\/]/).pop()).toLowerCase().replace(/[^a-z0-9_]/g,'_').replace(/^[^a-z]+/,'d');
const laneIds=[];
for(let i=0;i<laneCount;i++){
 const laneId=`${stem}_l${i+1}`.slice(0,32);
 if(!databaseId.test(laneId))throw new Error('derived lane database_id is invalid: '+laneId);
 laneIds.push(laneId);
}

const reportDir=dirname(resolve(output));
const laneOutputs=laneIds.map(laneId=>join(reportDir,`${laneId}.report.json`));
const startedAt=new Date().toISOString();
const results=await Promise.all(laneIds.map((laneId,index)=>{
 const laneOutput=laneOutputs[index];
 write(laneOutput.replace(/\.report\.json$/,'.job.json'),laneJob(laneId));
 return run(server,laneJob(laneId),laneOutput,config).then(report=>({laneId,output:laneOutput,report}))
  .catch(error=>({laneId,output:laneOutput,error:String(error)}));
}));

const laneSummaries=results.map(entry=>({
 database:entry.laneId,
 report:entry.output,
 ok:entry.report?.ok??false,
 cleanup_ok:entry.report?.cleanup_ok??false,
 error:entry.error??entry.report?.error??null,
 steps:entry.report?.results?.length??0,
}));
const merged={
 schema_version:1,
 started_at:startedAt,
 completed_at:new Date().toISOString(),
 server,
 lane_count:laneCount,
 lanes:laneSummaries,
 actual_mcp_connection:results.every(entry=>entry.report?.actual_mcp_connection===true),
 target_execution:false,
 identity:results.find(entry=>entry.report?.identity)?.report.identity??null,
 cache_policy:results.find(entry=>entry.report?.cache_policy)?.report.cache_policy??null,
 evidence_limits:results.find(entry=>entry.report?.evidence_limits)?.report.evidence_limits??[],
 results:results.flatMap(entry=>(entry.report?.results??[]).map(result=>({...result,lane:entry.laneId,database:entry.laneId}))),
 cleanup:results.flatMap(entry=>(entry.report?.cleanup??[]).map(item=>({...item,lane:entry.laneId}))),
 lock_events:results.flatMap(entry=>(entry.report?.lock_events??[]).map(event=>({...event,lane:entry.laneId}))),
 ok:results.every(entry=>entry.report?.ok===true),
 cleanup_ok:results.every(entry=>entry.report?.cleanup_ok===true),
};
write(output,merged);
console.log(JSON.stringify({ok:merged.ok,lanes:laneSummaries.map(lane=>({database:lane.database,ok:lane.ok,steps:lane.steps,error:lane.error})),results:merged.results.length,cleanup_ok:merged.cleanup_ok}));
if(!merged.ok)process.exitCode=1;
