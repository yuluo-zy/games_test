import { Client } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import { StdioClientTransport } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import { parse } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/smol-toml/dist/index.js';
import { readFileSync, writeFileSync } from 'node:fs';
const c=parse(readFileSync('C:/Users/liyu/.codex/config.toml','utf8')).mcp_servers.re_mcp_ghidra;
const client=new Client({name:'scope-priorities',version:'1'});
const t=new StdioClientTransport({command:c.command,args:c.args,env:{...process.env,...c.env},stderr:'pipe'});
const database='scope_original';const out={results:[],target:'isolated copy of original RNE',full_auto_analysis:false};
const call=async(name,args)=>{const r=await client.callTool({name,arguments:args},{timeout:600000});out.results.push({name,args,result:r});writeFileSync('D:/game/reverse-engineering/tiny-glade/evidence/scope-priorities/mcp-read.json',JSON.stringify(out,null,2));console.log(JSON.stringify({name,isError:r.isError??false}));return r;};
try {await client.connect(t,{timeout:120000});
 await call('open_database',{file_path:'D:/game/reverse-engineering/tiny-glade/workspace/scope-priorities/scope-original.exe',run_auto_analysis:false,database_id:database});
 await call('wait_for_analysis',{database});
 await call('get_database_info',{database});
 await call('decompile_function',{database,address:'0x140ac7f30'});
 await call('decompile_function',{database,address:'0x1421ab7c0'});
}finally {try{await call('close_database',{database,save:true});}finally{await client.close();}}
