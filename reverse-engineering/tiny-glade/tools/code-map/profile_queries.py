"""Measure real read API components without changing query semantics."""
import argparse
import collections
import hashlib
import statistics
import time
from unittest.mock import patch
import api
from artifacts import RUNTIME,atomic,canonical
from graph import Graph
from validation import tasks,percentile

def profile(label,repetitions=30):
    snapshot=Graph().current();records=[];answers={}
    original_query=Graph._execute;original_status=api.evidence_status
    spans=[]
    def query(self,statement,*args,**kwargs):
        began=time.perf_counter()
        try:return original_query(self,statement,*args,**kwargs)
        finally:spans.append(('graph',time.perf_counter()-began))
    def status():
        began=time.perf_counter()
        try:return original_status()
        finally:spans.append(('evidence',time.perf_counter()-began))
    with patch.object(Graph,'_execute',query),patch.object(api,'evidence_status',status):
        for task in tasks():
            args=dict(task['args'],snapshot=snapshot)
            answer=api.query_tool(task['tool'],args)
            answers[task['name']]=answer
            if task['expected'] not in canonical(answer).decode():raise AssertionError(task['name'])
            samples=[];graph_times=[];evidence_times=[];calls=[]
            for _ in range(repetitions):
                spans.clear();began=time.perf_counter();result=api.query_tool(task['tool'],args)
                elapsed=time.perf_counter()-began
                if canonical(result)!=canonical(answer):raise AssertionError('Unstable answer: '+task['name'])
                samples.append(elapsed)
                graph_times.append(sum(t for k,t in spans if k=='graph'))
                evidence_times.append(sum(t for k,t in spans if k=='evidence'))
                calls.append(sum(k=='graph' for k,t in spans))
            records.append({'task':task['name'],'total_p50_ms':statistics.median(samples)*1000,
                            'total_p95_ms':percentile(samples,.95)*1000,
                            'graph_p50_ms':statistics.median(graph_times)*1000,
                            'evidence_p50_ms':statistics.median(evidence_times)*1000,
                            'graph_calls':statistics.median(calls)})
    report={'label':label,'snapshot':snapshot,'repetitions':repetitions,'tasks':records}
    atomic(RUNTIME/f'optimization-{label}-profile.json',report)
    atomic(RUNTIME/f'optimization-{label}-answers.json',answers)
    return report

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('label');parser.add_argument('--repetitions',type=int,default=30)
    args=parser.parse_args()
    import json
    print(json.dumps(profile(args.label,args.repetitions),indent=2))
