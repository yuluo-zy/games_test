use motomoto::{Writer,Reader};
use std::io::{self,Write,Seek,SeekFrom,Cursor};
use std::sync::{Arc,Mutex};
#[derive(Clone,Copy,Debug,PartialEq,Eq)] enum Fault {None,Position,PatchSeek,PatchWrite,Restore}
#[derive(Debug)]struct State {io:Cursor<Vec<u8>>,ops:Vec<String>,fault:Fault,phase:u8}
struct Stream(Arc<Mutex<State>>);
impl Write for Stream {
 fn write(&mut self,b:&[u8])->io::Result<usize>{let mut s=self.0.lock().unwrap();let pos=s.io.position();s.ops.push(format!("write({pos},{})",b.len()));if s.phase==1 && s.fault==Fault::PatchWrite{return Err(io::ErrorKind::Other.into())}s.io.write(b)}
 fn flush(&mut self)->io::Result<()>{let mut s=self.0.lock().unwrap();s.ops.push("flush".into());s.io.flush()}
}
impl Seek for Stream {fn seek(&mut self,p:SeekFrom)->io::Result<u64>{let mut s=self.0.lock().unwrap();s.ops.push(format!("seek({p:?})"));let fail= match p {SeekFrom::Current(0)=>s.phase==0 && s.fault==Fault::Position, SeekFrom::Start(8)=>{s.phase=1;s.fault==Fault::PatchSeek},SeekFrom::Start(17)=>{s.phase=2;s.fault==Fault::Restore},_=>false};if fail{return Err(io::ErrorKind::Other.into())}s.io.seek(p)}}
fn main(){
 // Same twenty original Cursor fixtures. Existing bytes mean overwriting,
 // allowing bounded original machine execution without Vec allocation calls.
 for start in [0u64,3,31,1024]{for n in [0usize,1,7,64,2048]{let mut c=Cursor::new(vec![0xa5;4096]);c.set_position(start);let mut w=Writer::new(c);{let mut ch=w.add_chunk(0x6867666564636261u64).unwrap();ch.write_all(&(0..n).map(|i|((i*31)%256) as u8).collect::<Vec<_>>()).unwrap();}let c=w.finish().unwrap();assert_eq!(c.position(),start+16+n as u64);let b=c.get_ref();assert_eq!(&b[start as usize..start as usize+8],b"abcdefgh");assert_eq!(&b[start as usize+8..start as usize+16],&(n as u64).to_le_bytes());}}
 for fault in [Fault::None,Fault::Position,Fault::PatchSeek,Fault::PatchWrite,Fault::Restore]{let shared=Arc::new(Mutex::new(State{io:Cursor::new(Vec::new()),ops:Vec::new(),fault:Fault::None,phase:0}));let mut w=Writer::new(Stream(shared.clone()));{let mut ch=w.add_chunk(7u64).unwrap();ch.write_all(b"x").unwrap();shared.lock().unwrap().fault=fault;}let failed=w.finish().is_err();assert_eq!(failed,fault!=Fault::None);let s=shared.lock().unwrap();println!("fault={fault:?} finish_err={failed} position={} ops={:?}",s.io.position(),s.ops);}
 for declared in [1u64<<63,u64::MAX-16,100]{let mut b=Vec::new();b.extend_from_slice(&7u64.to_le_bytes());b.extend_from_slice(&declared.to_le_bytes());b.resize(80,0);let r=Reader::new(&b);let mut it=r.iter();let kind=it.next().unwrap().unwrap_err().kind();if declared==100{assert_eq!(kind,io::ErrorKind::UnexpectedEof);for _ in 0..4{assert!(it.next().is_none())}}else{assert_eq!(kind,io::ErrorKind::InvalidInput);for _ in 0..3{let c=it.next().unwrap().unwrap();assert_eq!(c.tag,0);assert!(c.data.is_empty())}assert!(it.next().is_none())}}
 // A payload panic still unwinds through Drop and publishes partial bytes.
 let mut w=Writer::new(Cursor::new(Vec::new()));let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let mut c=w.add_chunk(7u64).unwrap();c.write_all(b"partial").unwrap();panic!("injected payload failure")}));assert!(result.is_err());let c=w.finish().unwrap();assert_eq!(Reader::new(c.get_ref()).iter().next().unwrap().unwrap().data,b"partial");
 // Borrowed reader slices point directly into the source; iteration performs
 // constant work per header and no payload copy or heap allocation.
 let mut w=Writer::new(Cursor::new(Vec::new()));for _ in 0..4096{w.add_chunk(7u64).unwrap().write_all(&[1;8]).unwrap();}let c=w.finish().unwrap();let bytes=c.get_ref();let now=std::time::Instant::now();let mut count=0;for _ in 0..100{for (i,ch) in Reader::new(bytes).iter().enumerate(){let ch=ch.unwrap();assert_eq!(ch.data.as_ptr(),bytes[24*i+16..].as_ptr());count+=1;}}println!("reader_count={count} elapsed_ns={} encoded_bytes={} payload_zero_copy=true",now.elapsed().as_nanos(),bytes.len());
 println!("review assertions passed");
}
