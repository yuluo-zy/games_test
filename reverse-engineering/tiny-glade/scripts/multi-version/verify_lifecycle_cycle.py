"""Verify composed game creation -> animation -> deletion-request -> fade/removal."""
import json,random,struct,subprocess
from verify_parameters import ROOT,baseline,bits
from verify_roof_creation import execute as create
from verify_lifecycle import animation,tile,mark
from verification_provenance import candidate_provenance

def main():
    rng=random.Random(20261016);lines=[];expected=[]
    for i in range(128):
        wall=bytearray(672);struct.pack_into('<Q',wall,0x10,3);struct.pack_into('<f',wall,0x30,2.)
        wall[0x108]=1;struct.pack_into('<f',wall,0x10c,7.5)
        struct.pack_into('<I6f',wall,0x258,i%2,1.,0.,2.,3.,4.,5.);wall[0x299]=i%4
        record=bytearray(rng.randbytes(64));height=[0.,.5,1.][i%3]
        struct.pack_into('<6f',record,32,.2,.3,.5,height,1. if i%4==0 else .7,.25)
        record[56]=i%2;roof=rng.randbytes(88);tile_bytes=rng.randbytes(24)
        grow=[0.,.05,.35,1.][i%4];fade=[0.,.05,.35,1.][(i//4)%4]
        created=bytes.fromhex(create(roof,bytes(record),bytes(wall)).split()[0])
        sim=tile(tile_bytes,bits(7.5))
        grown=animation(bytes(8),created,0,bits(grow)).split()
        wall_id=struct.unpack_from('<Q',created,0)[0]
        marking=mark(created,0x100000029,[wall_id]);assert marking!='-'
        faded=animation(bytes.fromhex(grown[0]),created,1,bits(fade)).split()
        tick=2 if fade!=0 else (1 if grow!=0 else 0)
        expected.append(f'{created.hex()} {sim} {grown[0]} {faded[0]} 1 {faded[3]} {tick:08x}')
        lines.append(f'cycle {record.hex()} {wall.hex()} {roof.hex()} {tile_bytes.hex()} {bits(grow):08x} {bits(fade):08x}')
    result=subprocess.run([str(ROOT/'reconstruction/target/release/lifecycle-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True)
    actual=result.stdout.splitlines();assert len(actual)==len(expected)
    failures=[{'index':i,'original':want,'rust':got} for i,(want,got) in enumerate(zip(expected,actual)) if want!=got]
    report={'ok':not failures,'source_sha256':baseline['binary']['sha256'],'cases':len(lines),
        'scope':'B game initialization, animation growth, delete-request match/pivot and committed deletion fade/despawn intent composed through ResolvedRoofLifecycle',
        'excluded':['Real Bevy command flush timing','Drawable/spire/support bundles','Full scene/runtime ownership','A/C complete lifecycle'],
        'failure_count':len(failures),'failures':failures[:10],**candidate_provenance()}
    (ROOT/'evidence/building-lifecycle/B-cycle-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'ok':report['ok'],'cases':len(lines),'failure_count':len(failures)}))
    if failures:print(json.dumps(failures[:2]));raise SystemExit(1)

if __name__=='__main__':main()
