// Current Ghidra route; REA was retired after the 2026-10-11 toolchain audit.
// Official Python MCP SDK is already installed in the retained bridge venv.
import {spawn} from 'node:child_process';
const child=spawn('/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python',
 ['/Users/yuluo/.codex/mcp-tools/ghidra/client.py',...process.argv.slice(2)],
 {stdio:'inherit',env:process.env});
child.on('error',error=>{console.error(String(error));process.exitCode=1;});
child.on('exit',(code,signal)=>{process.exitCode=code??1;});
