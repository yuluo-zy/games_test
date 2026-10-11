"""Compare original replay -> create initialization -> wall-driven roof update."""
import json, random, struct, subprocess
from verify_parameters import ROOT, baseline, bits
from verify_replay import HISTORY
from verify_discrete_replay import execute as original_replay
from verify_roof_creation import execute as original_create
from verify_building_state import spatial as original_spatial
from verification_provenance import candidate_provenance

def make_wall(rng):
    wall=bytearray(0x2a0)
    struct.pack_into('<Q',wall,0x10,3);struct.pack_into('<f',wall,0x30,2.)
    wall[0x108]=1;struct.pack_into('<f',wall,0x10c,rng.uniform(0,14))
    struct.pack_into('<I6f',wall,0x258,rng.randrange(2),*[rng.uniform(-4,4) for _ in range(6)])
    wall[0x299]=rng.randrange(4)
    return wall

def main():
    rng=random.Random(20261013);lines=[];expected=[]
    for i in range(512):
        undo=i%2;tag=0 if not undo else 1
        state=struct.pack('<IfQfBBxx',i%4,1.,HISTORY,0.,undo,0)
        payload=bytearray(rng.randbytes(64));payload[0]=tag
        # Set the confirmed parameter portion, retaining opaque appearance bytes.
        struct.pack_into('<6f',payload,20,.2,.3,.5,[0.,.5,1.][i%3],1. if i%4==0 else .7,.25)
        payload[44]=rng.randrange(2)
        roof=rng.randbytes(88);related=rng.randbytes(48);wall=make_wall(rng)
        updated=bytearray(wall) if i%3==0 else make_wall(rng)
        if i%7==0:struct.pack_into('<I',updated,0x258,2) # unsupported new wall shape skipped
        entity=rng.getrandbits(64)
        replay_state,event_tag,fields=original_replay(state,bytes(payload),bits(.25)).split()
        assert event_tag=='0'
        event=bytearray([0xa5]*64);event[8:61]=bytes.fromhex(fields)
        created=original_create(roof,bytes(event),bytes(wall));created_hex,pitched,gable=created.split()
        wall_id=struct.unpack_from('<Q',event,8)[0]
        updated_result=original_spatial(bytes.fromhex(created_hex),related,bytes(updated),wall_id,entity)
        expected.append(f'{replay_state} {created_hex} {pitched} {gable} {updated_result}')
        lines.append(f'chain {state.hex()} {payload.hex()} {roof.hex()} {related.hex()} {wall.hex()} {updated.hex()} {entity:x}')
    process=subprocess.run([str(ROOT/'reconstruction/target/release/building-probe')],
        input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True)
    actual=process.stdout.splitlines();assert len(actual)==len(expected)
    failures=[{'index':i,'original':want,'rust':got} for i,(want,got) in enumerate(zip(expected,actual)) if want!=got]
    report={'ok':not failures,'source_sha256':baseline['binary']['sha256'],'cases':len(lines),
        'scope':'Composed B original create/undo-delete replay event -> Roof initialization -> resolved wall-driven spatial update; exact state/component bytes and ordered notifications',
        'sample_source':'Synthetic confirmed parameter/shape layouts with opaque appearance bytes; not packaged scene replay',
        'game_update_stubs':[],'dependency_implementations_reconstructed':[],
        'excluded':['Entity registration/ECS resolution','Other component initialization and visual/audio systems',
            'JSON appearance/rim defaults','Complete history transaction/cursor/snapshot selection','Rendering/CPU tile generation','A/C behavior'],
        'failure_count':len(failures),'failures':failures[:10],**candidate_provenance()}
    (ROOT/'evidence/building-state/B-chain-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'ok':report['ok'],'cases':len(lines),'failure_count':len(failures)}))
    if failures:print(json.dumps(failures[:2]));raise SystemExit(1)

if __name__=='__main__':main()
