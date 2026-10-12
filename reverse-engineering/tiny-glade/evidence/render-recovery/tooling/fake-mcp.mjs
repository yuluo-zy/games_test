// Local deterministic protocol fixture. Does not open Ghidra or the original.
import {createInterface} from 'node:readline';
import {appendFileSync} from 'node:fs';
const tools=['open_database','close_database','decompile_function','get_xrefs_to'].map(name=>({name,inputSchema:{type:'object'}}));
const lines=createInterface({input:process.stdin});
const reply=(id,result)=>process.stdout.write(JSON.stringify({jsonrpc:'2.0',id,result})+'\n');
lines.on('line',line=>{
 const request=JSON.parse(line);if(request.id===undefined)return;
 if(request.method==='initialize')return reply(request.id,{protocolVersion:request.params.protocolVersion,capabilities:{tools:{}},serverInfo:{name:'local-fake-mcp',version:'1'}});
 if(request.method==='tools/list')return reply(request.id,{tools});
 if(request.method==='tools/call'){
  const {name,arguments:args}=request.params;
  if(process.env.FAKE_LOG)appendFileSync(process.env.FAKE_LOG,JSON.stringify({name,args})+'\n');
  if(args.throw)return process.stdout.write(JSON.stringify({jsonrpc:'2.0',id:request.id,error:{code:-32000,message:'fixture protocol error'}})+'\n');
  const failed=args.fail||(name==='close_database'&&args.database==='fail-close');
  const result={isError:!!failed,content:[{type:'text',text:JSON.stringify({ok:!failed,tool:name,args,fixture:true})}]};
  if(args.delay_ms)return setTimeout(()=>reply(request.id,result),args.delay_ms);
  return reply(request.id,result);
 }
 reply(request.id,{});
});
