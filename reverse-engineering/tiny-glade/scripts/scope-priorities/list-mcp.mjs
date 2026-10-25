import { Client } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import { StdioClientTransport } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import { parse } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/smol-toml/dist/index.js';
import { readFileSync, writeFileSync } from 'node:fs';
const c=parse(readFileSync('C:/Users/liyu/.codex/config.toml','utf8')).mcp_servers.re_mcp_ghidra;
const client=new Client({name:'scope-priorities',version:'1'});
const t=new StdioClientTransport({command:c.command,args:c.args,env:{...process.env,...c.env},stderr:'pipe'});
t.stderr?.on('data',x=>process.stderr.write(x));
try { await client.connect(t,{timeout:120000});const r=await client.listTools();writeFileSync('D:/game/reverse-engineering/tiny-glade/evidence/scope-priorities/mcp-tool-list.json',JSON.stringify(r,null,2));console.log(r.tools.map(x=>x.name).join('\n')); }
finally {await client.close();}
