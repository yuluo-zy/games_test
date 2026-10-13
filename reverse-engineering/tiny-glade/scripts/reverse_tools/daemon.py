"""Owned persistent ReVa/Ghidra backend, independent of Codex session lifetime.

Uses the installed ReVa Java server and Python launcher, with an explicit project
directory instead of its default disposable project. No installed package edits.
"""
import argparse
import json
import os
import secrets
import sys
import time
from pathlib import Path
from common import GHIDRA, JAVA, atomic_json, process_birth, worker_dir

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('worker')
    parser.add_argument('--startup-token', required=True)
    args = parser.parse_args()
    directory = worker_dir(args.worker)
    os.environ['GHIDRA_INSTALL_DIR'] = str(GHIDRA)
    os.environ['JAVA_HOME'] = str(JAVA)
    os.environ['REVA_ANALYSIS_MAX_CONCURRENCY'] = '1'
    import pyghidra
    pyghidra.start()
    from reva_cli.launcher import ReVaLauncher
    from reva.headless import RevaHeadlessLauncher
    from java.io import File
    from ghidra.app.script import GhidraScriptUtil
    from reva.plugin import RevaProgramManager
    from ghidra.framework.main import AppInfo
    from ghidra.util.task import TaskMonitor

    class PersistentLauncher(ReVaLauncher):
        def start(self):
            GhidraScriptUtil.acquireBundleHostReference()
            self._script_bundle_host = GhidraScriptUtil
            project = directory / 'project'
            project.mkdir(parents=True, exist_ok=True)
            self.java_launcher = RevaHeadlessLauncher(
                None, True, True, File(str(project)), 'TinyGlade', self.api_key)
            self._apply_tool_group_config(self.java_launcher)
            self.java_launcher.start()
            if not self.java_launcher.waitForServer(60000):
                raise TimeoutError('ReVa server did not become ready')
            self.port = int(self.java_launcher.getPort())
            return self.port

    generation = secrets.token_hex(16)
    launcher = PersistentLauncher(api_key='ReVa-' + secrets.token_hex(32))
    state = {'worker': args.worker, 'pid': os.getpid(), 'process_birth': process_birth(os.getpid()),
             'generation': generation, 'startup_token': args.startup_token,
             'project': str(directory / 'project/TinyGlade.gpr')}
    try:
        port = launcher.start()
        state.update(status='ready', port=port, api_key=launcher.api_key)
        atomic_json(directory / 'state.json', state)
        handled_checkpoint = None
        while True:
            checkpoint = directory / 'checkpoint.json'
            if checkpoint.exists():
                message = json.loads(checkpoint.read_text(encoding='utf-8'))
                if message.get('generation') == generation and message.get('nonce') != handled_checkpoint:
                    handled_checkpoint = message['nonce']
                    result = {'nonce': handled_checkpoint, 'success': False}
                    try:
                        p = RevaProgramManager.getProgramByPath('/tiny-glade.exe')
                        if p is None: raise ValueError('B program not open')
                        changed = bool(p.isChanged())
                        if changed: p.save('Owned worker local checkpoint', TaskMonitor.DUMMY)
                        result.update(success=True, was_changed=changed, changed_after_save=bool(p.isChanged()),
                                      path=str(p.getDomainFile().getPathname()), saved_locally=True)
                    except Exception as error:
                        result['error'] = str(error)
                    atomic_json(directory / 'checkpoint-result.json', result)
            request = directory / 'stop.json'
            if request.exists():
                if json.loads(request.read_text(encoding='utf-8')).get('generation') == generation:
                    break
            time.sleep(.2)
        # Save every program before releasing consumers/closing the project.
        project = AppInfo.getActiveProject()
        def save_folder(folder):
            for df in folder.getFiles():
                if str(df.getContentType()) == 'Program':
                    p = RevaProgramManager.getProgramByPath(str(df.getPathname()))
                    if p is not None and p.isChanged():
                        p.save('Owned worker shutdown', TaskMonitor.DUMMY)
            for child in folder.getFolders():
                save_folder(child)
        if project is not None:
            save_folder(project.getProjectData().getRootFolder())
        RevaProgramManager.cleanup()
        state['status'] = 'stopped'
    except BaseException as error:
        import traceback
        traceback.print_exc()
        state.update(status='failed', error=str(error))
    finally:
        state.pop('api_key', None)
        try:
            launcher.stop()
        finally:
            atomic_json(directory / 'state.json', state)

if __name__ == '__main__':
    main()
