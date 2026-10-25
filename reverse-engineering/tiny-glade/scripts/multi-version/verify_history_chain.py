"""Compare actual B history -> replay -> resolved roof edit with original code."""
from pathlib import Path
import hashlib, importlib.util, json, struct, subprocess
from verify_replay import execute as original_replay, HISTORY
from verify_parameters import ROOT, baseline, bits
from verification_provenance import candidate_provenance

spec=importlib.util.spec_from_file_location('original_edit',ROOT/'scripts/source-edits/verify.py')
original_edit=importlib.util.module_from_spec(spec);spec.loader.exec_module(original_edit)

def payload(edit):
    record=bytearray(64);record[0]=2
    for field,offset in [('prev_height',4),('prev_profile',8),('prev_ridge',12),('prev_eave',16),
            ('new_height',20),('new_profile',24),('new_ridge',28),('new_eave',32)]:
        struct.pack_into('<f',record,offset,edit[field])
    struct.pack_into('<Q',record,40,edit['wall_index'])
    for field,offset in [('prev_tip_offset',48),('new_tip_offset',56)]:
        struct.pack_into('<ff',record,offset,*edit[field])
    return bytes(record)

def main():
    source=Path(baseline['root']);runner=ROOT/'reconstruction/target/release/tg-reconstruction'
    failures=[];count=0;examples=[]
    for path in sorted((source/'assets/starting-builds').rglob('history.json')):
        document=json.loads(path.read_text())
        for i,entry in enumerate(document['History']['edit_chain']):
            for j,command in enumerate(entry['edits']):
                edit=command.get('Roof',{}).get('VisualEdit')
                if edit is None:continue
                record=payload(edit)
                for undo in [0,1]:
                    for dt in [0.,0.25,1.]:
                        state=struct.pack('<IfQfBBxx',1,1.,HISTORY,0.,undo,0)
                        replay_state,event=original_replay(state,record,bits(dt)).split()
                        roof=original_edit.base_roof()
                        expected=original_edit.execute(roof,bytes.fromhex(event)+bytes([0xa5])*11)
                        result=subprocess.run([str(runner),'roof-history-step',str(path),str(i),str(j),
                            roof.hex(),'1','0',str(dt),str(undo)],capture_output=True,text=True)
                        if result.returncode:
                            failures.append({'path':str(path),'entry':i,'edit':j,'error':result.stderr});continue
                        actual=json.loads(result.stdout);want=expected.split();state_bytes=bytes.fromhex(replay_state)
                        ok=(actual['roof_hex']==want[0] and actual['changed_tick']==1
                            and actual['changed_fields']==int(want[2])
                            and actual['elapsed_bits']==f'{struct.unpack_from("<I",state_bytes,16)[0]:08x}'
                            and actual['finished']==bool(state_bytes[21])
                            and actual['feedback_count']==expected.count('|'))
                        count+=1
                        if not ok:
                            failures.append({'path':str(path),'entry':i,'edit':j,'dt':dt,'undo':undo,
                                             'original':expected,'rust':actual})
                        if len(examples)<3:
                            examples.append({'path':path.relative_to(source).as_posix(),'entry':i,'edit':j,'dt':dt,'undo':undo,'ok':ok})
    report={'ok':not failures,'source_sha256':baseline['binary']['sha256'],'cases':count,
        'scope':'Actual B VisualEdit JSON to original interpolation and resolved edit suffix; exact Roof88 bytes and replay state fields, write mask and feedback count',
        'excluded':['ECS resolution/registration','Creation/deletion/other commands','Snapshot/cursor/undo transaction semantics','Feedback numeric payloads in this chain test','GPU/scene behavior'],
        'failures':failures[:10],'failure_count':len(failures),'examples':examples,
        'cargo_lock_sha256':hashlib.sha256((ROOT/'reconstruction/Cargo.lock').read_bytes()).hexdigest()}
    report.update(candidate_provenance())
    (ROOT/'evidence/multi-version/B-history-chain-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'ok':report['ok'],'cases':count,'failure_count':len(failures)}))
    if failures:print(json.dumps(failures[:2]));raise SystemExit(1)

if __name__=='__main__':main()
