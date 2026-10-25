from pathlib import Path
import difflib,hashlib,json
here=Path(__file__).resolve().parent;root=here.parents[4]
updates={}

def change(relative,transform):
    before=(root/relative).read_bytes();text=before.decode().replace('\r\n','\n')
    after=transform(text)
    if b'\r\n' in before:after=after.replace('\n','\r\n')
    updates[relative]=(before,after.encode())

addition='''
    /// Original140b02520/676B + factory140b027d0/1585B. Submit the existing
    /// named-mesh signifier, preserving the original Debug-format identity.
    pub fn show(&self, stream: &mut UiSignifierStream) {
        let sphere = *self;
        let factory = SignifierFactory::from_closure(
            &self.signifier_name(),
            Arc::new(move |meshes, shaders, commands| {
                // Original builder's named-mesh branch14089dde1..de45.
                // Missing required assets fail; no invisible fallback entity.
                let mesh = meshes.get_handle_by_name(sphere.color.sphere_mesh_name());
                let shader = shaders
                    .get_raster_handle_by_name("vertex_color_shader")
                    .expect("SignifierEntityBuilder: vertex_color_shader is not registered");
                let entity = commands.spawn((
                    DrawableMeshBundle {
                        mesh,
                        shader,
                        transform: Transform {
                            translation: sphere.center,
                            scale: Vec3::ZERO,
                            ..Transform::identity()
                        },
                    },
                    SignifierAnimation {
                        // Native easing enum tag1. Its inactive union padding
                        // does not carry game state; initialize that padding.
                        head: [1, 0, 0, 0, 0, 0, 0, 0],
                        flag_8: 0,
                        flag_9: 1,
                        value: sphere.radius,
                        duration: 0.0,
                        t: 0.0,
                        eased: 0.0,
                        ..SignifierAnimation::default()
                    },
                )).id();
                // Original builder emits these two markers in this order.
                commands.entity(entity).insert(UiPass);
                commands.entity(entity).insert(DontCastShadow);
                // The legacy public field alpha is native line_width; solid
                // is native filled_in. Preserve their existing typed callers.
                if !sphere.solid {
                    commands.entity(entity).insert(DrawTopology::Alt(sphere.alpha));
                }
                vec![entity]
            }),
        );
        stream.write_factory(&factory);
    }

    fn signifier_name(&self) -> String {
        let color = match self.color {
            DebugColor::White => "White".to_owned(),
            DebugColor::Red => "Red".to_owned(),
            DebugColor::Blue => "Blue".to_owned(),
            DebugColor::Green => "Green".to_owned(),
            DebugColor::Grey => "Grey".to_owned(),
            DebugColor::Custom(rgb) => format!("Custom({:?})", rgb.to_array()),
        };
        format!(
            "DebugSphere {{ color: {}, radius: {:?}, position: {:?}, line_width: {:?}, filled_in: {:?} }}",
            color, self.radius, self.center, self.alpha, self.solid
        )
    }
'''

def add_sphere(text):
    anchor='''    pub fn solid(mut self) -> Self {
        self.solid = true;
        self
    }
}'''
    assert text.count(anchor)==1
    return text.replace(anchor,anchor[:-1]+addition+'}')
change('tiny-glade/crates/country-core/src/utils/debug_viz.rs',add_sphere)
change('tiny-glade/crates/country-core/src/systems/wall/wall_state/mod.rs',lambda text:text+'\npub mod debug_viz_threshold_curve;\n')
def register(text):
    anchor='''        sys::wall::construct_wall_bricks::construct_wall_bricks,
'''
    assert text.count(anchor)==1
    return text.replace(anchor,anchor+'        sys::wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve,\n')
change('tiny-glade/crates/tiny-glade/src/startup_construction_runtime.rs',register)
for name in ['debug_viz_threshold_curve.rs','debug_threshold_policy.rs']:
    relative='tiny-glade/crates/country-core/src/systems/wall/wall_state/'+name
    updates[relative]=(b'',(here/'proposed'/relative).read_bytes())

patch='';manifest=[]
for relative,(before,after) in updates.items():
    target=here/'proposed'/relative;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(after)
    patch+=''.join(difflib.unified_diff(before.decode().splitlines(True),after.decode().splitlines(True),'a/'+relative if before else '/dev/null','b/'+relative))
    manifest.append({'path':relative,'base_sha256':hashlib.sha256(before).hexdigest() if before else None,'candidate_sha256':hashlib.sha256(after).hexdigest()})
(here/'production-review.patch').write_bytes(patch.encode())
(here/'candidate-checkpoint.json').write_text(json.dumps({'files':manifest,'status':'evidence candidate; native full producer/Sphere/show/builder branch executed; standalone leaf verification recorded separately; no Cargo or production writes'},indent=2),encoding='utf8')
print('debug threshold complete5-file narrow candidate generated')
