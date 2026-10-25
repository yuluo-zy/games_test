import {Client} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import {StdioClientTransport} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import {readFileSync,writeFileSync} from 'node:fs';
const base='D:/game/reverse-engineering/tiny-glade/evidence/render-recovery/parallel-batch-03/sequence-domain';
const config=JSON.parse(readFileSync('D:/tools/reverse-engineering/verification/server-config.json','utf8')).re_mcp_ghidra;
const client=new Client({name:'tiny-glade-sequence-domain-r5',version:'1'});
const transport=new StdioClientTransport({...config,env:{...process.env,...config.env},stderr:'pipe'});
transport.stderr?.on('data',data=>writeFileSync(base+'/mcp-stderr.log',data,{flag:'a'}));
const database='tg_render_arch';
const report={target:'D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe',target_execution:false,database_write:false,results:[]};
const call=async(name,args)=>{const result=await client.callTool({name,arguments:args},{timeout:180000});report.results.push({name,args,result});writeFileSync(base+'/mcp-report.json',JSON.stringify(report,null,2));console.log(JSON.stringify({tool:name,address:args.address,isError:result.isError??false}));return result;};
try {
 await client.connect(transport,{timeout:60000});
 await call('open_database',{file_path:report.target,run_auto_analysis:false,database_id:database});
 await call('wait_for_analysis',{database});
 for(const address of ['0x140bef4a0','0x140bdb940','0x140bed4c0','0x1418c3b80']) {
  const result=await call('decompile_function',{database,address});
  const structured=result.structuredContent??JSON.parse(result.content.find(item=>item.type==='text')?.text??'{}');
  if(structured.decompiled_code)writeFileSync(base+'/'+address+'.c',structured.decompiled_code);
  await call('get_xrefs_to',{database,address,limit:40});
 }
} catch(error){report.error=String(error);console.error(String(error));process.exitCode=1;}
finally{try{await call('close_database',{database,save:false});}catch(error){report.cleanup_error=String(error);}await client.close();writeFileSync(base+'/mcp-report.json',JSON.stringify(report,null,2));}
