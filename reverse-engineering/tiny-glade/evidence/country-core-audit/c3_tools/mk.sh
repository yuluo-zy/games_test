#!/usr/bin/env bash
# 生成 /tmp/h_c3（隔离测试台）并运行测试。用法: mk.sh [cargo test args]
set -euo pipefail
export PATH=$HOME/.cargo/bin:$PATH
REAL=/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/reconstruction/country-core/src
/tmp/a4c_prep.sh
H=/tmp/h_c3; W=/tmp/w_c3
rm -rf $H $W; mkdir -p $H; cp -r /tmp/h_a4_compute/. $H/
# 挂载树（#[path] 文件按 mod-rs 解析子模块，故用符号链接重排）
M=$H/mnt; mkdir -p $M
ln -s $REAL/systems/render_loop/indirect_params.rs $M/indirect_params.rs
ln -s $REAL/systems/render_loop/indirect_params/pass_env.rs $M/pass_env.rs
ln -s $REAL/systems/render_loop/indirect_params/pass_env $M/pass_env
python3 -I - "$REAL" "$H" <<'PY'
import sys,os,glob
real,h=sys.argv[1],sys.argv[2]
mods=sorted(os.path.basename(f)[:-3] for f in glob.glob(real+'/systems/render_loop/compute_passes/*.rs') if os.path.basename(f)[:-3] in os.environ.get('C3_MODS','').split(','))
s=open(h+'/src/lib.rs').read()
s+='\npub mod systems { pub mod render_loop {\n    #[path="'+h+'/mnt/indirect_params.rs"] pub mod indirect_params;\n    pub mod compute_passes {\n'
for m in mods:
    s+='        #[path="@SRC@/systems/render_loop/compute_passes/%s.rs"] pub mod %s;\n'%(m,m)
s+='    }\n}}\n'
s=s.replace('pub mod typed_gap {','#[path="@SRC@/resources/cursor_terrain_raycast.rs"]\npub mod cursor_terrain_raycast_mount;\npub mod typed_gap {',1) if False else s
s=s.replace("pub mod resources {","pub mod resources {\n    #[path=\"@SRC@/resources/cursor_terrain_raycast.rs\"]\n    pub mod cursor_terrain_raycast;",1)
s=s.replace("pub struct Gap;","#[derive(Clone,Copy,Debug,Default,PartialEq,Eq,Hash)]\npub struct Gap;")
open(h+"/src/lib.rs","w").write(s)
# xxh3 占位：blue_noise 取真实值，其余 FNV
p=h+'/xxhash_shim/src/lib.rs'
t=open(p).read().replace('let mut h: u64 = 0xcbf29ce484222325;','if bytes == b"blue_noise" { return 0x579f39ab91a01865; }\n        let mut h: u64 = 0xcbf29ce484222325;')
open(p,'w').write(t)
PY
python3 -I - $H <<'PY2'
import sys,re
h=sys.argv[1]
p=h+'/src/render/mod.rs'
t=open(p).read()
i=t.index('#[path = "@SRC@/render/mesh_atlas.rs"]')
t=t[:i]
open(p,'w').write(t)
q=h+'/src/lib.rs'
u=open(q).read().replace('#[path = "@SRC@/resources/render/mod.rs"]\n    pub mod render;','')
open(q,'w').write(u)
PY2
cp -r $H/. $W/ 2>/dev/null || { mkdir -p $W; cp -r $H/. $W/; }
find $W/src -name '*.rs' -print0 | xargs -0 sed -i "s|@SRC@|$REAL|g"
cd $W && CARGO_TARGET_DIR=/tmp/l0h-target cargo test --offline "$@" 2>&1
