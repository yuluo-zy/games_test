import unittest
from parse_native_pass_trace import FIELDS, parse_trace


def record(scope, graph, index, command, dispatch, name="same name", ok="Some(true)", valid=True, unwind=False, missing=False):
    import json
    fields = {field: 0 for field in FIELDS}
    fields["direct_dispatch_api_calls"] = dispatch
    body = ", ".join(f"{key}: {value}" for key, value in fields.items())
    native = "None" if missing else f"Some(CommandWorkload {{ valid: {str(valid).lower()}, {body} }})"
    return (f"RHAPSODY_RECORD_TRACE id={scope} graph={graph} pass={index} command={command} "
            f"name={json.dumps(name, ensure_ascii=False)} callback_ok={ok} unwind={str(unwind).lower()} "
            f"total_ns=1 inclusive_phases_ns=[] counts=[] native_workload={native}")


def chunk(graph, command, dispatch, passes, valid=1):
    return (f"RHAPSODY_SUBMIT_TRACE scope=record_owner_transfer total_ns=1 phases_ns=[] "
            f"counts={[( 'owners', 0), ('command', command), ('graph', graph), ('direct_dispatch_api_calls', dispatch), ('native_valid', valid), ('recorded_passes', passes)]}")


class NativePassParserTests(unittest.TestCase):
    def test_out_of_order_workers_duplicate_names_keep_real_identity_and_conserve(self):
        text = "\n".join([record(2, 5, 1, 8, 3), record(1, 5, 0, 8, 2), chunk(5, 8, 5, 2)])
        report = parse_trace(text)
        self.assertTrue(report["trace_valid"], report["errors"])
        self.assertEqual(report["chunks"][0]["scope_ids"], [1, 2])
        self.assertEqual(len(report["records"]), 2)
        self.assertEqual(report["raw_pass_totals"]["direct_dispatch_api_calls"], 5)
        self.assertIsNone(report["run_valid"])
        self.assertFalse(report["clean_cpu_acceptance_eligible"])

    def test_interleaved_commands_and_reused_handles_do_not_merge_executions(self):
        report = parse_trace("\n".join([record(1, 5, 0, 8, 2), record(2, 5, 1, 9, 3),
            chunk(5, 9, 3, 1), chunk(5, 8, 2, 1), record(3, 5, 0, 8, 4), chunk(5, 8, 4, 1)]))
        self.assertTrue(report["trace_valid"], report["errors"])
        self.assertEqual([r["scope_ids"] for r in report["chunks"]], [[2], [1], [3]])

    def test_one_chunk_can_contain_distinct_declared_graph_identities(self):
        report = parse_trace("\n".join([record(2, 99, 1, 8, 3), record(1, 5, 0, 8, 2), chunk(5, 8, 5, 2)]))
        self.assertTrue(report["trace_valid"], report["errors"])
        self.assertEqual({r["graph"] for r in report["records"]}, {5, 99})

    def test_dispatch_mismatch_and_missing_record_invalid(self):
        for text in [record(1, 5, 0, 8, 2) + "\n" + chunk(5, 8, 3, 1),
                     record(1, 5, 0, 8, 2) + "\n" + chunk(5, 8, 2, 2)]:
            self.assertFalse(parse_trace(text)["trace_valid"])

    def test_failure_none_and_unwind_preserve_raw_prefix_but_never_pass(self):
        for kwargs in [{"ok": "Some(false)"}, {"missing": True}, {"unwind": True}, {"valid": False}]:
            report = parse_trace(record(1, 5, 0, 8, 2, **kwargs) + "\n" + chunk(5, 8, 2, 1, 0))
            self.assertFalse(report["trace_valid"])
            self.assertEqual(len(report["records"]), 1)

    def test_duplicate_id_duplicate_pass_and_partial_tail_invalid(self):
        cases = [[record(1, 5, 0, 8, 2), record(1, 5, 1, 8, 3), chunk(5, 8, 5, 2)],
                 [record(1, 5, 0, 8, 2), record(2, 5, 0, 8, 3), chunk(5, 8, 5, 2)],
                 [record(1, 5, 0, 8, 2)]]
        for lines in cases:
            self.assertFalse(parse_trace("\n".join(lines))["trace_valid"])

    def test_malformed_missing_fields_overflow_and_old_schema_invalid(self):
        for line in [record(1, 5, 0, 8, 1).replace("direct_dispatch_api_calls: 1", "direct_dispatch_api_calls: 18446744073709551616"),
                     record(1, 5, 0, 8, 1).replace("cached_d2d_copy_recorded_bytes: 0", "unknown: 0"),
                     "RHAPSODY_RECORD_TRACE old=1", "RHAPSODY_SUBMIT_TRACE scope=record_owner_transfer total_ns=1 counts=[]", ""]:
            self.assertFalse(parse_trace(line)["trace_valid"])

    def test_escaped_unicode_names_are_preserved(self):
        name = '墙 "same" \\ sample'
        report = parse_trace(record(1, 5, 0, 8, 1, name=name) + "\n" + chunk(5, 8, 1, 1))
        self.assertTrue(report["trace_valid"], report["errors"])
        self.assertEqual(report["records"][0]["name"], name)


if __name__ == "__main__":
    unittest.main()
