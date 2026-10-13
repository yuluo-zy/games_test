"""Publish the current local route; retain historical Mac manifest unchanged."""
import json
from pathlib import Path
from common import GHIDRA, JAVA, PYTHON, ROOT, SHA256, atomic_json

path=ROOT/'tool-usage/manifest.json'
old=json.loads(path.read_text(encoding='utf-8'))
archive=path.with_name('manifest-macos-2026-10-11.json')
if old.get('format_version')==1 and not archive.exists():archive.write_bytes(path.read_bytes())
atomic_json(path,{'format_version':2,'validated_on':'2026-10-13','platform':'Windows',
    'guide':str(ROOT/'AI_REVERSE_TOOLS.md'),'interpreter':str(PYTHON),
    'operator':str(Path(__file__).with_name('reverse_operator.py')),
    'ghidra_install':str(GHIDRA),'java_home':str(JAVA),
    'argv_format':['interpreter','operator','ghidra|reva','job_absolute_path','optional_result_absolute_path','--worker','unique_worker','optional_--write'],
    'persistent_project':'D:/game/tools/reverse-runtime/<worker>/project/TinyGlade.gpr',
    'program_sha256':SHA256,'preexisting_mac_overlays':False,
    'profiles':{'ghidra':{'transport':'ReVa MCP / Ghidra engine compatibility adapter',
        'selectors':{'program':'tiny-glade.exe'},
        'read_tools':['get_metadata','find_functions','get_functions','get_xrefs_to','get_xrefs_from','read_memory'],
        'write_tools':['prepare_functions','save_program'],'max_function_batch':20,
        'job':'jobs/windows-ghidra-read.json','schema':'operator ghidra --schema <tool>'},
      'reva':{'transport':'Official MCP SDK Streamable HTTP / installed ReVa server',
        'selectors':{'programPath':'/tiny-glade.exe'},'job':'jobs/windows-reva-read.json',
        'schema':'operator reva --schema <native-tool>','full_catalog':'worker catalog.json; keep off model context'}},
    'evidence':['../evidence/toolchain-repair-2026-10-13/summary.json','../evidence/toolchain-repair-2026-10-13/workers-summary.json'],
    'historical_macos_manifest':archive.name,
    'limits':['The historical optional REA/radare2/RE-MCP validations do not establish current Windows readiness.',
              'Tool validation is not game-source recovery.']})
print('Current Windows manifest published')
