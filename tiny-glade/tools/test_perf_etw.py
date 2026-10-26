import tempfile
from pathlib import Path
import unittest
from perf_etw import Timeline, account, read_trace, subtract, union


class AccountingTests(unittest.TestCase):
    def test_nested_interrupts_are_removed_once(self):
        self.assertEqual(union([(3, 7), (4, 6), (7, 9)]), [(3, 9)])
        self.assertEqual(subtract([(0, 5), (5, 12)], [(3, 7), (4, 6)]), [(0, 3), (7, 12)])

    def test_partial_frame_overlaps_and_migration(self):
        timeline = Timeline([(0, 10), (20, 30)])
        self.assertEqual(timeline.between(5, 25), 10)
        self.assertEqual(timeline.between(10, 20), 0)
        with self.assertRaises(ValueError):
            Timeline([(0, 10), (9, 15)])

    def trace(self, text):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        path = Path(temporary.name)/'trace.csv'
        path.write_text(text, encoding='utf-8')
        return path

    def test_raw_qpc_alignment_does_not_use_rounded_csv_time(self):
        header = 'CSwitch,TimeStamp,New Process Name ( PID),New TID,Old Process Name ( PID),Old TID,CPU\n'
        body = '''OS Version: test, Events Lost: 0, Buffers lost: 0,
# 0 # 0x1 # 0
CSwitch,0,game (7),11,Idle (0),0,0
# 0 # 0x65 # 0
CSwitch,0,Idle (0),0,game (7),11,0
# 1 # 0x96 # 0
CSwitch,0,game (7),11,Idle (0),0,1
# 1 # 0xfa # 0
CSwitch,0,Idle (0),0,game (7),11,1
'''
        path = self.trace(header+body)
        timelines, errors, _, _ = read_trace(path, 7, 1_000_000)
        self.assertFalse(errors)
        self.assertEqual(timelines[11].between(50, 200), 101)

    def test_lost_events_invalidate_measurement(self):
        with self.assertRaisesRegex(ValueError, 'lost events'):
            read_trace(self.trace('OS Version: test, Events Lost: 1, Buffers lost: 0,\n'), 1, 1)

    def test_missing_loss_metadata_cannot_be_accepted(self):
        with self.assertRaises(ValueError):
            read_trace(self.trace('BeginHeader\nEndHeader\n'), 1, 1)


if __name__ == '__main__':
    unittest.main()
