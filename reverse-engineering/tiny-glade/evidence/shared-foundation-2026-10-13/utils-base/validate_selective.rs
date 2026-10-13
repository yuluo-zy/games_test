// Standalone comparison driver, linked against exact workspace serde_json.
// Cases check the recovered visitor policy; original JSON parser is reused.
use utils::selective_deserialize::SaveHistorySnapshots;
use serde::Deserialize;
fn read(bytes:&[u8])->Vec<u64> {
    let mut deserializer=serde_json::Deserializer::from_slice(bytes);
    SaveHistorySnapshots::<Vec<u64>>::deserialize(&mut deserializer).unwrap().value.value
}
fn main() {
    let cases:&[(&[u8],&[u64])]=&[
        (br#"{"History":{"snapshots":[11,13]}}"#,&[11,13]),
        (br#"{"unknown":{"deep":[3,4]},"History":{"unknown":true,"snapshots":[7]}}"#,&[7]),
        (br#"{"snapshots":[5]}"#,&[]),
        (br#"{"History":{}}"#,&[]),
        (br#"{"History":{"snapshots":"wrong type"}}"#,&[]),
        (br#"{"History":{"snapshots":[1,"wrong type"]}}"#,&[]),
        (br#"{"History":{"snapshots":[1,2],"truncated": }}"#,&[1,2]),
        (br#"{"History":{"snapshots":[4]},"truncated": }"#,&[4]),
        (br#"{"History":{"snapshots":[1],"snapshots":[2,3]}}"#,&[2,3]),
        (br#"{"History":{"snapshots":[1],"snapshots":"wrong type"}}"#,&[1]),
        (br#"{"History":{"snapshots":[1]},"History":{"snapshots":[2]}}"#,&[2]),
        (br#"{"History":{"snapshots":[1]},"History":5}"#,&[]),
    ];
    for (input,expected) in cases {assert_eq!(read(input),*expected,"input={}",String::from_utf8_lossy(input));}
    println!("12 selected-field/nested/partial-success candidate policy cases passed; original parser reused; original visitor full-runtime diff not executed");
}
