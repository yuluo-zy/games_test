import subprocess,sys,re
import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import typeid_resolve as tid
D={}
for l in open('/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv'):
    p=l.rstrip('\n').split('\t')
    if len(p)>=5: D[p[0]]=p[4]

EMU='/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/tools/country-core-audit/dsh_emu.py'
for a in sys.argv[1:]:
    out=subprocess.run(['python3','-I',EMU,a],capture_output=True,text=True).stdout.splitlines()
    print('=====',a, re.sub(r'.*closure_env\$(\d+).*',r'env\1',out[0]) if out else '')
    i=0
    while i<len(out):
        l=out[i]
        if ' CALL ' in l:
            m=re.match(r'(\w+) CALL (\w+) (.*)',l)
            if not m:
                i+=1; continue
            name=D.get(hex(int(m.group(2),16)),m.group(3))
            name=re.sub(r'.*?(write_uniform<|read_storage_buffer<|bind_texture_to_draw_builder<|to_draw_multi_instanced<)(.*)',lambda mm: mm.group(1)+mm.group(2),name)
            name=re.sub(r'country_core::render::pass_builder::','',name)
            name=re.sub(r'country_core::','',name); name=name.replace('country_core::','').replace('render::pass_builder::','').replace('resources::texture_presets::','').replace('systems::render_loop::','').replace('draw_builder::','')
            name=name[:260]
            args=out[i+1].strip() if i+1<len(out) else ''
            args=re.sub(r' stk\w+=\?','',args); args=re.sub(r' r\d+=\?','',args); args=re.sub(r'rdx=\?|r8=\?|r9=\?','',args)
            extra=''
            if name.endswith('get_id') or name.endswith('get_resource_id'):
                mm=re.search(r'rdx=(0x[0-9a-f]+) r8=(0x[0-9a-f]+)',args)
                if mm:
                    o=None; c=dict(tid.resolve2(int(mm.group(1),16),int(mm.group(2),16)))
                    extra=' ==> '+(','.join(c) if c else 'UNRESOLVED '+str(o))
            print(m.group(1),name,'|',args,extra)
            i+=2; continue
        i+=1
