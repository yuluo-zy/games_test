import { Client } from '/Users/yuluo/Desktop/game/tools/node/lib/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import { StdioClientTransport } from '/Users/yuluo/Desktop/game/tools/node/lib/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import {writeFileSync} from 'node:fs';
const root='/Users/yuluo/Desktop/game',dir=root+'/games_test/reverse-engineering/tiny-glade/evidence/toolchain-revalidation/rea-radare';
const test=process.argv[2]??'pdb',report={test,started:new Date().toISOString(),calls:[]};
const transport=new StdioClientTransport({command:'/opt/homebrew/bin/r2pm',args:['-r','r2mcp','-r','-C','both',...(test==='pdb-fixed'?['-g','disk,files']:['pdb-compatible','pdb-publics'].includes(test)?['-g','all']:[])],env:{...process.env,PATH:'/opt/homebrew/bin:'+process.env.PATH},stderr:'pipe'});
transport.stderr?.on('data',d=>report.stderr=(report.stderr??'')+String(d));
const client=new Client({name:'r2-deep-'+test,version:'1'});const save=()=>writeFileSync(dir+'/radare-'+test+'.json',JSON.stringify(report,null,2));
async function call(name,args){let t=Date.now();const result=await client.callTool({name,arguments:args},{timeout:180000});report.calls.push({name,args,elapsed_ms:Date.now()-t,result});save();console.log(JSON.stringify({name,ms:Date.now()-t,isError:result.isError??false}));return result;}
try{await client.connect(transport);report.server=client.getServerVersion();
if(test.startsWith('pdb')){
 await call('open_file',{file_path:root+'/ljxsj_92385/Tiny Glade/tiny-glade.rne'});
 if(test==='pdb-publics')await call('run_command',{command:'e scr.limit=0'});
 await call('run_command',{command:(['pdb-direct','pdb-publics'].includes(test)?'idpi ':'idp ')+root+'/ljxsj_92385/Tiny Glade/tiny_glade.pdb'+(test==='pdb-publics'?'~rust_begin_unwind':''),page_size:10});
 await call('lookup_symbol',{address:'0x140949c20'});
 await call('run_command',{command:'f~aspect_ratio',page_size:20});
 await call('run_command',{command:'tn',page_size:10});
}else{
 await call('open_file',{file_path:dir+(test==='ghidra-two'?'/isolation-fixture.exe':'/B-aspect-call-fixture.exe')});
 await call('analyze',{level:2});
 await call('xrefs_to',{address:'0x140001000'});
 await call('list_decompilers',{});
 await call('use_decompiler',{name:'ghidra'});
 await call('decompile_function',{address:'0x140001000'});
}
 await call('close_file',{});report.ok=!report.calls.some(c=>c.result.isError || JSON.stringify(c.result).includes('[ERROR]')); 
}catch(e){report.error=String(e);process.exitCode=1;}finally{await client.close();report.finished=new Date().toISOString();save();}
