#[path="../../../reconstruction/rhapsody-rebuild/src/shader_pipeline/shader_defines.rs"]
mod original_shader_defines;
fn h(p:&[(&str,&str)])->u64{original_shader_defines::ShaderDefines(p.iter().map(|(n,v)|(n.to_string(),v.to_string())).collect()).artifact_hash()}
fn main(){for name in ["PERM_quality","PERM_oc_pass","PERM_photomode","PERM_write"]{for value in ["0","1"]{println!("{}={} {:x}",name,value,h(&[(name,value)]));}} for wave in ["0","1"]{for sharpen in ["0","1"]{println!("TGSR {},{} {:x}",wave,sharpen,h(&[("PERM_wave32",wave),("PERM_sharpen",sharpen)]));}} for ty in ["float","float4"]{for fmt in ["rgba32f","r16"]{println!("Gauss {},{} {:x}",fmt,ty,h(&[("PERM_fmt",&format!("\"{}\"",fmt)),("PERM_ty",ty)]));}}}
