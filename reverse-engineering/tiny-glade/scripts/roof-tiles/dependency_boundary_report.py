from pathlib import Path
import json,tomllib,hashlib
R=Path('D:/game/reverse-engineering/tiny-glade');O=R/'evidence/roof-tiles';origin=Path('D:/game/ljxsj_92385/Tiny Glade/build-info/Cargo.lock')
old=tomllib.loads(origin.read_text(encoding='utf-8'));new=tomllib.loads((R/'reconstruction/Cargo.lock').read_text(encoding='utf-8'));deps=[]
for p in new['package']:
 if p['name']=='tiny-glade-reconstruction':continue
 a=next(x for x in old['package'] if x['name']==p['name'] and x['version']==p['version']);assert a.get('checksum')==p.get('checksum')
 deps.append(dict(name=p['name'],version=p['version'],checksum=p.get('checksum'),direct=p['name'] in ['fastrand','glam','half']))
report=dict(original_lock_path=str(origin),original_lock_sha256=hashlib.sha256(origin.read_bytes()).hexdigest(),reconstructed_lock_sha256=hashlib.sha256((R/'reconstruction/Cargo.lock').read_bytes()).hexdigest(),exact_published_dependencies=deps,
 adapters=[dict(game_interface='TileRng::next_f32',published_api='fastrand::Rng::with_seed / f32 / get_seed',manual_algorithm_removed=True),dict(game_interface='tiles::half',published_api='half::f16::from_f32 / to_bits',manual_algorithm_removed=True),dict(game_interface='tiles::matrix_quat',published_api='glam::Mat3::from_cols / Quat::from_mat3',manual_algorithm_removed=True),dict(game_interface='circular tile frame direction',published_api='glam::Quat::from_axis_angle * glam::Vec3',manual_algorithm_removed=True)],
 retained_game_code=['utils::random_splits','Curve2 and normalized/world profile sampling','roof shape/ridge parameter rules','roof row/column placement and tile emission order','game packed record and flags'],
 scope='Reuse dependencies rather than recover/reimplement their sources; no GPU or scene work',limitations=['User states Bevy is a fork; exact fork repository/commit is still unverified and no engine dependency is invented here'])
(O/'dependency-boundary.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
index=json.loads((O/'source-map.json').read_text(encoding='utf-8'));index['dependency_boundary']=report;index['known_layouts']['TileRng'].pop('math',None);index['known_layouts']['TileRng']['implementation']='Published fastrand 1.8.0 API adapter; dependency source is reused rather than reconstructed'
for p in index['procedures']:
 p['reconstruction_scope']='dependency callable metadata/oracle only, not recovered dependency source' if p['name'].startswith('fastrand::') else 'game or self-authored utils rule / metadata'
(O/'source-map.json').write_text(json.dumps(index,ensure_ascii=False,indent=2),encoding='utf-8');print(json.dumps(deps))
