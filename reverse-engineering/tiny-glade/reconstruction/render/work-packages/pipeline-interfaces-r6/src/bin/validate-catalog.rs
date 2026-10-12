use serde_json::{json, Value};
use std::{env, fs};
use tiny_glade_pipeline_interfaces_r6::{
    merge_bindings, validate_push_constants, validate_stage_link, DescriptorBinding,
    PushConstantRange, StageLocation,
};

fn strip_names(value: &Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.iter().map(strip_names).collect()),
        Value::Object(values) => Value::Object(
            values
                .iter()
                .filter(|(key, _)| key.as_str() != "name")
                .map(|(key, value)| (key.clone(), strip_names(value)))
                .collect(),
        ),
        _ => value.clone(),
    }
}

fn locations(entry: &Value, key: &str) -> Vec<StageLocation> {
    entry[key]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| StageLocation {
            location: value["location"].as_u64().unwrap() as u32,
            component: value["component"].as_u64().unwrap() as u32,
            type_signature: serde_json::to_string(&strip_names(&value["type"])).unwrap(),
        })
        .collect()
}

fn main() {
    let path = env::args().nth(1).expect("catalog.json path");
    let catalog: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    let mut results = Vec::new();
    let mut descriptor_failures = 0;
    let mut stage_failures = 0;
    let mut push_failures = 0;
    let mut raster_pairs = 0;
    for file in catalog["files"].as_array().unwrap() {
        let entries: Vec<_> = file["modules"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|module| module["entries"].as_array().into_iter().flatten())
            .collect();
        let mut bindings = Vec::new();
        let mut pushes = Vec::new();
        for entry in &entries {
            let stage = entry["stage"].as_str().unwrap().to_string();
            for descriptor in entry["descriptors"].as_array().unwrap() {
                bindings.push(DescriptorBinding {
                    set: descriptor["set"].as_u64().unwrap() as u32,
                    binding: descriptor["binding"].as_u64().unwrap() as u32,
                    descriptor_type: descriptor["descriptor_type"]
                        .as_str()
                        .unwrap()
                        .split('(')
                        .next()
                        .unwrap()
                        .into(),
                    count: descriptor["count"].as_u64().unwrap() as u32,
                    stages: vec![stage.clone()],
                });
            }
            for push in entry["push_constants"].as_array().unwrap() {
                let members = push["type"]["detail"]["members"].as_array();
                let offset = members
                    .and_then(|members| {
                        members
                            .iter()
                            .filter_map(|member| member["offset"].as_u64())
                            .min()
                    })
                    .unwrap_or(0) as u32;
                let end = push["type"]["minimum_bytes"].as_u64().unwrap_or(0) as u32;
                pushes.push(PushConstantRange {
                    offset,
                    size: end.saturating_sub(offset),
                    stages: vec![stage.clone()],
                });
            }
        }
        let merged = merge_bindings(&bindings);
        let push_result = validate_push_constants(&pushes);
        descriptor_failures += usize::from(merged.is_err());
        push_failures += usize::from(push_result.is_err());
        let vertex = entries.iter().find(|entry| entry["stage"] == "Vertex");
        let fragment = entries.iter().find(|entry| entry["stage"] == "Fragment");
        let link = if let (Some(vertex), Some(fragment)) = (vertex, fragment) {
            raster_pairs += 1;
            validate_stage_link(
                &locations(vertex, "outputs"),
                &locations(fragment, "inputs"),
            )
        } else {
            Ok(())
        };
        stage_failures += usize::from(link.is_err());
        results.push(json!({"file":file["file"],"merged_descriptor_bindings":merged.as_ref().ok(),"descriptor_error":merged.err().map(|error|format!("{error:?}")),"push_constant_ranges":pushes,"push_error":push_result.err().map(|error|format!("{error:?}")),"location_link_error":link.err().map(|error|format!("{error:?}"))}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema_version":1,"files":results.len(),"raster_pairs":raster_pairs,"descriptor_failures":descriptor_failures,"push_range_failures":push_failures,"stage_location_failures":stage_failures,"results":results,"limits":["Location/component exact-type checks only; built-ins and full Vulkan linking not validated","Descriptor count0 remains runtime sized","Push ranges omit device maxPushConstantsSize check"]})).unwrap());
}
