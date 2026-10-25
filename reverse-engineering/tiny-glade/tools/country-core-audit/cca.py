"""country-core 审计工具的公共模块：路径、PDB 过程表、调用图加载。

路径均可用环境变量覆盖：
  CCA_EXE   原版 EXE（默认 <repo>/tiny-glade.exe，版本 B，sha256 cf048e27…）
  CCA_PDB   原版 PDB（默认 <repo>/tiny_glade.pdb）
  CCA_WORK  生成数据目录（默认 evidence/country-core-audit/data；procs.tsv 约 54MB，勿提交）
  CCA_OBJDUMP  llvm-objdump 路径（默认自动查找 rustup 自带的 llvm-objdump，再退到 PATH）
"""
import bisect, glob, os, re, shutil, subprocess, sys
from pathlib import Path

TG = Path(__file__).resolve().parents[2]            # reverse-engineering/tiny-glade
REPO = TG.parents[1]                                 # games_test
RECON = TG / 'reconstruction'
CC_SRC = RECON / 'country-core' / 'src'
CC_WP = RECON / 'country-core-render'
EXE = Path(os.environ.get('CCA_EXE', REPO / 'tiny-glade.exe'))
PDB = Path(os.environ.get('CCA_PDB', REPO / 'tiny_glade.pdb'))
WORK = Path(os.environ.get('CCA_WORK', TG / 'evidence' / 'country-core-audit' / 'data'))
OUT = WORK.parent
PROCS_TSV = WORK / 'procs.tsv'
CALLS_JSON = WORK / 'scope_calls.json'
BASE = 0x140000000


def objdump():
    p = os.environ.get('CCA_OBJDUMP')
    if p:
        return p
    g = glob.glob(os.path.expanduser('~/.rustup/toolchains/*/lib/rustlib/*/bin/llvm-objdump'))
    return g[0] if g else (shutil.which('llvm-objdump') or sys.exit('找不到 llvm-objdump，请设置 CCA_OBJDUMP'))


def load_procs(prefix=None):
    """{addr: (size, name)}；首次使用前需运行 pdbprocs.py 生成 procs.tsv。"""
    if not PROCS_TSV.exists():
        sys.exit(f'缺少 {PROCS_TSV}，请先运行: python3 -I pdbprocs.py')
    procs = {}
    for line in open(PROCS_TSV, encoding='utf8'):
        a, sz, _mi, _obj, nm = line.rstrip('\n').split('\t', 4)
        if prefix is None or nm.startswith(prefix):
            procs.setdefault(int(a, 16), (int(sz), nm))
    return procs


class Owner:
    """地址 -> 所在 PDB 过程起始地址（含函数内部地址）。"""
    def __init__(self, procs):
        self.procs = procs
        self.starts = sorted(procs)

    def __call__(self, a):
        i = bisect.bisect_right(self.starts, a) - 1
        if i >= 0 and a < self.starts[i] + self.procs[self.starts[i]][0]:
            return self.starts[i]
        return None


ADDR_RE = re.compile(r'0x(?:0x)?(14[0-9a-fA-F]{7})\b')
