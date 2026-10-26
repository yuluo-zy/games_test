"""读取原注册器有界指令，保留真实FnSystem.name和资源初始化写入。

本探针不执行Bevy注册，只截获调用参数。未写入的enum非活动负载及padding不解释为零。
"""
from pathlib import Path
import hashlib,json,re,struct,subprocess
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE,UC_HOOK_MEM_WRITE
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parents[4]
raw=(ROOT/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
def read(va,n):return image[va-base:va-base+n]
inputs=json.loads(Path(__file__).with_name('shape-registration-contract-input.json').read_text())
assert all(not s['result_is_error'] for s in inputs['sources'].values())
wrappers=inputs['wrappers']
names=[]
for va,f in wrappers.items():
    asm=subprocess.check_output(['/opt/homebrew/opt/llvm/bin/llvm-objdump','-d','--start-address='+va,'--stop-address='+hex(int(va,16)+f['size']),str(ROOT/'B版本内容/tiny-glade.rne')],text=True)
    lines=asm.splitlines()
    for i,line in enumerate(lines):
        m=re.search(r'leaq.*# (0x142[0-9a-f]+)',line)
        if m and read(int(m[1],16),12)==b'country_core':
            size=int(re.search(r'movq\s+\$(0x[0-9a-f]+)',lines[i+2])[1],16)
            names.append({'constructor':va,'name_va':m[1],'name_len':size,'name':read(int(m[1],16),size).decode(),'instructions':lines[i:i+3]})
inline=[(0x142ae8dac,0x4c),(0x142ae8cfe,0x5b),(0x142ae8cb5,0x49),(0x142ccc8f5,0x5e),(0x142ccc953,0x61),(0x142cd639d,0x35),(0x142c5fc6c,0x29)]
inline=[{'name_va':hex(a),'name_len':n,'name':read(a,n).decode()} for a,n in inline]
types=inputs['types']
resource_sizes={0x1415424d0:80,0x141535050:80,0x141542a30:1,0x141531be0:64,0x141538d80:80,0x14154db10:44,0x14153ecb0:36,0x14154a1c0:16,0x141547450:16}
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data,stack=0x20000000,0x30000000;u.mem_map(data,0x10000);u.mem_map(stack,0x10000);u.mem_write(stack,bytes([0xaa])*0x10000)
bp=stack+0x8000;u.reg_write(UC_X86_REG_RBP,bp);u.reg_write(UC_X86_REG_RSP,stack+0x7000)
u.reg_write(UC_X86_REG_RSI,data);u.reg_write(UC_X86_REG_RAX,data)
written=set();captured=[];events=[];calls=[]
def writes(uc,access,address,size,value,opaque):written.update(range(address,address+size))
def hook(uc,address,size,opaque):
    if address==0x1409465a7:uc.emu_stop();return
    op=bytes(uc.mem_read(address,1))
    if op!=b'\xe8':return
    target=address+5+struct.unpack('<i',uc.mem_read(address+1,4))[0];calls.append(hex(target))
    if target in resource_sizes:
        n=resource_sizes[target];ptr=uc.reg_read(UC_X86_REG_RDX)
        if target==0x141542a30:values=bytes([ptr&255]);mask=[0]
        else:values=bytes(uc.mem_read(ptr,n));mask=[i for i in range(n) if ptr+i in written]
        captured.append({'call_site':hex(address),'target':hex(target),'type':types[hex(target)]['name'],'native_size':n,'explicitly_written_offsets':mask,'explicitly_written_bytes':{hex(i):values[i] for i in mask},'note':'只记录本次构造显式写入的byte；未标记字段不是已证明为零。'})
        written.clear()
    elif hex(target) in types and 'register_event<' in types[hex(target)]['name']:events.append({'call_site':hex(address),'target':hex(target),'type':types[hex(target)]['name']})
    # 所有调用都是本探针外Bevy边界，只截获参数；lookup强制missing以覆盖原register_event分支。
    uc.reg_write(UC_X86_REG_RAX,0 if target==0x1400dd6f0 else data)
    uc.reg_write(UC_X86_REG_RIP,address+5)
h1=u.hook_add(UC_HOOK_CODE,hook);h2=u.hook_add(UC_HOOK_MEM_WRITE,writes)
u.emu_start(0x140946154,0x1409465b2,count=2000)
assert u.reg_read(UC_X86_REG_RIP)==0x1409465a7
u.hook_del(h1);u.hook_del(h2)
assert len(captured)==9 and len(events)==8,(len(captured),len(events))
report={'binary_sha256':sha,'database_revision':'1791719479858:445f58e8:3','original_register':'0x140945f50','name_evidence':names,'inline_name_evidence':inline,'resources':captured,'events':events,'event_defaults':'原register_event仅缺失时创建Events：两个空Vec、各起始计数0、总event_count=0；已存在事件资源不清空。第三方Bevy语义复用。','run_if':{'name':'country_core::systems::is_gameplay_active','target':'0x140a90e00','bytes':read(0x140a90e00,22).hex(),'predicate':'GameTime+0x30 byte ==1 && State<AppLoadingState>.0 byte ==1'},'tree':{'chain_ignore_deferred':[{'chain_ignore_deferred':['shape_placement_params','shape_placement_signifiers','calc_shape_height','ui_edit_circle','signifier_edit_circle_arrow','signifier_edit_circle_line','ui_rotate_circle','ui_create_circle','create_circle']},{'chain_ignore_deferred':['ui_move_shapes','ui_move_freehand_walls','ui_edit_freehand_walls',{'chain':[['ui_edit_rectangle_dims','ui_edit_rectangle_rotate','ui_raise_shape','ui_adjust_wall_height'],['do_ui_edit_rectangle_dims','do_ui_edit_rectangle_rotate']]},'ui_focus_mode_shape_height','impl_raise_shape','ui_create_rectangle','signifier_edit_dims','create_rectangle','edit_rectangle','finish_edit_rectangle']}],'set':17,'run_if':'is_gameplay_active'},'native_execution':{'range':['0x140946154','0x1409465a7'],'boundary':'跳过全部Bevy调用，仅截获参数，lookup返回missing；不声明框架注册或业务运行通过。','hooked_calls':calls},'scope':'只读原注册树/系统名/原default显式写入，不修改production、不编译、不验收交互。'}
out=Path(__file__).with_name('shape-registration-contract-native.json');out.write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'names':len(names)+len(inline),'resources':len(captured),'events':len(events),'report':str(out)},ensure_ascii=False))
