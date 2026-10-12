//! 用本工程公开资源接口生成可对照输出；不要求候选枚举具有原 ABI 布局。
use rhapsody::rg::resource::{
    BorrowedResource, OriginTypeError, ResourceHandle, ResourceOriginLayout, RuntimeResource,
    descriptor_from_buffer, descriptor_from_texture, validate_read_access,
};
use rhapsody::rg::resource_registry::{Buffer, BufferDesc, RegistryResource, Texture, TextureDesc};
use serde_json::{Value, json};
use std::io::{self, BufRead};
use std::sync::Arc;

fn parse_hex(value: &str) -> Vec<u8> {
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn hex(value: &[u8]) -> String {
    value.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn probe(case: &Value) -> Value {
    let input = &case["input"];
    match case["function"].as_str().unwrap() {
        "ResourceHandle::raw" => {
            let raw = ResourceHandle::from_raw(input["rcx"].as_u64().unwrap() as u32);
            json!({"returned": true, "raw": raw.raw()})
        }
        "ResourceOrigin::ty" => {
            let mut layout = ResourceOriginLayout {
                tag: input["origin_tag"].as_u64().unwrap(),
                payload: [0; 64],
            };
            layout.payload[46] = input["descriptor_byte_at_54"].as_u64().unwrap_or(0) as u8;
            layout.payload[0] = input["kind_byte_at_8"].as_u64().unwrap_or(0) as u8;
            match layout.ty() {
                Ok(kind) => json!({"returned": true, "kind": kind}),
                Err(OriginTypeError::DescriptorKind8) => {
                    json!({"returned": false, "panic_boundary": "unsupported_descriptor"})
                }
                Err(OriginTypeError::InvalidOrigin(_)) => {
                    json!({"returned": false, "panic_boundary": "kind"})
                }
            }
        }
        "RuntimeResource::borrow" => {
            let texture = || Texture {
                handle: 0xabcdu64,
                desc: TextureDesc {
                    words_at_00_through_18: [0; 7],
                    bytes_at_1c_through_1e: [0; 3],
                },
            };
            let buffer = || Buffer {
                handle: 0x1234u64,
                desc: BufferDesc {
                    word_at_00: 0,
                    word_at_08: 0,
                    bytes_at_0c_through_0e: [0; 3],
                },
            };
            let arc = Arc::new(0xcafeu64);
            let resource: RuntimeResource<u64, u64, u64> =
                match input["runtime_variant_tag"].as_u64().unwrap() {
                    0 => RegistryResource::OwnedTexture(texture()),
                    1 => RegistryResource::UnownedTexture(texture()),
                    2 => RegistryResource::OwnedBuffer(buffer()),
                    3 => RegistryResource::UnownedBuffer(buffer()),
                    4 => RegistryResource::Other(Arc::clone(&arc)),
                    5 => RegistryResource::Empty,
                    _ => unreachable!(),
                };
            // 检查返回引用确实指向原载荷，不能通过复制一个同值对象假装借用。
            match std::panic::catch_unwind(|| resource.borrow()) {
                Ok(BorrowedResource::Image(value)) => {
                    let points_to_payload = match &resource {
                        RegistryResource::OwnedTexture(original)
                        | RegistryResource::UnownedTexture(original) => {
                            std::ptr::eq(value, original)
                        }
                        _ => false,
                    };
                    json!({"returned": true, "kind": 0, "points_to_payload": points_to_payload})
                }
                Ok(BorrowedResource::Buffer(value)) => {
                    let points_to_payload = match &resource {
                        RegistryResource::OwnedBuffer(original)
                        | RegistryResource::UnownedBuffer(original) => {
                            std::ptr::eq(value, original)
                        }
                        _ => false,
                    };
                    json!({"returned": true, "kind": 1, "points_to_payload": points_to_payload})
                }
                Ok(BorrowedResource::Other(value)) => {
                    json!({"returned": true, "kind": 2, "points_to_payload": std::ptr::eq(value, arc.as_ref())})
                }
                Err(_) => json!({"returned": false, "panic_boundary": "kind"}),
            }
        }
        "descriptor_from_texture" => {
            let source: [u8; 32] = parse_hex(input["source_hex"].as_str().unwrap())
                .try_into()
                .unwrap();
            json!({"returned": true, "output_hex": hex(&descriptor_from_texture(&source))})
        }
        "descriptor_from_buffer" => {
            let source = parse_hex(input["source_hex"].as_str().unwrap());
            let first: [u8; 16] = source[..16].try_into().unwrap();
            let mut output: [u8; 32] = parse_hex(input["output_before_hex"].as_str().unwrap())
                .try_into()
                .unwrap();
            descriptor_from_buffer(&first, &mut output);
            json!({"returned": true, "output_hex": hex(&output)})
        }
        "validate_read_access" => {
            validate_read_access(0xff);
            json!({"returned": true})
        }
        _ => unreachable!(),
    }
}

fn main() {
    assert_eq!(std::mem::size_of::<ResourceOriginLayout>(), 72);
    // Empty 的实际候选借用会触发 panic，本探针只记录失败，不输出测试噪声。
    std::panic::set_hook(Box::new(|_| {}));
    for line in io::stdin().lock().lines() {
        let value: Value = serde_json::from_str(&line.unwrap()).unwrap();
        println!("{}", probe(&value));
    }
}
