// Real MCP calls through the official SDK; no local decompiler replacement.
import {Client} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import {StdioClientTransport} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import {parse} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/smol-toml/dist/index.js';
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
const root='D:/game/reverse-engineering/tiny-glade';
const [server,jobFile,output]=process.argv.slice(2);
const config=parse(readFileSync('C:/Users/liyu/.codex/config.toml','utf8')).mcp_servers[server];
if(!config)throw Error('Configured MCP server not found: '+server);
const job=jobFile?JSON.parse(readFileSync(jobFile,'utf8')):[];
mkdirSync(root+'/evidence/mcp',{recursive:true});
const transport=new StdioClientTransport({command:config.command,args:config.args??[],env:{...process.env,...config.env},cwd:root,stderr:'pipe'});
transport.stderr?.on('data',data=>{writeFileSync(root+'/evidence/mcp/'+server+'-stderr.log',data,{flag:'a'});});
const client=new Client({name:'tiny-glade-source-investigator',version:'1.0'});
const report={server,actual_mcp_connection:false,results:[],target_execution:false};
const opened=new Set();
try{
 await client.connect(transport,{timeout:120000});
 const catalog=await client.listTools();
 report.actual_mcp_connection=true;
 writeFileSync(root+'/evidence/mcp/'+server+'-catalog.json',JSON.stringify(catalog,null,2));
 console.log(JSON.stringify({server,connected:true,tools:catalog.tools.map(t=>t.name)}));
 for(const step of job){
  const result=await client.callTool({name:step.tool,arguments:step.args??{}},{timeout:step.timeout_ms??600000});
  report.results.push({tool:step.tool,args:step.args,result});
  if(step.tool==='open_database'&&!result.isError&&step.args?.database_id)opened.add(step.args.database_id);
  if(step.tool==='close_database'&&!result.isError)opened.delete(step.args?.database);
  writeFileSync(output??root+'/evidence/mcp/'+server+'-job.json',JSON.stringify(report,null,2));
  console.log(JSON.stringify({tool:step.tool,ok:!result.isError}));
  if(result.isError&&!step.allow_error)throw Error(step.tool+' returned error');
 }
 report.ok=true;
}catch(error){report.ok=false;report.error=String(error);process.exitCode=1;console.error(String(error));}
finally{
 for(const database of opened){try{const result=await client.callTool({name:'close_database',arguments:{database,save:true}},{timeout:120000});report.results.push({tool:'close_database',args:{database,save:true},cleanup:true,result});}catch(error){report.cleanup_error=String(error);}}
 await client.close();writeFileSync(output??root+'/evidence/mcp/'+server+'-job.json',JSON.stringify(report,null,2));
}
