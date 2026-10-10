import {parse} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/smol-toml/dist/index.js';
import {readFileSync,writeFileSync,copyFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const root='D:/game/reverse-engineering/tiny-glade';
const original='D:/game/ljxsj_92385/Tiny Glade/build-info/Cargo.lock';
const raw=readFileSync(original);const lock=parse(raw.toString('utf8'));
const privateCandidates=new Set(['tiny-glade','country-core','system-roof','system-wall-constructor','system-decorator','system-clutter','system-color','system-critters','system-glade-border','system-onboarding','system-tree','system-water','forest','timberframe','paths','utils','asset','asset-pipe','asset-pipe-bin','lang-lib','rhapsody','state-stream','toolbar','tg-text','tg-iced','tabloid','motomoto','theme-defs','theme-fetcher','theme-server-proto']);
const packages=lock.package.map(p=>({...p,
 policy:p.source?'reuse_exact_source_do_not_reverse':privateCandidates.has(p.name)?'game_owned_candidate_verify_source_attribution':'reuse_or_classify_first_do_not_reverse_by_default',
 ownership_verified:false,
 reason:p.source?'公开依赖，复用锁定版本/commit；不恢复库实现':privateCandidates.has(p.name)?'候选游戏私有模块；按PDB源路径/调用责任核实具体函数':'local/path不等于游戏自有；优先查已有上游或SDK'}));
const engine={name:'Bevy fork',provenance:'explicit_user_instruction',confirmed_lock_family_version:'0.16.0',fork_repository:null,fork_commit:null,
 rule:'复用Bevy/分叉；只研究游戏调用、数据及必要分叉差异，禁止重新还原整个引擎'};
const report={original_lock_sha256:createHash('sha256').update(raw).digest('hex'),engine,package_count:packages.length,
 original_lock_copy:'tracking/original-Cargo.lock',packages,
 excluded_symbol_prefixes:['std::','core::','alloc::','bevy_','glam::','fastrand::','half::','serde::','serde_json::','ron::','rkyv::','slotmap::','smallvec::','ash::','winit::'],
 rule:'锁定同名多版本的实际调用方；公开git fork也是依赖，不复制或逆向其实现。游戏泛型实例可观察数据边界，不恢复容器/引擎/解析器实现。'};
copyFileSync(original,root+'/tracking/original-Cargo.lock');
writeFileSync(root+'/tracking/dependency-policy.json',JSON.stringify(report,null,2));
console.log(JSON.stringify({packages:packages.length,reuse_exact:packages.filter(p=>p.source).length,private_candidates:packages.filter(p=>p.policy.startsWith('game_')).length,engine}));
