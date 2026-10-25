"""从原 ash 头文件版本的 Khronos 注册表生成格式元数据，不手写公开格式算法。"""
from pathlib import Path
import xml.etree.ElementTree as E,json,hashlib,sys,subprocess,datetime
ROOT=Path(__file__).resolve().parents[1];source=ROOT/'evidence/vk-registry-1.3.281.xml';xml=E.parse(source).getroot()
values={};aliases={}
for group in xml.findall("./enums[@name='VkFormat']"):
 for enum in group.findall('enum'):
  if 'value' in enum.attrib:values[enum.attrib['name']]=int(enum.attrib['value'],0)
  elif 'alias' in enum.attrib:aliases[enum.attrib['name']]=enum.attrib['alias']
for extension in xml.findall('./extensions/extension'):
 for enum in extension.findall("./require/enum[@extends='VkFormat']"):
  a=enum.attrib
  if 'alias' in a:aliases[a['name']]=a['alias']
  elif 'offset' in a:values[a['name']]=(1000000000+(int(a.get('extnumber',extension.attrib['number']))-1)*1000+int(a['offset']))*(-1 if a.get('dir')=='-' else 1)
  elif 'value' in a:values[a['name']]=int(a['value'],0)
for feature in xml.findall('./feature'):
 for enum in feature.findall("./require/enum[@extends='VkFormat']"):
  a=enum.attrib
  if 'alias' in a:aliases[a['name']]=a['alias']
  elif 'offset' in a:values[a['name']]=(1000000000+(int(a['extnumber'])-1)*1000+int(a['offset']))*(-1 if a.get('dir')=='-' else 1)
  elif 'value' in a:values[a['name']]=int(a['value'],0)
for _ in range(5):
 for name,alias in aliases.items():
  if alias in values:values[name]=values[alias]
formats=[]
for f in xml.find('formats'):
 a=f.attrib;name=a['name'];value=values[name];extent=[int(n) for n in a.get('blockExtent','1,1,1').split(',')]
 components={c.attrib['name'] for c in f.findall('component')};aspect=(2 if 'D' in components else 0)|(4 if 'S' in components else 0)
 if not aspect:aspect=1
 planes=[{'width_divisor':int(p.attrib['widthDivisor']),'height_divisor':int(p.attrib['heightDivisor']),'compatible':values[p.attrib['compatible']],'compatible_name':p.attrib['compatible'].removeprefix('VK_FORMAT_')} for p in f.findall('plane')]
 formats.append({'name':name,'value':value,'extent':extent,'bytes':int(a['blockSize']),'aspect':aspect,'planes':planes})
lines=['//! 自动生成的公开 Vulkan 格式元数据。来源：Khronos Vulkan-Headers v1.3.281 vk.xml。','//! 生成器：scripts/generate_formats.py；不代表原作私有格式分支已对照。','use super::{FormatInfo,PlaneInfo};','use ash::vk;','pub fn registry_info(format:vk::Format)->Option<FormatInfo>{Some(match format.as_raw(){']
for f in formats:lines.append(f"    {f['value']} => FormatInfo{{block_extent:{f['extent']},block_bytes:{f['bytes']},aspect:vk::ImageAspectFlags::from_raw({f['aspect']})}}, // {f['name']}")
lines+=['    _=>return None,','})}','pub fn registry_planes(format:vk::Format)->&\'static[PlaneInfo]{match format.as_raw(){']
for f in formats:
 if f['planes']:
  expr=','.join(f"PlaneInfo{{width_divisor:{p['width_divisor']},height_divisor:{p['height_divisor']},compatible:vk::Format::{p['compatible_name']}}}" for p in f['planes'])
  lines.append(f"    {f['value']} => &[{expr}],")
lines+=['    _=>&[],','}}']
# 原私有表从已确认 SHA 的原机器码映像提取，与公共 XML 元数据分开命名。
sys.path.insert(0,str(ROOT/'scripts/native_support'))
import pe_fixture as native
lines += ['// 原构建观测数据表；源码类型名称未知，不把公共注册表冒充原表。',f'// 原PE SHA256：{native.EXPECTED_SHA}']
for name,address,count,kind in [('OBSERVED_ASPECTS',0x142ccd890,184,'I'),('OBSERVED_INFO_WORD00',0x142ccdb70,147,'I'),('OBSERVED_INFO_WORD04',0x142ccddbc,147,'I'),('OBSERVED_INFO_BYTE08',0x142cce008,147,'B')]:
 table=list(native.read(address,f'<{count}{kind}'))
 ty='u32' if kind=='I' else 'u8'
 lines.append(f'// 原表地址：{address:#x}。')
 lines.append(f'pub const {name}:[{ty};{count}]={table};')
(ROOT/'src/raw/format_table.rs').write_text('\n'.join(lines)+'\n',encoding='utf-8')
(ROOT/'evidence/format-registry.json').write_text(json.dumps({'source':'https://github.com/KhronosGroup/Vulkan-Headers/blob/v1.3.281/registry/vk.xml','xml_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'header_version_matches_original_ash':'0.38.0+1.3.281','format_count':len(formats),'formats':formats,'scope':'公开Vulkan完整注册表元数据；未知future格式返回None，原私有函数布局仍未全部对照'},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('格式元数据条目',len(formats))

if '--validate' in sys.argv:
 subprocess.run(['rustfmt','--edition','2024',str(ROOT/'src/raw/format_table.rs')],check=True)
 subprocess.run(['cargo','test','--offline','--locked','--manifest-path',str(ROOT/'Cargo.toml'),'--test','formats'],check=True)
 result=subprocess.run(['cargo','test','--offline','--locked','--manifest-path',str(ROOT/'Cargo.toml'),'--test','formats','emit_original_comparison_fixture','--','--nocapture'],check=True,text=True,capture_output=True)
 line=next(line for line in result.stdout.splitlines() if line.startswith('FORMAT_CANDIDATE_JSON='))
 candidate=json.loads(line.split('=',1)[1])
 from unicorn.x86_const import UC_X86_REG_RSP,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RAX
 def execute(entry,rcx,rdx=0):
  native.STATE.clear();native.STATE.update(visited=set(),cut=native.STOP,cut_reached=False,panic='ok')
  native.write(native.STACK+0x10008,'<Q',native.STOP)
  for register,value in [(UC_X86_REG_RSP,native.STACK+0x10008),(UC_X86_REG_RCX,rcx),(UC_X86_REG_RDX,rdx)]:native.CPU.reg_write(register,value)
  native.CPU.emu_start(entry,native.STOP,count=10000)
  return native.CPU.reg_read(UC_X86_REG_RAX)&0xffffffff if native.STATE['panic']=='ok' else None
 cases=[]
 for case in candidate['cases']:
  raw=case['format']&0xffffffff
  aspects=execute(0x141738fe0,raw)
  block=execute(0x1417390e0,raw)
  native.CPU.mem_write(native.MEMORY,bytes([0x5a]*16))
  pointer=execute(0x141739060,native.MEMORY,raw)
  output=list(bytes(native.CPU.mem_read(native.MEMORY,16)))
  info=output[:10] if pointer is not None else None
  assert aspects==case['aspects'] and block==case['block_bytes'] and info==case['info'] and output==case['output_bytes'],(case,aspects,block,info,output)
  cases.append(dict(case,native_aspects=aspects,native_block_bytes=block,native_info=info))
 tables=[]
 for name,address,count,kind in [('aspects',0x142ccd890,184,'I'),('info_word00',0x142ccdb70,147,'I'),('info_word04',0x142ccddbc,147,'I'),('info_byte08',0x142cce008,147,'B')]:
  data=bytes(native.CPU.mem_read(address,count*(4 if kind=='I' else 1)))
  tables.append({'table':name,'address':hex(address),'entries':count,'sha256':hashlib.sha256(data).hexdigest()})
 inputs=['src/raw/format.rs','src/raw/format_table.rs','tests/formats.rs','scripts/generate_formats.py','scripts/native_support/pe_fixture.py','evidence/format-registry.json','evidence/vk-registry-1.3.281.xml','Cargo.toml','Cargo.lock']
 report={'timestamp_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':'original_format_entries_and_public_registry_validated','original_binary_sha256':native.EXPECTED_SHA,
 'original_entries':{'aspect':'0x141738fe0','info':'0x141739060','block_bytes':'0x1417390e0'},'format_inputs':len(cases),'original_entry_comparisons':len(cases)*3,'mismatches':0,'tests_passed':5,'cases':cases,'original_tables':tables,
 'observed_semantics':{'aspect_domain':'1..=184，其他格式panic','info_domain':'0..=146，含UNDEFINED；其他格式panic','info_writes':'原入口仅写out+0/4两个u32和+8/+9同一u8，共10字节；尾部保持','block_bytes_domain':{'1..=130':1,'131,132,139':8,'137,138,141,142,145,146':16,'other':'panic'},'block_bytes_warning':'原返回不能等同Khronos每像素/块存储字节数；BC1_RGBA/BC2/BC6等原函数明确失败'},
 'public_registry':{'source':'https://github.com/KhronosGroup/Vulkan-Headers/blob/v1.3.281/registry/vk.xml','xml_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'formats':len(formats),'namespace':'raw::format::khronos','legacy_top_level_api':'保持公共注册表候选兼容，不冒充原同名入口'},
 'consumer_evidence':[{'consumer':'ResourceRegistry::populate_resources','va':'0x140be7ba0','sites':['0x140be8350','0x140be8362'],'observed':'随后比较两个word及末尾u16，共10字节info'},{'consumer':'GraphicsDevice::init_texture_data','va':'0x140bfe270','sites':['0x140bfe31a','0x140bfe3c1'],'observed':'一个初始化分支要求block_bytes返回1；另一个取info+4及+8/+9生产数据布局参数'},{'consumer':'GraphicsDevice::update_texture_impl','va':'0x140bfeac0','site':'0x140bfeb8d','evidence':'已保存workgraph直接调用边，未声称完整消费者行为恢复'}],
 'boundaries':['运行原三个完整函数本体，unsupported格式在公共panic helper处停止，未替换格式函数或表','Rust integration test编译产物打印候选输出，原info额外核对out10..15未改','只验证格式入口与公共元数据；未新增原完整texture初始化/更新consumer oracle'],
 'limitations':['10字节是观察到的写入载体，不代表原类型/全部ABI获取','两个word与普通通道数/存储大小、两个byte与block宽高吻合属于语义推断，字段名仍保留偏移','标准XML元数据与原私有函数分开；实际GPU调用者保留兼容候选API','无原构建未来/扩展格式处理推断；保持原有限域失败'],
 'input_hashes':{name:hashlib.sha256((ROOT/name).read_bytes()).hexdigest() for name in inputs}}
 (ROOT/'evidence/format-functional-validation.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
 print(json.dumps({'format_inputs':len(cases),'original_entry_comparisons':len(cases)*3,'mismatches':0,'public_formats':len(formats)}))
