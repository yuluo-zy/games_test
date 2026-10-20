const FOV_OFFSETS:[usize;6]=[0x8c,0x144,0x1c4,0x264,0x31c,0x3a4];
struct MainCamera {raw:[u8;0x4e0]}
impl MainCamera {
fn read_f32(&self,o:usize)->f32{f32::from_le_bytes(self.raw[o..o+4].try_into().unwrap())}
fn write_f32(&mut self,o:usize,v:f32){self.raw[o..o+4].copy_from_slice(&v.to_le_bytes())}
fn bytes(&self)->&[u8]{&self.raw}fn mode_byte(&self)->u8{self.raw[0x4d0]}}
#[derive(Clone)]struct Row{params_18:[f32;4],params_30:[f32;2],word_38:u32}
struct ImageFormationConfigs{configs:[Row;3]}
impl ImageFormationConfigs{fn get(&self,n:usize)->Option<&Row>{self.configs.get(n)} fn iter_non_default_mut(&mut self)->impl Iterator<Item=&mut Row>{self.configs.iter_mut().skip(1)}}
pub const PATH_DEFAULT: f32 = 0.8;
pub const PATH_RANGE: (f32, f32) = (f32::from_bits(0x3ea3_d70b), f32::from_bits(0x3f99_999a));
/// 水工具：默认 2.0（0x40000000），[1.0, 5.0]（0x142925904 / 0x142925988）。
pub const WATER_DEFAULT: f32 = 2.0;
pub const WATER_RANGE: (f32, f32) = (1.0, 5.0);
/// 花园工具：默认 3.0（0x40400000），[2.0, 4.0]（0x14292e6f0 / 0x142934694）。
pub const GARDEN_DEFAULT: f32 = 3.0;
pub const GARDEN_RANGE: (f32, f32) = (2.0, 4.0);

/// `maxss dst, src`：任一操作数为 NaN 时返回 `src`，所以 NaN 会一路穿过两次钳制。
fn maxss(dst: f32, src: f32) -> f32 {
    if dst > src { dst } else { src }
}

/// `minss dst, src`：同上，NaN 返回 `src`。
fn minss(dst: f32, src: f32) -> f32 {
    if dst < src { dst } else { src }
}

fn clamp_like_exe((lower, upper): (f32, f32), value: f32) -> f32 {
    minss(upper, maxss(lower, value))
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct BrushSizes {
    pub path: f32,
    pub water: f32,
    pub garden: f32,
}

impl BrushSizes {
    /// O：0x1409f1860。
    pub fn reset_path(&mut self) {
        self.path = PATH_DEFAULT;
    }

    /// O：0x1409f1870。
    pub fn set_path(&mut self, size: f32) {
        self.path = clamp_like_exe(PATH_RANGE, size);
    }

    /// O：0x1409f1890。
    pub fn reset_water(&mut self) {
        self.water = WATER_DEFAULT;
    }

    /// O：0x1409f18a0。
    pub fn set_water(&mut self, size: f32) {
        self.water = clamp_like_exe(WATER_RANGE, size);
    }

    /// O：0x1409f18c0。
    pub fn reset_garden(&mut self) {
        self.garden = GARDEN_DEFAULT;
    }

    /// O：0x1409f18d0。
    pub fn set_garden(&mut self, size: f32) {
        self.garden = clamp_like_exe(GARDEN_RANGE, size);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogSlider {
    /// [0] 最小值。
    pub min: f32,
    /// [1] 最大值。
    pub max: f32,
    /// [2] 未用/步进候选（U）。
    pub _pad: f32,
    /// [3] exp2 指数基数（O 读面：`exp2f(clamp+[3])`；语义 I）。
    pub log2_base: f32,
}

impl Default for LogSlider {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 1.0,
            _pad: 0.0,
            log2_base: 0.0,
        }
    }
}

impl LogSlider {
    /// O 0x142383730 `param_to_value(param)`：
    /// `p = param.clamp(min, max)`（min>max 触发 `f32::clamp` 的 do_panic::runtime）；
    /// 返回 `exp2f(p + log2_base)`。**判等**。
    pub fn param_to_value(&self, param: f32) -> f32 {
        let p = param.clamp(self.min, self.max);
        (p + self.log2_base).exp2()
    }
    /// O `LogSlider::value_to_param` (inline, 2 instances; host
    /// ui_slider_reset case 7): inverse of `param_to_value`, i.e.
    /// `log2(value) - log2_base` clamped to `[min, max]`.
    /// B slider_val_01 @142349eea..142349f11 adds 3 for base -3.
    pub fn value_to_param(&self, value: f32) -> f32 {
        (value.log2() - self.log2_base).clamp(self.min, self.max)
    }
}


fn focus_offset(camera: &MainCamera) -> usize {
    0x100 + [0x70, 0x128, 0x1a8, 0x248, 0x300, 0x388][camera.mode_byte() as usize]
}
fn set_focus_distance(camera: &mut MainCamera, value: f32, force: bool) {
    let focus = focus_offset(camera);
    if force || camera.read_f32(focus + 0x18) != value {
        camera.write_f32(focus + 0x18, value);
        let mode = u32::from_le_bytes(camera.bytes()[focus..focus + 4].try_into().unwrap());
        if mode < 2 {
            camera.write_f32(focus, f32::from_bits(3));
            let position = [camera.read_f32(focus + 0x10), camera.read_f32(focus + 0x14)];
            camera.write_f32(focus + 4, position[0]);
            camera.write_f32(focus + 8, position[1]);
        }
    }
}
fn reset_slider(
    kind: u8,
    formation: &mut ImageFormationConfigs,
    camera: &mut MainCamera,
    brushes: &mut BrushSizes,
) -> bool {
    let lens = 0x100 + FOV_OFFSETS[camera.mode_byte() as usize];
    let defaults = formation.get(0).unwrap().clone();
    match kind {
        0..=2 => {
            for row in formation.iter_non_default_mut() {
                match kind {
                    0 => row.params_18[1] = defaults.params_18[1],
                    1 => row.params_18[2] = defaults.params_18[2],
                    2 => row.params_18[3] = defaults.params_18[3],
                    _ => unreachable!(),
                }
            }
            true
        }
        3..=5 => {
            for row in formation.iter_non_default_mut() {
                match kind {
                    3 => row.params_30[0] = 0.,
                    4 => row.word_38 = 0,
                    5 => row.params_30[1] = 0.015,
                    _ => unreachable!(),
                }
            }
            true
        }
        6 => {
            let default = camera.read_f32(lens + 0x14);
            camera.write_f32(lens + 8, default);
            true
        }
        7 => {
            let default = LogSlider { min: -2.0, max: 2.0, _pad: 0.5, log2_base: -3.0 }
                .param_to_value(0.5);
            set_focus_distance(camera, default, true);
            true
        }
        8 => {
            let default = camera.read_f32(lens + 0x24);
            camera.write_f32(lens + 0x18, default);
            true
        }
        0xa => {
            brushes.reset_path();
            true
        }
        0xb => {
            brushes.reset_water();
            true
        }
        0xc => {
            brushes.reset_garden();
            true
        }
        _ => false,
    }
}
fn n(b:&[u8],o:usize)->u32{u32::from_le_bytes(b[o..o+4].try_into().unwrap())}
fn main(){let data=include_bytes!("reset-native.bin");
for (i,b) in data.chunks_exact(1472).enumerate(){
let kind=n(b,0) as u8;let mode=n(b,4) as usize;let focus_mode=n(b,8);
let mut camera=MainCamera{raw:[0;0x4e0]};camera.raw[0x4d0]=mode as u8;
let lens=0x100+FOV_OFFSETS[mode];let focus=focus_offset(&camera);
for (o,v) in [(8,12.),(0x14,73.),(0x18,2.5),(0x24,7.5)]{camera.write_f32(lens+o,v);}
camera.write_f32(focus,f32::from_bits(focus_mode));
for (o,v) in [(4,-10.),(8,25.),(0x10,17.),(0x14,-23.),(0x18,0.125)]{camera.write_f32(focus+o,v);}
let mut formation=ImageFormationConfigs{configs:[
Row{params_18:[0.,-0.75,1.25,0.85],params_30:[0.6,0.005],word_38:0.07f32.to_bits()},
Row{params_18:[0.,10.,20.,30.],params_30:[40.,60.],word_38:50.0f32.to_bits()},
Row{params_18:[0.,11.,21.,31.],params_30:[41.,61.],word_38:51.0f32.to_bits()}]};
let mut brushes=BrushSizes{path:0.1,water:0.2,garden:0.3};
reset_slider(kind,&mut formation,&mut camera,&mut brushes);
assert_eq!(camera.raw,&b[212..1460],"reset {i} camera");
for (row,c) in formation.configs.iter().enumerate(){
let values=[c.params_18[1].to_bits(),c.params_18[2].to_bits(),c.params_18[3].to_bits(),c.params_30[0].to_bits(),c.word_38,c.params_30[1].to_bits()];
for (v,offset) in values.into_iter().zip([0x1c,0x20,0x24,0x30,0x38,0x34]){assert_eq!(v,n(b,12+row*64+offset),"reset {i} config row{row} offset{offset}");}}
for (j,v) in [brushes.path,brushes.water,brushes.garden].into_iter().enumerate(){assert_eq!(v.to_bits(),n(b,1460+j*4),"reset {i} brushes {j}");}
}
println!("native reset parameter cases passed: {}",data.len()/1472);}
