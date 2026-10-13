"""Real MCP SDK jobs against named persistent Windows ReVa workers."""
import argparse
import asyncio
import hashlib
import io
import json
import os
import secrets
from pathlib import Path
import subprocess
import sys
import time
import traceback
from common import PYTHON, SHA256, TARGET, WorkerLock, atomic_json, clone_seed, owned_alive, target_digest, worker_dir

READ_TOOLS = {'list-project-files', 'get-function-count', 'get-decompilation',
              'find-cross-references', 'read-memory', 'get-memory-blocks', 'list-imports',
              'list-exports', 'get-data', 'get-structure-info', 'get-comments',
              'get-symbols', 'get-symbols-count', 'get-strings', 'get-strings-count',
              'get-bookmarks', 'list-analyzers', 'resolve-thunk'}

def text_payload(result):
    return '\n'.join(c.text for c in result.content if c.type == 'text')

def validate_result(result, name):
    if result.isError:
        raise RuntimeError(name + ': MCP isError: ' + text_payload(result)[:600])
    try:
        payload = json.loads(text_payload(result))
    except (ValueError, TypeError):
        return
    if isinstance(payload, dict):
        if payload.get('success') is False or payload.get('timedOut') or payload.get('stdoutTruncated') or payload.get('stderrTruncated'):
            raise RuntimeError(name + ': failed, timed out or truncated; inspect the retained result')

def ensure_backend(directory, worker):
    state_path = directory / 'state.json'
    state = json.loads(state_path.read_text(encoding='utf-8')) if state_path.exists() else {}
    if owned_alive(state):
        if state.get('status') != 'ready':
            raise RuntimeError('Owned backend is not ready: ' + str(state.get('status')))
        return state, True
    startup_token = secrets.token_hex(16)
    clone_seed(directory)
    with (directory / 'backend.log').open('ab') as log:
        child = subprocess.Popen([str(PYTHON), str(Path(__file__).with_name('daemon.py')), worker,
            '--startup-token', startup_token],
            stdin=subprocess.DEVNULL, stdout=log, stderr=log,
            creationflags=subprocess.CREATE_NO_WINDOW, close_fds=True)
    deadline = time.monotonic() + 120
    while time.monotonic() < deadline:
        if state_path.exists():
            state = json.loads(state_path.read_text(encoding='utf-8'))
            # Windows venv python.exe can launch a second interpreter PID.
            # Match this launch's nonce, then bind the real backend PID + birth time.
            if state.get('startup_token') == startup_token:
                if state.get('status') == 'failed':
                    raise RuntimeError('Backend failed; inspect ' + str(directory / 'backend.log'))
                if state.get('status') == 'ready' and owned_alive(state):
                    return state, False
        if child.poll() is not None:
            raise RuntimeError('Backend exited; inspect ' + str(directory / 'backend.log'))
        time.sleep(.2)
    raise TimeoutError('Backend startup timed out; inspect backend.log before retrying')

def spool_code(code, stdout_path):
    """Retain complete script output on disk, avoiding ReVa's 64 KiB stdout cap."""
    return ('import io as _tg_io, contextlib as _tg_ctx, json as _tg_json, hashlib as _tg_hash, builtins as _tg_builtins\n'
            'from pathlib import Path as _tg_Path\n'
            '_tg_buf = _tg_io.StringIO()\n'
            '_tg_native_print = print\n'
            'globals()["print"] = _tg_builtins.print\n'
            'try:\n'
            '    with _tg_ctx.redirect_stdout(_tg_buf):\n'
            '        exec(compile(' + repr(code) + ', "<retained-job>", "exec"), globals())\n'
            'finally:\n'
            '    globals()["print"] = _tg_native_print\n'
            '    _tg_out = _tg_buf.getvalue()\n'
            '    _tg_Path(' + repr(str(stdout_path)) + ').write_text(_tg_out, encoding="utf-8", newline="\\n")\n'
            'print(_tg_json.dumps({"artifact": ' + repr(str(stdout_path)) + ', '
            '"bytes": len(_tg_out.encode("utf-8")), "sha256": _tg_hash.sha256(_tg_out.encode("utf-8")).hexdigest()}))\n')

async def checkpoint(state, worker):
    directory = worker_dir(worker)
    nonce = secrets.token_hex(16)
    atomic_json(directory / 'checkpoint.json', {'generation':state['generation'], 'nonce':nonce})
    deadline = time.monotonic() + 60
    while time.monotonic() < deadline:
        response = directory / 'checkpoint-result.json'
        if response.exists():
            payload = json.loads(response.read_text(encoding='utf-8'))
            if payload.get('nonce') == nonce:
                if not payload.get('success') or payload.get('changed_after_save'):
                    raise RuntimeError('Local checkpoint failed: ' + str(payload))
                return payload
        if not owned_alive(state): raise RuntimeError('Owned worker exited during checkpoint')
        await asyncio.sleep(.1)
    raise TimeoutError('Local checkpoint did not finish')

async def run(args, state, reused):
    from mcp import ClientSession
    from mcp.client.streamable_http import streamable_http_client
    import httpx
    output = Path(args.output or worker_dir(args.worker) / ('result-' + str(os.getpid()) + '.json')).resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    report = {'backend': args.backend, 'transport': 'official_mcp_sdk_streamable_http',
              'worker': args.worker, 'pid': state['pid'], 'project': state['project'],
              'backend_reused': reused, 'actual_mcp_connection': False, 'target_execution': False,
              'ok': False, 'results': []}
    started = time.monotonic()
    def save():
        report['elapsed_seconds'] = round(time.monotonic() - started, 3)
        atomic_json(output, report)
    try:
        async with httpx.AsyncClient(headers={'X-API-Key': state['api_key']}, timeout=600,
                limits=httpx.Limits(max_keepalive_connections=0)) as http_client, streamable_http_client(
                f'http://127.0.0.1:{state["port"]}/mcp/message', http_client=http_client) as (read, write, _):
            async with ClientSession(read, write) as session:
                await asyncio.wait_for(session.initialize(), 30)
                report['actual_mcp_connection'] = True
                catalog, cursor = [], None
                while True:
                    page = await session.list_tools(cursor=cursor)
                    catalog.extend(t.model_dump(mode='json') for t in page.tools)
                    cursor = page.nextCursor
                    if not cursor: break
                atomic_json(worker_dir(args.worker) / 'catalog.json', {'tools': catalog})
                schemas = {t['name']: t['inputSchema'] for t in catalog}
                report['tool_count'] = len(catalog)
                save()
                if args.schema:
                    if args.backend == 'ghidra':
                        import ghidra_queries
                        requested_schema = ghidra_queries.schema(args.schema.replace('-', '_'))
                    else:
                        name = args.schema.replace('_', '-')
                        if name not in schemas: raise ValueError('Unknown tool: ' + args.schema)
                        requested_schema = schemas[name]
                    print(json.dumps(requested_schema, ensure_ascii=False, indent=2))
                    report['ok'] = True; save(); return
                # Bootstrap is explicit, bounded, identity-checked; no full auto-analysis.
                result = await session.call_tool('list-project-files', {'folderPath': '/', 'recursive': False})
                validate_result(result, 'list-project-files')
                if 'tiny-glade.exe' not in text_payload(result):
                    target_digest()
                    result = await session.call_tool('import-file', {'path': str(TARGET), 'destinationFolder': '/',
                        'analyzeAfterImport': False, 'enableVersionControl': False, 'recursive': False})
                    report['bootstrap'] = result.model_dump(mode='json'); save()
                    validate_result(result, 'import-file')
                identity = await session.call_tool('run-script', {'programPath': '/tiny-glade.exe', 'timeoutSeconds': 30,
                    'code': 'import json\nprint(json.dumps({"sha256":currentProgram.getExecutableSHA256(), "path":currentProgram.getDomainFile().getPathname(), "functions":currentProgram.getFunctionManager().getFunctionCount()}))'})
                report['identity_result'] = identity.model_dump(mode='json'); save()
                validate_result(identity, 'identity')
                identity_json = json.loads(json.loads(text_payload(identity))['stdout'])
                if identity_json['sha256'] != SHA256 or identity_json['path'] != '/tiny-glade.exe':
                    raise RuntimeError('Program identity mismatch')
                report['identity'] = identity_json
                job = json.loads(Path(args.job).read_text(encoding='utf-8')) if args.job else []
                if not isinstance(job, list): raise ValueError('Job must be a JSON array')
                for index, step in enumerate(job):
                    requested_name = step.get('tool', step.get('name', ''))
                    name = requested_name.replace('_', '-')
                    values = dict(step.get('args', {}))
                    entry = {'tool': requested_name, 'requested_args': values.copy()}
                    report['results'].append(entry); save()
                    adapter_read = False
                    if args.backend == 'ghidra' and requested_name.replace('-', '_') == 'save_program':
                        if not args.write: raise ValueError('save_program requires --write')
                        if values.get('program') != 'tiny-glade.exe': raise ValueError('Specify program="tiny-glade.exe"')
                        entry.update(local_checkpoint=await checkpoint(state,args.worker),passed=True)
                        save()
                        continue
                    elif args.backend == 'ghidra':
                        import ghidra_queries
                        original_name = requested_name.replace('-', '_')
                        adapter_read = original_name in ghidra_queries.READ
                        if not adapter_read and not args.write:
                            raise ValueError(original_name + ' requires --write')
                        values = {'programPath': '/tiny-glade.exe', 'timeoutSeconds': 180,
                                  'code': ghidra_queries.script(original_name, values)}
                        name = 'run-script'
                        entry['native_tool'] = name
                    if name not in schemas: raise ValueError('Unknown tool: ' + name)
                    if not args.write and name not in READ_TOOLS and not adapter_read:
                        raise ValueError(name + ' requires --write and a dedicated worker')
                    if 'programPath' in schemas[name].get('properties', {}) and values.get('programPath') != '/tiny-glade.exe':
                        raise ValueError('Specify programPath="/tiny-glade.exe" for every program-level tool')
                    if name == 'run-script' and 'code' in values:
                        artifacts = output.parent / (output.stem + '.artifacts')
                        artifacts.mkdir(exist_ok=True)
                        stdout_path = artifacts / f'{index:03d}.stdout.txt'
                        values['code'] = spool_code(values['code'], stdout_path)
                        entry['stdout_artifact'] = str(stdout_path)
                    tick = time.monotonic()
                    result = await asyncio.wait_for(session.call_tool(name, values), step.get('timeout_seconds', 300))
                    entry.update(result=result.model_dump(mode='json'), elapsed_seconds=round(time.monotonic()-tick, 3))
                    if entry.get('stdout_artifact') and Path(entry['stdout_artifact']).exists():
                        entry['stdout'] = Path(entry['stdout_artifact']).read_text(encoding='utf-8')
                    save()
                    try:
                        validate_result(result, name)
                        if entry.get('stdout_artifact'):
                            manifest = json.loads(json.loads(text_payload(result))['stdout'])
                            artifact = Path(entry['stdout_artifact'])
                            if not artifact.exists() or manifest.get('artifact') != str(artifact):
                                raise RuntimeError('Script stdout artifact missing or path mismatched')
                            raw = artifact.read_bytes()
                            if len(raw) != manifest.get('bytes') or hashlib.sha256(raw).hexdigest() != manifest.get('sha256'):
                                raise RuntimeError('Script stdout artifact checksum mismatch')
                        entry['passed'] = True
                    except RuntimeError as error:
                        entry.update(passed=False, error=str(error))
                        if not step.get('allow_error', False): raise
                    print(json.dumps({'tool': name, 'passed': entry['passed'], 'seconds': entry['elapsed_seconds']}), flush=True)
                # Explicit save is needed: run-script commits do not auto-save the domain file.
                # Save outside GhidraScript.execute's implicit enclosing transaction.
                report['save_result'] = await checkpoint(state,args.worker)
                save()
                report['ok'] = all(r.get('passed') for r in report['results'])
                save()
    except BaseException as error:
        report['error'] = str(error)
        report['exception_trace'] = ''.join(traceback.format_exception(error))
        save(); raise
    print(json.dumps({'ok': report['ok'], 'report': str(output), 'backend_reused': reused,
                      'pid': state['pid'], 'seconds': report['elapsed_seconds']}), flush=True)
    if not report['ok']: raise RuntimeError('One or more expected-error steps failed; report ok remains false')

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('backend', choices=['reva', 'ghidra'])
    parser.add_argument('job', nargs='?')
    parser.add_argument('output', nargs='?')
    parser.add_argument('--worker', default='shared')
    parser.add_argument('--write', action='store_true')
    parser.add_argument('--schema')
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--stop', action='store_true')
    args = parser.parse_args()
    if args.write and args.worker == 'shared': parser.error('Use a dedicated worker for writes')
    directory = worker_dir(args.worker)
    with WorkerLock(directory):
        path = directory / 'state.json'
        state = json.loads(path.read_text(encoding='utf-8')) if path.exists() else {}
        if args.check:
            print(json.dumps({'worker': args.worker, 'running': bool(owned_alive(state)),
                'status': state.get('status'), 'pid': state.get('pid'), 'project': state.get('project')})); return
        if args.stop:
            if owned_alive(state):
                atomic_json(directory / 'stop.json', {'generation': state['generation']})
                deadline = time.monotonic() + 60
                while owned_alive(state) and time.monotonic() < deadline: time.sleep(.2)
                if owned_alive(state): raise TimeoutError('Graceful shutdown still pending; no unrelated PID was stopped')
            final = json.loads(path.read_text(encoding='utf-8')) if path.exists() else {}
            print(json.dumps({'worker': args.worker, 'running': bool(owned_alive(final)), 'status': final.get('status')}))
            if final.get('status') == 'failed': raise RuntimeError('Backend shutdown failed; inspect backend.log')
            return
        state, reused = ensure_backend(directory, args.worker)
        asyncio.run(run(args, state, reused))

if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        print(str(error), file=sys.stderr)
        sys.exit(1)
