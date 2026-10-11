"""Bounded MCP jobs for named, isolated reverse-engineering workers."""
import argparse
import asyncio
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

ROOT = Path('/Users/yuluo/Desktop/game')
HERE = Path(__file__).resolve().parent
ALLOWED = {
 'ghidra': None, 'reva': None,
 'rea': {'open_binary','close_binary','binary_session','procedure_info','procedure_pseudo_code','read_bytes','xrefs','batch_decompile','inspect_native_instruction','trace_native_values'},
 'radare2': {'open_file','close_file','show_info','list_sections','list_decompilers','analyze','disassemble','disassemble_function','decompile_function','xrefs_to','hexdump','list_functions'},
 're-mcp': {'open_database','wait_for_analysis','get_database_info','list_databases','list_functions','decompile_function','disassemble_function','get_xrefs_to','get_xrefs_from','get_type_info','search_tools','get_schema','get_comment','set_comment','save_database','close_database'},
}
WRITES = {'set_comment','save_database','set_function_prototype','set_variable_type','rename_variables','set-function-prototype','run-script','write-script','edit-script','save_program'}


def stage_re_mcp(source, worker):
    source = Path(source).resolve()
    with source.open('rb') as stream:
        digest = hashlib.file_digest(stream, 'sha256').hexdigest()
    directory = ROOT / 'tools/re-mcp-inputs' / worker / digest[:16]
    directory.mkdir(parents=True, exist_ok=True)
    target = directory / source.name
    if not target.exists():
        subprocess.run(['/bin/cp','-c',str(source),str(target)],check=True)
    seed_sha = 'cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323'
    if digest == seed_sha:
        project = directory / 'ghidra_projects'
        project.mkdir(exist_ok=True)
        for suffix in ('.gpr','.rep'):
            destination = project / (source.name + suffix)
            if not destination.exists():
                subprocess.run(['/bin/cp','-cR',str(ROOT/'tools/ghidra-projects'/('TinyGlade'+suffix)),str(destination)],check=True)
    return str(target), digest


async def run(args):
    if not re.fullmatch(r'[a-zA-Z0-9][a-zA-Z0-9_-]{0,63}',args.worker):
        raise ValueError('Use a simple worker name')
    if args.write and args.worker=='shared':
        raise ValueError('Use a dedicated worker for writes')
    if args.backend == 're-mcp' and args.worker == 'shared':
        args.worker = 'read-' + str(os.getpid())
    job=json.loads(Path(args.job).read_text()) if args.job else []
    output=Path(args.output) if args.output else ROOT/'games_test/reverse-engineering/tiny-glade/evidence/toolchain'/('session-'+args.backend+'-'+args.worker+'-'+str(os.getpid())+'.json')
    output.parent.mkdir(parents=True,exist_ok=True)
    env={'TG_MCP_WORKER':args.worker,'TG_MCP_ACCESS':'write' if args.write else 'read',
         'GHIDRA_INSTALL_DIR':str(ROOT/'tools/ghidra_12.1.4_PUBLIC'),
         'JAVA_HOME':'/Users/yuluo/Library/Java/JavaVirtualMachines/corretto-21.0.5/Contents/Home',
         'PATH':str(ROOT/'tools/node/bin')+':'+os.environ.get('PATH',''),
         'REA_ANALYSIS_PROVIDER':'ghidra'}
    commands={
      'ghidra':[str(ROOT/'tools/venv/bin/python'),str(HERE/'ghidra/launch.py')],
      'reva':[str(ROOT/'tools/mcp-servers/ghidra-mcp/.venv/bin/python'),str(HERE/'reva/proxy.py')],
      'rea':[str(ROOT/'tools/node/bin/node'),str(ROOT/'tools/node/bin/rea'),'--mcp'],
      'radare2':['/opt/homebrew/bin/r2pm','-r','r2mcp','-r','-d','pdg','-C','both'],
      're-mcp':[str(ROOT/'tools/mcp-servers/re-mcp/.venv/bin/re-mcp-ghidra'),'proxy'],
    }
    command=commands[args.backend]
    report={'backend':args.backend,'worker':args.worker,'access':env['TG_MCP_ACCESS'],
            'actual_mcp_connection':False,'target_execution':False,'results':[]}
    def save():output.write_text(json.dumps(report,indent=2))
    try:
        params=StdioServerParameters(command=command[0],args=command[1:],env=env)
        async with stdio_client(params) as (read,write):
            async with ClientSession(read,write) as session:
                await asyncio.wait_for(session.initialize(),90)
                report['actual_mcp_connection']=True
                catalog=await session.list_tools()
                report['catalog']=catalog.model_dump(mode='json');save()
                names={tool.name for tool in catalog.tools}
                print(json.dumps({'connected':True,'backend':args.backend,'worker':args.worker,'tools':len(names)}),flush=True)
                for step in job:
                    name=step.get('tool',step.get('name'));values=dict(step.get('args',{}))
                    if name not in names or (ALLOWED[args.backend] is not None and name not in ALLOWED[args.backend]):
                        raise ValueError('Tool is outside this tested profile: '+str(name))
                    if not args.write and name in WRITES:
                        raise ValueError('Write tool requires a dedicated writable worker')
                    requested=dict(values)
                    if args.backend=='re-mcp' and name=='open_database':
                        values['file_path'],digest=stage_re_mcp(values['file_path'],args.worker)
                        values.setdefault('run_auto_analysis',False)
                        if values.get('force_new'):
                            raise ValueError('force_new would delete retained analysis; select a fresh worker instead')
                    result=await asyncio.wait_for(session.call_tool(name,values),step.get('timeout_seconds',300))
                    report['results'].append({'tool':name,'requested_args':requested,'args':values,'result':result.model_dump(mode='json')});save()
                    print(json.dumps({'tool':name,'isError':result.isError}),flush=True)
                    if result.isError and not step.get('allow_error',False):
                        raise RuntimeError(name+' failed; inspect '+str(output))
                    if args.backend == 'reva' and name == 'run-script':
                        payload = json.loads('\n'.join(block.text for block in result.content if block.type == 'text'))
                        if payload.get('success') is not True or payload.get('timedOut'):
                            if not step.get('allow_error', False):
                                raise RuntimeError('ReVa script failed or timed out; MCP transport success does not establish script success: ' + str(output))
                if args.backend == 're-mcp':
                    barrier = await session.call_tool('list_databases', {})
                    if barrier.isError:
                        raise RuntimeError('RE-MCP completion barrier failed')
                    report['completion_barrier'] = barrier.model_dump(mode='json')
                report['ok']=True
    except Exception as error:
        report['ok']=False;report['error']=str(error);raise
    finally:save()


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('backend',choices=ALLOWED)
    parser.add_argument('job',nargs='?')
    parser.add_argument('output',nargs='?')
    parser.add_argument('--worker',default='shared')
    parser.add_argument('--write',action='store_true')
    asyncio.run(run(parser.parse_args()))
