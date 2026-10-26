"""Offline QPC scheduling accounting. No cycles-to-frequency conversion.

xperf dumper -add_rawdata supplies absolute QPC timestamps preceding each event.
Only this process's scheduling intervals are retained. Interrupt/DPC intervals
are unioned per CPU before subtraction, so nested interrupts are not counted twice.
"""
import bisect
import csv
import math
import re
from collections import defaultdict

RAW = re.compile(r'^#\s+(\d+)\s+#\s+(0x[0-9a-fA-F]+)')
PID = re.compile(r'\(\s*(\d+)\s*\)\s*$')


def union(intervals):
    result = []
    for start, end in sorted(intervals):
        if end <= start:
            continue
        if result and start <= result[-1][1]:
            result[-1] = (result[-1][0], max(result[-1][1], end))
        else:
            result.append((start, end))
    return result


def subtract(intervals, exclusions):
    exclusions = union(exclusions)
    ends = [end for _, end in exclusions]
    result = []
    for start, end in intervals:
        at = bisect.bisect_right(ends, start)
        position = start
        while at < len(exclusions) and exclusions[at][0] < end:
            left, right = exclusions[at]
            if left > position:
                result.append((position, min(left, end)))
            position = max(position, right)
            at += 1
        if position < end:
            result.append((position, end))
    return result


class Timeline:
    def __init__(self, intervals):
        # A single thread cannot run on two CPUs simultaneously.
        self.intervals = sorted(intervals)
        self.starts = [left for left, _ in self.intervals]
        self.ends = [right for _, right in self.intervals]
        self.prefix = [0]
        for index, (left, right) in enumerate(self.intervals):
            if index and left < self.ends[index - 1]:
                raise ValueError('overlapping scheduling intervals for one thread')
            self.prefix.append(self.prefix[-1] + right - left)

    def until(self, time):
        at = bisect.bisect_right(self.ends, time)
        total = self.prefix[at]
        if at < len(self.starts) and self.starts[at] < time:
            total += time - self.starts[at]
        return total

    def between(self, start, end):
        if end <= start:
            raise ValueError('completion clock did not advance')
        return self.until(end) - self.until(start)


def read_trace(path, process_id, frequency):
    headers, running = {}, {}
    by_cpu = defaultdict(list)
    exclusions = defaultdict(list)
    failures = []
    lost_checked = False
    raw_time = None
    first = last = None
    with open(path, encoding='utf-8-sig', errors='replace') as source:
        for line in source:
            raw = RAW.match(line)
            if raw:
                raw_time = int(raw.group(2), 16)
                continue
            if 'Events Lost:' in line:
                match = re.search(r'Events Lost:\s*(\d+),\s*Buffers lost:\s*(\d+)', line)
                if not match:
                    raise ValueError('cannot decode trace loss counters')
                lost_checked = True
                if int(match[1]) or int(match[2]):
                    raise ValueError('ETW lost events or buffers; measurement invalid')
            if ',' not in line:
                continue
            # Process names are quoted by xperf, but spaces precede quotes.
            row = [value.strip() for value in next(csv.reader([line], skipinitialspace=True))]
            event = row[0]
            if len(row) > 1 and row[1] == 'TimeStamp':
                headers[event] = {name: index for index, name in enumerate(row)}
                continue
            if event not in {'CSwitch', 'DPC', 'Interrupt'} or raw_time is None:
                continue
            fields = headers.get(event)
            if fields is None:
                raise ValueError('missing ETW event header: ' + event)
            first = raw_time if first is None else min(first, raw_time)
            last = raw_time if last is None else max(last, raw_time)
            cpu = int(row[fields['CPU']])
            if event in {'DPC', 'Interrupt'}:
                duration_ticks = round(float(row[fields['ElapsedTime']]) * frequency / 1_000_000)
                # Kernel ISR/DPC header QPC is the routine completion timestamp.
                # Microsoft PerfView: ElapsedTimeMSec = TimeStampRelativeMSec - InitialTimeQPC.
                exclusions[cpu].append((raw_time - duration_ticks, raw_time))
                continue
            new_tid = int(row[fields['New TID']])
            old_tid = int(row[fields['Old TID']])
            old_pid_match = PID.search(row[fields['Old Process Name ( PID)']])
            new_pid_match = PID.search(row[fields['New Process Name ( PID)']])
            if old_pid_match and int(old_pid_match[1]) == process_id:
                previous = running.get(cpu)
                if previous and previous[0] == old_tid:
                    by_cpu[cpu].append((old_tid, previous[1], raw_time))
                else:
                    failures.append((raw_time, old_tid))
            if new_pid_match and int(new_pid_match[1]) == process_id:
                running[cpu] = (new_tid, raw_time)
            else:
                running.pop(cpu, None)
    if not lost_checked or first is None:
        raise ValueError('trace loss/clock metadata absent')
    intervals = defaultdict(list)
    for cpu, entries in by_cpu.items():
        excluded = union(exclusions[cpu])
        grouped = defaultdict(list)
        for tid, start, end in entries:
            grouped[tid].append((start, end))
        for tid, spans in grouped.items():
            intervals[tid].extend(subtract(spans, excluded))
    return {tid: Timeline(spans) for tid, spans in intervals.items()}, failures, first, last


def distribution(values):
    ordered = sorted(values)
    if not ordered:
        raise ValueError('no CPU intervals')
    def percentile(p):
        return ordered[max(0, math.ceil(len(ordered) * p) - 1)]
    return {'n': len(values), 'mean_ms': sum(values)/len(values),
            'p50_ms': percentile(.50), 'p95_ms': percentile(.95), 'p99_ms': percentile(.99)}


def account(report, csv_path, independent_totals=None):
    render = report['render']
    timelines, errors, trace_start, trace_end = read_trace(csv_path, render['process_id'], render['qpc_frequency'])
    results = {}
    for name in ('render', 'main'):
        session = report[name]
        if not session['valid'] or not session['complete']:
            raise ValueError(name + ' session invalid/incomplete')
        points = session['completion_points']
        start, end = points[0]['qpc'], points[-1]['qpc']
        if not trace_start < start < end < trace_end:
            raise ValueError('ETW trace does not enclose measured boundaries')
        if any(start <= time <= end for time, _ in errors):
            raise ValueError('unmatched target-process scheduling event within measurement')
        frequency = session['qpc_frequency']
        timeline = timelines.get(session['thread_id'])
        if timeline is None:
            raise ValueError('owner thread missing in trace')
        frames = []
        for previous, current in zip(points, points[1:]):
            if previous['app_epoch'] != current['app_epoch']:
                continue
            if not previous['observed'] or not current['observed']:
                raise ValueError('unobserved completion in clean measurement')
            if current['receipt'] <= previous['receipt'] or current['attempt'] <= previous['attempt']:
                raise ValueError('non-increasing completion attribution')
            ticks = timeline.between(previous['qpc'], current['qpc'])
            frames.append({'receipt': current['receipt'], 'attempt': current['attempt'],
                'world_tick': current['world_tick'], 'app_epoch': current['app_epoch'],
                'cpu_ms': ticks * 1000 / frequency,
                'wall_ms': (current['qpc']-previous['qpc']) * 1000/frequency})
        if len(frames) != session['requested_samples']:
            raise ValueError('not enough precisely attributed CPU intervals')
        stats = distribution([frame['cpu_ms'] for frame in frames])
        etw_total_ns = timeline.between(start, end) * 1_000_000_000/frequency
        os_total_ns = points[-1]['cumulative_cpu_ns']-points[0]['cumulative_cpu_ns']
        coarse_error = abs(etw_total_ns-os_total_ns)/max(etw_total_ns, 1)
        reference = independent_totals[name] if independent_totals is not None else os_total_ns
        error = abs(etw_total_ns-reference)/max(etw_total_ns, 1)
        process_ms = sum(thread.between(start, end) for thread in timelines.values())*1000/frequency
        results[name] = {**stats, 'os_cpu_ns': os_total_ns, 'etw_cpu_ns': etw_total_ns,
            'coarse_os_relative_error': coarse_error,
            'independent_reference': 'xperf cswitch -thread -exc_dpcisr' if independent_totals is not None else 'GetThreadTimes',
            'independent_reference_cpu_ns': reference,
            'os_crosscheck_relative_error': error, 'crosscheck_passed': error <= .02,
            'process_cpu_work_ms_per_frame': process_ms/len(frames), 'frames': frames}
    return {'schema': 'windows-etw-cpu-v1', 'source': 'CSwitch QPC with unioned DPC/ISR subtraction',
            'valid': all(value['crosscheck_passed'] for value in results.values()), **results}
