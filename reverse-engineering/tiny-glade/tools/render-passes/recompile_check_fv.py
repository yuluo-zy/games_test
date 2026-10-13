#!/usr/bin/env python3
"""F-V wrapper around recompile_check.py that tolerates push-constant bindings.

The stock spirv_parse.structural_fingerprint sorts bindings tuples containing
set/binding=None (PushConstant resources have no DescriptorSet/Binding) and
raises TypeError. This wrapper monkeypatches a None-safe fingerprint (using
parse_module's own substitution rule: None -> 0) then runs recompile_check.main().
For modules without push constants the fingerprint is byte-identical to stock.

Usage: same as recompile_check.py
"""
import sys
from pathlib import Path

TOOLS = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOLS))

import spirv_parse


def _safe_fp(module):
    bindings = sorted(
        (b['set'] if b['set'] is not None else 0,
         b['binding'] if b['binding'] is not None else 0,
         b['storage'], b['name'], b['kind'])
        for b in module['bindings'])
    return {
        'entries': sorted((e['stage'], e['name']) for e in module['entries']),
        'local_size': spirv_parse.local_size(module['execution_modes']),
        'capabilities': sorted(module['capabilities']),
        'bindings': bindings,
        'opcode_histogram': dict(sorted(
            module['opcode_histogram'].items(), key=lambda kv: kv[0])),
    }


spirv_parse.structural_fingerprint = _safe_fp

import recompile_check  # noqa: E402

if __name__ == '__main__':
    recompile_check.main()
