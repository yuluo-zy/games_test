"""Persisted CLI check against original resolved creation/spatial game ranges."""
import json,struct,subprocess
from verify_roof_creation import execute as original_create
from verify_building_state import spatial as original_spatial
from verification_provenance import ROOT,candidate_provenance

def main():
    runner=ROOT/'reconstruction/target/release/tg-reconstruction'
    wall=bytearray(672);struct.pack_into('<Q',wall,0x10,3);struct.pack_into('<f',wall,0x30,2.)
    wall[0x108]=1;struct.pack_into('<f',wall,0x10c,7.5);struct.pack_into('<I6f',wall,0x258,1,1.,0.,2.,3.,4.,5.);wall[0x299]=2
    record=bytearray(64);struct.pack_into('<Q',record,8,0xffffffffffffffff);struct.pack_into('<6f',record,32,.2,.3,.5,.5,1.,.25)
    roof=bytes([0xa5])*88;related=bytes([0x5a])*48
    want=original_create(roof,bytes(record),bytes(wall)).split()
    result=subprocess.run([str(runner),'building-roof-create',wall.hex(),record.hex(),roof.hex()],capture_output=True,text=True,check=True)
    created=json.loads(result.stdout)
    assert created['roof_hex']==want[0] and created['pitched']==bool(int(want[1]))
    assert created['gable_notification_wall_id']==str(0xffffffffffffffff) and not created['entity_spawned']
    struct.pack_into('<f',wall,0x10c,8.);wall[0x299]=0;struct.pack_into('<f',wall,0x264,6.)
    want=original_spatial(bytes.fromhex(created['roof_hex']),related,bytes(wall),0xffffffffffffffff,0x100000029).split()
    result=subprocess.run([str(runner),'building-roof-update',wall.hex(),created['roof_hex'],related.hex(),'ffffffffffffffff','100000029'],capture_output=True,text=True,check=True)
    updated=json.loads(result.stdout)
    assert updated['roof_hex']==want[0] and updated['related_hex']==want[1] and updated['roof_tick']==updated['related_tick']==1
    assert len(updated['geometry_changes'])==int(want[4])==2 and updated['style_changed'] and updated['shape_changed']
    assert all(e=={'wall_id':str(0xffffffffffffffff),'roof_entity':str(0x100000029),'kind':2} for e in updated['geometry_changes'])
    for bad in ['bad','💥']:
        result=subprocess.run([str(runner),'building-roof-create',bad,record.hex(),roof.hex()],capture_output=True,text=True)
        assert result.returncode==1 and 'hexadecimal input' in result.stderr
    report={'ok':True,'valid_cli_cases':2,'malformed_input_cases':2,
        'scope':'CLI resolved creation and wall-update against B original core; IDs preserved as strings; malformed input rejected',**candidate_provenance()}
    (ROOT/'evidence/building-state/B-cli-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'ok':True,'cli_cases':4}))

if __name__=='__main__':main()
