from pathlib import Path
import hashlib,json,subprocess,sys
root=Path('D:/game');here=Path(__file__).parent;deps=root/'tiny-glade/target-round12/debug/deps'
core=max(deps.glob('libcountry_core-*.rlib'),key=lambda p:p.stat().st_mtime)
serde=max(deps.glob('libserde_json-*.rlib'),key=lambda p:p.stat().st_mtime)
brick='--bricks' in sys.argv
tail='--resume-tail' in sys.argv
stem=('brick' if brick else 'plank')+'-pool'+('-tail' if tail else '')
source=r'''use country_core::render::{sparse_instance_buffer::{SparseInstanceBuffer,SparseSsbo},ssbo::{StorageElement,rhapsody_boundary::{GraphicsDevice,Buffer,BufferDesc,BackendError}}};
const WORDS:usize=16;
#[repr(C)] #[derive(Clone,Copy,Debug,Default)] struct Elem<const N:usize>([u32;WORDS]);
impl<const N:usize> StorageElement for Elem<N>{const NAME:&'static str="native_pool_fixture";const BUFFER_LEN:usize=N*32;}
unsafe impl<const N:usize> SparseSsbo for Elem<N>{const CHUNK_COUNT:usize=N;}
struct Device;
impl GraphicsDevice for Device{fn create_buffer(&self,d:&BufferDesc,_:&str,_:Option<&[u8]>)->Result<Buffer,BackendError>{Ok(Buffer{byte_size:d.size as u64,..Buffer::default()})}fn update_buffer_bytes(&self,_:&Buffer,_:&[u8],_:&[std::ops::Range<usize>],_:usize)->Result<(),BackendError>{Err(BackendError::invalid_usage("CPU native-differential fixture has no upload device"))}}
fn check<const N:usize>(case:&serde_json::Value){
 let mut pool=SparseInstanceBuffer::<Elem<N>>::try_new(&Device).unwrap();
 let allocations=[pool.new_allocation(),pool.new_allocation()];let mut seed=1u32;
 for step in case["steps"].as_array().unwrap(){
   let wall=step["wall"].as_u64().unwrap() as usize;let mut writer=pool.writer(&allocations[wall]);
   if step["operation"]=="clear"{writer.clear();}else{
     for failed in step["failures"].as_array().unwrap(){
       let record=Elem::<N>(std::array::from_fn(|j| seed*WORDS as u32+j as u32));seed+=1;
       assert_eq!(writer.push(&record).is_err(),failed.as_bool().unwrap());
     }
   }
   drop(writer);let expected=&step["state"];
   let counts:Vec<_>=expected["counts"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();
   assert_eq!(pool.byte_map.as_ref(),counts.as_slice());
   assert_eq!(pool.active_bitset[0].load(std::sync::atomic::Ordering::Relaxed),expected["active"].as_u64().unwrap() as usize);
   assert_eq!(pool.dirty_bitset[0].load(std::sync::atomic::Ordering::Relaxed),expected["dirty"].as_u64().unwrap() as usize);
   let free:Vec<_>=expected["free"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u32).collect();assert_eq!(*pool.free_chunks.read(),free);
   for (i,a) in allocations.iter().enumerate(){let ids:Vec<_>=expected["allocations"][i].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u32).collect();assert_eq!(*a.handle.0.as_ref().unwrap().chunk_ids.lock(),ids);}
   let observed:String=pool.data.iter().flat_map(|v|v.0.iter().flat_map(|v|v.to_le_bytes())).map(|b|format!("{b:02x}")).collect();assert_eq!(observed,expected["data"].as_str().unwrap());
 }
}
fn main(){let fixture:serde_json::Value=serde_json::from_str(include_str!("plank-pool-native-cases.json")).unwrap();for c in fixture["cases"].as_array().unwrap(){match c["chunks"].as_u64().unwrap(){1=>check::<1>(c),2=>check::<2>(c),3=>check::<3>(c),_=>unreachable!()}}println!("24 original sparse pool cases / 192 checkpoints matched every push failure, count, allocation, active/dirty bit, free order and64B payload");}
'''
source=source.replace('include_str!("plank-pool-native-cases.json")','include_str!("'+stem+'-native-cases.json")')
if brick:source=source.replace('const WORDS:usize=16;', 'const WORDS:usize=24;').replace('and64B payload','and96B payload')
if tail:source=source.replace('let mut writer=pool.writer(', 'let mut writer=pool.append_writer(')
driver=here/(stem+'-check.rs');driver.write_text(source)
exe=here/(stem+'-check.exe')
if '--prepare-only' in sys.argv:
    print(json.dumps({'driver':str(driver),'fixture':stem+'-native-cases.json','compiled':False}))
    raise SystemExit(0)
subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustc.exe','--edition','2024','-O',str(driver),'-L','dependency='+str(deps),'--extern','country_core='+str(core),'--extern','serde_json='+str(serde),'-o',str(exe)],check=True)
result=subprocess.run([str(exe)],text=True,capture_output=True,check=True)
(here/(stem+'-check.log')).write_text(result.stdout)
(here/(stem+'-check-build.json')).write_text(json.dumps({'core_rlib':str(core),'core_rlib_sha256':hashlib.sha256(core.read_bytes()).hexdigest(),'current_sparse_source_sha256':hashlib.sha256((root/'tiny-glade/crates/country-core/src/render/sparse_instance_buffer.rs').read_bytes()).hexdigest(),'command_scope':'standalone rustc against existing central Cargo artifacts; no Cargo launched; GPU absent'},indent=2))
print(result.stdout,end='')
