import {Client} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import {StdioClientTransport} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import {parse} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/smol-toml/dist/index.js';
import {readFileSync,writeFileSync} from 'node:fs';
const root='D:/game/reverse-engineering/tiny-glade';
const [jobFile,output]=process.argv.slice(2);
const config=parse(readFileSync('C:/Users/liyu/.codex/config.toml','utf8')).mcp_servers['re_mcp_ghidra'];
const job=JSON.parse(readFileSync(jobFile,'utf8'));
const transport=new StdioClientTransport({command:config.command,args:config.args??[],env:{...process.env,...config.env},cwd:root,stderr:'pipe'});
const client=new Client({name:'wcb',version:'1.0'});
const report={results:[]};
try{
 await client.connect(transport,{timeout:120000});
 for(const step of job){
  const result=await client.callTool({name:step.tool,arguments:step.args??{}},{timeout:step.timeout_ms??900000});
  report.results.push({tool:step.tool,result});
  writeFileSync(output,JSON.stringify(report,null,2));
  console.log(JSON.stringify({tool:step.tool,err:!!result.isError}));
  if(step.tool==='decompile_function'&&result.content?.[0]?.text){
    writeFileSync(output+'.c.txt',result.content[0].text);
  }
 }
}catch(e){report.error=String(e);console.error(String(e));process.exitCode=1;}
finally{await client.close();writeFileSync(output,JSON.stringify(report,null,2));}
