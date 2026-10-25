import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';

const args = Object.fromEntries(process.argv.slice(2).reduce((a,v,i,all)=>i%2===0?[...a,[v,all[i+1]]]:a,[]));
const jobsPath = path.resolve(args['--jobs'] ?? '');
if(!args['--jobs']) throw Error('Usage: node run.mjs --jobs <manifest.json> [--concurrency 2] [--max-tokens 10000] [--timeout-ms 240000]');
const key = process.env.TG_LLM_API_KEY;
if(!key) throw Error('Set TG_LLM_API_KEY in the process environment; never put credentials in jobs or source');
const endpoint = new URL(process.env.TG_LLM_BASE_URL ?? 'https://it-litellm.espressif.cn/v1/');
if(endpoint.protocol !== 'https:') throw Error('HTTPS required');
const model = process.env.TG_LLM_MODEL ?? 'deepseek-ai/DeepSeek-V4.1-Flash';
const jobs = JSON.parse(fs.readFileSync(jobsPath,'utf8'));
const base = path.dirname(jobsPath), outputs=new Set(), ids=new Set();
for(const j of jobs) {
  if(!j.id || ids.has(j.id) || (j.depends_on?.length ?? 0)) throw Error('Only uniquely named independent jobs may run together');
  ids.add(j.id);
  const out=path.resolve(base,j.output); if(outputs.has(out)) throw Error('Distinct output paths required'); outputs.add(out);
}
const concurrency = Number(args['--concurrency'] ?? 2);
if(!Number.isInteger(concurrency)||concurrency<1||concurrency>4) throw Error('Concurrency must be 1..4');
const maxTokens = Number(args['--max-tokens'] ?? 18000);
const timeout = Number(args['--timeout-ms'] ?? 240000);
const defaultThinking = args['--thinking'] ?? 'template-low';
const modes=['default','disabled','enabled-low','template-disabled','template-low'];
if(!modes.includes(defaultThinking)) throw Error('Unknown thinking mode');
const hash = text=>crypto.createHash('sha256').update(text).digest('hex');
const required=['summary','observations','branch_matrix','side_effects','dependency_reuse','unknowns','integration_checks','independent_followups'];
let next=0, failures=0;
async function run(j) {
  const thinking=j.thinking??defaultThinking, jobMaxTokens=j.max_tokens??maxTokens;
  if(!modes.includes(thinking)||!Number.isInteger(jobMaxTokens)||jobMaxTokens<256||jobMaxTokens>32768)throw Error('Invalid job model options');
  const packet=JSON.parse(fs.readFileSync(path.resolve(base,j.packet),'utf8'));
  const prompt=fs.readFileSync(path.resolve(base,j.prompt),'utf8');
  const {sources,...metadata}=packet;
  const user=['PACKET METADATA',JSON.stringify(metadata,null,2),...sources.map(s=>`\nSOURCE ${s.id} (${s.kind}; ${s.line_count} numbered lines; sha256=${s.sha256??'see input manifest'})\n${s.text}\nEND SOURCE ${s.id}`)].join('\n');
  const out=path.resolve(base,j.output);
  const inputSha=hash(JSON.stringify({model,endpoint:endpoint.href,prompt,user,maxTokens:jobMaxTokens,thinking}));
  if(fs.existsSync(out)) {const cached=JSON.parse(fs.readFileSync(out,'utf8'));if(cached.input_sha256===inputSha&&cached.accepted_transport){console.log(JSON.stringify({id:j.id,cached:true,output:out}));return;}}
  const request={model,messages:[{role:'system',content:prompt},{role:'user',content:user}],max_tokens:jobMaxTokens,stream:false};
  if(['disabled','enabled-low'].includes(thinking))request.thinking={type:thinking==='disabled'?'disabled':'enabled'};
  if(thinking==='enabled-low')request.reasoning_effort='low';
  if(thinking==='template-disabled')request.chat_template_kwargs={thinking:false,enable_thinking:false};
  if(thinking==='template-low')request.chat_template_kwargs={thinking:true,reasoning_effort:25};
  const started=Date.now();
  const response=await fetch(new URL('chat/completions',endpoint.href.endsWith('/')?endpoint:new URL(endpoint.href+'/')),{method:'POST',redirect:'error',headers:{Authorization:`Bearer ${key}`,'Content-Type':'application/json'},body:JSON.stringify(request),signal:AbortSignal.timeout(timeout)});
  if(!response.ok) throw Error(`HTTP ${response.status}; response body suppressed to avoid credential leakage`);
  const raw=await response.json(), choice=raw.choices?.[0], content=choice?.message?.content??'';
  let analysis=null,validation=[];
  try {analysis=JSON.parse(content);for(const k of required) if(!(k in analysis)) validation.push(`Missing ${k}`);} catch {validation.push('Invalid JSON');}
  const sourceMap=new Map(packet.sources.map(s=>[s.id,s.line_count]));
  function check(v) {if(!v||typeof v!=='object')return;for(const [k,x] of Object.entries(v)){if(k==='refs'){if(!Array.isArray(x)||x.length===0)validation.push('Missing refs');else for(const ref of x){const m=/^(\w+):L(\d+)(?:-L?(\d+))?$/.exec(ref);if(!m||!sourceMap.has(m[1])||+m[2]<1||+(m[3]??m[2])<+m[2]||+(m[3]??m[2])>sourceMap.get(m[1]))validation.push(`Invalid ref ${ref}`);}}else check(x);}}
  check(analysis);
  for(const section of ['observations','branch_matrix','side_effects','dependency_reuse','unknowns']) {
    if(!Array.isArray(analysis?.[section])) validation.push(`Invalid array ${section}`);
    else for(const item of analysis[section]) if(!Array.isArray(item.refs)||item.refs.length===0) validation.push(`Missing evidence in ${section}`);
  }
  if(choice?.finish_reason!=='stop')validation.push(`Incomplete finish_reason=${choice?.finish_reason}`);
  const record={id:j.id,input_sha256:inputSha,prompt_sha256:hash(prompt),packet_sha256:hash(JSON.stringify(packet)),rendered_input_sha256:hash(user),render_format:'numbered-plain-text-v1',model:raw.model,endpoint_origin:endpoint.origin,request_options:{max_tokens:jobMaxTokens,thinking},elapsed_seconds:(Date.now()-started)/1000,finish_reason:choice?.finish_reason,usage:raw.usage,accepted_transport:validation.length===0,semantically_verified:false,validation_errors:validation,analysis,content};
  fs.mkdirSync(path.dirname(out),{recursive:true});fs.writeFileSync(out,JSON.stringify(record,null,2)+'\n');
  console.log(JSON.stringify({id:j.id,elapsed_seconds:record.elapsed_seconds,usage:raw.usage,accepted_transport:record.accepted_transport,validation_errors:validation,output:out}));
  if(validation.length) failures++;
}
await Promise.all(Array.from({length:Math.min(concurrency,jobs.length)},async()=>{while(next<jobs.length){const j=jobs[next++];try{await run(j);}catch(e){failures++;console.error(JSON.stringify({id:j.id,error:String(e.message).replaceAll(key,'[REDACTED]'),cause:e.cause?.code}));}}}));
if(failures)process.exitCode=1;
