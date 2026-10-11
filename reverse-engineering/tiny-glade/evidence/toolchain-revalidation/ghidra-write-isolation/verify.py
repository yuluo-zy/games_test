import asyncio, json, os, time
from pathlib import Path
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client
OUT=Path(__file__).resolve().parent
GATE="/Users/yuluo/.codex/mcp-tools/ghidra/launch.py"
PROGRAM="tiny-glade.exe"
barrier=asyncio.Barrier(3)
events=[]
async def actor(worker, access):
 env={"TG_MCP_WORKER":worker,"TG_MCP_ACCESS":access}
 params=StdioServerParameters(command="/Users/yuluo/Desktop/game/tools/venv/bin/python",args=[GATE],env=env)
 row={"worker":worker,"access":access,"calls":[]}
 async with stdio_client(params) as (rd,wr):
  async with ClientSession(rd,wr) as session:
   await session.initialize();cat=await session.list_tools();names={t.name for t in cat.tools};row["tools"]=sorted(names)
   assert ("set_function_prototype" in names)==(access=="write")
   async def call(name,args):
    start=time.monotonic();result=await session.call_tool(name,args);end=time.monotonic();assert not result.isError, str(result)
    text="\n".join(x.text for x in result.content if x.type=="text");q=json.loads(text);row["calls"].append({"tool":name,"args":args,"start":start,"end":end,"result":q});return q
   instances=await call("list_instances",{});assert len(instances["instances"])==1
   row["backend_pid"]=instances["instances"][0]["pid"]
   await barrier.wait()
   if access=="write":
    param=worker.replace("-","_")+"_size"
    q=await call("set_function_prototype",{"program":PROGRAM,"function_address":"0x140949c20","prototype":"float __fastcall aspect_ratio(unsigned int *"+param+")"});assert q["status"]=="success"
    await call("set_comment",{"program":PROGRAM,"address":"0x140949c20","comment":"Tool validation isolated writer "+worker,"type":"plate"})
    await call("save_program",{"program":PROGRAM})
   else:
    q=await call("get_functions",{"program":PROGRAM,"function":"0x140949c20","fields":"signature,decompiled_code,parameters"});assert q["parameters"][0]["name"]=="size"
   await barrier.wait()
   q=await call("get_functions",{"program":PROGRAM,"function":"0x140949c20","fields":"signature,decompiled_code,parameters"})
   expected="size" if access=="read" else worker.replace("-","_")+"_size"
   assert q["parameters"][0]["name"]==expected,(worker,q)
   assert q["revision"]["unsaved_changes"]==False
   row["expected_parameter"]=expected
   code=await call("read_memory",{"program":PROGRAM,"address":"0x140949c20","length":20});row["instruction_hex"]=code["hex"]
 row["ok"]=True;(OUT/(worker+".json")).write_text(json.dumps(row,indent=2));return row
async def main():
 rows=await asyncio.gather(actor("proof-a","write"),actor("proof-b","write"),actor("shared","read"))
 assert len({r["backend_pid"] for r in rows})==3
 assert len({r["instruction_hex"] for r in rows})==1
 report={"ok":True,"workers":rows,"different_backends":True,"same_machine_code":True,"annotations_isolated":True}
 (OUT/"summary.json").write_text(json.dumps(report,indent=2));print(json.dumps({"ok":True,"backend_pids":[r["backend_pid"]for r in rows],"annotations_isolated":True}),flush=True)
asyncio.run(main())
