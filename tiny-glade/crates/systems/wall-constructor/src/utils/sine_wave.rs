//! 正弦波工具（utils/sine_wave.rs）。
//!
//! `BrickRowSine`（module 401 inlinee 24627 `new`/24641 `sine_wave_deform`/
//! 24629 `random_f`；source_line 12/45/82）：纯内联，宿主 =
//! `WallConstructor::from_curve`（0x140769760），无独立编译实例。逻辑已从
//! 宿主反编译（`from_curve_140769760.c`）体内按公式完整提取，逐语句判等。
//!
//! 公式（re-analysis hardpoints-ecstate §4.3 + 宿主反编译行 919-971/1194-1221）：
//! - `random_f`：hash rng。`a = sin((x*10 + phase)*12.9898)`；
//!   `r0 = frac(a*43758.547)`；`b = sin((y*10 + phase)*12.9898)`；
//!   `r1 = frac(b*43758.547)`；`p0 = frac(sin(2*r0*12.9898)*43758.547)*1000`；
//!   `p1 = frac(sin(2*r1*12.9898)*43758.547)*1000`。
//!   （宿主逐字节：`fVar63 = sinf((fVar57+fVar57)*12.9898)`，fVar57 即 r0；
//!   `local_270 = (fVar63*43758.547 - truncf)*1000`。`frac` = `x - trunc(x)`，
//!   与 Rust `f32::fract()` 逐位一致。）
//! - `sine_wave_deform`：`s = 0.5 + 0.5*sin(x*0.2)`；`y1`（顶边，门
//!   `0.01 <= width`）= `0.06*sin((r0*s + 1)*x + p0)`；`y2`（底边，open
//!   侧才取）= `0.06*sin(x*(s*r1 + 1) + p1)`。
//! 应用于 BrickRow 顶/底边正弦波偏移（幅度 0.06）。

/// B `BrickRowSine`：砖行顶/底边正弦波偏移参数（4 个 random_f 值）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BrickRowSine {
    /// r0（顶边随机分量，`local_278`）。
    pub r0: f32,
    /// r1（底边随机分量，`local_274`）。
    pub r1: f32,
    /// p0（顶边相位×1000，`local_270`）。
    pub p0: f32,
    /// p1（底边相位×1000，`local_26c`）。
    pub p1: f32,
}

impl BrickRowSine {
    /// B `BrickRowSine::new`（inlinee 24627，source_line 12）：
    /// 以 (x, y, phase) 计算 4 个 random_f 值并装配。
    pub fn new(x: f32, y: f32, phase: f32) -> Self {
        let (r0, r1, p0, p1) = Self::random_f(x, y, phase);
        Self { r0, r1, p0, p1 }
    }

    /// B `random_f`（inlinee 24629，source_line 82）：hash rng。
    /// 宿主反编译逐语句（`from_curve_140769760.c` 行 919-971）：
    /// ```
    /// fVar49 = sinf((x*10.0 + phase)*12.9898);
    /// r0 = fVar49*43758.547 - truncf;   // frac
    /// fVar63 = sinf((y*10.0 + phase)*12.9898);
    /// r1 = fVar63*43758.547 - truncf;   // frac
    /// p0 = (sinf((r0+r0)*12.9898)*43758.547 - truncf)*1000.0;
    /// p1 = (sinf((r1+r1)*12.9898)*43758.547 - truncf)*1000.0;
    /// ```
    pub fn random_f(x: f32, y: f32, phase: f32) -> (f32, f32, f32, f32) {
        const A: f32 = 12.9898;
        const B: f32 = 43758.547;
        let a = ((x * 10.0 + phase) * A).sin();
        let r0 = (a * B).fract();
        let b = ((y * 10.0 + phase) * A).sin();
        let r1 = (b * B).fract();
        // 宿主：sin 实参 = `(r0 + r0) * 12.9898`（先相加再乘），非 2*r0*A 折叠。
        let p0 = ((r0 + r0) * A).sin();
        let p0 = (p0 * B).fract() * 1000.0;
        let p1 = ((r1 + r1) * A).sin();
        let p1 = (p1 * B).fract() * 1000.0;
        (r0, r1, p0, p1)
    }

    /// B `sine_wave_deform`（inlinee 24641，source_line 45）：对世界坐标 x
    /// 求顶/底边正弦偏移。宿主反编译逐语句（`from_curve_140769760.c` 行
    /// 1194-1221）：
    /// ```
    /// s = 0.5 + 0.5*sin(x*0.2);
    /// y1 = 0.0; if (0.01 <= width) y1 = 0.06*sin((r0*s + 1)*x + p0);
    /// y2 = 0.0; if (!closed)      y2 = 0.06*sin(x*(s*r1 + 1) + p1);
    /// ```
    /// `closed` 对应宿主 `bVar45`（闭合轮廓侧不产生底边偏移）。
    /// 返回 `(顶边偏移, 底边偏移)`。
    pub fn sine_wave_deform(&self, x: f32, width: f32, closed: bool) -> (f32, f32) {
        let s = 0.5 + 0.5 * (x * 0.2).sin();
        let mut top = 0.0f32;
        let mut bottom = 0.0f32;
        if 0.01 <= width {
            top = ((self.r0 * s + 1.0) * x + self.p0).sin() * 0.06;
        }
        if !closed {
            bottom = (x * (s * self.r1 + 1.0) + self.p1).sin() * 0.06;
        }
        (top, bottom)
    }
}
