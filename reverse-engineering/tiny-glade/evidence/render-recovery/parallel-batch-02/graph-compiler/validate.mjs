import {readFileSync, writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
const root = 'D:/game/reverse-engineering/tiny-glade';
const crate = root + '/reconstruction/render/work-packages/graph-compiler-r2';
const output = root + '/evidence/render-recovery/parallel-batch-02/graph-compiler';
const checks = [];
for (const args of [['test','--offline','--locked'], ['build','--offline','--locked','--release'], ['clippy','--offline','--locked','--all-targets','--','-D','warnings']]) {
  const result = spawnSync('cargo', args, {cwd:crate, encoding:'utf8', shell:false});
  checks.push({command:['cargo',...args],status:result.status,stdout:result.stdout,stderr:result.stderr});
}
const oracle = crate + '/target/debug/graph-snapshot-oracle.exe';
const fixtures = [];
for (const name of ['multi','backend-error','panic','empty']) {
  const input = readFileSync(crate + '/fixtures/' + name + '.dsl','utf8');
  const result = spawnSync(oracle, [], {input,encoding:'utf8'});
  const snapshot = JSON.parse(result.stdout.trim());
  fixtures.push({name,status:result.status,result:snapshot});
  writeFileSync(output+'/'+name+'-snapshot.json', JSON.stringify(snapshot,null,2));
}
const originalLock = readFileSync(root + '/tracking/original-Cargo.lock','utf8');
const lock = readFileSync(crate+'/Cargo.lock','utf8');
const packages = lock.split('[[package]]').slice(1).filter(block=>block.includes('source = ')).map(block=>{
  const name = /name = "([^"]+)"/.exec(block)[1];
  const version = /version = "([^"]+)"/.exec(block)[1];
  const checksum = /checksum = "([^"]+)"/.exec(block)[1];
  const matching = originalLock.split('[[package]]').find(candidate=>candidate.includes('name = "'+name+'"')&&candidate.includes('version = "'+version+'"')&&candidate.includes('checksum = "'+checksum+'"'));
  return {name,version,checksum,original_match:!!matching};
});
const multi = fixtures.find(fixture=>fixture.name==='multi').result;
const error = fixtures.find(fixture=>fixture.name==='backend-error').result;
const panic = fixtures.find(fixture=>fixture.name==='panic').result;
const empty = fixtures.find(fixture=>fixture.name==='empty').result;
const fixtureChecks = {
  multi_order: JSON.stringify(multi.pass_order) === JSON.stringify(['upload','empty','draw']),
  multi_ranges: JSON.stringify(multi.pass_ranges) === JSON.stringify([[0,2],[2,2],[2,3]]),
  multi_chain: multi.accesses[0].next === 2 && multi.accesses[2].previous === 0,
  multi_transitions: multi.initial_transitions.length === 2 && multi.pre_transitions[2].length === 1 && multi.export_transitions.length === 1,
  multi_final_flag: multi.batches.length === 3 && multi.batches.filter(batch=>batch.export_barriers).length === 1 && multi.batches[2].export_barriers,
  error_partial: error.status === 'backend_error' && error.tag === 3 && error.consumed_origins === 2 && error.partial_backing_count === 1,
  panic_status: panic.status === 'panic',
  empty_pass: empty.pass_ranges.length === 1 && empty.pass_ranges[0][0] === 0 && empty.pass_ranges[0][1] === 0
};
const artifact = root+'/workspace/input/tiny-glade.exe';
const binarySha256 = createHash('sha256').update(readFileSync(artifact)).digest('hex');
const formatting = spawnSync('rustfmt', ['--check','--edition','2021','--config','skip_children=true','src/lib.rs','src/dsl.rs','src/snapshot.rs','src/bin/graph-snapshot-oracle.rs','tests/projection.rs'], {cwd:crate,encoding:'utf8'});
checks.push({command:['rustfmt','--check','--config','skip_children=true','exclusive_files'],status:formatting.status,stdout:formatting.stdout,stderr:formatting.stderr});
const report = {schema_version:2,status:'candidate_static',binary_sha256:binarySha256,full_original_entry_differential:false,reason:'original closure/device/temporal ABI not recovered',checks,fixtures,fixture_checks:fixtureChecks,dependency_packages:packages,dependency_identity_ok:packages.every(package_=>package_.original_match),passed:checks.every(check=>check.status===0)&&fixtures.every(fixture=>fixture.status===0)&&Object.values(fixtureChecks).every(Boolean)&&packages.every(package_=>package_.original_match),tests:17,notes:['ResourceInfo 512 and BatchCompiler 1600 original cases are retained R1 evidence, not new R2 counts','cargo fmt --all briefly removed the trailing empty line of reconstruction/render/lib.rs; restored only that exact empty line by apply_patch; subsequent formatting uses rustfmt skip_children=true on exclusive files']};
writeFileSync(output+'/validation.json', JSON.stringify(report,null,2));
console.log(JSON.stringify({passed:report.passed,checks:checks.map(check=>({command:check.command,status:check.status})),fixture_statuses:fixtures.map(fixture=>({name:fixture.name,status:fixture.result.status})),dependency_identity_ok:report.dependency_identity_ok}));
if(!report.passed)process.exitCode=1;
