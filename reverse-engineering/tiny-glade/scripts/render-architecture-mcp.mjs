import {Client} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import {StdioClientTransport} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import {readFileSync,writeFileSync} from 'node:fs';
const base='D:/game/reverse-engineering/tiny-glade/evidence/render-architecture';
const config=JSON.parse(readFileSync('D:/tools/reverse-engineering/verification/server-config.json','utf8')).re_mcp_ghidra;
const client=new Client({name:'tiny-glade-render-architecture',version:'1'});
const transport=new StdioClientTransport({...config,env:{...process.env,...config.env},stderr:'pipe'});
transport.stderr?.on('data',d=>writeFileSync(base+'/mcp-stderr.log',d,{flag:'a'}));
const report={target:'D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe',target_execution:false,actual_connection:false,results:[]};
const database='tg_render_arch';
const reportFile=base+'/'+(process.env.TG_RENDER_REPORT??'mcp-report.json');
const call=async(name,args)=>{let r=await client.callTool({name,arguments:args},{timeout:180000});report.results.push({name,args,result:r});writeFileSync(reportFile,JSON.stringify(report,null,2));console.log(JSON.stringify({tool:name,address:args.address,isError:r.isError??false}));return r;};
try{
 await client.connect(transport,{timeout:60000});report.actual_connection=true;
 const catalog=await client.listTools();writeFileSync(base+'/mcp-catalog.json',JSON.stringify(catalog,null,2));
 await call('open_database',{file_path:report.target,run_auto_analysis:false,database_id:database});
 await call('wait_for_analysis',{database});
 await call('get_database_info',{database});
 for(const address of process.argv.slice(2)){
  let result=await call('decompile_function',{database,address});
  if(result.isError){await call('create_function',{database,address});await call('decompile_function',{database,address});}
  await call('get_xrefs_to',{database,address,limit:20});
 }
}catch(e){report.error=String(e);console.error(String(e));process.exitCode=1;}
finally{try{await call('close_database',{database,save:true});}catch(e){report.cleanup_error=String(e);}await client.close();writeFileSync(reportFile,JSON.stringify(report,null,2));}
