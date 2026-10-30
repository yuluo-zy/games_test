"""Failure-oriented tests. Run with the project Python; no running Neo4j required."""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import zipfile

import artifacts
import graph
import api
import transport
from search_index import chunks,CHUNK
from validation import verification_ok

class CodeMapTests(unittest.TestCase):
    def test_transport_connections_are_isolated_between_threads(self):
        import threading
        from concurrent.futures import ThreadPoolExecutor
        barrier=threading.Barrier(2)
        def connection(*args,**kwargs):
            client=unittest.mock.Mock();client.sock=None
            response=unittest.mock.Mock(status=200,will_close=False)
            response.read.return_value=b'{}';client.getresponse.return_value=response
            return client
        def worker():
            config=({},'127.0.0.1',17474,'','thread-test')
            try:
                barrier.wait(timeout=2);transport.request(config,b'{}',12)
                first=transport._state.connection
                barrier.wait(timeout=2);transport.request(config,b'{}',12)
                self.assertIs(first,transport._state.connection)
                barrier.wait(timeout=2)
                return id(first)
            finally:transport.close_connection()
        with patch.object(transport.http.client,'HTTPConnection',side_effect=connection) as factory:
            with ThreadPoolExecutor(max_workers=2) as pool:
                futures=[pool.submit(worker) for _ in range(2)]
                clients=[future.result() for future in futures]
        self.assertEqual(factory.call_count,2);self.assertNotEqual(clients[0],clients[1])

    def test_long_symbol_chunks_preserve_cross_boundary_unicode_queries(self):
        text=''.join(chr(0x10000+i) for i in range(22000))
        pieces=list(chunks(text))
        self.assertTrue(all(len(value.encode())<=32000 for _,value in pieces))
        for start in range(CHUNK-600,CHUNK+600,31):
            needle=text[start:start+512]
            self.assertTrue(any(needle in value for _,value in pieces),start)
        self.assertEqual(pieces[-1][1][-1],text[-1])

    def test_incomplete_search_projection_uses_original_search(self):
        fake=unittest.mock.Mock();fake.current.return_value='s'
        fake.query.side_effect=[[],[{'node':{'key':'expected'}}]]
        fake.read_snapshot=fake.query
        with patch.object(graph,'Graph',return_value=fake):
            result=graph.query_tool('search',{'text':'ray'})
        self.assertIn('db.index.fulltext.queryNodes',fake.query.call_args.args[0])
        self.assertEqual(result['rows'][0]['node']['key'],'expected')
    def test_transport_reuses_connection_and_does_not_replay_failure(self):
        config=({},'127.0.0.1',17474,'','test-auth')
        connection=unittest.mock.Mock();connection.sock=None
        response=unittest.mock.Mock(status=200,will_close=False)
        response.read.return_value=b'{"results":[],"errors":[]}'
        connection.getresponse.return_value=response
        transport.close_connection()
        try:
            with patch.object(transport.http.client,'HTTPConnection',return_value=connection) as factory:
                for _ in range(3):transport.request(config,b'{}',12)
                self.assertEqual(factory.call_count,1)
                self.assertEqual(connection.request.call_count,3)
                connection.request.side_effect=ConnectionResetError('ambiguous commit')
                with self.assertRaises(ConnectionResetError):transport.request(config,b'write',12)
                self.assertEqual(connection.request.call_count,4)
                self.assertIsNone(transport._state.connection)
        finally:transport.close_connection()

    def test_transport_configuration_refreshes_after_same_size_rewrite(self):
        with tempfile.TemporaryDirectory() as root:
            path=Path(root)/'connection.json'
            path.write_text(json.dumps({'url':'http://127.0.0.1:17474','user':'neo4j','password':'one'}))
            original=transport.configuration(path)
            stamp=path.stat()
            path.write_text(json.dumps({'url':'http://127.0.0.1:17474','user':'neo4j','password':'two'}))
            import os
            os.utime(path,ns=(stamp.st_atime_ns,stamp.st_mtime_ns))
            self.assertNotEqual(original[-1],transport.configuration(path)[-1])
    def test_decoder_reports_incomplete_instruction_and_relative_call(self):
        decoder=artifacts.Decoder()
        try:
            rows,gaps=decoder.decode(bytes.fromhex('e805000000c3ff'),0x1000,0x140000000)
            self.assertEqual(rows[0][:2],[0x1000,'e805000000'])
            self.assertEqual(artifacts.reference(rows[0]),('CALLS',0x100a))
            self.assertEqual(gaps,[0x1006])
        finally:decoder.close()

    def test_blob_tampering_is_not_a_cache_miss(self):
        with tempfile.TemporaryDirectory() as root:
            p=Path(root)/'assembly.zip'
            raw=b'{"instructions":[]}'
            with zipfile.ZipFile(p,'w') as z:z.writestr('x',raw)
            row={'blob':p.name,'member':'x','sha':hashlib.sha256(raw).hexdigest()}
            with patch.object(artifacts,'RUNTIME',Path(root)):
                self.assertEqual(artifacts.read_blob(row),{'instructions':[]})
                row['sha']='0'*64
                with self.assertRaisesRegex(ValueError,'integrity'):artifacts.read_blob(row)

    def test_wrong_binary_identity_fails_before_pe_read(self):
        with tempfile.TemporaryDirectory() as root:
            p=Path(root)/'binary';p.write_bytes(b'wrong binary')
            b={'binary':{'path':str(p),'sha256':'0'*64},'pdb':{'path':str(p),'sha256':'0'*64}}
            with patch.object(artifacts,'load',return_value={'baselines':{'B':b}}):
                with self.assertRaisesRegex(ValueError,'SHA mismatch'):artifacts.baseline('B')

    def test_existing_assembly_rejects_wrong_version(self):
        with self.assertRaisesRegex(ValueError,'identity'):
            artifacts.validate_existing({'artifact':{'sha256':'A'}},None,'B')

    def test_key_is_parameterized_and_depth_cannot_inject_cypher(self):
        fake=unittest.mock.Mock();fake.current.return_value='snapshot';fake.query.return_value=[]
        fake.read_snapshot=fake.query
        key="function:B:0 MATCH (n) DETACH DELETE n"
        with patch.object(graph,'Graph',return_value=fake):
            graph.query_tool('neighbors',{'key':key,'kinds':['CALLS']})
            statement,parameters=fake.query.call_args.args
            self.assertNotIn(key,statement)
            self.assertIn(key,parameters['id'])
            for depth in [0,7,True,'2 MATCH (n)']:
                with self.assertRaises(ValueError):graph.query_tool('paths',{'key':'x','depth':depth})

    def test_large_results_are_explicitly_truncated(self):
        fake=unittest.mock.Mock();fake.current.return_value='snapshot'
        fake.query.return_value=[{'node':i} for i in range(3)]
        fake.read_snapshot=fake.query
        with patch.object(graph,'Graph',return_value=fake):
            r=graph.query_tool('search',{'text':'ray','limit':2})
        self.assertTrue(r['truncated']);self.assertEqual(r['next_offset'],2);self.assertEqual(len(r['rows']),2)

    def test_evidence_pack_prefers_outgoing_dependencies_over_high_fan_in_callers(self):
        fake=unittest.mock.Mock();fake.current.return_value='s';fake.query.return_value=[]
        fake.read_snapshot=fake.query
        with patch.object(graph,'Graph',return_value=fake):graph.query_tool('evidence_pack',{'key':'function:B:8cc2e0'})
        statement=fake.query.call_args.args[0]
        self.assertIn('-[:LINK*1..2]->',statement)

    def test_unpublished_snapshot_cannot_be_read(self):
        fake=unittest.mock.Mock();fake.read_snapshot.side_effect=ValueError('Snapshot is not complete')
        fake.query.return_value=[]
        with patch.object(graph,'Graph',return_value=fake):
            with self.assertRaisesRegex(ValueError,'not complete'):
                api.query_tool('search',{'snapshot':'in-progress','text':'ray'})

    def test_guarded_read_checks_completion_again_and_uses_one_batch(self):
        instance=graph.Graph.__new__(graph.Graph)
        good={'columns':['complete'],'data':[{'row':[True]}]}
        bad={'columns':['complete'],'data':[{'row':[False]}]}
        data={'columns':['value'],'data':[{'row':[7]}]}
        with patch.object(instance,'_execute',side_effect=[[good,data],[bad,data]]) as execute:
            self.assertEqual(instance.read_snapshot('RETURN 7 AS value',{'s':'same-id'}),[{'value':7}])
            with self.assertRaisesRegex(ValueError,'not complete'):
                instance.read_snapshot('RETURN 7 AS value',{'s':'same-id'})
            self.assertEqual(execute.call_count,2)
            self.assertEqual(len(execute.call_args.args[0]),2)
            self.assertIn('complete:true',execute.call_args.args[0][1]['statement'])

    def test_missing_independent_range_cannot_pass_aggregate_verification(self):
        self.assertFalse(verification_ok([], [{'kind':'independent_objdump','ranges':[{'ok':False,'error':'Exact PDB range absent'}]}]))
        self.assertTrue(verification_ok([], [{'kind':'independent_objdump','ranges':[{'ok':True}]}]))

    def test_mixed_version_key_is_rejected(self):
        fake=unittest.mock.Mock();fake.current.return_value='s'
        with patch.object(graph,'Graph',return_value=fake):
            with self.assertRaisesRegex(ValueError,'disagree'):
                graph.query_tool('neighbors',{'version':'A','key':'function:B:1'})

    def test_game_type_inside_dependency_generic_does_not_change_defining_owner(self):
        policy={'definition_namespaces_always_excluded':['core'],'external_package_definition_roots':['bevy_ecs']}
        name='bevy_ecs::FunctionSystem<country_core::Roof>'
        self.assertEqual(graph.scope_class(name,['crates/country-core/src/lib.rs'],policy),'dependency_reuse_boundary')

    def test_assembly_reads_snapshot_blobs_and_deduplicates_with_provenance(self):
        fake=unittest.mock.Mock();fake.current.return_value='s'
        fake.query.return_value=[{'version':'B','start':1,'end':3,'sha':'a'},
                                 {'version':'B','start':1,'end':4,'sha':'b'}]
        fake.read_snapshot=fake.query
        con=unittest.mock.Mock()
        con.execute.return_value.fetchone.return_value={'blob':'x','member':'m'}
        document={'instructions':[[1,'90','nop']],'undecoded':[]}
        with patch.object(api,'Graph',return_value=fake),patch.object(api,'db',return_value=con),\
             patch.object(api,'read_blob',return_value=document),patch.object(api,'evidence_status',return_value={}),\
             patch.object(api,'identities',return_value={}):
            result=api.query_tool('assembly',{'key':'function:B:1'})
        self.assertEqual(result['total'],1)
        self.assertEqual(result['rows'][0]['ranges'],[[1,3],[1,4]])

if __name__=='__main__':unittest.main()
