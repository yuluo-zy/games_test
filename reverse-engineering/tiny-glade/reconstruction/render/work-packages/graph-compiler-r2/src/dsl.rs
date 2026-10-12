use super::*;

pub struct Fixture {
    pub graph: GraphDeclaration,
    pub pool: Pool,
    pub backend: EventBackend,
    pub dependency: FixtureDependency,
}
#[derive(Default)]
pub struct FixtureDependency {
    pub table: std::collections::BTreeMap<u8, (u32, bool)>,
}
impl AccessDependency for FixtureDependency {
    fn access_mask(&mut self, code: u8) -> u32 {
        self.table
            .get(&code)
            .expect("missing external access-info fixture")
            .0
    }
    fn is_write(&mut self, code: u8) -> bool {
        self.table
            .get(&code)
            .expect("missing external write-classifier fixture")
            .1
    }
}

pub fn parse(input: &str) -> Result<Fixture, String> {
    let mut graph = GraphDeclaration::default();
    let mut dependency = FixtureDependency::default();
    let mut backend = EventBackend::default();
    for (line, text) in input.lines().enumerate() {
        let fields = text.split_whitespace().collect::<Vec<_>>();
        if fields.is_empty() || fields[0].starts_with('#') {
            continue;
        }
        match fields[0] {
            "access-info" if fields.len() == 4 => {
                let code = fields[1]
                    .parse::<u8>()
                    .map_err(|_| format!("line {line}: dependency code"))?;
                let mask = if let Some(hex) = fields[2].strip_prefix("0x") {
                    u32::from_str_radix(hex, 16)
                } else {
                    fields[2].parse::<u32>()
                }
                .map_err(|_| format!("line {line}: dependency mask"))?;
                let write = fields[3]
                    .parse::<bool>()
                    .map_err(|_| format!("line {line}: dependency write"))?;
                dependency.table.insert(code, (mask, write));
            }
            "batch" if fields.get(1) == Some(&"per-pass") => graph.batch_per_pass = true,
            "resource" if fields.len() >= 4 => {
                let index = fields[1]
                    .parse::<usize>()
                    .map_err(|_| format!("line {line}: resource index"))?;
                if index != graph.resources.len() {
                    return Err(format!("line {line}: resources must be contiguous"));
                }
                let descriptor_id = fields[2]
                    .parse::<u32>()
                    .map_err(|_| format!("line {line}: descriptor"))?;
                let descriptor = if fields[3] == "image" {
                    Descriptor::Image(TextureDesc {
                        words_at_00_through_18: [descriptor_id, 0, 0, 0, 0, 0, 0],
                        bytes_at_1c_through_1e: [0, 0, 0],
                    })
                } else if fields[3] == "buffer" {
                    Descriptor::Buffer(BufferDesc {
                        word_at_00: descriptor_id as u64,
                        word_at_08: descriptor_id,
                        bytes_at_0c_through_0e: [0, 0, 0],
                    })
                } else {
                    return Err(format!("line {line}: resource kind"));
                };
                let source = if fields.contains(&"import-owned") {
                    OriginSource::ImportOwned {
                        token: descriptor_id as u64,
                        initial_access: fields
                            .get(5)
                            .and_then(|value| value.parse().ok())
                            .unwrap_or(0),
                    }
                } else if fields.contains(&"import-unowned") {
                    OriginSource::ImportUnowned {
                        token: descriptor_id as u64,
                        initial_access: fields
                            .get(5)
                            .and_then(|value| value.parse().ok())
                            .unwrap_or(0),
                    }
                } else {
                    OriginSource::Create
                };
                graph.resources.push(ResourceDeclaration {
                    name: format!("resource-{index}"),
                    descriptor,
                    source,
                });
            }
            "pass" if fields.len() >= 2 => {
                let accesses = fields[2..]
                    .iter()
                    .map(|spec| {
                        let values = spec.split(':').collect::<Vec<_>>();
                        if values.len() != 3 {
                            return Err(format!("line {line}: access"));
                        }
                        Ok(AccessDeclaration {
                            resource_index: values[0]
                                .parse()
                                .map_err(|_| format!("line {line}: resource"))?,
                            synchronization_group: values[1]
                                .parse()
                                .map_err(|_| format!("line {line}: group"))?,
                            access_code: values[2]
                                .parse()
                                .map_err(|_| format!("line {line}: code"))?,
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                graph.passes.push(PassDeclaration {
                    name: fields[1].to_string(),
                    accesses,
                });
            }
            "export" if fields.len() == 3 => {
                let resource_index = fields[1]
                    .parse()
                    .map_err(|_| format!("line {line}: resource"))?;
                let tag = graph
                    .resources
                    .get(resource_index as usize)
                    .map(|resource| resource.descriptor.kind_code())
                    .unwrap_or(0);
                graph.exports.push(ExportDeclaration {
                    tag,
                    resource_index,
                    access_code: fields[2]
                        .parse()
                        .map_err(|_| format!("line {line}: code"))?,
                });
            }
            "fail-create" if fields.len() == 3 => {
                backend.fail_at_creation = Some((
                    fields[1]
                        .parse()
                        .map_err(|_| format!("line {line}: failure attempt"))?,
                    fields[2]
                        .parse()
                        .map_err(|_| format!("line {line}: failure tag"))?,
                ));
            }
            _ => return Err(format!("line {line}: unknown directive")),
        }
    }
    Ok(Fixture {
        graph,
        pool: Pool::default(),
        backend,
        dependency,
    })
}
