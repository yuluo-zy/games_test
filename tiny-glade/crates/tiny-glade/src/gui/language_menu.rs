//! 原140150bb0及语言行14032d8b0：四行共15种语言，实际加载与配置权威写回。
//! 原标签、locale、行间距和按钮样式见toolbar-repair-consumers-native.json。
use country_core::game_config::GameConfig;
use toolbar::mei::{core::{NodeContext,GlobalId,calculate_hash}, layout::{horizontal,LengthPercentage}, widgets::MenuButtonContent};

/// 原静态表按行排列；名称保留各语言自身文字，不从文件名排序或使用OS语言名。
pub const LANGUAGE_ROWS: &[&[(&str,&str)]] = &[
    &[("Deutsch","de-DE"),("English","en-US"),("Español","es-ES"),("Français","fr-FR")],
    &[("Italiano","it-IT"),("Norsk","no-NO"),("Polski","pl-PL"),("Português-Brasil","pt-BR")],
    &[("Русский","ru-RU"),("Svenska","sv-SE"),("Türkçe","tr-TR"),("Українська","uk-UA")],
    &[("한국어","ko-KR"),("日本語","ja-JP"),("简体中文","zh-CN")],
];
pub fn do_gui_language_menu(node: NodeContext, changed: &mut bool, config: &mut GameConfig) {
    for (index,languages) in LANGUAGE_ROWS.iter().enumerate() {
        // 原child<usize>使用行下标，不能用翻译结果重建行身份。
        let local=calculate_hash(&(index as u64,0xa79b7268a2a968d9u64));
        let row=node.child_with_global_id(GlobalId(calculate_hash(&(node.id.0,local))));
        row.layout(horizontal().gap(LengthPercentage::absolute(80.)));
        for &(name,locale) in *languages {
            let button=row.menu_button(name);
            let content=button.content_mut::<MenuButtonContent>();
            content.text.set_weight(tg_text::Weight(700));
            content.color=0xffffffa0;
            if button.node().used() {
                // 原失败分支unwrap；现有lang-lib保持事务加载，失败不会清掉旧语言包。
                lang_lib::load_locale(locale).expect("原语言包加载失败");
                config.lang_override=Some(locale.to_owned());
                *changed=true;
            }
        }
    }
}
