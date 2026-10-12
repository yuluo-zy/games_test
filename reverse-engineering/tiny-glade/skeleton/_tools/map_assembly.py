#!/usr/bin/env python3
"""Bind existing roof skeleton functions to original PE/PDB assembly evidence.

Only metadata/assembly is generated. No game, dependency or ABI implementation.
"""
import argparse
from collections import Counter
import gzip
import hashlib
import json
import re
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / '_assembly' / 'roof-v1'
PREFIX = 'crates/systems/roof/src/'
COLLECTOR = Path('/Users/yuluo/.codex/skills/native-evidence-pack/scripts/collect.py')
BEGIN = '// === ORIGINAL ASSEMBLY V1 BEGIN ==='
END = '// === ORIGINAL ASSEMBLY V1 END ==='


def read(path):
    with (gzip.open(path, 'rt') if str(path).endswith('.gz') else open(path)) as f:
        return json.load(f)


def sha(path):
    h = hashlib.sha256()
    with open(path, 'rb') as f:
        for b in iter(lambda: f.read(1024 * 1024), b''):
            h.update(b)
    return h.hexdigest()


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    text = json.dumps(value, ensure_ascii=False, indent=2) + '\n'
    tmp = path.with_name(path.name + '.tmp')
    if str(path).endswith('.gz'):
        with gzip.open(tmp, 'wt') as f:
            f.write(text)
    else:
        tmp.write_text(text)
    tmp.replace(path)


def prepare():
    baseline = read(ROOT.parent / 'tracking/baselines.json')['baselines']
    for version in 'ABC':
        index_path = ROOT / '_pdb' / version / 'enrichment/procedure-index.json.gz'
        procs = read(index_path)
        byid = {p['procedure_id']: p for p in procs}
        selected = {p['procedure_id']: ['compiled_source_mapping'] for p in procs
                    if any(f.startswith(PREFIX) for f in p['source_files'])}
        source_files = sorted((ROOT / version / PREFIX).rglob('*.rs.enriched.json.gz'))
        bindings = {}
        for path in source_files:
            bindings[str(path.relative_to(ROOT))] = sha(path)
            for function in read(path)['functions']:
                for host in function.get('inline_hosts', []):
                    pid = host['host_procedure_id']
                    if pid not in byid:
                        raise RuntimeError(f'Unresolved inline host: {version} {pid}')
                    selected.setdefault(pid, []).append('inline_host')
        ranges = {}
        for pid, reasons in selected.items():
            p = byid[pid]
            if not 0 < p['code_size'] <= 65536:
                raise RuntimeError(f'Out-of-contract PDB range: {version} {pid}')
            key = (p['rva'], p['code_size'])
            rec = ranges.setdefault(key, dict(id=f'rva_{key[0]:x}_len_{key[1]:x}',
                                               rva=key[0], length=key[1], procedures=[]))
            rec['procedures'].append({k: p.get(k) for k in
                                     ('procedure_id', 'name', 'module', 'record_offset',
                                      'source_files', 'type_index', 'mapping_status')})
            rec['procedures'][-1]['selection_reasons'] = sorted(set(reasons))
        # Conservative byte sum bounds full-instruction symbolization under 32768 points.
        packs, current, size = [], [], 0
        for rec in sorted(ranges.values(), key=lambda x: x['rva']):
            if current and size + rec['length'] > 30000:
                packs.append(current)
                current, size = [], 0
            current.append(rec)
            size += rec['length']
        if current:
            packs.append(current)
        b = baseline[version]
        artifact = dict(id='tiny-glade-original', version=version,
                        path=b['binary']['path'], sha256=b['binary']['sha256'],
                        debug=dict(path=b['pdb']['path'], sha256=b['pdb']['sha256']))
        for n, pack in enumerate(packs):
            for rec in pack:
                rec['pack'] = n
            write(OUT / version / f'manifest-{n:03}.json', dict(
                schema_version=1, artifact=artifact,
                tools=dict(symbolizer='/opt/homebrew/opt/llvm/bin/llvm-symbolizer',
                           objdump='/opt/homebrew/opt/llvm/bin/llvm-objdump'),
                requests=[dict(id=r['id'], rva=r['rva'], length=r['length'],
                               allow_missing_symbols=True) for r in pack]))
        write(OUT / version / 'plan.json', dict(
            schema_version=1, version=version, scope=PREFIX,
            index_path=str(index_path.relative_to(ROOT)), index_sha256=sha(index_path),
            source_sidecar_sha256=bindings, artifact=artifact,
            ranges=list(ranges.values()), packs=len(packs),
            target_executed=False, parameter_register_mapping='unknown'))
        print(json.dumps(dict(version=version, procedure_records=len(selected),
                              ranges=len(ranges), packs=len(packs), bytes=sum(k[1] for k in ranges))))


def collect(version):
    plan = read(OUT / version / 'plan.json')
    for n in range(plan['packs']):
        manifest = OUT / version / f'manifest-{n:03}.json'
        output = OUT / version / f'pack-{n:03}'
        result = subprocess.run([sys.executable, str(COLLECTOR), str(manifest),
                                 '--output', str(output), '--cache', str(OUT / 'cache')],
                                text=True, capture_output=True)
        report = read(output / 'report.json')
        print(json.dumps(dict(version=version, pack=n, status=report['status'],
                              stats=report.get('stats'), error=report.get('error'))), flush=True)
        if result.returncode or report['status'] != 'ok':
            raise RuntimeError(result.stdout + result.stderr)


def rel(path):
    return str(path.relative_to(ROOT))


def without_block(text):
    if BEGIN in text:
        if text.count(BEGIN) != 1 or text.count(END) != 1:
            raise RuntimeError('Malformed assembly section')
        head, rest = text.split(BEGIN, 1)
        suffix = rest.split(END, 1)[1]
        # New blocks include one terminal newline; old pre-release blocks did not.
        if suffix.startswith('\n'):
            suffix = suffix[1:]
        return head + suffix
    return text


def normalized_name(name):
    # Only the terminal Rust symbol hash is ignored. Do not guess template names.
    return re.sub(r'::h[0-9a-f]{16}$', '', name)


def source_path(path):
    path = path.replace('\\', '/')
    if '/country-slice-private/' in path:
        return path.split('/country-slice-private/', 1)[1]
    return path


def spans(instructions):
    result = []
    for i in instructions:
        start = int(i['rva'], 16)
        end = start + i['size']
        if result and result[-1]['end_rva_exclusive'] == start:
            result[-1]['end_rva_exclusive'] = end
            result[-1]['instructions'] += 1
        else:
            result.append(dict(start_rva=start, end_rva_exclusive=end, instructions=1))
    return result


def check_inputs(version, plan):
    for path, expected in {plan['index_path']: plan['index_sha256'],
                           **plan['source_sidecar_sha256']}.items():
        if sha(ROOT / path) != expected:
            raise RuntimeError(f'Metadata changed since prepare: {version} {path}')


def merge():
    overall = dict(schema_version=1, scope='Roof source functions and their inline hosts; A/B/C',
                   target_executed=False, versions={}, preserved_source_sha256={},
                   evidence_sha256={}, unknowns=[
                       'original parameter names and parameter-to-register/stack ABI',
                       'original source text, variable names and missing type layouts',
                       'complete reachable CFG and runtime indirect call destinations'])
    for version in 'ABC':
        plan = read(OUT / version / 'plan.json')
        check_inputs(version, plan)
        aliases = read(ROOT / '_pdb' / version / 'enrichment/address-aliases.json.gz')
        ranges, bypid, rows = {}, {}, []
        aggregate = dict(ranges=0, instructions=0, inline_frame_instruction_starts=0,
                         unknown_source_instruction_starts=0, instruction_overhang_bytes=0,
                         files=0, source_functions=0, compiled_function_instances=0,
                         mapped_source_functions=0, unmapped_source_functions=0,
                         inline_instances=0, inline_instances_with_exact_frame_match=0,
                         inline_instances_without_exact_frame_match=0,
                         repeated_inline_sites_with_unknown_site_specific_spans=0,
                         analysis_processes=0, version_processes=0,
                         evidence_output_bytes=0, cache_hits=0, cache_misses=0)
        for n in range(plan['packs']):
            report_path = OUT / version / f'pack-{n:03}/report.json'
            report = read(report_path)
            if report['status'] != 'ok':
                raise RuntimeError(f'Failed collection: {report_path}')
            for key in ('analysis_processes', 'version_processes', 'evidence_output_bytes',
                        'cache_hits', 'cache_misses'):
                aggregate[key] += report['stats'][key]
            overall['evidence_sha256'][rel(report_path)] = sha(report_path)
        for rec in plan['ranges']:
            evidence = OUT / version / f"pack-{rec['pack']:03}" / (rec['id'] + '.json')
            payload = read(evidence)
            if payload['status'] != 'ok' or payload['range']['length'] != rec['length']:
                raise RuntimeError(f'Wrong evidence: {evidence}')
            artifact = payload['artifact']
            if artifact['sha256'] != plan['artifact']['sha256'] or artifact['debug']['sha256'] != plan['artifact']['debug']['sha256']:
                raise RuntimeError(f'Artifact mismatch: {evidence}')
            symbols = {int(s['Address'], 16): s['Symbol'] for s in payload['symbolization']}
            if set(symbols) != {int(i['rva'], 16) for i in payload['instructions']}:
                raise RuntimeError('Not every instruction start was symbolized')
            rec.update(evidence_path=rel(evidence), assembly_path=rel(evidence.with_suffix('.asm.txt')),
                       instruction_count=len(payload['instructions']),
                       instruction_overhang_bytes=payload['range']['instruction_overhang_bytes'],
                       same_address_aliases=aliases.get(str(rec['rva']), []),
                       function_boundary_evidence='PDB procedure range; complete CFG unknown')
            rec['payload'] = payload
            rec['symbols'] = symbols
            ranges[rec['id']] = rec
            for p in rec['procedures']:
                bypid[p['procedure_id']] = rec
            aggregate['ranges'] += 1
            aggregate['instructions'] += len(payload['instructions'])
            aggregate['unknown_source_instruction_starts'] += len(payload['missing_symbol_addresses'])
            aggregate['inline_frame_instruction_starts'] += sum(len(f) > 1 for f in symbols.values())
            aggregate['instruction_overhang_bytes'] += rec['instruction_overhang_bytes']
            overall['evidence_sha256'][rel(evidence)] = sha(evidence)
            overall['evidence_sha256'][rec['assembly_path']] = sha(ROOT / rec['assembly_path'])
        for metadata_rel in plan['source_sidecar_sha256']:
            source = ROOT / metadata_rel.removesuffix('.enriched.json.gz')
            data = read(ROOT / metadata_rel)
            file_ranges, functions = {}, []
            for f in data['functions']:
                fn = dict(id=f['id'], name=f['name'],
                          candidate_interface_ids=f.get('candidate_interface_ids', []),
                          parameter_register_mapping='unknown',
                          reference_callers=f.get('reference_callers', []),
                          compiled_instances=[], inline_instances=[])
                host_counts = Counter(h['host_procedure_id'] for h in f.get('inline_hosts', []))
                for instance in f.get('compiled_instances', []):
                    r = bypid.get(instance['procedure_id'])
                    if not r:
                        raise RuntimeError(f'Missing compiled range: {source} {instance}')
                    fn['compiled_instances'].append(dict(**instance, range_id=r['id'],
                        evidence_path=r['evidence_path'], assembly_path=r['assembly_path'],
                        mapping_evidence='PDB procedure entry/size and existing source attribution'))
                    file_ranges[r['id']] = r
                    aggregate['compiled_function_instances'] += 1
                for host in f.get('inline_hosts', []):
                    r = bypid[host['host_procedure_id']]
                    matches = [i for i in r['payload']['instructions'] if any(
                        normalized_name(frame.get('FunctionName', '')) == normalized_name(f['name'])
                        for frame in r['symbols'][int(i['rva'], 16)])]
                    fn['inline_instances'].append(dict(**host, range_id=r['id'],
                        evidence_path=r['evidence_path'],
                        sampled_instruction_spans=spans(matches),
                        sampled_instruction_spans_scope='same function name in physical host; shared by repeated inline sites',
                        site_specific_spans_status=('unknown_repeated_same_name_site' if
                            host_counts[host['host_procedure_id']] > 1 else 'unique_function_name_site_in_host'),
                        instruction_range_status=('exact_symbolizer_function_frame' if matches
                                                  else 'unknown_no_exact_frame_match'),
                        is_independent_machine_function=False))
                    file_ranges[r['id']] = r
                    aggregate['inline_instances'] += 1
                    aggregate['inline_instances_with_exact_frame_match' if matches else
                              'inline_instances_without_exact_frame_match'] += 1
                    if host_counts[host['host_procedure_id']] > 1:
                        aggregate['repeated_inline_sites_with_unknown_site_specific_spans'] += 1
                aggregate['mapped_source_functions' if fn['compiled_instances'] or fn['inline_instances']
                          else 'unmapped_source_functions'] += 1
                functions.append(fn)
            frames, frame_ids, rendered_ranges = {}, {}, []
            text = [BEGIN, '// Original PE bytes, AT&T x86-64 assembly; no target execution.',
                    '// PDB procedure bounds are retained; complete reachable CFG is unknown.',
                    '// Candidate parameter types remain in the enrichment section; register/stack mapping: UNKNOWN.',
                    '// Instruction frame chains run from innermost inline frame to outermost host.',
                    f'// Machine-readable mapping: {source.name}.assembly.json.gz', '']
            for fn in functions:
                text.extend([f"// FUNCTION {fn['name']}", f"// Candidate interfaces: {', '.join(fn['candidate_interface_ids']) or 'UNKNOWN'}",
                             f"// Static incoming reference records: {len(fn['reference_callers'])}; complete records in enriched/assembly sidecars."])
                for instance in fn['compiled_instances']:
                    text.append(f"// Compiled procedure {instance['procedure_id']} -> {instance['range_id']}")
                for instance in fn['inline_instances']:
                    text.append(f"// Inline site {instance['module']}:{instance['inline_record_offset']} -> host {instance['host_procedure_id']} / {instance['range_id']}; {instance['instruction_range_status']}")
                    if instance['site_specific_spans_status'] == 'unknown_repeated_same_name_site':
                        text.append('// Site-specific split: UNKNOWN; following spans match this function across all same-name sites in this host.')
                    for s in instance['sampled_instruction_spans']:
                        text.append(f"//   Function-frame RVA span [{s['start_rva']:#x}, {s['end_rva_exclusive']:#x}), {s['instructions']} instructions")
                if not fn['compiled_instances'] and not fn['inline_instances']:
                    text.append('// Machine-code mapping: UNKNOWN (no compiled/inline host record).')
                text.append('//')
            text.append('// SOURCE FRAME CATALOG follows the instruction listing. F ids apply to this file only.')
            for r in sorted(file_ranges.values(), key=lambda r: r['rva']):
                text.extend(['//', f"// RANGE {r['id']}: RVA {r['rva']:#x}, {r['length']} PDB bytes; {r['instruction_count']} instructions",
                             f"// Evidence: {r['evidence_path']}",
                             f"// Same-address PDB aliases: {len(r['same_address_aliases'])}; retain all, no exclusive ownership claim."])
                for p in r['procedures']:
                    text.append(f"// PDB {p['procedure_id']}: {p['name']}")
                instructions, previous = [], None
                for i in r['payload']['instructions']:
                    chain = r['symbols'][int(i['rva'], 16)]
                    ids = []
                    for frame in chain:
                        key = json.dumps(frame, sort_keys=True)
                        if key not in frame_ids:
                            fid = 'F' + str(len(frames) + 1)
                            frame_ids[key] = fid
                            frames[fid] = frame
                        ids.append(frame_ids[key])
                    if ids != previous:
                        text.append('// Frames: ' + ' -> '.join(ids))
                        previous = ids
                    text.append(f"// {i['va']}  {i['bytes']:<30} {i['assembly']}")
                    instructions.append(dict(**i, frame_ids=ids))
                rendered_ranges.append(dict(**{k: val for k, val in r.items()
                                                if k not in ('payload', 'symbols')}, instructions=instructions))
            text.extend(['//', '// SOURCE FRAME CATALOG'])
            for fid, frame in frames.items():
                text.append(f"// {fid}: {source_path(frame.get('FileName', ''))}:{frame.get('Line', 0)}:{frame.get('Column', 0)} {frame.get('FunctionName', 'UNKNOWN')}")
            text.append(END)
            old = source.read_text()
            clean = without_block(old)
            overall['preserved_source_sha256'][rel(source)] = hashlib.sha256(clean.encode()).hexdigest()
            # Keep the original generator marker first for existing refresh tools.
            head, separator, tail = clean.partition('\n')
            source.write_text(head + separator + '\n'.join(text) + '\n' + tail)
            sidecar = source.with_name(source.name + '.assembly.json.gz')
            write(sidecar, dict(schema_version=1, version=version, source=rel(source),
                artifact=plan['artifact'], functions=functions, ranges=rendered_ranges,
                frame_catalog=frames, interface_catalog=data.get('interface_catalog'),
                caller_alias_catalog=data.get('caller_alias_catalog'),
                parameter_register_mapping='unknown', target_executed=False))
            rows.append(dict(source=rel(source), metadata=rel(sidecar),
                             functions=len(functions), ranges=sorted(file_ranges)))
            aggregate['files'] += 1
            aggregate['source_functions'] += len(functions)
        write(OUT / version / 'index.json.gz', dict(version=version, files=rows,
              ranges=[{k: val for k, val in r.items() if k not in ('payload', 'symbols')}
                      for r in ranges.values()], counts=aggregate))
        overall['versions'][version] = aggregate
    write(OUT / 'report.json', overall)
    print(json.dumps(overall['versions'], ensure_ascii=False))


def verify():
    report = read(OUT / 'report.json')
    counts = dict(source_files=0, code_ranges=0, instruction_starts=0,
                  inline_instances=0, original_byte_checks=0)
    for version in 'ABC':
        plan = read(OUT / version / 'plan.json')
        check_inputs(version, plan)
        b = read(ROOT.parent / 'tracking/baselines.json')['baselines'][version]
        if sha(b['binary']['path']) != b['binary']['sha256'] or sha(b['pdb']['path']) != b['pdb']['sha256']:
            raise RuntimeError('Original artifact changed')
        binary = Path(b['binary']['path']).read_bytes()
        index = read(OUT / version / 'index.json.gz')
        for r in index['ranges']:
            payload = read(ROOT / r['evidence_path'])
            for i in payload['instructions']:
                rva = int(i['rva'], 16)
                section = next(s for s in b['pe']['sections'] if s['rva'] <= rva < s['rva'] + s['raw_size'])
                off = section['raw_pointer'] + rva - section['rva']
                raw = bytes.fromhex(i['bytes'])
                if binary[off:off + len(raw)] != raw:
                    raise RuntimeError(f'Original byte mismatch: {version} {i}')
                counts['original_byte_checks'] += 1
            counts['code_ranges'] += 1
            counts['instruction_starts'] += len(payload['instructions'])
        for f in index['files']:
            source = ROOT / f['source']
            content = source.read_text()
            if content.count(BEGIN) != 1 or content.count(END) != 1:
                raise RuntimeError(f'Assembly block missing: {source}')
            if hashlib.sha256(without_block(content).encode()).hexdigest() != report['preserved_source_sha256'][f['source']]:
                raise RuntimeError(f'Pre-existing content changed: {source}')
            sidecar = read(ROOT / f['metadata'])
            if sidecar['parameter_register_mapping'] != 'unknown':
                raise RuntimeError('Unproven ABI claim')
            for r in sidecar['ranges']:
                payload = read(ROOT / r['evidence_path'])
                if [dict((k, val) for k, val in i.items() if k != 'frame_ids') for i in r['instructions']] != payload['instructions']:
                    raise RuntimeError('Source sidecar assembly differs')
                for i, s in zip(r['instructions'], payload['symbolization']):
                    if [sidecar['frame_catalog'][fid] for fid in i['frame_ids']] != s['Symbol']:
                        raise RuntimeError('Frame chain changed')
                    line = f"// {i['va']}  {i['bytes']:<30} {i['assembly']}"
                    if line not in content:
                        raise RuntimeError('Instruction missing from existing source file')
            for fn in sidecar['functions']:
                for instance in fn['inline_instances']:
                    if instance['is_independent_machine_function'] or instance['is_standalone_call']:
                        raise RuntimeError('Inline site incorrectly promoted to independent function')
                    counts['inline_instances'] += 1
            counts['source_files'] += 1
    for path, expected in report['evidence_sha256'].items():
        if sha(ROOT / path) != expected:
            raise RuntimeError(f'Evidence changed: {path}')
    write(OUT / 'verification.json', dict(status='ok', counts=counts,
          existing_source_content_preserved=True, all_instruction_bytes_verified=True,
          target_executed=False, report_sha256=sha(OUT / 'report.json')))
    print(json.dumps(dict(status='ok', counts=counts)))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('action', choices=['prepare', 'collect', 'merge', 'verify'])
    parser.add_argument('--version', choices=list('ABC'))
    args = parser.parse_args()
    if args.action == 'prepare':
        prepare()
    elif args.action == 'collect':
        if not args.version:
            parser.error('collect requires --version')
        collect(args.version)
    elif args.action == 'merge':
        merge()
    else:
        verify()


if __name__ == '__main__':
    main()
