"""Produce an evidence-only review patch; never mutate the shared source tree."""
from pathlib import Path
import difflib, hashlib, json, struct

ROOT = Path('D:/game')
OUT = Path(__file__).resolve().parent
changes = []

def edit(relative, transform):
    path = ROOT / relative
    before = path.read_text(encoding='utf-8')
    after = transform(before)
    target = OUT / 'candidate' / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(after, encoding='utf-8', newline='\n')
    changes.append((relative, before, after))

def replace(text, old, new):
    assert old in text, f'missing anchor: {old[:120]}'
    return text.replace(old, new, 1)

def session(s):
    s = replace(s, '    LoadOnLoad = 0x8000_0000_0000_0002,', '''    LoadOnLoad = 0x8000_0000_0000_0002,
    /// Load UI already parsed this save; the request owns the complete result.
    LoadParsedGame = 0x8000_0000_0000_0003,''')
    s = replace(s, '    pub ignore_snapshot: bool,\n}', '''    pub ignore_snapshot: bool,
    /// Safe owner replacing the native Box+metadata wrapper at +0x30.
    /// Never put an allocation address in template_payload.
    parsed_data: Option<Box<crate::systems::save::load_game::ParsedGameData>>,
}''')
    # Both canonical constructors initialize an absent parsed owner.
    s = s.replace('            ignore_snapshot: false,\n        }', '            ignore_snapshot: false,\n            parsed_data: None,\n        }', 2)
    anchor = '    /// `NextSession::ignore_snapshot`'
    at = s.index(anchor)
    s = s[:at] + '''    /// Ordinary load, backup restore and theme switch transfer the actual parsed
    /// result (@141e20d5c..e1a / @140fc4cc3..4d6f), without reopening history.json.
    pub fn from_parsed(
        id: SessionId,
        data: crate::systems::save::load_game::ParsedGameData,
    ) -> Self {
        let mut request = Self::load(id);
        request.kind = NextSessionKind::LoadParsedGame;
        request.parsed_data = Some(Box::new(data));
        request
    }

    /// Move only after startup has prepared the destination metadata/settings.
    /// A malformed public template must fail before publishing partial state.
    pub(crate) fn take_parsed_data(
        &mut self,
    ) -> anyhow::Result<Box<crate::systems::save::load_game::ParsedGameData>> {
        anyhow::ensure!(self.kind == NextSessionKind::LoadParsedGame,
            "NextSession does not own parsed game data");
        self.parsed_data.take().ok_or_else(|| anyhow::anyhow!(
            "NextSession parsed variant has no owned ParsedGameData"))
    }

''' + s[at:]
    return s

def startup(s):
    s = replace(s, '        NextSessionKind::LoadOnLoad => Some(request.id),', '''        NextSessionKind::LoadOnLoad | NextSessionKind::LoadParsedGame => Some(request.id),''')
    s = replace(s, '''    let valid_session =
        requested_id.filter(|id| is_session_ok_to_load(*id, &auto_save.unsaved_files).is_ok());''', '''    // Native case3 does not run readiness again or fall back to a starting build.
    // In particular, backup restore must use the owned backup result even when
    // the selected session's current history.json is broken or absent.
    let valid_session = if request.kind == NextSessionKind::LoadParsedGame {
        Some(request.id)
    } else {
        requested_id.filter(|id| is_session_ok_to_load(*id, &auto_save.unsaved_files).is_ok())
    };''')
    s = replace(s, '    if !history_path.is_file() {', '    if request.kind != NextSessionKind::LoadParsedGame && !history_path.is_file() {')
    s = replace(s, '''    let pending_actions = make_post_load_commands(std::mem::take(&mut request.on_load));
    load_manager.begin(
        LoadGameSource::Path(history_path),''', '''    let source = if request.kind == NextSessionKind::LoadParsedGame {
        // This is the last fallible request operation: retain the owner and all
        // post-load actions until the complete destination has been prepared.
        let data = request.take_parsed_data()?;
        LoadGameSource::Parsed {
            data,
            snapshot_root: session_id.save_folder_path().join("snapshots"),
        }
    } else {
        LoadGameSource::Path(history_path)
    };
    let pending_actions = make_post_load_commands(std::mem::take(&mut request.on_load));
    load_manager.begin(
        source,''')
    return s

def loader(s):
    s = replace(s, 'pub struct ParsedGameData {\n', '''pub struct ParsedGameData {
    /// All four original wrapper flags, carried through the owning request.
    /// None denotes the explicitly unclosed original old-version migration;
    /// it must never be treated as a wrapper full of false flags.
    pub migration_metadata: Option<ParsedLoadMetadata>,
''')
    at = s.index('/// `Time of day`')
    s = s[:at] + '''/// The native successful return owns a 240B body plus these four flags.
/// Its other four-byte field is the target chain_index, already stored above.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParsedLoadMetadata {
    /// Native wrapper+c: parsed source version!=27 OR an actual migration ran.
    pub requires_upgrade_backup: bool,
    /// Native wrapper+d: WINDOW snapshot colors, then invalidate snapshot refs.
    pub upgrade_window_colors: bool,
    /// Native wrapper+e: ARROW SLIT snapshot colors. Not the previous swapped bit.
    pub upgrade_arrow_slit_colors: bool,
    /// Native wrapper+f: source version<19; Err uses the separate niche value2.
    pub legacy_version_notice: bool,
}
impl ParsedLoadMetadata {
    /// Proven native v27 branch @141471bfe..1c23 / @141472e76..eb3.
    pub const VERSION_27: Self = Self {
        requires_upgrade_backup: false,
        upgrade_window_colors: false,
        upgrade_arrow_slit_colors: false,
        legacy_version_notice: false,
    };

    pub fn snapshot_color_upgrade_flags(self) -> crate::resources::save_file::LoadUpgradeFlags {
        crate::resources::save_file::LoadUpgradeFlags(
            ((self.upgrade_window_colors as u64) << 40)
                | ((self.upgrade_arrow_slit_colors as u64) << 48))
    }
}

''' + s[at:]
    s = replace(s, 'pub fn parse_saved_game_data(path: &Path) -> anyhow::Result<ParsedGameData> {', '''pub fn parse_saved_game_data(
    path: &Path,
    settings: &crate::resources::glade_settings::GladeSettings,
) -> anyhow::Result<ParsedGameData> {''')
    s = replace(s, '    // Colors', '''    let initial_colors = || GladeColorCustomization {
        freeform_wall: crate::resources::color_id::ColorId(settings.default_freeform_wall_color_id),
        building: crate::resources::color_id::ColorId(settings.default_building_color_id),
        roof: crate::resources::color_id::ColorId(settings.default_roof_color_id),
        stairs: crate::resources::color_id::ColorId(100003),
        lantern_brightness: 1.0,
        building_wall_style: 1,
        ..GladeColorCustomization::default()
    };
    // Colors''')
    # Only the two fallback sites; the lambda's struct update stays ordinary.
    a = s.index('    // Colors')
    b = s.index('    // History', a)
    s = s[:a] + s[a:b].replace('GladeColorCustomization::default()', 'initial_colors()') + s[b:]
    s = replace(s, '    Ok(ParsedGameData {\n', '''    Ok(ParsedGameData {
        migration_metadata: (version == LATEST_SAVE_VERSION)
            .then_some(ParsedLoadMetadata::VERSION_27),
''')
    # Explicit settings also reach the old pure helper; no production default.
    s = replace(s, '''    save_path: Option<&Path>,
) -> Result<(Option<ParsedGameData>''', '''    save_path: Option<&Path>,
    settings: &crate::resources::glade_settings::GladeSettings,
) -> Result<(Option<ParsedGameData>''')
    s = s.replace('let parsed = parse_saved_game_data(path)?;', 'let parsed = parse_saved_game_data(path, settings)?;')
    s = replace(s, '''    upgrade_flags: crate::resources::save_file::LoadUpgradeFlags,
) -> Result<''', '''    upgrade_flags: crate::resources::save_file::LoadUpgradeFlags,
    settings: &crate::resources::glade_settings::GladeSettings,
) -> Result<''')
    s = replace(s, 'advance_load_game(manager, next, save_path)?', 'advance_load_game(manager, next, save_path, settings)?')
    s = s.replace('    next: &NextSession,', '    next: &mut NextSession,')
    s = replace(s, '''            let selected = clamp_next_session_kind(next.kind.decoded());''', '''            if next.kind == crate::resources::session::NextSessionKind::LoadParsedGame {
                let parsed = *next.take_parsed_data()?;
                manager.progress_counter = manager.progress_counter.wrapping_add(1);
                return Ok((Some(parsed), Vec::new()));
            }
            let selected = clamp_next_session_kind(next.kind.decoded());''')
    # The actual consumer takes the source owner once; it does not deep-clone
    # the graph and does not parse a backup's unrelated current history file.
    s = replace(s, '''                let (mut parsed, snapshot_root, source_file) = match &manager.source {''', '''                let source = std::mem::take(&mut world.resource_mut::<LoadGameManager>().source);
                let (mut parsed, snapshot_root, source_file) = match source {''')
    s = replace(s, '''                        parse_saved_game_data(path)
                            .with_context(|| format!("B load_game source {}", path.display()))?,''', '''                        parse_saved_game_data(&path,
                            world.resource::<crate::resources::glade_settings::GladeSettings>())
                            .with_context(|| format!("B load_game source {}", path.display()))?,''')
    s = replace(s, '} => ((**data).clone(), snapshot_root.clone(), None),', '} => (*data, snapshot_root, None),')
    # Preserve existing Path's incomplete older-version support scope. New
    # prepared-data UI cannot silently advertise unsupported wrapper semantics.
    s = replace(s, '''                // 1. 选择快照''', '''                if source_file.is_none() && parsed.migration_metadata.is_none() {
                    anyhow::bail!("Original saved-game migration metadata for version {} is not yet restored; the failed load does not report compatibility", parsed.version);
                }
                // 1. 选择快照''')
    # Before applying parsed data, guard Path's known backup policy. A missing
    # metadata is explicit residual, never guessed as false or inferred solely
    # from version !=27, because native flags include actual migration results.
    s = replace(s, '''    if let Some(path) = source_file {
        let _ = make_upgrade_backup(path);
    }''', '''    if let Some(path) = source_file {
        match parsed.migration_metadata {
            Some(metadata) if metadata.requires_upgrade_backup && !options.new_session => {
                make_upgrade_backup(path).context("backing up save before upgrade")?;
            }
            Some(_) => {}
            None => {
                // Existing old-version implementation remains explicitly
                // incomplete; do not present it as recovered compatibility.
                log::warn!("Original migration metadata remains unclosed for source version {}", parsed.version);
                let _ = make_upgrade_backup(path);
            }
        }
    }''')
    return s

edit('tiny-glade/crates/country-core/src/resources/session.rs', session)
edit('tiny-glade/crates/country-core/src/startup/startup_session.rs', startup)
edit('tiny-glade/crates/country-core/src/systems/save/load_game.rs', loader)

def save_file(s):
    # Correct the legacy bit helper with independent native consumer evidence.
    s = s.replace('/// 位 40："Detected old version of arrow slits', '/// 位 48："Detected old version of arrow slits')
    s = s.replace('/// 位 48："Detected old version of window', '/// 位 40："Detected old version of window')
    s = replace(s, 'pub fn upgrade_arrow_slit_color_ids(self) -> bool {\n        self.0 >> 40', 'pub fn upgrade_arrow_slit_color_ids(self) -> bool {\n        self.0 >> 48')
    s = replace(s, 'pub fn upgrade_window_color_ids(self) -> bool {\n        self.0 >> 48', 'pub fn upgrade_window_color_ids(self) -> bool {\n        self.0 >> 40')
    return s
edit('tiny-glade/crates/country-core/src/resources/save_file.rs', save_file)

def tests(s):
    s = s.replace('parse_saved_game_data(&path)', 'parse_saved_game_data(&path, &crate::resources::glade_settings::GladeSettings::default())')
    s = s.replace('advance_load_game(&mut manager, &session, None)', 'advance_load_game(&mut manager, &mut session, None, &crate::resources::glade_settings::GladeSettings::default())')
    s += '''

#[test]
fn parsed_request_moves_owned_edits_and_all_native_metadata_without_disk_reparse() {
    use super::load_game::{advance_load_game, LoadGameManager, ParsedGameData, ParsedLoadMetadata};
    use crate::resources::{glade_settings::GladeSettings, session::{NextSession, SessionId}};
    let metadata = ParsedLoadMetadata {
        requires_upgrade_backup: true,
        upgrade_window_colors: true,
        upgrade_arrow_slit_colors: false,
        legacy_version_notice: true,
    };
    let mut data = ParsedGameData {
        version: 27,
        chain_index: 8,
        migration_metadata: Some(metadata),
        ..Default::default()
    };
    data.edits.reserve_exact(4);
    let original_allocation = data.edits.as_ptr();
    let mut request = NextSession::from_parsed(SessionId::new(), data);
    let mut manager = LoadGameManager::new();
    let (loaded, _) = advance_load_game(&mut manager, &mut request,
        Some(std::path::Path::new("this-file-must-not-be-opened.json")),
        &GladeSettings::default()).unwrap();
    let loaded = loaded.unwrap();
    assert_eq!(loaded.edits.as_ptr(), original_allocation);
    assert_eq!(loaded.chain_index, 8);
    assert_eq!(loaded.migration_metadata, Some(metadata));
    assert!(metadata.snapshot_color_upgrade_flags().upgrade_window_color_ids());
    assert!(!metadata.snapshot_color_upgrade_flags().upgrade_arrow_slit_color_ids());
    assert!(advance_load_game(&mut manager, &mut request, None,
        &GladeSettings::default()).is_err(), "the same owner cannot be taken twice");
}
'''
    return s
edit('tiny-glade/crates/country-core/src/systems/save/tests.rs', tests)

edit('tiny-glade/crates/country-core/src/startup/tests.rs', lambda s: replace(s,
    'advance_load_game(&mut manager, &request, None)',
    'advance_load_game(&mut manager, &mut request, None, &crate::resources::glade_settings::GladeSettings::default())'))

edit('tiny-glade/crates/tiny-glade/src/run_game.rs', lambda s: replace(s,
    'parse_saved_game_data(&path)',
    'parse_saved_game_data(&path, &country_core::resources::glade_settings::GladeSettings::default())'))

patch = ''.join(''.join(difflib.unified_diff(a.splitlines(True), b.splitlines(True), fromfile='a/'+p, tofile='b/'+p)) for p,a,b in changes)
(OUT / 'ownership-v27-review.patch').write_text(patch, encoding='utf-8', newline='\n')
(OUT / 'source-baseline.json').write_text(json.dumps({p:hashlib.sha256(a.encode()).hexdigest() for p,a,b in changes}, indent=2), encoding='utf-8')
print(json.dumps({'files':len(changes), 'patch_bytes':len(patch.encode()), 'production_sources_modified':False}))
