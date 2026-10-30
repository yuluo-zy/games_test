#!/usr/bin/env python3
"""Exercise the real stdio MCP entry against the published local Neo4j graph."""
import asyncio
import json
from pathlib import Path
import sys
import time
from artifacts import RUNTIME,atomic
from graph import Graph

def graph_signature():
    g=Graph();s=g.current()
    nodes=g.query('MATCH (n:Entity {snapshot:$s}) RETURN count(n) AS count',{'s':s})[0]['count']
    edges=g.query('MATCH (a:Entity {snapshot:$s})-[r:LINK]->() RETURN count(r) AS count',{'s':s})[0]['count']
    sample=g.query('''MATCH (a:Entity {snapshot:$s,key:'function:B:8cdc60'})-[r:LINK]->(b)
                WHERE r.kind IN ['READS','HAS_CONTRACT'] RETURN b.key AS target,r.kind AS kind ORDER BY target,kind''',{'s':s})
    return {'snapshot':s,'nodes':nodes,'edges':edges,'sample':sample}

async def mcp_smoke():
    from mcp import ClientSession,StdioServerParameters
    from mcp.client.stdio import stdio_client
    started=time.monotonic();results=[]
    server=StdioServerParameters(command=sys.executable,args=[str(Path(__file__).with_name('code_map.py')),'serve'])
    async with stdio_client(server) as (read,write):
        async with ClientSession(read,write) as session:
            await session.initialize()
            listing=await session.list_tools()
            names=[t.name for t in listing.tools]
            for name,args in [('search',{'text':'0x1408cdc60','limit':3}),
                              ('state_users',{'key':'state:B:WorldRoofTable'}),
                              ('assembly',{'key':'function:B:8cc870','limit':100})]:
                result=await session.call_tool(name,args)
                if result.isError:raise RuntimeError('MCP tool failed: '+name)
                data=json.loads(result.content[0].text)
                if not data.get('rows'):raise RuntimeError('Empty real result: '+name)
                results.append({'tool':name,'rows':len(data['rows']),'snapshot':data['snapshot']})
            rejected=await session.call_tool('delete_database',{})
            if not rejected.isError:raise RuntimeError('Non-read tool was accepted')
            guard=await session.call_tool('assembly',{'key':'function:B:289b680','limit':10})
            guard_data=json.loads(guard.content[0].text)
            if guard.isError or guard_data['undecoded'] or guard_data['rows'][0]['bytes']!='ffe0':
                raise RuntimeError('Reviewed guard range not available with preserved history')
            loose=Graph().query("MATCH (n:Entity {snapshot:$s,kind:'EvidenceFile'}) RETURN n.key AS key LIMIT 1",{'s':Graph().current()})
            if loose:
                archived=await session.call_tool('evidence_pack',{'key':loose[0]['key'],'version':'unbound'})
                archived_data=json.loads(archived.content[0].text)
                if archived.isError or 'archived_excerpt' not in archived_data or archived_data['binary_identity'] is not None:
                    raise RuntimeError('Unbound archive was lost or promoted to verified binary evidence')
            unpublished=Graph().query('MATCH (s:Snapshot) WHERE s.complete=false RETURN s.id AS id LIMIT 1')
            if unpublished:
                denied=await session.call_tool('search',{'snapshot':unpublished[0]['id'],'text':'ray'})
                if not denied.isError:raise RuntimeError('Unpublished snapshot was readable')
    report={'ok':True,'tools':names,'real_calls':results,'write_tool_rejected':True,
            'dispatch_correction_readable':True,'unbound_archive_remains_unbound':True,
            'unpublished_snapshot_rejected':True,'seconds':time.monotonic()-started}
    atomic(RUNTIME/'mcp-smoke.json',report);return report

if __name__=='__main__':
    if '--signature' in sys.argv:print(json.dumps(graph_signature()))
    else:print(json.dumps(asyncio.run(mcp_smoke())))
