"""Preserve optional defensive projection API; restore original business API.

Only evidence candidate. The safe try adapter remains separately callable by
current users; native roof code uses project_onto_ray and exact constructors.
"""
from pathlib import Path
import hashlib,json
root=Path('D:/game');relative='tiny-glade/crates/country-core/src/utils/project_cursor_onto_ray.rs'
src=root/relative;base=src.read_bytes();txt=base.decode('utf8')
txt=txt.replace('let size = Vec2::new(size[0].max(1) as f32, size[1].max(1) as f32);','let size = Vec2::new(size[0] as f32, size[1] as f32);')
a=txt.index('        let normalized = Vec2::new(screen_x, screen_y) / self.size;')
b=txt.index('\n    }',a)
txt=txt[:a]+'''        // B140c906d0: its Y expression is intentionally (1-y)/height,
        // then doubled and incremented. Do not replace by conventional NDC.
        let x = screen_x / self.size.x;
        let y = (1.0 - screen_y) / self.size.y;
        Vec2::new((x + x - 1.0) * self.aspect, (y + y + 1.0) * self.unit)'''+txt[b:]
a=txt.index('        self.try_project_onto_ray(screen, world_to_view, view_to_clip, ray, range)')
b=txt.index('\n    }',a)
txt=txt[:a]+'''        // Original140ac84a0: no behind-camera, finite-value or fallback
        // branches. These two MAXSS constants are from original RNE bytes.
        let near = ray.origin;
        let far = ray.origin + ray.direction;
        let project = |point: Vec3| {
            let view = world_to_view.transform_point3(point);
            *view_to_clip * view.extend(1.0)
        };
        let a = project(near);
        let b = project(far);
        let scale = Vec2::new(screen.aspect, screen.unit);
        let start = a.truncate().truncate() / a.w * scale;
        let end = b.truncate().truncate() / b.w * scale;
        let direction = end - start;
        let numerator = (self.0 - start) * direction;
        let squared = direction * direction;
        let divisor = if squared.x + squared.y > MIN_PROJECTED_LENGTH_SQUARED {
            squared.x + squared.y
        } else {
            MIN_PROJECTED_LENGTH_SQUARED
        };
        let parameter = (numerator.x + numerator.y) / divisor;
        // MAXSS/MINSS select their second operand on equality or unordered.
        let parameter = if range[0] > parameter { range[0] } else { parameter };
        let parameter = if range[1] < parameter { range[1] } else { parameter };
        let inverse_a = 1.0 / a.w;
        let inverse_b = 1.0 / b.w;
        let numerator = 0.0 / a.w + (inverse_b - 0.0 / a.w) * parameter;
        let denominator = inverse_a + (inverse_b - inverse_a) * parameter;
        let denominator = if denominator > MIN_CLIP_WEIGHT { denominator } else { MIN_CLIP_WEIGHT };
        let weight = numerator / denominator;
        // Match native interpolation, including cancellation and signed zero.
        near * (1.0 - weight) + far * weight'''+txt[b:]
txt=txt.replace('/// 最小化窗口的零尺寸按 1 像素处理，避免产生无穷宽高比。','/// Original140ac8420 converts both u32 dimensions directly, including zero.')
txt=txt.replace('/// 投影失败时保持射线起点，避免无效窗口或相机参数污染编辑状态。\n    /// 新调用方可使用 try_project_onto_ray 区分投影失败。','/// Original723B projection with exact perspective correction and MAXSS policy.\n    /// The optional defensive adapter below is a separate host API.')
# Keep unrelated safe optional adapter and existing tests as review context;
# exact native fixtures will supersede tests which assumed modified behavior.
dst=Path(__file__).parent/'proposed'/relative;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_text(txt,encoding='utf8')
print(json.dumps({'path':relative,'base_sha256':hashlib.sha256(base).hexdigest(),'candidate_sha256':hashlib.sha256(dst.read_bytes()).hexdigest(),'tests_need_native_fixture_update':True},indent=2))
