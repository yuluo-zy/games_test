"""原发布 SPIR-V 调试源记录审计；只读取发行文件，不作为恢复交付。"""
from pathlib import Path
import collections
import hashlib
import json
import struct

ROOT = Path(__file__).resolve().parents[2]
BIN_DIR = ROOT.parents[1] / 'ljxsj_92385/Tiny Glade/compiled-shaders'
CATALOG = ROOT / 'evidence/render-passes/validation/compile-commands.json'
OUTPUT = Path(__file__).with_name('original-debug-payloads.json')


def main():
    modules = []
    generators = collections.Counter()
    source_records = 0
    lf_only = 0
    non_lf = []
    for file in json.loads(CATALOG.read_text(encoding='utf-8'))['compile_commands']:
        data = (BIN_DIR / file['file']).read_bytes()
        for item in file['modules']:
            start = item['offset']
            count = struct.unpack_from('<I', data, start - 4)[0]
            payload = data[start:start + count * 4]
            words = struct.unpack('<' + 'I' * count, payload)
            assert words[0] == 0x07230203
            generators[words[2]] += 1
            row = {'file': file['file'], 'entry': item['entry'],
                   'generator_word': words[2], 'tool_id': words[2] >> 16,
                   'version_field': words[2] & 0xffff,
                   'module_sha256': hashlib.sha256(payload).hexdigest(),
                   'sources': []}
            position = 5
            while position < count:
                size, opcode = words[position] >> 16, words[position] & 0xffff
                assert size > 0 and position + size <= count
                if opcode in (2, 3):
                    skip = 4 if opcode == 3 else 1
                    raw = struct.pack('<' + 'I' * (size - skip),
                                      *words[position + skip:position + size]).split(b'\0')[0]
                    chars = sorted(set(raw))
                    record = {'opcode': opcode, 'length': len(raw),
                              'lf_count': raw.count(b'\n'), 'byte_values': chars,
                              'only_lf': all(value == 10 for value in raw)}
                    row['sources'].append(record)
                    source_records += 1
                    lf_only += record['only_lf']
                    if not record['only_lf']:
                        non_lf.append({'file': file['file'], 'entry': item['entry'],
                                       'offset': position * 4, **record})
                position += size
            modules.append(row)
    summary = {'modules': len(modules), 'source_records': source_records,
               'lf_only_records': lf_only, 'non_lf_records': len(non_lf),
               'generator_words': dict(generators)}
    OUTPUT.write_text(json.dumps({'summary': summary, 'non_lf': non_lf,
                                  'modules': modules}, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(summary))


if __name__ == '__main__':
    main()
