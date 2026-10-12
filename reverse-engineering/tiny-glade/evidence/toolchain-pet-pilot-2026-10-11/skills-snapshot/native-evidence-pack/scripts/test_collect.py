#!/usr/bin/env python3
"""Contract/failure tests using tiny file-backed PE and fake CLI tools.
These test the pack mechanics, not target business logic or LLVM decoding.
"""
import json
from pathlib import Path
import struct
import tempfile
import unittest
import collect

FAKE_TOOL = '''#!/usr/bin/env python3
import json,sys
from pathlib import Path
args=sys.argv[1:]
if '--version' in args:
 print('Fixture LLVM tool 1.0');sys.exit(0)
if '--disassemble' in args:
 start=int(next(a.split('=')[1] for a in args if a.startswith('--start-address=')),0)
 stop=int(next(a.split('=')[1] for a in args if a.startswith('--stop-address=')),0)
 for address in range(start,stop):
  print(f'{address:x}: 90     nop')
else:
 rows=[dict(Address=line.strip(),Symbol=[dict(FileName='fixture.rs',FunctionName='fixture::owner',Line=1,Column=0)]) for line in sys.stdin if line.strip()]
 print(json.dumps(rows))
'''

class Contracts(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        data = bytearray(1024)
        data[:2] = b'MZ';struct.pack_into('<I', data, 60, 0x80)
        data[0x80:0x84] = b'PE\0\0'
        struct.pack_into('<HH',data,0x84,0x8664,1)
        struct.pack_into('<H',data,0x94,240)
        opt=0x98;struct.pack_into('<H',data,opt,0x20b);struct.pack_into('<Q',data,opt+24,0x10000000)
        section=opt+240;data[section:section+5]=b'.text'
        struct.pack_into('<IIII',data,section+8,64,0x1000,64,512)
        struct.pack_into('<I',data,section+36,0x60000020)
        data[512:576]=bytes([0x90])*64
        self.binary=self.root/'fixture.exe';self.binary.write_bytes(data)
        self.tool=self.root/'fake-llvm';self.tool.write_text(FAKE_TOOL);self.tool.chmod(0o755)
        self.value=dict(schema_version=1,artifact=dict(id='fixture',version='unit',path=str(self.binary),sha256=collect.digest(self.binary)),
            tools=dict(symbolizer=str(self.tool),objdump=str(self.tool)),
            requests=[dict(id='one',rva='0x1000',length=4),dict(id='two',rva='0x1008',length=4)])
        self.manifest=self.root/'manifest.json';self.cache=self.root/'cache'

    def tearDown(self):
        self.temp.cleanup()

    def run_pack(self,name):
        self.manifest.write_text(json.dumps(self.value))
        return collect.collect(self.manifest,self.root/name,self.cache,timeout=2)

    def test_cold_warm_batch_reuse(self):
        cold=self.run_pack('cold');warm=self.run_pack('warm')
        self.assertEqual(cold['status'],'ok');self.assertEqual(warm['status'],'ok')
        self.assertEqual(cold['stats']['analysis_processes'],3)
        self.assertEqual(warm['stats']['analysis_processes'],0)
        self.assertEqual(warm['stats']['cache_hits'],2)
        self.assertEqual(warm['stats']['version_processes'],2)
        a=json.loads((self.root/'cold/one.json').read_text());b=json.loads((self.root/'warm/one.json').read_text())
        self.assertEqual(a,b)

    def test_malformed_manifest_retains_failure_report(self):
        self.value = []
        report = self.run_pack('malformed')
        self.assertEqual(report['status'], 'error')
        self.assertIn('JSON object', report['error'])
        self.assertEqual(report['stats']['tool_processes'], 0)

    def test_duplicate_request_one_analysis(self):
        self.value['requests'][1].update(rva='0x1000',length=4)
        report=self.run_pack('duplicate')
        self.assertEqual(report['status'],'ok');self.assertEqual(report['stats']['unique_requests'],1)
        self.assertEqual(report['stats']['analysis_processes'],2);self.assertEqual(report['stats']['cache_hits'],1)

    def test_identity_mismatch_stops_before_tools(self):
        self.value['artifact']['sha256']='0'*64
        report=self.run_pack('bad-sha')
        self.assertEqual(report['status'],'error');self.assertIn('SHA-256 mismatch',report['error'])
        self.assertEqual(report['stats']['tool_processes'],0)

    def test_virtual_or_unmapped_range_stops_before_tools(self):
        self.value['requests'][0]['rva']='0x2000'
        report=self.run_pack('bad-range')
        self.assertEqual(report['status'],'error');self.assertEqual(report['stats']['tool_processes'],0)

    def test_corrupt_cache_is_not_silently_replaced(self):
        first=self.run_pack('first');self.assertEqual(first['status'],'ok')
        key=first['requests'][0]['cache_key'];path=self.cache/key[:2]/(key+'.json')
        envelope=json.loads(path.read_text());envelope['payload']['instructions'][0]['bytes']='cc'
        path.write_text(json.dumps(envelope))
        report=self.run_pack('corrupt')
        self.assertEqual(report['status'],'error');self.assertIn('Cache integrity',report['error'])
        self.assertEqual(report['stats']['analysis_processes'],0)

    def test_unknown_symbols_not_false_success(self):
        self.tool.write_text(FAKE_TOOL.replace("FileName='fixture.rs',FunctionName='fixture::owner',Line=1", "FileName='',FunctionName='??',Line=0"))
        report=self.run_pack('unknown')
        self.assertEqual(report['status'],'partial')
        self.assertEqual(report['requests'][0]['status'],'missing_symbols')
        warm=self.run_pack('unknown-warm')
        self.assertEqual(warm['status'],'partial');self.assertEqual(warm['stats']['analysis_processes'],0)

    def test_assembly_byte_mismatch_stops_before_symbols(self):
        self.tool.write_text(FAKE_TOOL.replace('90     nop','cc     int3'))
        report=self.run_pack('wrong-bytes')
        self.assertEqual(report['status'],'error');self.assertIn('bytes do not match',report['error'])
        self.assertEqual(report['stats']['analysis_processes'],1)
        self.assertFalse(self.cache.exists())

if __name__=='__main__':
    unittest.main(verbosity=2)
