from pathlib import Path
s=Path('/Users/yuluo/Desktop/game/tiny-glade/crates/country-core/src/resources/walls/decorator_archivist.rs').read_text()
f=s[s.index('fn history_states_equal('):s.index('\nimpl<C:',s.index('fn history_states_equal('))]
s2=Path('/Users/yuluo/Desktop/game/tiny-glade/crates/country-core/src/resources/history/decorator_edit.rs').read_text();a=s2.index('pub struct DecoratorHistoryState {');b=s2.index('\n}\n',a)+2
st=s2[a:b]
main='''
use std::io::{self,BufRead};
fn decode(s:&str)->DecoratorHistoryState {
 let bytes:Vec<u8>=(0..s.len()).step_by(2).map(|i|u8::from_str_radix(&s[i..i+2],16).unwrap()).collect();
 let u64_at=|i|u64::from_le_bytes(bytes[i..i+8].try_into().unwrap());
 let u32_at=|i|u32::from_le_bytes(bytes[i..i+4].try_into().unwrap());
 DecoratorHistoryState {dst_wall:u64_at(0),dst_secondary:u64_at(8),dst_payload:u64_at(16),u18:u64_at(24),u20:u32_at(32),u24:u32_at(36),u28:u64_at(40),u30:u32_at(48),u34:u32_at(52),u38:u32_at(56),u3c:u32_at(60),u40:u32_at(64),u44:u32_at(68),rank:bytes[72],u49:bytes[73],ty:bytes[74],u4b:bytes[75],u4c:bytes[76],u4d:bytes[77],style:bytes[78]}
}
fn main(){for l in io::stdin().lock().lines(){let l=l.unwrap();let parts:Vec<_>=l.split_whitespace().collect();println!("{}",u8::from(history_states_equal(&decode(parts[0]),&decode(parts[1]))));}}
'''
Path('/tmp/newdaily-archive-eq-candidate.rs').write_text(st+'\n'+f+main)
