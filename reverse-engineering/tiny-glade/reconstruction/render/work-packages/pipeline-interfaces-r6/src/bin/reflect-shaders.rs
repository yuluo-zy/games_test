use serde_json::{json, Value};
use spirq::{ty::Type, ReflectConfig, SpirvBinary, Variable};
use std::{env, fs, path::Path};

fn type_json(ty: &Type) -> Value {
    let detail = match ty {
        Type::Struct(value) => {
            json!({"kind":"Struct","members":value.members.iter().map(|member| json!({"name":member.name,"offset":member.offset,"type":type_json(&member.ty)})).collect::<Vec<_>>()})
        }
        Type::Array(value) => {
            json!({"kind":"Array","count":value.nrepeat,"stride":value.stride,"element":type_json(&value.proto_ty)})
        }
        Type::Matrix(value) => {
            json!({"kind":"Matrix","axis_order":value.major.map(|major|format!("{major:?}")),"vector":value.vec_ty.to_string(),"count":value.nvec,"stride":value.stride})
        }
        Type::DevicePointer(value) => {
            json!({"kind":"DevicePointer","target":type_json(&value.pointee_ty)})
        }
        _ => json!({"kind":ty.to_string()}),
    };
    json!({"minimum_bytes":ty.nbyte(),"detail":detail})
}

fn reflect(words: &[u32]) -> Result<Vec<Value>, String> {
    let entries = ReflectConfig::new()
        .spv(SpirvBinary::from(words))
        .ref_all_rscs(true)
        .combine_img_samplers(false)
        .gen_unique_names(false)
        .reflect()
        .map_err(|error| error.to_string())?;
    Ok(entries.into_iter().map(|entry| {
        let mut descriptors = Vec::new();
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        let mut push_constants = Vec::new();
        let mut spec_constants = Vec::new();
        for variable in entry.vars {
            match variable {
                Variable::Descriptor { name, desc_bind, desc_ty, ty, nbind } => descriptors.push(json!({"name":name,"set":desc_bind.set(),"binding":desc_bind.bind(),"descriptor_type":format!("{desc_ty:?}"),"count":nbind,"type":type_json(&ty)})),
                Variable::Input { name, location, ty } => inputs.push(json!({"name":name,"location":location.loc(),"component":location.comp(),"type":type_json(&ty)})),
                Variable::Output { name, location, ty } => outputs.push(json!({"name":name,"location":location.loc(),"component":location.comp(),"type":type_json(&ty)})),
                Variable::PushConstant { name, ty } => push_constants.push(json!({"name":name,"type":type_json(&ty)})),
                Variable::SpecConstant { name, spec_id, ty } => spec_constants.push(json!({"name":name,"spec_id":spec_id,"type":type_json(&ty)})),
            }
        }
        descriptors.sort_by_key(|value| (value["set"].as_u64(), value["binding"].as_u64()));
        inputs.sort_by_key(|value| (value["location"].as_u64(), value["component"].as_u64()));
        outputs.sort_by_key(|value| (value["location"].as_u64(), value["component"].as_u64()));
        json!({"stage":format!("{:?}",entry.exec_model),"entry_point":entry.name,"descriptors":descriptors,"inputs":inputs,"outputs":outputs,"push_constants":push_constants,"specialization_constants":spec_constants,
            "execution_modes":entry.exec_modes.iter().map(|mode|json!({"mode":format!("{:?}",mode.exec_mode),"operands":mode.operands.iter().map(|operand|json!({"value":operand.value.to_u32(),"spec_id":operand.spec_id})).collect::<Vec<_>>()})).collect::<Vec<_>>()})
    }).collect())
}

fn modules(path: &Path) -> Value {
    let bytes = fs::read(path).unwrap();
    let mut cursor = 0;
    let mut modules = Vec::new();
    while cursor + 4 <= bytes.len() {
        let Some(relative) = bytes[cursor..]
            .windows(4)
            .position(|window| window == [3, 2, 35, 7])
        else {
            break;
        };
        let offset = cursor + relative;
        if offset < 4 {
            modules.push(json!({"offset":offset,"error":"no preceding module word count"}));
            cursor = offset + 4;
            continue;
        }
        let count = u32::from_le_bytes(bytes[offset - 4..offset].try_into().unwrap()) as usize;
        let Some(end) = count
            .checked_mul(4)
            .and_then(|size| offset.checked_add(size))
        else {
            break;
        };
        if count < 5 || end > bytes.len() {
            modules.push(json!({"offset":offset,"error":"invalid module size","word_count":count}));
            cursor = offset + 4;
            continue;
        }
        let words: Vec<u32> = bytes[offset..end]
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect();
        let result = std::panic::catch_unwind(|| reflect(&words));
        modules.push(match result {
            Ok(Ok(entries)) => json!({"offset":offset,"word_count":count,"spirv_version":words[1],"entries":entries}),
            Ok(Err(error)) => json!({"offset":offset,"word_count":count,"error":error}),
            Err(_) => json!({"offset":offset,"word_count":count,"error":"reflection library panic"}),
        });
        cursor = end;
    }
    json!({"file":path.file_name().unwrap().to_string_lossy(),"bytes":bytes.len(),"modules":modules})
}

fn main() {
    let directory = env::args().nth(1).expect("compiled-shaders directory");
    let mut paths: Vec<_> = fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "bin"))
        .collect();
    paths.sort();
    let files: Vec<_> = paths.iter().map(|path| modules(path)).collect();
    println!("{}", serde_json::to_string(&files).unwrap());
}
