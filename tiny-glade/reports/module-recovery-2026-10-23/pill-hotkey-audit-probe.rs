// 独立审计探针：原B6816e2/681938读写同一+44字节。
// 直接调用当前库的实际控件构造/热键，并检查pre_update所读的动作字段。
// 此探针验证字段交接，不代替实际输入、pre_update执行或原二进制执行。
use toolbar::MeiContext;
use toolbar::mei::widgets::PillButtonContent;

fn main() {
    let mut mei = MeiContext::default();
    let button = mei.safe_frame("audit", false).pill_button("confirm", "menu/back");
    button.pill_hotkey(19);
    let content = button.content_mut::<PillButtonContent>();
    println!("hotkey={} pre_update_action={}", content.hotkey, content.action);
    assert_eq!(content.action, 19, "原同字节热键合同未满足：setter未发布到pre_update读取的动作字段");
}
