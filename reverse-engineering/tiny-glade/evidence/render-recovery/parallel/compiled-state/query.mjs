import {Client} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import {StdioClientTransport} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const root='D:/game/reverse-engineering/tiny-glade';
const base=root+'/evidence/render-recovery/parallel/compiled-state';
const target=root+'/workspace/input/tiny-glade.exe';
const hash=createHash('sha256').update(readFileSync(target)).digest('hex');
if(hash!=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03')throw Error('Target identity mismatch');
const config=JSON.parse(readFileSync('D:/tools/reverse-engineering/verification/server-config.json','utf8')).re_mcp_ghidra;
const transport=new StdioClientTransport({...config,env:{...process.env,...config.env},stderr:'pipe'});
transport.stderr?.on('data',data=>writeFileSync(base+'/mcp-stderr.log',data,{flag:'a'}));
const client=new Client({name:'tiny-glade-compiled-state',version:'1'});
const report={target:'workspace/input/tiny-glade.exe',sha256:hash,target_execution:false,database_mutation_requested:false,actual_connection:false,results:[]};
const call=async(name,args)=>{const result=await client.callTool({name,arguments:args},{timeout:180000});report.results.push({name,args,result});writeFileSync(base+'/mcp-report.json',JSON.stringify(report,null,2));console.log(JSON.stringify({name,address:args.address,isError:result.isError??false}));return result;};
try{
 await client.connect(transport,{timeout:60000});report.actual_connection=true;
 const catalog=await client.listTools();writeFileSync(base+'/mcp-catalog.json',JSON.stringify(catalog,null,2));
 await call('list_databases',{});
 await call('open_database',{file_path:target,run_auto_analysis:false,database_id:'tg_render_arch'});
 await call('wait_for_analysis',{database:'tg_render_arch'});
 await call('get_database_info',{database:'tg_render_arch'});
 for(const address of process.argv.slice(2)){
  const result=await call('decompile_function',{database:'tg_render_arch',address});
  let data=result.structuredContent;
  if(!data)for(const item of result.content??[])if(item.type==='text'){try{data=JSON.parse(item.text);}catch{}}
  if(data?.decompiled_code)writeFileSync(base+'/'+address+'.c',data.decompiled_code);
 }
}catch(error){report.error=String(error);process.exitCode=1;console.error(String(error));}
finally{try{await call('close_database',{database:'tg_render_arch',save:false});}catch(error){report.cleanup_error=String(error);}await client.close();writeFileSync(base+'/mcp-report.json',JSON.stringify(report,null,2));}
