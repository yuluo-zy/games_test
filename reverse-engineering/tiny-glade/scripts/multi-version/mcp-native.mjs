// Invoke the existing filtered REA server without changing Codex registration.
// Full results stay on disk; stdout contains only bounded status summaries.
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {dirname,resolve} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
const root=resolve(dirname(fileURLToPath(import.meta.url)),'../..');
const toolsRoot=resolve(root,'../../../tools');
const sdk=pathToFileURL(toolsRoot+'/node/lib/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/').href;
const {Client}=await import(sdk+'index.mjs');
const {StdioClientTransport}=await import(sdk+'stdio.mjs');
const out=resolve(root,process.argv[3]??'evidence/multi-version/mcp-session.json');
mkdirSync(dirname(out),{recursive:true});
const job=process.argv[2]?JSON.parse(readFileSync(process.argv[2],'utf8')):[];
const client=new Client({name:'tiny-glade-three-version',version:'1.0'});
const transport=new StdioClientTransport({
  command:'/Users/yuluo/.codex/mcp-tools/toolgate/toolgate.py',
  args:['/Users/yuluo/.codex/mcp-tools/toolgate/rea.json'],cwd:root,stderr:'pipe',
  env:{...process.env,PATH:toolsRoot+'/node/bin:'+process.env.PATH,
    GHIDRA_INSTALL_DIR:toolsRoot+'/ghidra_12.1.4_PUBLIC',
    JAVA_HOME:'/Users/yuluo/Library/Java/JavaVirtualMachines/corretto-21.0.5'},
});
transport.stderr?.on('data',chunk=>writeFileSync(out+'.stderr.log',chunk,{flag:'a'}));
const report={target_execution:false,actual_mcp_connection:false,results:[]};
try {
  await client.connect(transport,{timeout:120000});
  const catalog=await client.listTools();
  writeFileSync(resolve(dirname(out),'catalog.json'),JSON.stringify(catalog,null,2));
  report.actual_mcp_connection=true;
  console.log(JSON.stringify({connected:true,filtered_tools:catalog.tools.length,catalog:resolve(dirname(out),'catalog.json')}));
  for(const step of job) {
    const result=await client.callTool({name:step.tool,arguments:step.args??{}},{timeout:step.timeout_ms??600000});
    report.results.push({tool:step.tool,args:step.args??{},result});
    writeFileSync(out,JSON.stringify(report,null,2));
    console.log(JSON.stringify({tool:step.tool,isError:result.isError??false,output:out}));
    if(result.isError&&!step.allow_error)throw Error(step.tool+' failed; inspect saved result');
  }
  report.ok=true;
} catch(error) {
  report.ok=false;report.error=String(error);process.exitCode=1;
  console.error(String(error));
} finally {
  await client.close();
  writeFileSync(out,JSON.stringify(report,null,2));
}
