//! 原 B 损坏存档告警：事件打开、模态阻挡、真实确认按钮关闭；不改写存档。
use crate::mei::{core::MeiContext, widgets::MenuButtonContent};
use bevy_ecs::prelude::*;
use country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning;

/// 原UI的0=显示、1=已确认。原System::initialize140ff8a20将Local字节写1；
/// Local在会话Reload之间保留，后续事件可再次打开。
pub struct MessageScreenState {
    dismissed: bool,
}
impl Default for MessageScreenState {
    fn default() -> Self {
        Self { dismissed: true }
    }
}

/// 原141ebfbf0与按钮槽140fbf460；每次新事件重开，读完事件后不残留旧请求。
pub fn ui(
    mut mei: ResMut<MeiContext>,
    mut state: Local<MessageScreenState>,
    mut events: EventReader<DisplayCorruptedSaveFileWarning>,
) {
    if events.read().count() != 0 {
        state.dismissed = false;
    }
    if state.dismissed {
        return;
    }
    let root = mei.safe_frame("corrupted_save", true);
    root.node_mut().local_z = 3.;
    root.template("menu/corrupted_save")
        .with_slot("buttons", |slot| {
            let button = slot.menu_button(&lang_lib::lookup("Ok"));
            button
                .content_mut::<MenuButtonContent>()
                .text
                .set_metrics(25.)
                .expect("原损坏存档按钮字号");
            if button.node().used() {
                state.dismissed = true;
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::{App, Update};
    use std::{path::PathBuf, sync::Arc};
    #[test]
    #[ignore = "需TG_INSTALL_ROOT指向原版模板资产"]
    fn original_warning_event_modal_confirm_and_reopen() {
        let installed =
            PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("需原版TG_INSTALL_ROOT"));
        let config = crate::mei::config::read_node_config_from_file(
            &installed.join("assets/ui/menu/corrupted_save.ron"),
        )
        .unwrap();
        let mut mei = MeiContext::default();
        mei.configs
            .insert("menu/corrupted_save".into(), Arc::new(config));
        mei.load_font_table(&installed.join("assets/fonts/table.ron"))
            .unwrap();
        let mut app = App::new();
        app.insert_resource(mei)
            .add_event::<DisplayCorruptedSaveFileWarning>()
            .add_systems(Update, ui);
        app.update();
        assert!(app.world().resource::<MeiContext>().nodes.is_empty());
        for _ in 0..2 {
            // 该CPU探针不运行GPU post_update；补其已核实的帧尾激活位清理。
            for node in app
                .world_mut()
                .resource_mut::<MeiContext>()
                .previous_nodes
                .values_mut()
            {
                node.state_bits &= !0x30;
            }
            app.world_mut().send_event(DisplayCorruptedSaveFileWarning);
            app.update();
            let mei = app.world_mut().resource_mut::<MeiContext>().into_inner();
            let root = mei.nodes.values().find(|node| node.local_z == 3.).unwrap();
            assert_ne!(
                root.flags & crate::mei::core::NodeFlags::FORCE_INTERACTIVE,
                0
            );
            let button = mei
                .nodes
                .values_mut()
                .find(|node| node.content.type_name() == "toolbar::mei::widgets::MenuButtonContent")
                .unwrap();
            button.state_bits |= 0x10;
            mei.recycle_frame_nodes();
            app.update();
            app.world_mut()
                .resource_mut::<MeiContext>()
                .recycle_frame_nodes();
            app.update();
            assert!(app.world().resource::<MeiContext>().nodes.is_empty());
        }
    }
}
