//! 主题时间序列和天气初始化；新配置解析成功后才替换活动资源。
use crate::resources::{
    glade_settings::GladeSettings, session::ActiveSession, time_of_day::TodSequence,
    weather::Weather,
};
use bevy_ecs::prelude::*;

pub fn load_glade_tod_and_weather(
    mut sequence: ResMut<TodSequence>,
    mut weather: ResMut<Weather>,
    settings: Res<GladeSettings>,
    session: Res<ActiveSession>,
) -> bevy_ecs::error::Result {
    use anyhow::Context;
    let glade_path = session.assets_root().join("tod.json");
    let input = match std::fs::File::open(&glade_path) {
        Ok(file) => file,
        Err(_) => std::fs::File::open("assets/tod/default.json").context("读取默认时间序列")?,
    };
    let replacement = serde_json::from_reader(std::io::BufReader::new(input))
        .with_context(|| format!("解析主题时间序列：{}", glade_path.display()))?;
    *sequence = replacement;
    *weather = Weather::default();
    weather.weather_fog = settings.weather_fog;
    weather.wind_strength = settings.wind_strength;
    weather.field_8 = 0.0;
    weather.snow_amount = if session.is_winter() { 1.0 } else { 0.0 };
    Ok(())
}
