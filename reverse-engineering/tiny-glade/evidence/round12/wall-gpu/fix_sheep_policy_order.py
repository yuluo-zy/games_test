from pathlib import Path
p=Path('D:/game/reverse-engineering/tiny-glade/evidence/round12/wall-gpu/candidates-sheep/crates/systems/critters/src/startup_sheep.rs');s=p.read_text(encoding='utf-8');a=s.index('        let position = Vec3::new(');z=s.index('        let bird_spots =',a);s=s[:a]+'''        let pitch = [0.0, 0.3, 1.0][index];
        let mut audio = instantiate_audio("event:/SFX/Sheep/Baah")?;
        audio.set_parameter("Pitch", pitch);
        let x = rng.f32();
        let z = rng.f32();
        let id = birds.gen_id();
        let position = Vec3::new((x - 0.5) * 40.0, 0.0, (z - 0.5) * 40.0);
        let transform = Transform {
            rotation: Quat::IDENTITY,
            translation: position,
            scale: Vec3::splat(size),
        };
'''+s[z:];p.write_text(s,encoding='utf-8')
