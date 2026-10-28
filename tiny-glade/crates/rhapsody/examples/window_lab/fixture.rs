use serde::Deserialize;
use std::path::{Path, PathBuf};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub const WIDTH: u32 = 320;
pub const HEIGHT: u32 = 180;

pub fn default_playlist() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let assets = if root.join("assets/window_lab").is_dir() {
        root.join("assets/window_lab")
    } else {
        // The standalone tools/window-lab manifest uses the exact same source files.
        root.join("../../crates/rhapsody/assets/window_lab")
    };
    assets.join("playlist.json")
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    version: u32,
    clips: Vec<Clip>,
}
#[derive(Deserialize)]
pub struct Clip {
    pub name: String,
    pub frames: u32,
    #[serde(flatten)]
    source: Source,
    #[serde(skip)]
    image: Option<Ppm>,
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Source {
    Bars,
    Checker,
    Motion,
    Ppm { path: PathBuf },
}
pub struct Playlist {
    pub clips: Vec<Clip>,
    pub fingerprint: String,
}
pub struct Ppm {
    width: usize,
    height: usize,
    rgb: Vec<u8>,
}

pub fn hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    })
}

impl Playlist {
    pub fn load(path: &Path) -> Result<Self> {
        let data = std::fs::read(path)?;
        let mut manifest: Manifest = serde_json::from_slice(&data)?;
        if manifest.version != 1 || manifest.clips.is_empty() || manifest.clips.len() > 128 {
            return Err("playlist version must be 1 with 1..=128 clips".into());
        }
        let root = path.parent().unwrap_or(Path::new("."));
        let mut fingerprint = hash(&data);
        let mut total_bytes = 0u64;
        for clip in &mut manifest.clips {
            if clip.frames == 0 || clip.name.is_empty() {
                return Err("each clip needs a name and a positive frame count".into());
            }
            if let Source::Ppm { path } = &clip.source {
                let path = root.join(path);
                let size = std::fs::metadata(&path)?.len();
                total_bytes = total_bytes.checked_add(size).ok_or("asset size overflow")?;
                if total_bytes > 64 * 1024 * 1024 {
                    return Err("playlist images exceed the 64 MiB input budget".into());
                }
                let bytes = std::fs::read(path)?;
                fingerprint = fingerprint.rotate_left(7) ^ hash(&bytes);
                clip.image = Some(Ppm::parse(&bytes)?);
            }
        }
        Ok(Self {
            clips: manifest.clips,
            fingerprint: format!("fnv1a64:{fingerprint:016x}"),
        })
    }
    pub fn render(&self, index: usize, frame: u32, output: &mut [u8]) {
        assert_eq!(output.len(), (WIDTH * HEIGHT * 4) as usize);
        let clip = &self.clips[index];
        let colors = [
            [255, 255, 255],
            [255, 255, 0],
            [0, 255, 255],
            [0, 255, 0],
            [255, 0, 255],
            [255, 0, 0],
            [0, 0, 255],
            [0, 0, 0],
        ];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let rgb = match &clip.source {
                    Source::Bars => {
                        if y < HEIGHT * 3 / 4 {
                            colors[(x * 8 / WIDTH) as usize]
                        } else {
                            let v = (x * 255 / (WIDTH - 1)) as u8;
                            [v, v, v]
                        }
                    }
                    Source::Checker => {
                        if ((x + frame) / 16 + y / 16).is_multiple_of(2) {
                            [32, 44, 64]
                        } else {
                            [224, 234, 244]
                        }
                    }
                    Source::Motion => {
                        let cx = 30 + frame.wrapping_mul(3) % (WIDTH - 60);
                        let cy = HEIGHT / 2;
                        let dx = x as i32 - cx as i32;
                        let dy = y as i32 - cy as i32;
                        if dx * dx + dy * dy < 25 * 25 {
                            [255, 176, 64]
                        } else {
                            [
                                16 + (x * 30 / WIDTH) as u8,
                                24,
                                48 + (y * 50 / HEIGHT) as u8,
                            ]
                        }
                    }
                    Source::Ppm { .. } => {
                        let image = clip.image.as_ref().expect("validated playlist image");
                        let p = ((y as usize * image.height / HEIGHT as usize) * image.width
                            + x as usize * image.width / WIDTH as usize)
                            * 3;
                        [image.rgb[p], image.rgb[p + 1], image.rgb[p + 2]]
                    }
                };
                let i = ((y * WIDTH + x) * 4) as usize;
                output[i..i + 4].copy_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
            }
        }
        // Deliberately asymmetric orientation markers: red top-left, blue bottom-right.
        for y in 0..8 {
            for x in 0..8 {
                let a = ((y * WIDTH + x) * 4) as usize;
                let b = (((HEIGHT - 1 - y) * WIDTH + WIDTH - 1 - x) * 4) as usize;
                output[a..a + 4].copy_from_slice(&[255, 0, 0, 255]);
                output[b..b + 4].copy_from_slice(&[0, 0, 255, 255]);
            }
        }
    }
}

impl Ppm {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        fn token<'a>(bytes: &'a [u8], at: &mut usize) -> Result<&'a str> {
            loop {
                while bytes.get(*at).is_some_and(u8::is_ascii_whitespace) {
                    *at += 1;
                }
                if bytes.get(*at) == Some(&b'#') {
                    while bytes.get(*at).is_some_and(|b| *b != b'\n') {
                        *at += 1;
                    }
                } else {
                    break;
                }
            }
            let start = *at;
            while bytes
                .get(*at)
                .is_some_and(|b| !b.is_ascii_whitespace() && *b != b'#')
            {
                *at += 1;
            }
            if start == *at {
                return Err("truncated PPM header or pixels".into());
            }
            Ok(std::str::from_utf8(&bytes[start..*at])?)
        }
        let mut at = 0;
        let magic = token(bytes, &mut at)?;
        if magic != "P3" && magic != "P6" {
            return Err("only P3/P6 PPM images are supported".into());
        }
        let width: usize = token(bytes, &mut at)?.parse()?;
        let height: usize = token(bytes, &mut at)?.parse()?;
        let max: u16 = token(bytes, &mut at)?.parse()?;
        let size = width
            .checked_mul(height)
            .and_then(|n| n.checked_mul(3))
            .ok_or("image overflow")?;
        if width == 0 || height == 0 || width > 4096 || height > 4096 || max != 255 {
            return Err("PPM must have 1..=4096 dimensions and maxval 255".into());
        }
        let rgb = if magic == "P6" {
            if !bytes.get(at).is_some_and(u8::is_ascii_whitespace) {
                return Err("missing PPM separator".into());
            }
            // Consume only the header delimiter: pixel values may themselves be whitespace.
            if bytes.get(at..at + 2) == Some(b"\r\n") {
                at += 2;
            } else {
                at += 1;
            }
            if bytes.len() - at != size {
                return Err("PPM pixel length mismatch".into());
            }
            bytes[at..].to_vec()
        } else {
            let mut rgb = Vec::with_capacity(size);
            for _ in 0..size {
                rgb.push(token(bytes, &mut at)?.parse::<u8>()?);
            }
            if token(bytes, &mut at).is_ok() {
                return Err("extra PPM pixels".into());
            }
            rgb
        };
        Ok(Self { width, height, rgb })
    }
}

pub fn write_ppm(path: &Path, rgba: &[u8]) -> Result<()> {
    use std::io::Write;
    let mut out = std::io::BufWriter::new(std::fs::File::create(path)?);
    write!(out, "P6\n{WIDTH} {HEIGHT}\n255\n")?;
    for pixel in rgba.as_chunks::<4>().0 {
        out.write_all(&pixel[..3])?;
    }
    out.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary_pixels_that_look_like_whitespace_are_preserved() {
        let image = Ppm::parse(b"P6\n1 1\n255\n\x0a\x20\x23").unwrap();
        assert_eq!(image.rgb, [10, 32, 35]);
    }
    #[test]
    fn invalid_sizes_and_truncated_images_are_rejected() {
        for bytes in [
            b"P3 0 1 255".as_slice(),
            b"P3 1 1 255 0 0",
            b"P6 1 1 255\n\0",
            b"P3 99999999999999999 9 255",
        ] {
            assert!(Ppm::parse(bytes).is_err());
        }
    }
    #[test]
    fn playlist_and_dynamic_frames_are_reproducible() {
        let path = default_playlist();
        let playlist = Playlist::load(&path).unwrap();
        let mut a = vec![0; (WIDTH * HEIGHT * 4) as usize];
        let mut b = a.clone();
        playlist.render(2, 10, &mut a);
        playlist.render(2, 10, &mut b);
        assert_eq!(a, b);
        playlist.render(2, 20, &mut b);
        assert_ne!(a, b);
        assert_eq!(&a[..4], &[255, 0, 0, 255]);
    }
}
