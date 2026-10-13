//! Two original sampling policies: texel-center fix and legacy corner sampling.
use glam::Vec2;
pub trait FloatGrid {
    fn dimensions(&self)->[usize;2];
    fn load_index_unchecked(&self,index:usize)->f32;
}
pub trait FloatGridSampler:FloatGrid {
    // B's max/min sequence converts NaN to zero; f32::clamp preserves NaN.
    #[allow(clippy::manual_clamp)]
    fn bilinear_sample_at_world_xz(&self,xz:Vec2,world_extent:Vec2,sampling_fix:bool)->f32 {
        let [width,height]=self.dimensions();assert!(width>=2 && height>=2);
        let normalized=xz/world_extent+Vec2::splat(0.5);
        let axis=|v:f32,n:usize|->(usize,usize,f32){
            if sampling_fix {
                let v=if v<1.0 {v}else{1.0};let texel=(v*n as f32-0.5).max(0.0);
                let floor=texel as u32;let fraction=texel-floor as f32;
                ((floor as usize).min(n-1),(floor.wrapping_add(1) as usize).min(n-1),fraction)
            }else{
                let texel=v.max(0.0).min(1.0)*n as f32;
                let floor=(texel as u32 as usize).min(n-2);(floor,floor+1,texel-floor as f32)
            }
        };
        let (x0,x1,tx)=axis(normalized.x,width);let (z0,z1,tz)=axis(normalized.y,height);
        let a=self.load_index_unchecked(z0*width+x0);let b=self.load_index_unchecked(z1*width+x0);
        let c=self.load_index_unchecked(z0*width+x1);let d=self.load_index_unchecked(z1*width+x1);
        let left=a+(b-a)*tz;let right=c+(d-c)*tz;left+(right-left)*tx
    }
}
impl<T:FloatGrid+?Sized> FloatGridSampler for T {}
/// Explicit normalized-byte storage adapter observed in B's 512x512 variants.
pub struct ByteFloatGrid<'a> {pub values:&'a [u8],pub width:usize,pub height:usize}
impl FloatGrid for ByteFloatGrid<'_> {
    fn dimensions(&self)->[usize;2] {[self.width,self.height]}
    fn load_index_unchecked(&self,index:usize)->f32 {self.values[index] as f32/255.0}
}
