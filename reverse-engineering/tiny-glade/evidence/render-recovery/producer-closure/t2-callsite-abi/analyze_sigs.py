#!/usr/bin/env python3
# Read-only forensic tool (t2-callsite-abi).
# Post-process signatures_all.json into a compact per-function signature table,
# an undecidable list, and aggregate statistics.
#
# Usage: python -I analyze_sigs.py <signatures_all.json> <out_dir>
import json, io, os, sys, collections

def main():
    src, outdir = sys.argv[1], sys.argv[2]
    o = json.load(io.open(src, encoding='utf-8'))
    sigs = {}
    undecidable = []
    arg_hist = collections.Counter()
    conf_hist = collections.Counter()
    n_sites_hist = collections.Counter()
    for va, info in o['functions'].items():
        sites = [p for p in info['callsites'] if 'defs' in p]
        if not sites:
            reasons = [p.get('error', 'no incoming direct call site')
                       for p in info['callsites']] or ['no incoming direct call site']
            undecidable.append({'va': va, 'name': info['name'],
                                'n_callsites': len(info['callsites']),
                                'reason': reasons[0]})
            continue
        # consensus = most common prefix count among call sites
        cnt = collections.Counter(p['prefix_count'] for p in sites)
        maxarg, votes = cnt.most_common(1)[0]
        # sources: pick the call site whose prefix equals the consensus
        pick = next(p for p in sites if p['prefix_count'] == maxarg)
        sources = {r: pick['defs'][r]['source'] for r in pick['arg_regs_observed'][:maxarg]}
        confs = collections.Counter(p['confidence'] for p in sites)
        conf = confs.most_common(1)[0][0]
        # overall confidence downgraded if sites disagree on arg count
        if len(cnt) > 1 and conf == 'high':
            conf = 'medium'
        sigs[va] = {'va': va, 'name': info['name'],
                    'observed_arg_count': maxarg,
                    'arg_regs': pick['arg_regs_observed'][:maxarg],
                    'arg_sources': sources,
                    'n_callsites': len(sites),
                    'n_callsites_agreeing': votes,
                    'confidence': conf,
                    'sources_disagree_on_count': len(cnt) > 1}
        arg_hist[maxarg] += 1
        conf_hist[conf] += 1
        n_sites_hist[len(sites)] += 1
    json.dump(sigs, io.open(os.path.join(outdir, 'signatures.json'), 'w', encoding='utf-8'),
              ensure_ascii=False, indent=1)
    json.dump(undecidable, io.open(os.path.join(outdir, 'undecidable.json'), 'w', encoding='utf-8'),
              ensure_ascii=False, indent=1)
    summary = {
        'targets': len(o['functions']),
        'pdata_functions': o.get('pdata_functions'),
        'recovered_with_argument_count': len(sigs),
        'undecidable_no_direct_callsite': len(undecidable),
        'arg_count_histogram': {str(k): v for k, v in sorted(arg_hist.items())},
        'confidence_histogram': dict(conf_hist),
        'callsites_per_function_histogram': {str(k): v for k, v in sorted(n_sites_hist.items())},
        'note': 'observed_arg_count = machine arg registers (RCX,RDX,R8,R9) proven to be '
                'written before the call; it is NOT the Rust source-argument count and '
                'carries NO type information.',
    }
    json.dump(summary, io.open(os.path.join(outdir, 'signatures_summary.json'), 'w', encoding='utf-8'),
              ensure_ascii=False, indent=1)
    print('recovered arg-count for', len(sigs), 'functions; undecidable', len(undecidable))
    print('arg-count histogram', summary['arg_count_histogram'])
    print('confidence', summary['confidence_histogram'])
    print('callsites/fn', summary['callsites_per_function_histogram'])

if __name__ == '__main__':
    main()
