// Windows route uses the installed official Python MCP SDK and persistent workers.
import {spawn} from 'node:child_process';
import {fileURLToPath} from 'node:url';
const script = fileURLToPath(new URL('./reverse_operator.py', import.meta.url));
const python = process.env.TG_REVERSE_PYTHON || 'D:/tools/reverse-engineering/venvs/reverse-engineering-assistant/Scripts/python.exe';
const child = spawn(python, [script, ...process.argv.slice(2)], {stdio:'inherit', env:process.env, windowsHide:true});
child.on('error', error => { console.error(String(error)); process.exitCode=1; });
child.on('exit', code => { process.exitCode=code??1; });
