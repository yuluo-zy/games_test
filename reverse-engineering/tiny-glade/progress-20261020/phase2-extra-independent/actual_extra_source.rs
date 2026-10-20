#![allow(dead_code)]
#[derive(Clone,Copy,Debug)]
pub enum DecoratorExtraInfo {
    /// disc 0/1：楼梯附加信息（20B 载荷，字段语义 U）。
    Stair(StairExtra),
    /// disc 6：窗附加信息 {u32@+0}。
    Window(WindowExtra),
    /// disc 5：旗附加信息 {u8@+0}。
    Flag(FlagExtra),
    /// disc 7：灯附加信息 {f32@+0}。
    Lantern(LanternExtra),
    /// disc 4：烟囱附加信息 {f32@+0, u8@+4}。
    Chimney(ChimneyExtra),
}
#[derive(Clone,Copy,Debug)]
pub struct StairExtra(pub [u8;20]);
#[derive(Clone,Copy,Debug)]
pub struct WindowExtra(pub u32);
#[derive(Clone,Copy,Debug)]
pub struct FlagExtra(pub u8);
#[derive(Clone,Copy,Debug)]
pub struct LanternExtra(pub f32);
#[derive(Clone,Copy,Debug)]
pub struct ChimneyExtra {
    /// +0：f32 载荷（判别访问 as_chimney 返回）。
    pub f: f32,
    /// +4：烟开关/浓度字节。
    pub smoke: u8,
}
#[repr(C)]
#[derive(Clone,Copy,Debug)]
pub struct DecoratorInfo {
    /// +0x00：destination payload (wall/roof coordinates, terrain XZ, or stair IDs).
    pub dst_wall: u64,
    /// +0x08：目的次级 id。
    pub dst_secondary: u64,
    /// +0x10：目的载荷（DecoratorExtraInfo 判别字节 @+0x11）。
    pub dst_payload: u64,
    /// +0x18：墙空间数据（字段语义 U）。
    pub u18: u64,
    /// +0x20：u32（dst tag 字节 @+0x22）。
    pub u20: u32,
    /// +0x24：u32（U）。
    pub u24: u32,
    /// +0x28：u32（U）。
    pub u28: u32,
    /// +0x2c：DecoratorId。
    pub decorator_id: u32,
    /// +0x30：u64（U）。
    pub u30: u64,
    /// +0x38：u32（U）。
    pub u38: u32,
    /// +0x3c：u32（U）。
    pub u3c: u32,
    /// +0x40：u32（U）。
    pub u40: u32,
    /// +0x44：rank（u8）。
    pub rank: u8,
    /// +0x45：u8（E2 取证 2026-10-16：rank(+0x44) 与 type(+0x46) 间 1 字节，
    /// rank/state 簇 +0x44..0x47，精确语义 U——待装饰渲染/实体消费端读面）。
    pub u45: u8,
    /// +0x46：DecoratorType（u8）。
    pub ty: u8,
    /// +0x47：状态字节（fresh 0x14155afe0：insert 写入 2=live/placed；U：状态语义
    /// 待取证）。
    pub state: u8,
}
impl DecoratorInfo {
pub fn extra_disc(self) -> u8 {
        ((self.u40 >> 8) & 0xff) as u8
    }
fn extra_bytes(self) -> [u8; 20] {
        let mut raw = [0u8; 20];
        raw[..8].copy_from_slice(&self.u30.to_le_bytes());
        raw[8..12].copy_from_slice(&self.u38.to_le_bytes());
        raw[12..16].copy_from_slice(&self.u3c.to_le_bytes());
        raw[16..20].copy_from_slice(&self.u40.to_le_bytes());
        raw
    }
pub fn extra(self) -> DecoratorExtraInfo {
        let raw = self.extra_bytes();
        match self.extra_disc() {
            0 | 1 => DecoratorExtraInfo::Stair(StairExtra(raw)),
            4 => DecoratorExtraInfo::Chimney(ChimneyExtra {
                f: f32::from_le_bytes(raw[..4].try_into().unwrap()),
                smoke: raw[4],
            }),
            5 => DecoratorExtraInfo::Flag(FlagExtra(raw[0])),
            6 => DecoratorExtraInfo::Window(WindowExtra(u32::from_le_bytes(
                raw[..4].try_into().unwrap(),
            ))),
            7 => DecoratorExtraInfo::Lantern(LanternExtra(f32::from_le_bytes(
                raw[..4].try_into().unwrap(),
            ))),
            d => panic!("unknown DecoratorExtraInfo discriminator {d:#x}"),
        }
    }
pub fn set_extra(&mut self, extra: DecoratorExtraInfo) {
        let mut raw = self.extra_bytes();
        match extra {
            DecoratorExtraInfo::Stair(v) => raw = v.0,
            DecoratorExtraInfo::Window(v) => {
                raw[..4].copy_from_slice(&v.0.to_le_bytes());
                raw[17] = 6;
            }
            DecoratorExtraInfo::Flag(v) => {
                raw[0] = v.0;
                raw[17] = 5;
            }
            DecoratorExtraInfo::Lantern(v) => {
                raw[..4].copy_from_slice(&v.0.to_le_bytes());
                raw[17] = 7;
            }
            DecoratorExtraInfo::Chimney(v) => {
                raw[..4].copy_from_slice(&v.f.to_le_bytes());
                raw[4] = v.smoke;
                raw[17] = 4;
            }
        }
        self.u30 = u64::from_le_bytes(raw[..8].try_into().unwrap());
        self.u38 = u32::from_le_bytes(raw[8..12].try_into().unwrap());
        self.u3c = u32::from_le_bytes(raw[12..16].try_into().unwrap());
        self.u40 = u32::from_le_bytes(raw[16..20].try_into().unwrap());
    }
pub fn extra_style(self) -> u8 {
        ((self.u40 >> 8) & 0xff) as u8
    }
}
impl DecoratorExtraInfo {
pub fn disc(self) -> u8 {
        match self {
            DecoratorExtraInfo::Stair(value) => value.0[17],
            DecoratorExtraInfo::Window(_) => 6,
            DecoratorExtraInfo::Flag(_) => 5,
            DecoratorExtraInfo::Lantern(_) => 7,
            DecoratorExtraInfo::Chimney(_) => 4,
        }
    }
}
fn main() {
    assert_eq!(std::mem::size_of::<DecoratorInfo>(),72);
    let bytes=include_bytes!("native-from-fixtures.bin");
    let mut total=0;let mut errors=0;
    for case in bytes.chunks_exact(165) {
        let kind=case[0];let before:[u8;72]=case[1..73].try_into().unwrap();let payload:[u8;20]=case[73..93].try_into().unwrap();let expected:[u8;72]=case[93..165].try_into().unwrap();
        let mut info:DecoratorInfo=unsafe{std::mem::transmute(before)};
        let value=match kind {0|1=>DecoratorExtraInfo::Stair(StairExtra(payload)),4=>DecoratorExtraInfo::Chimney(ChimneyExtra{f:f32::from_le_bytes(payload[..4].try_into().unwrap()),smoke:payload[4]}),5=>DecoratorExtraInfo::Flag(FlagExtra(payload[0])),6=>DecoratorExtraInfo::Window(WindowExtra(u32::from_le_bytes(payload[..4].try_into().unwrap()))),7=>DecoratorExtraInfo::Lantern(LanternExtra(f32::from_le_bytes(payload[..4].try_into().unwrap()))),_=>unreachable!()};
        info.set_extra(value);
        let actual:[u8;72]=unsafe{std::mem::transmute(info)};
        if actual!=expected {errors+=1;eprintln!("setter mismatch case{total} kind{kind}");}
        assert_eq!(info.extra_disc(),kind);assert_eq!(info.extra_style(),kind);
        let read=info.extra();
        let matches=match read {DecoratorExtraInfo::Stair(x)=>x.0==payload,DecoratorExtraInfo::Chimney(x)=>x.f.to_bits()==u32::from_le_bytes(payload[..4].try_into().unwrap())&&x.smoke==payload[4],DecoratorExtraInfo::Flag(x)=>x.0==payload[0],DecoratorExtraInfo::Window(x)=>x.0==u32::from_le_bytes(payload[..4].try_into().unwrap()),DecoratorExtraInfo::Lantern(x)=>x.0.to_bits()==u32::from_le_bytes(payload[..4].try_into().unwrap())};
        if !matches {errors+=1;eprintln!("getter mismatch case{total} kind{kind}");}
        if read.disc()!=kind {errors+=1;if errors<5{eprintln!("enumdisc mismatch case{total} kind{kind}, got{}",read.disc());}}
        // Independent destination union and rank/type/state remain untouched.
        assert_eq!(&actual[..48],&before[..48]);assert_eq!(&actual[68..],&before[68..]);
        total+=1;
    }
    println!("actual production ExtraInfo: {total} cases, {errors} errors");assert_eq!(errors,0);
}
