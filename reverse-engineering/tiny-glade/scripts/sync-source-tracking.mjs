import fs from 'node:fs';
import path from 'node:path';

// Record proven partial source scopes separately from final system acceptance.
const root = 'D:/game/reverse-engineering/tiny-glade';
const file = path.join(root, 'tracking/project.json');
const project = JSON.parse(fs.readFileSync(file, 'utf8'));
project.project_memory = 'PROJECT_MEMORY.md';
project.source_task_backlog = 'TASK_BACKLOG.md';
project.dependency_policy = 'tracking/dependency-policy.json';
project.engine = {
  family: 'Bevy', version: '0.16.0', fork: true,
  fork_provenance: 'explicit user statement',
  version_provenance: 'original Cargo.lock and PDB',
  fork_repository: null, fork_commit: null,
  policy: 'reuse dependency code; recover only game rules and necessary evidenced fork differences',
};
project.dependency_reuse = {
  status: 'verified', report: 'evidence/roof-tiles/dependency-boundary.json',
  versions: { fastrand: '1.8.0', glam: '0.29.3', half: '2.4.1' },
  counted_as_game_source_recovery: false,
};
const wallReport = 'evidence/scope-priorities/wall-rules-verification.json';
const proof = JSON.parse(fs.readFileSync(path.join(root, wallReport), 'utf8'));
if (proof.ok !== true) throw new Error('Wall source proof failed');
const wall = project.milestones.find(m => m.id === 'M08');
wall.status = 'in_progress';
wall.source_gate = {
  status: 'verified_partial',
  scope: `${proof.total_cases}案例：已解析状态的墙高约束后段及3个标高缓存叶函数；墙轮廓/砖/支撑/开口待恢复`,
};
wall.evidence = [...new Set([...wall.evidence, 'reconstruction/wall/height_rules.rs', wallReport])];
wall.remaining = ['clamp对象查询/合法性与特殊flag生产者', '墙轮廓、砖块/支撑/开口源码和原码校验', '最终场景验收后置'];
const algorithm = {
  name: 'Wall height numeric policy / cached heights', status: 'verified_local',
  source: 'reconstruction/wall/height_rules.rs', report: wallReport,
  scope: `${proof.total_cases}案例；clamp仅数值后段，3个缓存叶函数含缺失panic条件；ECS/哈希实现排除`,
};
project.local_algorithms = project.local_algorithms.filter(a => a.name !== algorithm.name);
project.local_algorithms.push(algorithm);
const editReport = 'evidence/source-edits/verification.json';
const edit = JSON.parse(fs.readFileSync(path.join(root, editReport), 'utf8'));
if (edit.ok !== true) throw new Error('Roof edit source proof failed');
const roofEdit = project.milestones.find(m => m.id === 'M06');
roofEdit.status = 'in_progress';
roofEdit.source_gate = {
  status: 'verified_partial',
  scope: `${edit.cases}案例：已解析Roof/Event的编辑后段、变化量与数值通知规则；ECS查询/调度未完成；场景后置`,
};
roofEdit.evidence = [...new Set([...roofEdit.evidence, 'reconstruction/roof/edit_rules.rs', editReport, 'evidence/source-edits/findings.md'])];
roofEdit.remaining = ['实体解析/失败分支与游戏系统注册、事件读取调度', 'create/update/delete/switch生命周期与游戏状态消费者', '最终场景验收后置'];
const editAlgorithm = {
  name: 'Roof resolved edit core / feedback rules', status: 'verified_local',
  source: 'reconstruction/roof/edit_rules.rs', report: editReport,
  scope: `${edit.cases}案例；完整88字节状态、changed tick、变化量、gable重建及反馈数值；已解析后段，不是完整ECS系统`,
};
project.local_algorithms = project.local_algorithms.filter(a => a.name !== editAlgorithm.name);
project.local_algorithms.push(editAlgorithm);
project.next_actions = [
  'S01/S03/S04：恢复Roof/Wall权威类型、create/update/edit状态消费者和事件触发；复用Bevy查询/容器',
  'S02/S05：定位特殊墙高flag生产者，恢复height history提交/撤销与游戏对象合法性规则',
  'S06/S07：优先恢复墙轮廓、construct_brick_columns与支撑源码；复用glam/RNG/half',
  'S21/S22：恢复原rkyv/ron fork之上的游戏schema与版本迁移；不运行场景回放',
];
fs.writeFileSync(file, JSON.stringify(project, null, 2) + '\n');
process.stdout.write(JSON.stringify({ wall_cases: proof.total_cases, roof_edit_cases: edit.cases, dependency_reuse: 'verified' }) + '\n');
