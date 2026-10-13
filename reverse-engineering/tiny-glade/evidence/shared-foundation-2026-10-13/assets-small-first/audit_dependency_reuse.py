"""Audit the delivered B chain, not historical reconstruction outside these crates."""
import hashlib,json,pathlib,re,tomllib
OUT=pathlib.Path(__file__).resolve().parent;ROOT=OUT.parents[2]
WORK=ROOT/'reconstruction/shared-foundation'
original_path=ROOT/'tracking/original-Cargo.lock';lock_path=WORK/'Cargo.lock'
original=tomllib.loads(original_path.read_text(encoding='utf8'))['package']
locked=tomllib.loads(lock_path.read_text(encoding='utf8'))['package']
def identity(p):return {k:p.get(k) for k in ['name','version','source','checksum']}
def find_dependency(dep):
    parts=dep.split();name=parts[0];version=parts[1] if len(parts)>1 and re.match(r'^\d+\.',parts[1]) else None
    matches=[p for p in locked if p['name']==name and (version is None or p['version']==version)]
    assert len(matches)==1,(dep,[identity(p) for p in matches])
    return matches[0]
queue=[next(p for p in locked if p['name']==name) for name in ['asset','asset-pipe']]
seen={}
while queue:
    p=queue.pop();key=(p['name'],p['version'],p.get('source'))
    if key in seen:continue
    seen[key]=p;queue.extend(find_dependency(d) for d in p.get('dependencies',[]))
packages=[]
for p in seen.values():
    if not p.get('source'):continue
    exact=[q for q in original if identity(q)==identity(p)]
    packages.append({'identity':identity(p),'original_lock_exact_match':bool(exact)})
assert all(p['original_lock_exact_match'] for p in packages)
public=json.loads((ROOT/'evidence/render-architecture/public-symbols.json').read_text(encoding='utf8'))
names={r['va']:r['demangled'] for r in public if r.get('va')}
pdb=json.loads((ROOT/'evidence/shared-foundation-2026-10-13/assets/functions.json').read_text(encoding='utf8'))
for row in pdb:names[row['va']]=row['name']
queried=json.loads((OUT/'bounded-report.json').read_text(encoding='utf8'))
callee_scope=json.loads(queried['results'][1]['stdout'])['functions']
def references(addresses):
    return [{'address':a,'PDB_name':names.get(a),'name_is_not_ABI':True} for a in addresses]
rows=[
 {'scope':'asset-pipe::texture::{process_regular_image,process_decoded_image,next_level}',
  'classification':'original_private_processing_policy + original_dependency_wrapper',
  'original_definition':['0x1417bbc10','0x1417bc930'],
  'calls':['image::io::Reader::open/decode','DynamicImage::resize_exact(FilterType::CatmullRom)',
           'DynamicImage::{to_rgba8,to_rgba32f,into_rgba8,into_rgba32f}'],
  'references':references(['0x140ba3a10','0x1420e2200','0x140badad0','0x140bad8d0','0x140bad960','0x140bad9f0','0x140bada60']),
  'private_work':'gamma strategy, dimension alignment, mip count, base sampling and RGBA8 edge materialization',
  'not_reimplemented':['PNG/JPEG decoding','CatmullRom filter kernel/resampling','float→u8 image conversion'],
  'validation':'12 passed crate tests; complete original pipeline differential pending absent source images'},
 {'scope':'asset-pipe::texture::{compress,bc7_settings}',
  'classification':'original_private_channel/preset_policy + original_dependency_wrapper',
  'original_definition':['0x1417bcd60'],
  'calls':['intel_tex_2::bc4::compress_blocks','intel_tex_2::bc5::compress_blocks','intel_tex_2::bc7::compress_blocks',
           'intel_tex_2::bc7::{opaque_basic_settings,alpha_basic_settings}'],
  'references':references(['0x1420e3720','0x1420e37b0','0x1420e3830']),
  'private_work':'R or R+A extraction and opaque/alpha basic-preset choice',
  'not_reimplemented':['BC4 encoder','BC5 encoder','BC7 encoder','ISPC SIMD kernels'],
  'validation':'original preset constants checked; contracts cover formats/mips/alpha choice; no native compressed-pixel differential'},
 {'scope':'asset-pipe::texture::{process_dds_image,process_dds}',
  'classification':'original_private_format/selection_policy + original_dependency_wrapper',
  'original_definition':['inlined 0x1417ba565..0x1417bafb7 within compile_texture_asset'],
  'calls':['ddsfile::Dds::read','Dds::{get_dxgi_format,get_d3d_format,get_data(0),get_num_mipmap_levels,get_depth,get_width,get_height}'],
  'references':references(['0x1420e1010','0x1417b7290','0x1420e1f30']),
  'private_work':'original whitelist, Vulkan format mapping and layer-zero-only copy',
  'not_reimplemented':['DDS header parser','DDS format pitch/array-stride algorithm','DDS codec'],
  'validation':'6 DXGI/2 legacy, layer-zero and rejection fixtures'},
 {'scope':'asset::{inheritable,prefab/config,prefab/scan,texture/config,texture/desc,texture/scan}',
  'classification':'original_private_inheritance/scan_policy + original_dependency_wrapper',
  'original_definition':['0x142009df0','0x14200a1b0','0x14200a5f0','0x14200ba50','0x1420d8ba0','0x1420d8d00','0x142456250','0x14268aa60'],
  'calls':['serde::Deserialize/Serialize derive','ron::from_str','ron::value::RawValue::deserialize','ron::Options::from_str(IMPLICIT_SOME)'],
  'references':references(['0x14268aa60','0x14268aee0','0x14268b260','0x14268b770']),
  'private_work':'defaults, missing vs explicit None, inheritance, selection of asset extensions and contiguous array names',
  'not_reimplemented':['RON lexer/parser','Serde visitor framework','enum parsing','generic JSON/RON parser'],
  'validation':'25 shipped prefab/28 dependency files; bare/Some/None/unit/bad enum contracts',
  'adaptation':'RawValue preserves enum names despite original fork Content type_name mismatch on Rust 1.97; fork source unchanged'},
 {'scope':'asset::texture::blob::TextureAssetBlob::{load,to_file_bytes}',
  'classification':'original_private_wire_schema + synthetic_safe_adapter + original_dependency_wrapper',
  'original_definition':['0x1417b55c0 producer specialization; original .texture roots'],
  'calls':['rkyv::Archive/Serialize/Deserialize derive','rkyv::check_archived_root','rkyv::Deserialize::deserialize','rkyv::to_bytes','bytecheck::CheckBytes derive'],
  'references':references(['0x1417b55c0']),
  'private_work':'magic/version header and recovered root field order; library performs relative-pointer/archive encoding',
  'not_reimplemented':['RKYV archive encoder','RKYV relative pointers','archive validation','generic serializer'],
  'validation':'388 original archives, 311679016 bytes, byte-identical roundtrip; native Rust type names not proven'},
 {'scope':'paths and filesystem in both owned crates',
  'classification':'original_dependency_wrapper + private_directory/naming_policy',
  'calls':['relative_path::{RelativePath,RelativePathBuf}::{join,with_extension,to_path}',
           'std::path::{Path,PathBuf}','std::fs::{read_dir,metadata,read_to_string,File::open,write,create_dir_all}',
           'std::io::Read::read_exact','std::time::SystemTime'],
  'references':references(['0x1420dfc40','0x1420dfdb0','0x1410312d0','0x140d76ed0']),
  'not_reimplemented':['path normalization/relative-path parsing','filesystem backend','OS timestamp','platform directory provider'],
  'validation':'scanning, compilation/cache/rebuild temporary fixture; no original installation mutated'},
 {'scope':'containers/names in asset-pipe metadata, asset inheritance/dependencies and PrefabName',
  'classification':'original_private_map/relationship_policy + standard_library_wrapper',
  'calls':['std::collections::HashMap','std::vec::Vec','std::sync::Arc<str>','std::option::Option','std::hash::Hash','std::borrow::Borrow','std::ops::Deref','std::clone::Clone'],
  'references':references(['0x141556080','0x141556130','0x142009df0','0x142424d90']),
  'private_work':'child field precedence, atomic shallow inheritance vs recursive merge, duplicate dependency vector entries, exact borrowed UTF-8 name view, names sorted and layer-major concatenation',
  'not_reimplemented':['hash table','vector allocator/growth','Arc reference counter','sorting algorithm'],
  'validation':'shipped prefab scan+inheritance, original 160000 shallow cases, exact name pointer/hash/standard HashMap Borrow tests; interface-batch-02/business-comparison.json'},
 {'scope':'asset-pipe::{srgb_oetf,srgb_eotf,mip_count,pad_edges,align,assemble_array,cache_is_current}',
  'classification':'original_private_numeric/input/output_policy; standard and image primitives reused',
  'original_definition':['0x1417bcd20','0x1417bbc10','0x1417bc930','0x1417b8f50'],
  'calls':['f32::powf','u32::leading_zeros','image::RgbaImage::{from_fn,get_pixel}','Vec::extend'],
  'private_work':'thresholds, mip limits, four-texel alignment, edge replication and game-specific cache/header predicates',
  'not_reimplemented':['powf','bit-scan instruction','image filter','archive codec','container implementation'],
  'validation':'OETF original64B 4364 cases zero differences; all other policies contract-tested/control-flow compared'},
 {'scope':'asset/src/lib.rs; asset/src/texture.rs; asset-pipe/src/lib.rs',
  'classification':'synthetic_module_glue',
  'private_work':'pub mod declarations; asset/src/texture.rs is not restoration of C original texture.rs',
  'not_reimplemented':['external algorithms'],
  'validation':'crate compilation/clippy; glue not counted as a recovered original function'},
 {'scope':'asset-pipe::texture::compile_texture_assets task scheduling',
  'classification':'original_private_task/result/persistence_policy + original_dependency_wrapper',
  'original_definition':['0x1417b7790'],
  'original_dependency':'easy-parallel3.2.0 Parallel::each→Parallel::run',
  'references':references(['0x1420dd5e0','0x1420dc1a0']),
  'current_behavior':'one closure per descriptor, original dependency spawns/joins tasks, results collected in insertion order; metadata/list persist only after all Results succeed',
  'not_reimplemented':['thread-pool algorithm','generic parallel scheduler','worker queue'],
  'validation':'10 pipeline contracts including successful compilation, error does not prevent other output, panic joins tasks before unwinding, no index/meta persistence on failure; original each/run call sites and exact dependency API verified',
  'strategy':'each original API has no threadcount/chunk parameter; no invented pool limit/configuration',
  'next_action':'Original outer logging/CLI flags and full native runtime differential remain separate gaps'}]
source_files=[]
for crate in ['asset','asset-pipe']:
    for p in sorted((WORK/'crates'/crate).rglob('*.rs')):
        source_files.append({'path':p.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
native_callees=[]
for f in callee_scope:
    named=[{'address':'0x'+c['address'],'PDB_name':names.get('0x'+c['address'])} for c in f.get('callees',[])]
    native_callees.append({'private_entry':f['address'],'callees':named})
report={'schema_version':1,'baseline':'B','scope':'only delivered reconstruction/shared-foundation/crates/asset and asset-pipe',
 'dependency_reuse_verified':True,'replacement_findings':[],
 'conclusion':'No generic-library wheel found among implemented algorithms in these delivered crates; codec/filter/parser/container/path/task scheduling algorithms delegate directly to original locked dependencies. Private policy identified by original definition/caller evidence.',
 'original_lock_sha256':hashlib.sha256(original_path.read_bytes()).hexdigest(),
 'current_shared_lock_sha256':hashlib.sha256(lock_path.read_bytes()).hexdigest(),
 'reachable_external_packages':len(packages),'all_reachable_external_version_source_checksum_match_original':True,
 'external_packages':sorted(packages,key=lambda r:(r['identity']['name'],r['identity']['version'])),
 'function_module_classification':rows,'bounded_original_callees':native_callees,'source_files':source_files,
 'original_dependency_feature_sets_fully_verified':False,
 'dependency_sources_modified':False,'library_implementations_reversed_or_reimplemented':False,
 'tests_repeated_for_audit':True,'test_reason':'Original easy-parallel and new original-private inheritance/borrow wrappers; current 19 crate tests including 388 archive roundtrips and error/panic completion contracts passed, strict Clippy; shallow native160000 and unchanged OETF native4364 evidence',
 'evidence':['delivery.json','bounded-report.json','gamma-verification.json','cargo-test.log','clippy.log',
             '../assets/functions.json','../assets/asm-0x1417bbc10.txt','../assets/asm-0x1417bc930.txt','../assets/asm-0x1417bcd60.txt',
             '../assets/asm-0x1417b8f50.txt'],
 'limits':['Audit does not certify historical reconstruction outside these two crates',
           'Exact original dependency features/fork patches beyond locked source identities remain separate evidence gaps',
           'Full original image pipeline/compressed-payload differential absent source PNG/JPG/DDS remains pending',
           'PDB names and raw callee addresses are provenance, not verified Rust ABI or original source-text identity']}
missing=[r for row in rows for r in row.get('references',[]) if not r['PDB_name']]
report['unresolved_reference_names']=missing
(OUT/'dependency-reuse-audit.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print(json.dumps({'packages':len(packages),'exact_original_identity':True,'scopes':len(rows),'wheel_findings':0,'unresolved_names':len(missing)}))
