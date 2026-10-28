"""完整原 DailyThemeInfo::get_name 有限输入 oracle；无依赖桥、无 OS 输入。"""
from pathlib import Path
import hashlib,json,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import UC_X86_REG_RSP,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_RAX,UC_X86_REG_RIP
root=Path(__file__).resolve().parents[4]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes();digest=hashlib.sha256(raw).hexdigest()
assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data,stack,stop=0x20000000,0x30000000,0x20005000
u.mem_map(data,0x6000);u.mem_map(stack,0x10000)
for i in range(15):
 value=f'Theme{i}'.encode();u.mem_write(data+0x100+24*i,struct.pack('<QQQ',len(value),data+0x1000+i*32,len(value)));u.mem_write(data+0x1000+i*32,value)
locales=['en-US','de-DE','es-ES','fr-FR','it-IT','ja-JP','ko-KR','pl-PL','pt-BR','ru-RU','sv-SE','tr-TR','uk-UA','zh-CN','no-NO','unsupported']
rows=[]
for length in range(16):
 u.mem_write(data,struct.pack('<QQQ',15,data+0x100,length))
 for locale in locales:
  value=locale.encode();u.mem_write(data+0x3000,value);rsp=stack+0xf008;u.mem_write(rsp,struct.pack('<Q',stop))
  for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,data+0x3000),(UC_X86_REG_R8,len(value))]:u.reg_write(reg,val)
  u.emu_start(0x14083c240,stop,count=10000);assert u.reg_read(UC_X86_REG_RIP)==stop
  result=bytes(u.mem_read(u.reg_read(UC_X86_REG_RAX),u.reg_read(UC_X86_REG_RDX))).decode()
  rows.append((locale,length,result))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-daily-name.ron'
fixture.write_text('[\n'+''.join('('+json.dumps(locale)+','+str(length)+','+json.dumps(result)+'),\n' for locale,length,result in rows)+']\n')
Path(__file__).with_name('toolbar-daily-name-native.json').write_text(json.dumps({'binary_sha256':digest,'function':'0x14083c240','size_bytes':511,'whole_function':True,'bridges':[],'cases':len(rows),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'scope':'16个语言表长度、14个原语言及no-NO/未知语言，全原函数；仅有界命名行为，不代表主题下载/游戏验收。'},ensure_ascii=False,indent=2)+'\n')
print('native cases',len(rows))
weekly=[]
threshold=432_000*10_000_000
for start in [0,1,116_444_736_000_000_000,2**63,2**64-threshold-2]:
 for delta in [-1,0,1,threshold-1,threshold,threshold+1]:
  end=start+delta
  if not 0<=end<2**64:continue
  u.mem_write(data+0xe0,struct.pack('<QQ',start,end));rsp=stack+0xf008;u.mem_write(rsp,struct.pack('<Q',stop))
  u.reg_write(UC_X86_REG_RSP,rsp);u.reg_write(UC_X86_REG_RCX,data)
  u.emu_start(0x14083c440,stop,count=10000);assert u.reg_read(UC_X86_REG_RIP)==stop
  weekly.append((start,end,bool(u.reg_read(UC_X86_REG_RAX)&0xff)))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-daily-weekly.ron'
fixture.write_text('[\n'+''.join(f'({start},{end},{str(result).lower()}),\n' for start,end,result in weekly)+']\n')
Path(__file__).with_name('toolbar-daily-weekly-native.json').write_text(json.dumps({'binary_sha256':digest,'function':'0x14083c440','size_bytes':61,'whole_function':True,'callees_executed':['0x140d63bd0'],'bridges':[],'cases':len(weekly),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'scope':'Windows SystemTime原始有符号100ns位值，五天±100ns/倒序/epoch/符号位边界。全原is_weekly及原duration_since，无桥。macOS测试按i64位解释后做相同epoch平移；高位案例不宣称有效活动日历时间。'},ensure_ascii=False,indent=2)+'\n')
print('native weekly cases',len(weekly))
