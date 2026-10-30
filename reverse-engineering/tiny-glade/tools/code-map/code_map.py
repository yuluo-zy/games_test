#!/usr/bin/env python3
"""CLI for the Tiny Glade dedicated Neo4j code map."""
import argparse
import json
import sys
import fcntl
from artifacts import RUNTIME, atomic, capacity, coverage, db, enrich_catalog, extract, inventory
from graph import import_graph, runtime
from api import query_tool
from validation import benchmark, evidence_status, verify

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    sub=parser.add_subparsers(dest='command',required=True)
    for command in ['inventory','capacity','enrich','associated','archive-existing','dispatch-boundaries','import','coverage','export-map','module-reviews','byte-coverage','receipt-audit','freeze-evidence','optimize-search']:sub.add_parser(command)
    p=sub.add_parser('extract');p.add_argument('--limit',type=int)
    p=sub.add_parser('runtime');p.add_argument('action',choices=['configure','start','stop','status'])
    p=sub.add_parser('query');p.add_argument('tool');p.add_argument('--args',default='{}');p.add_argument('--output')
    p=sub.add_parser('verify');p.add_argument('--full',action='store_true')
    p=sub.add_parser('benchmark');p.add_argument('--repetitions',type=int,default=30)
    p=sub.add_parser('cfg');p.add_argument('--key',required=True)
    sub.add_parser('serve')
    args=parser.parse_args()
    lock=None
    if args.command in ('inventory','enrich','associated','archive-existing','dispatch-boundaries','extract','import'):
        RUNTIME.mkdir(parents=True,exist_ok=True)
        lock=(RUNTIME/'catalog-writer.lock').open('a')
        try:fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
        except BlockingIOError:raise RuntimeError('Another catalog writer is active; run mutation phases sequentially')
    if args.command=='serve':
        from server import serve
        serve();return
    if args.command=='runtime':result=runtime(args.action)
    elif args.command=='inventory':result=inventory()
    elif args.command=='capacity':result=capacity()
    elif args.command=='enrich':result=enrich_catalog()
    elif args.command=='associated':
        from associated import import_associated
        result=import_associated()
    elif args.command=='dispatch-boundaries':
        from supplement import dispatch_boundaries
        result=dispatch_boundaries()
    elif args.command=='archive-existing':
        from archive import capture_existing
        result=capture_existing()
    elif args.command=='extract':result=extract(args.limit)
    elif args.command=='import':
        con=db()
        con.execute('CREATE INDEX IF NOT EXISTS spans_start ON spans(version,start,end)')
        con.commit();con.close()
        result=import_graph()
    elif args.command=='coverage':result=coverage()
    elif args.command=='module-reviews':
        from review import module_reviews
        result=module_reviews()
    elif args.command=='byte-coverage':
        from review import byte_coverage
        result=byte_coverage()
    elif args.command=='receipt-audit':
        from validation import audit_receipts
        result=audit_receipts()
    elif args.command=='freeze-evidence':
        from validation import freeze_evidence
        result=freeze_evidence()
    elif args.command=='optimize-search':
        from search_index import build
        result=build()
    elif args.command=='export-map':
        from review import export_maps
        result=export_maps()
    elif args.command=='verify':result=verify(args.full)
    elif args.command=='benchmark':result=benchmark(args.repetitions)
    elif args.command=='cfg':
        from cfg import candidate_cfg
        result=candidate_cfg(args.key)
    else:
        result=query_tool(args.tool,json.loads(args.args))
        result['source_binding']=evidence_status()
        if args.output:atomic(args.output,result)
    print(json.dumps(result,ensure_ascii=False,indent=2))
    if result.get('ok') is False:sys.exit(2)

if __name__=='__main__':
    try:main()
    except Exception as error:
        print(json.dumps({'ok':False,'error':type(error).__name__,'message':str(error)},ensure_ascii=False),file=sys.stderr)
        sys.exit(2)
