//! 实际枚举 GPU 与队列能力，为验收选择真实硬件；不伪造原作排序策略。
use rhapsody::raw::init::GraphicsInstance;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graphics = GraphicsInstance::new()?;
    let devices=graphics.devices.iter().enumerate().map(|(index,d)|serde_json::json!({"index":index,"name":d.name(),"vendor_id":d.properties.vendor_id,"device_id":d.properties.device_id,"queues":d.queues.iter().enumerate().map(|(family,q)|serde_json::json!({"family":family,"count":q.queue_count,"flags":q.queue_flags.as_raw()})).collect::<Vec<_>>()})).collect::<Vec<_>>();
    println!("{}", serde_json::json!({"devices":devices}));
    Ok(())
}
