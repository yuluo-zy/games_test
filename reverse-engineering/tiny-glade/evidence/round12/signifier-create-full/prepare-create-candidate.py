from pathlib import Path
import hashlib,json,difflib
root=Path('D:/game');out=root/'reverse-engineering/tiny-glade/evidence/round12/signifier-create-full';p=root/'tiny-glade/crates/country-core/src/systems/ui/signifiers/signifier_create.rs';s=p.read_text(encoding='utf8');base=s
s=s.replace('use crate::systems::ui::resources::ui_state::UiState;','use crate::systems::ui::resources::ui_state::{UiState, WidgetId};')
s=s.replace('pub struct SignifierId(pub u64);','''#[repr(C)]
pub struct SignifierId(pub u64, pub WidgetId);''')
s=s.replace('''    /// 本帧请求的标识，供删除系统读取，包含被去重复用的实体。
    active_ids: HashSet<u64>,
''','')
a=s.index('    /// 是否在最近一次创建阶段');z=s.index('    /// 已提交长度',a);s=s[:a]+s[z:]
a=s.index('/// 记录 → 指示符标识');s=s[:a]+'''/// Original14145a710 hashes the original WidgetId followed by Option<&str>.
/// Reuse std DefaultHasher through the existing canonical helper. Native78
/// fixtures prove discriminant u64 and the string FF Hash terminator.
pub fn record_id(widget: WidgetId, name: Option<&str>) -> u64 {
    crate::utils::calculate_hash(&(widget, name))
}

/// Original140b2cfa0: existing-entity hashes are a snapshot; this frame's
/// newly woken factories do not mutate that set. Duplicate same-frame names
/// can intentionally wake more than once.
pub fn signifier_create(
    mut ui_state: ResMut<UiState>,
    mesh_library: Res<AssetMeshLibrary>,
    shader_library: Res<AssetShaderLibrary>,
    mut commands: Commands,
    existing: Query<&SignifierId, Without<SignifierMarkedForDeletion>>,
    mut stream: ResMut<UiSignifierStream>,
) {
    let existing: HashSet<u64> = existing.iter().map(|id| id.0).collect();
    if ui_state.interaction().native_tag == 0 {
        // Original does not take/clear the global append-only stream here.
        return;
    }
    let entries = stream.take_entries();
    if !entries.is_empty() {
        // Same native unique Arc check as std Arc::get_mut. Do not make_mut
        // or clone the params to conceal a conflicting owner.
        let params = ui_state.active_params_mut().unwrap();
        for entry in &entries {
            let UiSignifierEntry::Factory(factory) = entry else {
                panic!("unrestored raw signifier producer: expected owned name and factory closure");
            };
            assert!(factory.has_closure(), "unrestored signifier callback owner");
            params.signifiers.push(factory.clone());
        }
    }
    drop(entries); // Original cloned String/closure ownership, then drops stream.
    let widget = ui_state.active_widget();
    let params = ui_state.active_params().unwrap();
    for factory in &params.signifiers {
        let hash = record_id(widget, factory.name());
        if existing.contains(&hash) { continue; }
        assert!(factory.has_closure(), "unrestored signifier callback owner");
        let entities = factory.wake(&mesh_library, &shader_library, &mut commands);
        for entity in entities {
            // Rust's iterator retains the native Option<Entity> nonzero niche.
            commands.entity(entity).insert(SignifierId(hash, widget));
        }
    }
}
'''
# Remove obsolete descriptions of the retired parallel active table.
s=s.replace('/// 待消费的指示符请求及本帧活动标识。','/// 待消费的指示符工厂请求。').replace('/// 独占可变访问由 Bevy 资源借用保证；活动集合在创建系统每次执行时重建。','/// 独占可变访问由 Bevy 资源借用保证；活跃身份由 UiState 的工厂列表决定。')
(out/'signifier_create.rs').write_text(s,encoding='utf8');(out/'signifier-create.patch').write_text(''.join(difflib.unified_diff(base.splitlines(True),s.splitlines(True),fromfile='a/'+p.relative_to(root).as_posix(),tofile='b/'+p.relative_to(root).as_posix())),encoding='utf8')
basepath=root/'reverse-engineering/tiny-glade/evidence/round12/signifier-builder-full/candidate/tiny-glade/crates/country-core/src/systems/ui/signifiers/signifier_mark_for_delete.rs';b=basepath.read_text(encoding='utf8');n=b.replace('use super::signifier_create::{SignifierId, UiSignifierStream};','use super::signifier_create::{record_id, SignifierId};\nuse crate::systems::ui::resources::ui_state::UiState;\nuse std::collections::HashSet;').replace('stream: Res<UiSignifierStream>','ui: Res<UiState>').replace('''    for (entity, id, lifespan) in &query {
        let is_live = stream.is_active(*id);''','''    // Original local set is rebuilt from the captured active widget's actual
    // factory list. No persistent Stream.active_ids table exists.
    let active: HashSet<u64> = if ui.hover_byte() != 0 {
        let widget = ui.active_widget();
        ui.active_params().unwrap().signifiers.iter()
            .map(|factory| record_id(widget, factory.name())).collect()
    } else { HashSet::new() };
    for (entity, id, lifespan) in &query {
        let is_live = active.contains(&id.0);''')
(out/'signifier_mark_for_delete.rs').write_text(n,encoding='utf8');(out/'mark-owner-on-builder-candidate.patch').write_text(''.join(difflib.unified_diff(b.splitlines(True),n.splitlines(True),fromfile='a/crates/country-core/src/systems/ui/signifiers/signifier_mark_for_delete.rs',tofile='b/crates/country-core/src/systems/ui/signifiers/signifier_mark_for_delete.rs')),encoding='utf8')
(out/'candidate-bases.json').write_text(json.dumps(dict(create_path=str(p),create_sha256=hashlib.sha256(base.encode()).hexdigest(),mark_candidate_path=str(basepath),mark_candidate_sha256=hashlib.sha256(b.encode()).hexdigest()),indent=2),encoding='utf8')
