"""Bind each current-workspace verification report to its Rust inputs."""
from pathlib import Path
import hashlib
import re

ROOT = Path(__file__).resolve().parents[2]

def workspace_test_count():
    log = (ROOT / 'evidence/multi-version/workspace-test.log').read_text()
    return sum(int(value) for value in re.findall(r'test result: ok\. (\d+) passed;', log))

def candidate_provenance():
    workspace = ROOT / 'reconstruction'
    paths = [workspace / 'Cargo.toml']
    for folder in ['src', 'crates', 'math', 'roof', 'wall', 'building', 'history', 'tests']:
        paths.extend((workspace / folder).rglob('*.rs'))
    paths.extend((workspace / 'crates').rglob('Cargo.toml'))
    return {
        'candidate_sources': {
            path.relative_to(ROOT).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest()
            for path in sorted(set(paths))
        },
        'candidate_cargo_lock_sha256': hashlib.sha256((workspace / 'Cargo.lock').read_bytes()).hexdigest(),
    }
