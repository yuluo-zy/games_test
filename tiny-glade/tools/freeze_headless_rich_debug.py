"""Rich-debug native-host freeze guard around the qualified existing builder.

Root must grant the serial Cargo slot before --execute. No runtime tool changes.
One cpu-probe EXE is frozen for subsequent separate trace-source OFF/ON runs.
Debug diagnostics are not formal Release performance acceptance.
"""
import argparse,datetime,hashlib,json,os,pathlib,subprocess,sys
if sys.flags.optimize:
    raise RuntimeError('Python optimization is forbidden: source guards require enabled assertions')
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def sig(value):return hashlib.sha256(json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
def isolated_git_env():
    env={key:value for key,value in os.environ.items() if not key.upper().startswith('GIT_')}
    env.update(GIT_CONFIG_NOSYSTEM='1',GIT_CONFIG_GLOBAL=os.devnull,GIT_CONFIG_SYSTEM=os.devnull,
        GIT_NO_REPLACE_OBJECTS='1',GIT_NO_LAZY_FETCH='1',GIT_TERMINAL_PROMPT='0',
        GIT_ALLOW_PROTOCOL='',GIT_OPTIONAL_LOCKS='0',GIT_ATTR_NOSYSTEM='1')
    return env
def git(root,*args):
    return subprocess.check_output(['git','-c','safe.directory='+root.as_posix(),'-c','core.fsmonitor=false',
        '-c','protocol.allow=never','-c','core.excludesFile='+os.devnull,'-c','core.attributesFile='+os.devnull,
        '-C',str(root),*args],text=False,env=isolated_git_env())
def snapshot(root):
    names=git(root,'ls-files','-z','--cached','--others','--exclude-standard')
    files={name:digest(root/name) if (root/name).is_file() else None for name in sorted(set(os.fsdecode(v) for v in names.split(b'\0') if v))}
    return {'commit':git(root,'rev-parse','HEAD').decode().strip(),
            'status':git(root,'status','--porcelain').decode().strip(),'files':files,'content_sha256':sig(files)}
def verify(guard):
    root=pathlib.Path(guard['root']);cache=pathlib.Path(guard['cache'])
    current=snapshot(root)
    assert current==guard['expected_source_identity'],'source commit/status/file SHA changed'
    assert not current['status']
    assert (root/'target').resolve(strict=True)==cache.resolve(strict=True)
    assert os.path.samefile(root/'target',cache)
    assert (root/'target/debug/deps').resolve(strict=True)==(cache/'debug/deps').resolve(strict=True)
    for name,expected in guard['qualified_tools_sha256'].items():assert digest(root/'tools'/name)==expected
    assert subprocess.check_output(['rustc','-Vv'],cwd=root,text=True)==guard['rustc_verbose'],'directory toolchain changed'
    return current
def main():
    p=argparse.ArgumentParser();p.add_argument('--guard',type=pathlib.Path,required=True);p.add_argument('--guard-sha256',required=True);p.add_argument('--execute',action='store_true')
    a=p.parse_args();assert digest(a.guard)==a.guard_sha256,'prepared guard manifest changed'
    guard=json.loads(a.guard.read_text());root=pathlib.Path(guard['root']);destination=pathlib.Path(guard['destination'])
    before=verify(guard);assert not destination.exists(),'frozen label already exists; never overwrite'
    env=isolated_git_env()
    for key in guard['reject_layout_environment']:assert not env.get(key),'undeclared layout environment '+key
    env.update(guard['fixed_environment'])
    command=[sys.executable,'-B',str(root/'tools/perf_headless.py'),'build',guard['label'],'--debug']
    summary={'schema':'prepared-rich-debug-on-execution-v1','execute':a.execute,'command':command,'cwd':str(root),
             'destination':str(destination),'source_commit':before['commit'],'source_content_sha256':before['content_sha256'],
             'source_file_count':len(before['files']),'script_sha256':digest(pathlib.Path(__file__)),
             'guard_sha256':a.guard_sha256,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
             'scope':'native Cgame rich-debug diagnostics; no formal Release/WSI performance acceptance'}
    print(json.dumps(summary,indent=2),flush=True)
    if not a.execute:return
    execution_log=pathlib.Path(guard['external_execution_log'])
    assert not execution_log.exists(),'execution guard log exists; never overwrite'
    summary.update(valid=False,child_exit_code=None,post_source_guard=False)
    try:
        summary['child_exit_code']=subprocess.run(command,cwd=root,env=env).returncode
        after=verify(guard);summary['post_source_guard']=after==before
        assert summary['child_exit_code']==0,'qualified build/freeze failed; inspect its preserved logs'
        build=json.loads((destination/'build.json').read_text())
        assert build['source_commit']==before['commit'] and build['source_content_sha256']==before['content_sha256']
        assert build['profile']=='debug' and build['dirty'] is False
        assert build['features']==['default(mimalloc)','cpu-probe'] and build['cpu_probe_feature'] is True
        assert build['jobs']==1 and build['incremental'] is False and build['allocator']=='mimalloc'
        assert build['command']==guard['expected_cargo_command']
        assert build['requested_test_target_completed'] is True
        assert build['cargo_lifecycle']['active_build_processes_remaining'] is False
        assert build['cargo_lifecycle']['requested_target_completed'] is True
        assert build['entire_cargo_build_passed']==build['cargo_lifecycle']['entire_cargo_build_passed']
        assert pathlib.Path(build['executable']).resolve().parent==destination.resolve()
        assert all(digest(destination/name)==expected for name,expected in build['files'].items())
        assert json.loads((destination/'source.json').read_text())==before
        assert build['management_controls']==guard['expected_management_controls']
        summary.update(valid=True,build_json_sha256=digest(destination/'build.json'),
                       executable_sha256=build['executable_sha256'],entire_cargo_build_passed=build['entire_cargo_build_passed'])
    finally:
        summary['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
        with execution_log.open('x',encoding='utf-8',newline='\n') as out:json.dump(summary,out,indent=2,ensure_ascii=False);out.write('\n')
    assert summary['valid']
if __name__=='__main__':main()
