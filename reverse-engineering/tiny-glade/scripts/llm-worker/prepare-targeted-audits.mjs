import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {fileURLToPath} from 'node:url';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'../..');
const out=path.join(root,'evidence/llm-worker-2026-10-19');
const packet=JSON.parse(fs.readFileSync(path.join(out,'stairs-packet.json'),'utf8'));
const lines=packet.sources[0].text.split('\n').map(s=>s.replace(/^D1:L\d+ /,''));
const sha=t=>crypto.createHash('sha256').update(t).digest('hex');
function source(id,a,b){const text=lines.slice(a-1,b).map((s,i)=>`${id}:L${i+1} ${s}`).join('\n');return{id,kind:'exact_decompilation_slice',original_range:`D1:L${a}-L${b}`,line_count:b-a+1,sha256:sha(text),text};}
const event={schema_version:1,task_id:'B-stairs-event-scope-audit',target:packet.target,task:'Audit only helper-dispatch mapping and output discriminant formulas across scheduling flag at record+8, nested payload flag at *(record+0x10)+0x10, and direction at record+0x1c. Previous unverified worker hypothesis: branch code_r0x000140af782f yields (0x12-direction)+0x11 because D1:L497 appears before the shared epilogue. Confirm or reject using exact braces/gotos; list all truth-table outcomes for the two flags and direction. Do not conflate scheduling-path selection with the later helper-dispatch target.',limits:['Static source slices only; not a full-function recovery task.','Hypothesis is unverified and may be wrong.','Source identifiers are local to this packet; original compact D1 ranges are supplied for owner mapping.'],sources:[source('E1',175,219),source('E2',312,380),source('E3',474,519)]};
const entry={schema_version:1,task_id:'B-stairs-entry-change-markers',target:packet.target,task:'Extract only entry early-write/guard order; the exact front-bucket index expression including wrapped-index case; and all param_3/param_4 mutation-marker writes preceding state-changing calls or output allocation. Track reassignment of lVar37: distinguish bucket byte offset from the later loaded command count; do not treat variable reuse as an unknown. Keep exact offsets and units, no guessed parameter types. State what occurs even when the empty guard returns.',limits:event.limits,sources:[source('E1',112,141),source('E2',183,219),source('E3',313,334),source('E4',355,380),source('E5',478,516)]};
for(const p of [event,entry])fs.writeFileSync(path.join(out,p.task_id+'-packet.json'),JSON.stringify(p,null,2)+'\n');
fs.writeFileSync(path.join(out,'jobs-targeted.json'),JSON.stringify([event,entry].map(p=>({id:p.task_id,packet:p.task_id+'-packet.json',prompt:'../../scripts/llm-worker/targeted-audit.txt',output:p.task_id+'-result.json',depends_on:[],thinking:'template-disabled',max_tokens:4000})),null,2)+'\n');
console.log(JSON.stringify({prepared:[event.task_id,entry.task_id],sources:'exact immutable original decompilation slices; no candidate Rust supplied'}));
