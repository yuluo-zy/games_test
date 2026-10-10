"""用 PDB 确认的边界提取原作函数，数据库保存到独立研究目录。"""
import hashlib
import json
import os
import argparse
from pathlib import Path

ROOT = Path('D:/game/reverse-engineering/tiny-glade')
parser = argparse.ArgumentParser()
parser.add_argument('--names', nargs='+', default=[
    'system_roof::create_roof::create_roof',
    'country_core::resources::history::History::finish_potential_mutation',
])
parser.add_argument('--report', default='slice-report.json')
args = parser.parse_args()
INPUT = ROOT / 'workspace/input/tiny-glade.exe'
OUT = ROOT / 'evidence/roof-ridge'
os.environ['GHIDRA_INSTALL_DIR'] = 'D:/tools/rea/ghidra/ghidra_12.1.4_PUBLIC'
os.environ['JAVA_HOME'] = 'D:/tools/rea/jdk/jdk-21.0.12.1+1'

from pyghidra.launcher import HeadlessPyGhidraLauncher
launcher = HeadlessPyGhidraLauncher()
launcher.vm_args = [arg for arg in launcher.vm_args if not arg.startswith('-Xmx')]
launcher.add_vmargs('-Xmx3G')
launcher.start()
from ghidra.base.project import GhidraProject
from ghidra.app.cmd.disassemble import DisassembleCommand
from ghidra.app.decompiler import DecompInterface
from ghidra.app.script import GhidraScriptUtil
from ghidra.program.model.address import AddressSet
from ghidra.program.model.symbol import SourceType
from ghidra.util.task import ConsoleTaskMonitor
from java.io import File

procedures = json.loads((ROOT / 'evidence/roof-ridge/procedures.json').read_text(encoding='utf-8'))['procedures']
targets = [p for p in procedures if p['name'] in args.names]
if len(targets) != len(args.names):
    raise ValueError('每个目标都需要 PDB 确认的唯一精确函数范围')
project_dir = ROOT / 'workspace/ghidra-ridge'
project_dir.mkdir(parents=True, exist_ok=True)
project_name = 'tiny-glade-ridge'
GhidraScriptUtil.acquireBundleHostReference()
project = None
program = None
report = {'input_sha256': hashlib.sha256(INPUT.read_bytes()).hexdigest(), 'target_execution': False,
          'analysis_scope': '只在PDB精确函数范围内反汇编；没有全程序分析或类型导入', 'functions': []}
try:
    if (project_dir / (project_name + '.gpr')).exists():
        project = GhidraProject.openProject(str(project_dir), project_name)
        program = project.openProgram('/', 'tiny-glade.exe', False)
    else:
        project = GhidraProject.createProject(str(project_dir), project_name, False)
        program = project.importProgram(File(str(INPUT)))
        project.saveAs(program, '/', 'tiny-glade.exe', True)
    print('持久数据库已打开', flush=True)
    monitor = ConsoleTaskMonitor()
    space = program.getAddressFactory().getDefaultAddressSpace()
    manager = program.getFunctionManager()
    listing = program.getListing()
    for target in targets:
        start = space.getAddress(target['va'])
        end = space.getAddress(hex(int(target['end_exclusive'], 16) - 1))
        body = AddressSet(start, end)
        command = DisassembleCommand(start, body, True)
        if not command.applyTo(program, monitor):
            raise RuntimeError('反汇编失败: ' + target['name'])
        function = manager.getFunctionAt(start)
        overlaps = [f for f in manager.getFunctions(body, True) if f.getEntryPoint() != start]
        merged_entries = [{'address': str(f.getEntryPoint()), 'name': str(f.getName())} for f in overlaps]
        # PDB 的过程范围包含加载器识别的辅助入口；仅在自有数据库中合并。
        for nested in overlaps:
            manager.removeFunction(nested.getEntryPoint())
        name = 'tg_' + target['name'].replace('::', '_')
        if function is None:
            function = manager.createFunction(name, start, body, SourceType.IMPORTED)
        else:
            function.setBody(body)
        instructions = [str(i.getAddress()) + ': ' + str(i) for i in listing.getInstructions(body, True)]
        decompiler = DecompInterface()
        decompiler.openProgram(program)
        try:
            result = decompiler.decompileFunction(function, 120, monitor)
            record = dict(target, instructions=instructions, merged_auxiliary_entries=merged_entries, decompile_completed=result.decompileCompleted(),
                          error=str(result.getErrorMessage()), signature_status='参数与类型由反编译器推测，尚未还原真实Rust签名')
            if result.decompileCompleted():
                record['pseudocode'] = str(result.getDecompiledFunction().getC()).replace('\r\n', '\n').replace('\r', '')
                (OUT / (name + '.c')).write_text(record['pseudocode'], encoding='utf-8')
            report['functions'].append(record)
            (OUT / args.report).write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding='utf-8')
            print(json.dumps({'function': target['name'], 'bytes': target['code_size'], 'decompiled': record['decompile_completed']}, ensure_ascii=False), flush=True)
        finally:
            decompiler.dispose()
    project.save(program)
    report['ok'] = all(f['decompile_completed'] for f in report['functions'])
finally:
    if project:
        if program:
            project.save(program)
        project.close()
    GhidraScriptUtil.releaseBundleHostReference()
    (OUT / args.report).write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding='utf-8')
