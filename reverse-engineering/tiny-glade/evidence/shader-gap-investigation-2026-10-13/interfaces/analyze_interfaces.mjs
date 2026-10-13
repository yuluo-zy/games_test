// 原发行与新编译SPIR-V接口调查；不向生产metadata注入原反射。
import fs from 'node:fs/promises';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import crypto from 'node:crypto';
const output=path.dirname(fileURLToPath(import.meta.url));
const project=path.resolve(output,'../../..');
const root=path.resolve(project,'../..');
const freshReport=process.argv[2]||path.join(project,'evidence/shader-build-2026-10-13/dxc/merged-audit.json');
const outputName=process.argv[3]||'raw-interfaces.json';
const merged=JSON.parse(await fs.readFile(freshReport,'utf8'));
const rowsToAudit=merged.rows.filter(x=>x.status==='fresh_package_generated');
const catalog=JSON.parse(await fs.readFile(path.join(project,'evidence/render-passes/validation/compile-commands.json'),'utf8')).compile_commands;
const catalogMap=new Map(catalog.map(x=>[x.file,x]));
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
function text(words){const b=Buffer.alloc(words.length*4);words.forEach((w,i)=>b.writeUInt32LE(w,i*4));return b.subarray(0,b.indexOf(0)<0?b.length:b.indexOf(0)).toString('utf8');}
export function unpack(bytes){let at=0;const u32=()=>{const n=bytes.readUInt32LE(at);at+=4;return n};const count=u32(),stages=[];for(let i=0;i<count;i++){const stage=u32(),n=u32();const entry=bytes.subarray(at,at+n).toString('utf8');at+=n;const size=u32()*4;const data=bytes.subarray(at,at+size);at+=size;if(data.readUInt32LE()!==0x07230203)throw Error('invalidprefix');stages.push({stage,entry,data});}return stages;}
export function inspect(bytes){const words=Array.from({length:bytes.length/4},(_,i)=>bytes.readUInt32LE(i*4));if(words[0]!==0x07230203)throw Error('badSPIRV');const names=new Map(),types=new Map(),decor=new Map(),members=new Map(),vars=[],ops={},records=[];for(let pos=5;pos<words.length;){const size=words[pos]>>>16,op=words[pos]&65535;if(size===0||pos+size>words.length)throw Error('badinstruction');const a=words.slice(pos+1,pos+size);ops[op]=(ops[op]||0)+1;records.push({word:pos,opcode:op,operands:a});if(op===5)names.set(a[0],text(a.slice(1)));if(op===6){const list=members.get(a[0])||[];list[a[1]]=text(a.slice(2));members.set(a[0],list)}if(op===71){const list=decor.get(a[0])||[];list.push({decoration:a[1],values:a.slice(2),word:pos});decor.set(a[0],list)}if(op===5632){const list=decor.get(a[0])||[];list.push({decoration:a[1],value:text(a.slice(2)),word:pos});decor.set(a[0],list)}if(op>=19&&op<=33)types.set(a[0],{opcode:op,args:a.slice(1)});if(op===59)vars.push({type:a[0],id:a[1],storage:a[2],word:pos});pos+=size;}
const describe=id=>{const t=types.get(id);if(!t)return {id};if(t.opcode===32)return describe(t.args[1]);if(t.opcode===22)return {float_bits:t.args[0]};if(t.opcode===21)return {integer_bits:t.args[0],signed:t.args[1]};if(t.opcode===23)return {vector:t.args[1],scalar:describe(t.args[0])};return t;};
const interfaces=vars.filter(v=>v.storage===1||v.storage===3).map(v=>({...v,name:names.get(v.id),type:describe(v.type),decorations:decor.get(v.id)||[]}));
const buffers=vars.filter(v=>v.storage===2).map(v=>{const ptr=types.get(v.type);const tid=ptr?.args[1];return {...v,name:names.get(v.id),decorations:decor.get(v.id)||[],members:(members.get(tid)||[]).map((name,index)=>({index,name,decorations:records.filter(i=>i.opcode===72&&i.operands[0]===tid&&i.operands[1]===index).map(i=>({word:i.word,decoration:i.operands[2],values:i.operands.slice(3)}))}))};});
return {sha256:sha(bytes),words:words.length,generator:words[2],inputs:interfaces.filter(v=>v.storage===1),outputs:interfaces.filter(v=>v.storage===3),uniform_buffers:buffers,metrics:{kill:ops[252]||0,demote:ops[5380]||0,selection_merge:ops[247]||0,loop_merge:ops[246]||0,branch_conditional:ops[250]||0,store:ops[62]||0},opcodes:ops};}
const rows=[];for(const row of rowsToAudit){const cat=catalogMap.get(row.file);const originalData=await fs.readFile(path.join(root,'ljxsj_92385/Tiny Glade/compiled-shaders',row.file));if(sha(originalData)!==cat.file_sha256)throw Error('originalidentity');const fresh=unpack(await fs.readFile(row.fresh_package));const stages=cat.modules.map((module,index)=>{const size=originalData.readUInt32LE(module.offset-4)*4;return {index,stage:module.stage,entry:module.entry,original:inspect(originalData.subarray(module.offset,module.offset+size)),fresh:inspect(fresh[index].data)};});rows.push({file:row.file,source:row.source,defines:row.defines,semantic_schema_equal:row.semantic_schema_equal,fields:Object.keys(row.original_schema).filter(k=>JSON.stringify(row.original_schema[k])!==JSON.stringify(row.fresh_schema[k])),stages});}
await fs.writeFile(path.join(output,outputName),JSON.stringify({original_asset_identity_checked:true,files:rows.length,rows},null,2)+'\n');console.log(JSON.stringify({files:rows.length,output:path.join(output,'raw-interfaces.json')}));
