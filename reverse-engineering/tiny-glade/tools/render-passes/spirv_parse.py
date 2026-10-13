#!/usr/bin/env python3
"""SPIR-V module parsing shared by the Tiny Glade shader toolchain.

This module is the importable extraction of the SPIR-V word-level parsing used
by `scripts/render-passes/parse_shader_catalog.py` (parse_module). It keeps the
same tolerant, reflection-oriented parsing so the recompile validator and the
catalog generator describe a module identically.

Loaded grammar:
    skeleton/_tools/spirv.core.grammar.json  (opcode names + operand-kind enums)

Parsed per module (matching the catalog schema):
    entries, execution_modes, capabilities, bindings (set/binding/storage/name/
    kind), debug_names, member_names, opcode_histogram.

A convenience `iter_modules` scans a blob for concatenated SPIR-V modules (the
game's .bin files hold 1 module for compute and 2 for raster: vertex+fragment).
"""

import collections
import hashlib
import json
import re
import struct
from pathlib import Path

SPIRV_MAGIC = b'\x03\x02\x23\x07'

OP_ENTRY_POINT = 15
OP_EXEC_MODE = 16
OP_CAPABILITY = 17
OP_NAME = 5
OP_MEMBER_NAME = 6
OP_DECORATE = 71
OP_MEMBER_DECORATE = 72
OP_VARIABLE = 59
OP_TYPE_POINTER = 32
OP_TYPE_IMAGE = 25
OP_TYPE_SAMPLER = 26
OP_TYPE_SAMPLED_IMAGE = 27
OP_TYPE_STRUCT = 30
OP_TYPE_ACCEL = 5349  # OpTypeAccelerationStructureKHR (SPIR-V 1.4)

EXECUTION_MODE_LOCAL_SIZE = 17


class Grammar:
    """SPIR-V core grammar: opcode names and operand-kind enumerants."""

    def __init__(self, path=None):
        self._ops = {}
        self._enums = {}
        if path and Path(path).exists():
            g = json.loads(Path(path).read_text())
            self._ops = {i['opcode']: i['opname'] for i in g['instructions']}
            for k in g['operand_kinds']:
                self._enums[k['kind']] = {
                    (int(v['value'], 0) if isinstance(v['value'], str) else v['value']): v['enumerant']
                    for v in k.get('enumerants', [])
                }

    def op(self, opcode):
        return self._ops.get(opcode, str(opcode))

    def enum(self, kind, value):
        return self._enums.get(kind, {}).get(value, str(value))


_GRAMMAR = Grammar()


def set_grammar(path):
    global _GRAMMAR
    _GRAMMAR = Grammar(path)
    return _GRAMMAR


def _words_to_bytes(ws):
    return struct.pack('<' + 'I' * len(ws), *ws)


def _words_to_string(ws):
    raw = _words_to_bytes(ws)
    return raw.split(b'\x00')[0].decode('utf-8', errors='replace')


def parse_module(payload, grammar=None):
    """Parse a single SPIR-V module payload into a reflection dict.

    Returns None if the payload does not start with the SPIR-V magic number.
    Mirrors parse_shader_catalog.parse_module.
    """
    g = grammar or _GRAMMAR
    n = len(payload) // 4
    w = struct.unpack('<' + 'I' * n, payload)
    if w[0] != 0x07230203:
        return None
    names = {}
    member_names = {}
    deco = collections.defaultdict(list)
    variables = {}
    pointed = {}
    image_info = {}
    sampler_types = set()
    sampledimage_types = set()
    struct_types = set()
    accel_types = set()
    entries = []
    modes = []
    caps = []
    opcount = collections.Counter()

    i = 5  # skip magic, version, generator, bound, schema
    while i < n:
        wc = w[i] >> 16
        op = w[i] & 0xffff
        if wc == 0 or i + wc > n:
            break
        a = w[i + 1:i + wc]
        opcount[g.op(op)] += 1
        if op == OP_NAME:
            if len(a) >= 2:
                names[a[0]] = _words_to_string(a[1:])
        elif op == OP_MEMBER_NAME:
            if len(a) >= 3:
                member_names[(a[0], a[1])] = _words_to_string(a[2:])
        elif op == OP_DECORATE:
            if len(a) >= 2:
                deco[a[0]].append({'kind': g.enum('Decoration', a[1]), 'args': list(a[2:])})
        elif op == OP_MEMBER_DECORATE:
            pass
        elif op == OP_VARIABLE:
            # operands: <id> result type, <id> result, Storage Class
            if len(a) >= 3:
                variables[a[1]] = {'type': a[0], 'storage': g.enum('StorageClass', a[2])}
        elif op == OP_ENTRY_POINT:
            # operands: Execution Model, <id> Entry Point, Name, ... interface ids
            if len(a) >= 3:
                entries.append({'stage': g.enum('ExecutionModel', a[0]),
                                'name': _words_to_string(a[2:])})
        elif op == OP_EXEC_MODE:
            # operands: <id> Entry Point, Execution Mode, ... literals
            if len(a) >= 2:
                modes.append({'mode': g.enum('ExecutionMode', a[1]), 'args': list(a[2:])})
        elif op == OP_CAPABILITY:
            if a:
                caps.append(g.enum('Capability', a[0]))
        elif op == OP_TYPE_POINTER:
            # operands: <id> Result, Storage Class, Type
            if len(a) >= 3:
                pointed[a[0]] = a[2]
        elif op == OP_TYPE_IMAGE:
            # operands: <id> Result, Sampled Type, Dim, Depth, Arrayed, MS, Sampled, Image Format[, Access]
            if len(a) >= 8:
                image_info[a[0]] = {
                    'dim': g.enum('Dim', a[2]),
                    'depth': a[3],
                    'arrayed': a[4],
                    'ms': a[5],
                    'sampled': a[6],
                    'format': g.enum('ImageFormat', a[7]),
                }
        elif op == OP_TYPE_SAMPLER:
            if a:
                sampler_types.add(a[0])
        elif op == OP_TYPE_SAMPLED_IMAGE:
            if a:
                sampledimage_types.add(a[0])
        elif op == OP_TYPE_STRUCT:
            if a:
                struct_types.add(a[0])
        elif op == OP_TYPE_ACCEL:
            if a:
                accel_types.add(a[0])
        i += wc

    def infer_kind(var_type_id, storage):
        if storage == 'PushConstant':
            return 'PushConstant'
        t = pointed.get(var_type_id)
        if t in image_info:
            img = image_info[t]
            if img['sampled'] == 2:
                return 'StorageImage(%s)' % img['dim']
            if img['sampled'] == 1:
                return 'SampledImage(%s)' % img['dim']
            return 'Image(sampled=runtime)'
        if t in sampledimage_types:
            return 'SampledImage'
        if t in sampler_types:
            return 'Sampler'
        if t in accel_types:
            return 'AccelerationStructure'
        if storage == 'UniformConstant':
            if t in struct_types:
                return 'UniformConstant(struct)'
            return 'UniformConstant(other)'
        if storage == 'Uniform':
            if t in struct_types:
                d = [x['kind'] for x in deco.get(t, [])]
                if 'Block' in d:
                    return 'UniformBuffer(UBO)'
                if 'BufferBlock' in d:
                    return 'StorageBuffer(SSBO,BufferBlock)'
                return 'Uniform-struct(no Block decor)'
            return 'Uniform(non-struct)'
        if storage == 'StorageBuffer':
            return 'StorageBuffer(SSBO)'
        return storage

    bindings = []
    for vid, info in variables.items():
        dset = None
        bind = None
        for d in deco.get(vid, []):
            if d['kind'] == 'DescriptorSet':
                dset = d['args'][0] if d['args'] else None
            elif d['kind'] == 'Binding':
                bind = d['args'][0] if d['args'] else None
        is_push = info['storage'] == 'PushConstant'
        if dset is not None or bind is not None or is_push:
            bindings.append({
                'name': names.get(vid),
                'storage': info['storage'],
                'set': dset,
                'binding': bind,
                'kind': infer_kind(info['type'], info['storage']),
            })

    bindings.sort(key=lambda b: (b['set'] if b['set'] is not None else 0,
                                 b['binding'] if b['binding'] is not None else 0))
    return {
        'entries': entries,
        'execution_modes': modes,
        'capabilities': caps,
        'bindings': bindings,
        'debug_names': [v for v in names.values()],
        'member_names': [{'struct_id': k[0], 'member': k[1], 'name': v}
                         for k, v in sorted(member_names.items())],
        'opcode_histogram': dict(opcount),
    }


def iter_modules(blob):
    """Yield (offset, payload) for each SPIR-V module in a blob.

    Handles both container layouts seen here:
      * game `.bin` files: a 4-byte module word-count precedes the SPIR-V magic
        (a small container header precedes the first module),
      * raw `.spv` / `-Fo` output: the SPIR-V magic sits at offset 0 with no
        length prefix, and the module runs to the end of the blob.
    """
    cursor = 0
    while True:
        at = blob.find(SPIRV_MAGIC, cursor)
        if at < 0:
            return
        if at >= 4:
            n = struct.unpack_from('<I', blob, at - 4)[0]
            payload = blob[at:at + 4 * n]
            if n and len(payload) == 4 * n and at + 4 * n <= len(blob):
                yield at, payload
                cursor = at + 4 * n
                continue
        # raw module (magic at 0, or length prefix invalid): to end of blob
        yield at, blob[at:]
        return


def module_sha256(payload):
    return hashlib.sha256(payload).hexdigest()


# ------------------------------------------------------------------------
# Path-normalized canonicalization.
#
# dxc `-Zi -Qembed_debug` stores strings as wordcount-prefixed SPIR-V
# instructions (OpSourceContinued [file id, path], OpString, and the opcode-330
# dxc-commit-hash / dxc-cl-option strings). Recompiling the same source from a
# different path changes the string CONTENT and therefore the instruction
# wordcounts, so byte-level string replacement breaks the length prefixes.
# canonicalize_module() rebuilds the module instruction-by-instruction and
# re-encodes normalized path strings with corrected wordcounts, yielding bytes
# that are equal for equal source/equal dxc/equal flags regardless of paths.
# ------------------------------------------------------------------------
_OP_SOURCE_CONTINUED = 7
_OP_STRING = 37
_OP_DXC_DEBUG_STRING = 330
_TMP_RE = re.compile(r'/tmp/\.tmp[A-Za-z0-9]+')
_HLSL_PATH_RE = re.compile(r'[A-Za-z0-9_./\\\-]*\.hlsl')


def _words_to_string_lenient(words):
    raw = _words_to_bytes(words)
    return raw.split(b'\x00')[0].decode('utf-8', errors='replace')


def _string_to_words(text):
    raw = text.encode('utf-8') + b'\x00'
    raw += b'\x00' * ((-len(raw)) % 4)
    return list(struct.unpack('<' + 'I' * (len(raw) // 4), raw))


def _normalize_path_string(text):
    return _TMP_RE.sub('<TMP>', _HLSL_PATH_RE.sub('<HLSLPATH>', text))


def canonicalize_module(payload):
    """Re-encode a module with path/temp/command strings normalized and the
    instruction wordcounts recomputed (see module docstring above)."""
    words = list(struct.unpack('<' + 'I' * (len(payload) // 4), payload))
    n = len(words)
    if n == 0 or words[0] != 0x07230203:
        return payload
    out = words[:5]  # header: magic, version, generator, bound, schema
    i = 5
    while i < n:
        wc = words[i] >> 16
        op = words[i] & 0xffff
        if wc == 0 or i + wc > n:
            break
        a = words[i + 1:i + wc]
        operands = a
        if op == _OP_SOURCE_CONTINUED:
            # [file id, path string] -> keep id, normalize path
            if a:
                operands = [a[0]] + _string_to_words(_normalize_path_string(_words_to_string_lenient(a[1:])))
        elif op == _OP_STRING:
            # [result id, string] -> keep id, normalize path-like strings
            if a:
                operands = [a[0]] + _string_to_words(_normalize_path_string(_words_to_string_lenient(a[1:])))
        elif op == _OP_DXC_DEBUG_STRING:
            s = _words_to_string_lenient(a)
            if s.startswith('dxc-cl-option'):
                operands = _string_to_words('<DXCCMD>')
            elif s.startswith('dxc-commit-hash'):
                operands = _string_to_words('<DXCOMMIT>')
            else:
                operands = _string_to_words(_normalize_path_string(s))
        # OpName/OpMemberName/OpEntryPoint/OpSource keep their content: names are
        # part of the semantic identity and OpSource text is not path-dependent.
        out.append(((1 + len(operands)) << 16) | op)
        out.extend(operands)
        i += wc
    return struct.pack('<' + 'I' * len(out), *out)


def local_size(execution_modes):
    """Return the LocalSize [x,y,z] from a module's execution modes, else None."""
    for m in execution_modes:
        if m['mode'] == 'LocalSize':
            return m['args'][:3]
    return None


def structural_fingerprint(module):
    """A stable, comparable fingerprint of a parsed module.

    Fields compared by the validator: entry point + stage, LocalSize, capability
    set, descriptor bindings (set/binding/storage class/name), opcode histogram.
    """
    return {
        'entries': sorted((e['stage'], e['name']) for e in module['entries']),
        'local_size': local_size(module['execution_modes']),
        'capabilities': sorted(module['capabilities']),
        'bindings': sorted(
            ((b['set'], b['binding'], b['storage'], b['name'], b['kind'])
             for b in module['bindings']),
            # Push-constant bindings carry set/binding == None; sort them first
            # so mixed None/int tuples compare deterministically.
            key=lambda t: (t[0] if t[0] is not None else -1,
                           t[1] if t[1] is not None else -1,
                           t[2], t[3], t[4])),
        'opcode_histogram': dict(sorted(
            module['opcode_histogram'].items(), key=lambda kv: kv[0])),
    }
