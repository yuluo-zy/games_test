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



class InterruptEndpointReviewTests(unittest.TestCase):
    trace = AccountingTests.trace
    def event_trace(self, events):
        lines = ['OS Version: test, Events Lost: 0, Buffers lost: 0,',
                 'CSwitch,TimeStamp,New Process Name ( PID),New TID,Old Process Name ( PID),Old TID,CPU',
                 'DPC,TimeStamp,CPU,ElapsedTime', 'Interrupt,TimeStamp,CPU,ElapsedTime',
                 '# 0 # 0x0 # 0', 'CSwitch,0,game (7),11,Idle (0),0,0']
        for event, end, duration in events:
            lines.extend([f'# 0 # {end:#x} # 0', f'{event},0,0,{duration*1000}'])
        lines.extend(['# 0 # 0xc8 # 0', 'CSwitch,0,Idle (0),0,game (7),11,0'])
        return self.trace('\n'.join(lines)+'\n')

    def test_dpc_and_isr_end_at_raw_qpc_crossing_phase_boundary(self):
        for event in ('DPC', 'Interrupt'):
            with self.subTest(event=event):
                timelines, errors, _, _ = read_trace(self.event_trace([(event, 110, 30)]), 7, 1000)
                self.assertFalse(errors)
                self.assertEqual(timelines[11].between(0, 100), 80)   # exclude [80,100]
                self.assertEqual(timelines[11].between(100, 200), 90) # exclude [100,110]
                self.assertEqual(timelines[11].between(0, 200), 170)

    def test_nested_dpc_isr_union_does_not_subtract_twice(self):
        # DPC [80,120] contains ISR [90,110]; union is exactly [80,120].
        timelines, errors, _, _ = read_trace(self.event_trace([('Interrupt', 110, 20), ('DPC', 120, 40)]), 7, 1000)
        self.assertFalse(errors)
        self.assertEqual(timelines[11].between(0, 100), 80)
        self.assertEqual(timelines[11].between(100, 200), 80)
        self.assertEqual(timelines[11].between(0, 200), 160)


if __name__ == '__main__':
    unittest.main()
