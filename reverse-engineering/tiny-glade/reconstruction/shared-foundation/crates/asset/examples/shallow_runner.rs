//! Line-oriented comparison harness for original B texture-option inline branches.
use asset::{
    inheritable::{Inherit, Inheritable as I},
    texture::{
        config::InheritableTextureOpts,
        desc::{CompressionMethod, Gamma, MipChain},
    },
};
use std::io::{self, BufRead, Write};
fn decode(v: &[u32]) -> InheritableTextureOpts {
    InheritableTextureOpts {
        resize: match v[0] {
            0 => I::Value(None),
            1 => I::Value(Some([v[1], v[2]])),
            2 => I::Inherit,
            3 => I::MissingDefault,
            _ => panic!("resize tag"),
        },
        mips: match v[3] {
            0 => I::Value(MipChain::None),
            1 => I::Value(MipChain::Full),
            2 => I::Value(MipChain::Limited(v[4])),
            3 => I::Inherit,
            4 => I::MissingDefault,
            _ => panic!("mips tag"),
        },
        gamma: match v[5] {
            0 => I::Value(Gamma::Linear),
            1 => I::Value(Gamma::Srgb),
            2 => I::Inherit,
            3 => I::MissingDefault,
            _ => panic!("gamma tag"),
        },
        compression: match v[6] {
            0 => I::Value(CompressionMethod::Color),
            1 => I::Value(CompressionMethod::Grayscale),
            2 => I::Value(CompressionMethod::None),
            3 => I::Inherit,
            4 => I::MissingDefault,
            _ => panic!("compression tag"),
        },
    }
}
fn encode(v: &InheritableTextureOpts) -> [u32; 7] {
    let (r, w, h) = match v.resize {
        I::Value(None) => (0, 0, 0),
        I::Value(Some([w, h])) => (1, w, h),
        I::Inherit => (2, 0, 0),
        I::MissingDefault => (3, 0, 0),
    };
    let (m, n) = match v.mips {
        I::Value(MipChain::None) => (0, 0),
        I::Value(MipChain::Full) => (1, 0),
        I::Value(MipChain::Limited(n)) => (2, n),
        I::Inherit => (3, 0),
        I::MissingDefault => (4, 0),
    };
    let g = match v.gamma {
        I::Value(Gamma::Linear) => 0,
        I::Value(Gamma::Srgb) => 1,
        I::Inherit => 2,
        I::MissingDefault => 3,
    };
    let c = match v.compression {
        I::Value(CompressionMethod::Color) => 0,
        I::Value(CompressionMethod::Grayscale) => 1,
        I::Value(CompressionMethod::None) => 2,
        I::Inherit => 3,
        I::MissingDefault => 4,
    };
    [r, w, h, m, n, g, c]
}
fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    for line in io::stdin().lock().lines() {
        let values: Vec<u32> = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        assert_eq!(values.len(), 14);
        let mut child = decode(&values[..7]);
        let parent = decode(&values[7..]);
        child.inherit_from(&parent);
        let fields = encode(&child);
        writeln!(
            out,
            "{} {} {} {} {} {} {}",
            fields[0], fields[1], fields[2], fields[3], fields[4], fields[5], fields[6]
        )
        .unwrap();
    }
}
