// Khronos validator + contract parity checks. Does not alter exported assets.
// node validate-house-kit.cjs PATH_TO_KHRONOS_VALIDATOR_PACKAGE [v4]
const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const validator = require(path.resolve(process.argv[2]));
const revision = process.argv[3] || 'v4';
assert(/^[a-z0-9]+$/i.test(revision));
const root = path.resolve(__dirname, '../../assets/themes/warm-stone', `house-kit-${revision}`);
const kit = JSON.parse(fs.readFileSync(path.join(root, 'kit.json')));

function accessor(doc, binary, index) {
  const a = doc.accessors[index];
  const view = doc.bufferViews[a.bufferView];
  const dimension = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4 }[a.type];
  const size = { 5123: 2, 5125: 4, 5126: 4 }[a.componentType];
  assert(dimension && size && !a.sparse && !a.normalized);
  const stride = view.byteStride || dimension * size;
  const offset = (view.byteOffset || 0) + (a.byteOffset || 0);
  return Array.from({ length: a.count }, (_, i) => Array.from({ length: dimension }, (_, j) => {
    const pos = offset + i * stride + j * size;
    return a.componentType === 5126 ? binary.readFloatLE(pos) : a.componentType === 5123 ? binary.readUInt16LE(pos) : binary.readUInt32LE(pos);
  }));
}
function canonical(points, uv) {
  // Unordered vertex triplets tolerate exporter index-order/diagonal choices.
  return points.map((p, i) => p.concat(uv[i]).map(v => Math.round(v * 10000)).join(',')).sort().join(';');
}
(async () => {
  const reports = {};
  for (const [name, module] of Object.entries(kit.modules)) {
    const file = path.join(root, name + '.glb');
    const bytes = fs.readFileSync(file);
    const result = await validator.validateBytes(new Uint8Array(bytes), { uri: name + '.glb', maxIssues: 1000 });
    const length = bytes.readUInt32LE(12);
    const doc = JSON.parse(bytes.subarray(20, 20 + length).toString());
    const binary = bytes.subarray(28 + length);
    assert.equal(result.issues.numErrors, 0, `${name}: glTF validation errors`);
    assert.equal(result.issues.numWarnings, 0, `${name}: glTF validation warnings`);
    const actual = {};
    for (const node of doc.nodes) {
      if (node.mesh === undefined) continue;
      assert(!node.matrix && !node.translation && !node.rotation && !node.scale, 'kit nodes must be identity in Y-up meters');
      for (const primitive of doc.meshes[node.mesh].primitives) {
        const role = doc.materials[primitive.material].name.replace('MAT_WarmStone_', '').toLowerCase();
        const positions = accessor(doc, binary, primitive.attributes.POSITION);
        const uvs = accessor(doc, binary, primitive.attributes.TEXCOORD_0);
        const indices = accessor(doc, binary, primitive.indices).flat();
        const vertices = indices.map(i => positions[i]);
        const uv = indices.map(i => uvs[i]);
        actual[role] ||= { positions: [], uvs: [] };
        actual[role].positions.push(...vertices);
        actual[role].uvs.push(...uv);
      }
    }
    for (const [role, batch] of Object.entries(module.batches)) {
      assert.equal(actual[role].positions.length, batch.indices.length);
      assert.equal(canonical(actual[role].positions, actual[role].uvs), canonical(batch.indices.map(i => batch.positions[i]), batch.indices.map(i => batch.uvs[i])), `${name}/${role}: GLB/compiled geometry or UV mismatch`);
    }
    reports[name] = { sha256:crypto.createHash('sha256').update(bytes).digest('hex'), errors: result.issues.numErrors, warnings: result.issues.numWarnings, messages: result.issues.messages, triangles: Object.values(module.batches).reduce((n,b)=>n+b.indices.length/3,0), glb_compiled_position_uv_parity: true };
    console.log(name, reports[name].triangles, 'triangles; zero errors; source/GLB/kit parity');
  }
  const report=JSON.stringify({ validator_version: validator.version(), kit_sha256:crypto.createHash('sha256').update(fs.readFileSync(path.join(root,'kit.json'))).digest('hex'), assets: reports }, null, 2);
  if(process.argv.includes('--report')) fs.writeFileSync(path.join(root,'gltf-validation.json'),report+'\n',{flag:'wx'});
  else console.log(report);
})().catch(error => { console.error(error); process.exitCode = 1; });
