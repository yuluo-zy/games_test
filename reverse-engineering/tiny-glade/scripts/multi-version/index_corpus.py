"""Identify three-version snapshot candidates without silently replacing files."""
from pathlib import Path
from collections import defaultdict
import hashlib, json

ROOT=Path(__file__).resolve().parents[2]
def sha(path):
    with path.open('rb') as file:return hashlib.file_digest(file,'sha256').hexdigest()

def main():
    baselines=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']
    snapshots=[];by_id=defaultdict(list);histories=[];missing=[]
    for version,baseline in baselines.items():
        directory=Path(baseline['root'])
        for path in sorted((directory/'assets/starting-builds').rglob('*.snapshot')):
            row={'baseline':version,'path':path.relative_to(directory).as_posix(),
                 'snapshot_id':path.stem,'sha256':sha(path),'bytes':path.stat().st_size}
            snapshots.append(row);by_id[path.stem].append(row)
    for version,baseline in baselines.items():
        directory=Path(baseline['root'])
        for path in sorted((directory/'assets/starting-builds').rglob('history.json')):
            document=json.loads(path.read_text());row={'baseline':version,
                'path':path.relative_to(directory).as_posix(),'sha256':sha(path),
                'version':document['Version'],'references':[]}
            for i,reference in enumerate(document['History']['snapshots']):
                expected=path.parent/'snapshots'/f"{reference['snapshot_id']}.snapshot"
                ref={'chain_index':reference['chain_index'],'snapshot_id':reference['snapshot_id'],
                     'file_exists':expected.exists()}
                row['references'].append(ref)
                if not expected.exists():
                    candidates=by_id[str(reference['snapshot_id'])]
                    missing.append({'baseline':version,'history':row['path'],
                        'pointer':f'/History/snapshots/{i}','reference':reference,
                        'same_id_candidates':candidates,
                        'replacement_verified':False})
            histories.append(row)
    report={'target_execution':False,'history_files':histories,'snapshots':snapshots,
        'unique_history_hashes':len({r['sha256']for r in histories}),
        'unique_snapshot_hashes':len({r['sha256']for r in snapshots}),
        'missing_references':missing,
        'policy':'Same snapshot ID is a candidate only; content/schema/world compatibility must be verified before replacement. Original files were not modified.'}
    (ROOT/'evidence/multi-version/corpus-index.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'histories':len(histories),'snapshots':len(snapshots),
        'unique_history_hashes':report['unique_history_hashes'],
        'unique_snapshot_hashes':report['unique_snapshot_hashes'],
        'B_missing':sum(r['baseline']=='B' for r in missing),
        'B_missing_with_candidates':sum(r['baseline']=='B' and bool(r['same_id_candidates']) for r in missing)}))
if __name__=='__main__':main()
