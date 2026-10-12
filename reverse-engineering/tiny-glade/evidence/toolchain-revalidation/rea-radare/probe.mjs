import { Client } from '/Users/yuluo/Desktop/game/tools/node/lib/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import { StdioClientTransport } from '/Users/yuluo/Desktop/game/tools/node/lib/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import {writeFileSync} from 'node:fs';
const root='/Users/yuluo/Desktop/game';const dir=root+'/games_test/reverse-engineering/tiny-glade/evidence/toolchain-revalidation/rea-radare';
const kind=process.argv[2];const label=process.argv[3]??'inventory';
const report={kind,label,started:new Date().toISOString(),calls:[],ok:false};
const env={...process.env,PATH:root+'/tools/node/bin:/opt/homebrew/bin:'+process.env.PATH,GHIDRA_INSTALL_DIR:root+'/tools/ghidra_12.1.4_PUBLIC',JAVA_HOME:'/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home',REA_ANALYSIS_PROVIDER:'ghidra',REA_GHIDRA_STARTUP_TIMEOUT_MS:'120000'};
const transport = new StdioClientTransport({command:kind==='rea'?root+'/tools/node/bin/node':'/opt/homebrew/bin/r2pm',args:kind==='rea'?[root+'/tools/node/bin/rea','--mcp']:['-r','r2mcp','-r','-C','both'],env,stderr:'pipe'});
const client=new Client({name:'toolchain-revalidate-'+kind+'-'+label,version:'1'});transport.stderr?.on('data',data=>{report.stderr=(report.stderr??'')+String(data);});
function save(){writeFileSync(dir+'/'+kind+'-'+label+'.json',JSON.stringify(report,null,2));}
async function call(name,args,timeout=120000){let t=Date.now();let r=await client.callTool({name,arguments:args},{timeout});report.calls.push({name,args,result:r,elapsed_ms:Date.now()-t});save();console.log(JSON.stringify({kind,label,name,ms:Date.now()-t,isError:r.isError??false}));return r;}
try{await client.connect(transport,{timeout:30000});const inventory=await client.listTools({},{timeout:30000});report.server=client.getServerVersion();report.tools=inventory.tools;save();
 if(label==='inventory'){report.ok=true;}
 else if(kind==='rea'){
  await call('open_binary',{path:root+'/ljxsj_92385/Tiny Glade/tiny-glade.rne',provider_id:'ghidra'});await call('binary_session',{});await call('close_binary',{});
  await call('open_binary',{path:dir+'/'+(label==='one'?'B-aspect-fixture.exe':'isolation-fixture.exe'),provider_id:'ghidra'});await call('analyze_function',{procedure:'0x140001000'},180000);await call('read_bytes',{address:'0x140001000',length:20});await call('procedure_references',{procedure:'0x140001000'});await call('binary_session',{});await call('close_binary',{});report.ok=!report.calls.some(c=>c.result.isError);
 }else{
  await call('open_file',{file_path:root+'/ljxsj_92385/Tiny Glade/tiny-glade.rne'});await call('show_info',{});await call('disassemble',{address:'0x140949c20',num_instructions:7});await call('run_command',{command:'af @ 0x140949c20'});await call('list_functions',{filter:'140949c20',max_length:5});await call('xrefs_to',{address:'0x140949c20'});await call('list_decompilers',{});await call('run_command',{command:'idp?'});await call('close_file',{});
  await call('open_file',{file_path:dir+'/'+(label==='one'?'B-aspect-fixture.exe':'isolation-fixture.exe')});await call('analyze',{level:0});await call('show_info',{});await call('disassemble',{address:'0x140001000',num_instructions:7});await call('list_functions',{max_length:5});await call('decompile_function',{address:'0x140001000'});await call('close_file',{});report.ok=!report.calls.some(c=>c.result.isError);
 }
}catch(e){report.error=String(e);console.error(report.error);process.exitCode=1;}finally{await client.close();report.finished=new Date().toISOString();save();}
