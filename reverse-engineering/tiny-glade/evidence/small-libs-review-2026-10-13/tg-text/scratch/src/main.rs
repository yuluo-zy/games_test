use serde::{Deserialize, Deserializer, de::{self, Visitor}};
use std::{alloc::{GlobalAlloc, Layout, System}, sync::atomic::{AtomicUsize, Ordering}, time::Instant, panic::{catch_unwind, AssertUnwindSafe}};
use tg_text::{ImageSource, ImageScaling, ShapeStream, RawShapeStream, FontTable};
struct Counter;
static ALLOCS: AtomicUsize=AtomicUsize::new(0);
unsafe impl GlobalAlloc for Counter {
 unsafe fn alloc(&self,l:Layout)->*mut u8{ALLOCS.fetch_add(1,Ordering::Relaxed); unsafe{System.alloc(l)}}
 unsafe fn dealloc(&self,p:*mut u8,l:Layout){unsafe{System.dealloc(p,l)}}
 unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{ALLOCS.fetch_add(1,Ordering::Relaxed);unsafe{System.realloc(p,l,n)}}
}
#[global_allocator] static GLOBAL:Counter=Counter;
struct Probe<'a>(&'a mut &'static str);
impl<'de> Deserializer<'de> for Probe<'_>{
 type Error=de::value::Error;
 fn deserialize_any<V:Visitor<'de>>(self,_:V)->Result<V::Value,Self::Error>{Err(de::Error::custom("unexpected request"))}
 fn deserialize_str<V:Visitor<'de>>(self,v:V)->Result<V::Value,Self::Error>{*self.0="str";v.visit_str("paper")}
 fn deserialize_string<V:Visitor<'de>>(self,v:V)->Result<V::Value,Self::Error>{*self.0="string";v.visit_str("paper")}
 serde::forward_to_deserialize_any!{bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any}
}
fn main(){
 let mut request="none"; let parsed=ImageSource::deserialize(Probe(&mut request)).unwrap(); assert_eq!(parsed,ImageSource::Name("paper".into()));
 let schema=[
 ("font_unknown", FontTable::from_ron(r#"(fonts:{Label:(family:"x",extra:(a:1,b:[2,3]))},unknown:[1,2])"#).is_ok()),
 ("image_unknown",ShapeStream::from_ron(r#"[Image(source:"x",scaling:Nine,unknown:[1,2])]"#).is_ok()),
 ("named_struct",FontTable::from_ron(r#"FontTable(fonts:{Label:FontTableEntry(family:"x")})"#).is_ok()),
 ("newtype_shape",ShapeStream::from_ron("ShapeStream([FillColor(1)])").is_ok()),
 ("raw_string",ShapeStream::from_ron(r##"[Image(source:r#"a\b"#,scaling:Stretch)]"##).is_ok()),
 ];
 let stream=ShapeStream::new().fill_color(7).image("x",ImageScaling::Nine).corner_radius(8.0);
 let mut partial=RawShapeStream(vec![99]); let panic=catch_unwind(AssertUnwindSafe(||partial.append(&stream,&mut |_|panic!("resolver fails")))).is_err();
 assert!(panic); assert_eq!(partial.words(),[99,0,7]);
 let mut measurements=Vec::new();
 for n in [1000usize,10000,100000]{
 let stream=ShapeStream((0..n).map(|i|tg_text::ShapeStreamItem::FillColor(i as u32)).collect());
 let mut raw=RawShapeStream::new(); ALLOCS.store(0,Ordering::SeqCst);raw.append(&stream,&mut |_|unreachable!());let fresh=ALLOCS.load(Ordering::SeqCst);
 raw.clear();let cap=raw.0.capacity(); ALLOCS.store(0,Ordering::SeqCst);raw.append(&stream,&mut |_|unreachable!()); let reuse=ALLOCS.load(Ordering::SeqCst);assert_eq!(reuse,0);assert_eq!(raw.0.len(),2*n);assert_eq!(raw.0.capacity(),cap);
 let loops=1000000/n; let start=Instant::now();for _ in 0..loops{raw.clear();raw.append(std::hint::black_box(&stream),&mut |_|unreachable!());std::hint::black_box(raw.words());}let nanos=start.elapsed().as_nanos();
 measurements.push(serde_json::json!({"n":n,"fresh_allocation_calls":fresh,"clear_reuse_allocation_calls":reuse,"loops":loops,"ns_per_item":nanos as f64/(loops*n) as f64,"capacity":cap}));
 }
 println!("{}",serde_json::json!({"image_source_deserializer_request":request,"schema_candidate_only":schema,"resolver_panic_kept_words":partial.words(),"performance_candidate_only":measurements}));
}
