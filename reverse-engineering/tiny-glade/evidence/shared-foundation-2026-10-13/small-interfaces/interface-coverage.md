# B 小模块原接口对照

分母是已观察PDB接口，不是完整作者API；仅定位名字不算业务分析。

| 模块 | 原接口 | 已分析 | 未触及 | 有映射实现 | 原入口对照 |
|---|---:|---:|---:|---:|---:|
| paths | 10 | 10 | 0 | 10 | 0 |
| lang-lib | 11 | 11 | 0 | 11 | 0 |
| tabloid | 6 | 3 | 3 | 3 | 0 |
| theme-defs | 4 | 4 | 0 | 4 | 0 |

| 原接口 | 状态 | 实现 | 剩余工作 |
|---|---|---|---|
| `lang_lib::game_locale` | implemented_static | lang_lib::game_locale | Full game initialization/consumer integration remains |
| `lang_lib::lang_pack_path` | implemented_static | lang_lib::lang_pack_path | Full game initialization/consumer integration remains |
| `lang_lib::language_library::LanguageLibrary::get` | implemented_static | lang_lib::language_library::LanguageLibrary::get | Full game initialization/consumer integration remains |
| `lang_lib::language_library::LanguageLibrary::locale` | implemented_static | LanguageLibrary::locale | Inline getter contract comparison is partial; typed signature not directly established |
| `lang_lib::language_library::LanguageLibrary::new` | implemented_static | LanguageLibrary::new | Full original ownership/lifecycle/caller comparison remains partial; native constructor entry not executed |
| `lang_lib::language_library::LanguageLibrary::try_from_yml` | implemented_static | lang_lib::language_library::LanguageLibrary::try_from_yml | Full game initialization/consumer integration remains |
| `lang_lib::language_library::LanguageLibrary::try_get` | implemented_static | LanguageLibrary::try_get | Inline getter contract comparison is partial; typed signature not directly established |
| `lang_lib::load_last_used_locale` | implemented_static | lang_lib::load_last_used_locale | Full game initialization/consumer integration remains |
| `lang_lib::load_locale` | implemented_static | lang_lib::load_locale | Full game initialization/consumer integration remains |
| `lang_lib::load_locale_impl` | implemented_static | lang_lib::load_locale_impl | Full game initialization/consumer integration remains |
| `lang_lib::lookup` | implemented_static | lang_lib::lookup | Full game initialization/consumer integration remains |
| `paths::backup_saves` | implemented_static | paths::backup_saves | Full startup consumer integration and original runtime comparison remain |
| `paths::config` | implemented_static | paths::config | Full startup consumer integration and original runtime comparison remain |
| `paths::deleted_saves` | implemented_static | paths::deleted_saves | Full startup consumer integration and original runtime comparison remain |
| `paths::ensure_path` | implemented_static | paths::ensure_path | Full startup consumer integration and original runtime comparison remain |
| `paths::find_standard_app_root_path` | implemented_static | paths::find_standard_app_root_path | Full startup consumer integration and original runtime comparison remain |
| `paths::initialize_paths` | implemented_static | paths::initialize_paths | Full startup consumer integration and original runtime comparison remain |
| `paths::installed` | implemented_static | paths::installed | Full startup consumer integration and original runtime comparison remain |
| `paths::saves` | implemented_static | paths::saves | Full startup consumer integration and original runtime comparison remain |
| `paths::standard_app_root_path` | implemented_static | paths::standard_app_root_path | Full startup consumer integration and original runtime comparison remain |
| `paths::tmp` | implemented_static | paths::tmp | Full startup consumer integration and original runtime comparison remain |
| `tabloid::Publisher::publish` | implemented_static | Publisher::publish | Real DirtyTiles/ColliderDiff consumer integration and original runtime comparison remain |
| `tabloid::Publisher::subscribe` | implemented_static | Publisher::subscribe | Real DirtyTiles/ColliderDiff consumer integration and original runtime comparison remain |
| `tabloid::Subscriber::take` | implemented_static | Subscriber::take | Real DirtyTiles/ColliderDiff consumer integration and original runtime comparison remain |
| `tabloid::impl$0::default` | untouched | 未核实映射 | Original inline impl owner/behavior needs independent comparison; candidate behavior alone does not count as analysis; Private drop retained in inventory: not automatically excluded as compiler/library glue |
| `tabloid::impl$2::drop` | untouched | 未核实映射 | Original inline impl owner/behavior needs independent comparison; candidate behavior alone does not count as analysis; Private drop retained in inventory: not automatically excluded as compiler/library glue |
| `tabloid::impl$3::deref` | untouched | 未核实映射 | Original inline impl owner/behavior needs independent comparison; candidate behavior alone does not count as analysis; Private drop retained in inventory: not automatically excluded as compiler/library glue |
| `theme_defs::impl$0::clone` | implemented_static | EventDataError::clone (derive) | Error/transport contract remains partial; original full producer-consumer comparison pending |
| `theme_defs::impl$2::eq` | implemented_static | EventDataError::eq (derive) | Error/transport contract remains partial; original full producer-consumer comparison pending |
| `theme_defs::impl$5::clone` | implemented_static | ActiveEvent::clone (derive) | Error/transport contract remains partial; original full producer-consumer comparison pending |
| `theme_defs::impl$7::eq` | implemented_static | ActiveEvent::eq (derive) | Error/transport contract remains partial; original full producer-consumer comparison pending |
