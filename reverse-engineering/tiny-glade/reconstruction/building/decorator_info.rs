//! UNVERIFIED B DecoratorInfo72 business-field decoding and placement source.
//! Static original accessors/serializers/stores only; no new native proof yet.
//! A uses destination tag+21; C uses+32 and payload+10. This is B-only layout.
use crate::{building_wall_state::WallId,building_decorator_subtype::WallWindowKind};
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct NativeDecoratorId(pub u32);
#[derive(Clone,Debug)]
pub struct DecoratorInfoObserved([u8;72]);
#[derive(Clone,Copy,Debug)]
pub struct WallAttachmentAnchorObserved {
    /// Serialized "platform", tested by original WallId::is_valid. This is
    /// not the enclosing wall storage address and not is_bottom_door.
    pub platform:WallId,
    pub wall_height_relative_y:f32,
    pub anchor_ws:[f32;3],
    pub corner_tag:u8,
    pub is_bottom_door:bool,
    /// Original WallAttachmentSide values0/1 also encode destination's niche.
    pub side_tag:u8,
}
#[derive(Clone,Copy,Debug)]
pub struct WallAttachmentObserved{pub coordinate:[f32;2],pub anchor:WallAttachmentAnchorObserved}
#[derive(Clone,Copy,Debug)]
pub struct WallOpeningPolicy {
    pub kind:WallWindowKind,pub rank:u8,pub coordinate:[f32;2],
    pub platform_is_valid:bool,pub corner_attached:bool,
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum DecoratorInfoError{NonWallDestination(u8),UnsupportedDecoratorType(u8),InvalidOpeningRank(u8),ReadonlyStorage,UnsupportedStorageAddress(u64)}
impl DecoratorInfoObserved{
    pub fn from_observed(bytes:[u8;72])->Self{Self(bytes)}
    pub fn bytes(&self)->&[u8;72]{&self.0}
    pub fn destination_tag(&self)->u8{self.0[0x22]}
    pub fn rank(&self)->u8{self.0[0x44]}
    pub fn decorator_type(&self)->u8{self.0[0x46]}
    pub fn decorator_id(&self)->NativeDecoratorId{NativeDecoratorId(u32::from_le_bytes(self.0[0x2c..0x30].try_into().unwrap()))}
    pub fn set_wall_coordinate(&mut self,coordinate:[f32;2])->Result<(),DecoratorInfoError>{
        self.dst_as_wall()?;
        self.0[..4].copy_from_slice(&coordinate[0].to_le_bytes());
        self.0[4..8].copy_from_slice(&coordinate[1].to_le_bytes());
        Ok(())
    }
    /// Original try_dst_as_wall is exactly destination-side/niche <2.
    pub fn try_dst_as_wall(&self)->Option<WallAttachmentObserved>{
        (self.destination_tag()<2).then(||self.wall_fields())
    }
    pub fn dst_as_wall(&self)->Result<WallAttachmentObserved,DecoratorInfoError>{self.try_dst_as_wall().ok_or(DecoratorInfoError::NonWallDestination(self.destination_tag()))}
    fn wall_fields(&self)->WallAttachmentObserved{
        let f=|at|f32::from_le_bytes(self.0[at..at+4].try_into().unwrap());
        WallAttachmentObserved{coordinate:[f(0),f(4)],anchor:WallAttachmentAnchorObserved{
            platform:WallId(u64::from_le_bytes(self.0[8..16].try_into().unwrap())),
            wall_height_relative_y:f(0x10),anchor_ws:[f(0x14),f(0x18),f(0x1c)],
            corner_tag:self.0[0x20],is_bottom_door:self.0[0x21]!=0,side_tag:self.0[0x22],
        }}
    }
    /// Real dtype and platform/corner predicates replace family/door intent.
    /// Arrow-slits and the other destination/render owners remain explicit gaps.
    pub fn wall_opening_policy(&self)->Result<WallOpeningPolicy,DecoratorInfoError>{
        let attachment=self.dst_as_wall()?;
        let kind=match self.decorator_type(){0x38=>WallWindowKind::Cottage,0x39=>WallWindowKind::Gothic,t=>return Err(DecoratorInfoError::UnsupportedDecoratorType(t))};
        let rank=self.rank();if !(1..=3).contains(&rank){return Err(DecoratorInfoError::InvalidOpeningRank(rank));}
        Ok(WallOpeningPolicy{kind,rank,coordinate:attachment.coordinate,platform_is_valid:attachment.anchor.platform.0!=0,corner_attached:attachment.anchor.corner_tag!=0})
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum DecoratorAnchorError{
    NonWallDestination(u8),MissingHeightCache,InvalidRawCurve,InvalidProjection,
    InvalidMappedCurve,InvalidArchHeights,InvalidWallSpaceLength,BottomDoorConsumerRequired,
}
/// The two lengths are deliberately separate: B Public.raw Curve2+30 feeds
/// projection, while Public.published source+28c is wall-space's period/end.
pub struct DecoratorAnchorContext<'a>{
    pub raw_curve:&'a crate::surface::Curve2,
    pub mapped_outline:&'a [[f32;3]],pub arch_heights:&'a [f32],
    pub public:&'a crate::building_wall_state::PublicWallBuildingState,
}
fn anchor_max(a:f32,b:f32)->f32{if a>b{a}else{b}}
fn anchor_min(a:f32,b:f32)->f32{if a<b{a}else{b}}
/// Exact ordinary branch of PublicWall.get_wall_attachment_height140b2a890.
/// Free-wall source multiplies the OLD coordinate by raw length before the
/// normalized Curve2 clamp; the unusual multiplication is retained.
pub fn wall_attachment_height(context:&DecoratorAnchorContext<'_>,old_x:f32)->Result<f32,DecoratorAnchorError>{
    let max_y=context.public.max_y().map_err(|_|DecoratorAnchorError::MissingHeightCache)?;
    if context.public.shape().tag()<2{return Ok(max_y);}
    let raw=context.raw_curve;
    if raw.points.len()<2||raw.points.len()!=raw.points_u.len(){return Err(DecoratorAnchorError::InvalidRawCurve);}
    let scaled=old_x*context.public.source_curve_length();
    if !scaled.is_finite(){return Err(DecoratorAnchorError::InvalidProjection);}
    let u=anchor_min(1.,anchor_max(0.,scaled));
    // Public's raw Curve2 coord_at_u owns its clamping/right-biased search.
    let(i,t)=raw.coord_at_u(u);let a=raw.points[i];let b=raw.points[i+1];
    let position=[a[0]*(1.-t)+b[0]*t,0.,a[1]*(1.-t)+b[1]*t];
    let(i,t)=crate::building_decorator_subtype::nearest_segment(context.mapped_outline,position)
        .map_err(|_|DecoratorAnchorError::InvalidMappedCurve)?;
    let a=*context.arch_heights.get(i).ok_or(DecoratorAnchorError::InvalidArchHeights)?;
    let b=*context.arch_heights.get(i+1).ok_or(DecoratorAnchorError::InvalidArchHeights)?;
    Ok(a+(b-a)*t)
}
/// B utils::wall_space::project_clamp140c91eb0. Closed-space uses fmodf,
/// preserving negative-zero remainder; free space clamps to published length.
pub fn project_wall_space_clamp(shape_tag:u32,published_length:f32,value:f32)->Result<f32,DecoratorAnchorError>{
    if shape_tag!=2{
        let remainder=value%published_length;
        Ok(if remainder<0.{published_length+remainder}else{remainder})
    }else{
        if !(published_length>=0.){return Err(DecoratorAnchorError::InvalidWallSpaceLength);}
        Ok(anchor_min(published_length,anchor_max(0.,value)))
    }
}
/// Ordinary game stores14081d202..14081d2e5. Bottom-door consumers require
/// the actual vertical profile and selected collision/subtype owner; return a
/// typed gap before commit instead of publishing an incomplete door position.
pub fn move_wall_decorator_following_anchor(info:&DecoratorInfoObserved,context:&DecoratorAnchorContext<'_>)
    ->Result<DecoratorInfoObserved,DecoratorAnchorError>{
    let attachment=info.try_dst_as_wall().ok_or(DecoratorAnchorError::NonWallDestination(info.destination_tag()))?;
    if attachment.anchor.is_bottom_door{return Err(DecoratorAnchorError::BottomDoorConsumerRequired);}
    let height=wall_attachment_height(context,attachment.coordinate[0])?;
    let raw=context.raw_curve;
    let(i,t)=crate::building_wall_pending_changed::approximate_curve_coordinate(raw,[attachment.anchor.anchor_ws[0],attachment.anchor.anchor_ws[2]])
        .map_err(|_|DecoratorAnchorError::InvalidProjection)?;
    let a=raw.points_u[i];let b=raw.points_u[i+1];
    // Original utils::lerp140c904d0 is subtract/multiply/add, not mix.
    let u=a+(b-a)*t;
    let x=project_wall_space_clamp(context.public.shape().tag(),context.public.published_source_curve_length(),u*context.public.source_curve_length())?;
    let y=height+attachment.anchor.wall_height_relative_y;
    let mut result=info.clone();result.set_wall_coordinate([x,y]).map_err(|_|DecoratorAnchorError::NonWallDestination(info.destination_tag()))?;
    Ok(result)
}
#[derive(Clone,Debug)]
pub struct DecoratorPlaceRequestObserved(pub [u8;88]);
#[derive(Clone,Copy,Debug)]
pub enum DecoratorPlaceStorageAddress{Wall(WallId),ObservedSpecial1,ObservedSpecial2}
#[derive(Clone,Debug)]
pub struct AppliedDecoratorSource{pub address:DecoratorPlaceStorageAddress,pub decorator_id:NativeDecoratorId,pub info:DecoratorInfoObserved}
/// Actual apply_decorator_place140a20c70 source stores. The original query/
/// container insertion and ownership remain the storage consumer. DEV global
/// is explicit, and the caller+R8 byte is preserved without guessing its name.
pub fn decorator_source_from_place(
    request:&DecoratorPlaceRequestObserved,dev_override_is_demo_mode:bool,
    observed_caller_byte:u8,storage_readonly:bool,
)->Result<AppliedDecoratorSource,DecoratorInfoError>{
    if storage_readonly{return Err(DecoratorInfoError::ReadonlyStorage);}
    let r=&request.0;let tag=u64::from_le_bytes(r[..8].try_into().unwrap());
    let address=match tag{0=>DecoratorPlaceStorageAddress::Wall(WallId(u64::from_le_bytes(r[8..16].try_into().unwrap()))),1=>DecoratorPlaceStorageAddress::ObservedSpecial1,2=>DecoratorPlaceStorageAddress::ObservedSpecial2,t=>return Err(DecoratorInfoError::UnsupportedStorageAddress(t))};
    let mut info=[0;72];info[..40].copy_from_slice(&r[0x10..0x38]);
    info[0x28..0x2c].copy_from_slice(&r[0x50..0x54]);
    info[0x2c..0x30].copy_from_slice(&r[0x4c..0x50]);
    info[0x30..0x44].copy_from_slice(&r[0x38..0x4c]);
    info[0x44]=r[0x56];info[0x45]=if dev_override_is_demo_mode{0}else{5};
    info[0x46]=r[0x55];info[0x47]=observed_caller_byte;
    let info=DecoratorInfoObserved::from_observed(info);Ok(AppliedDecoratorSource{address,decorator_id:info.decorator_id(),info})
}

#[derive(Clone,Debug)]
pub struct StoredDecoratorPlace{pub decorator_id:NativeDecoratorId,pub previous:Option<DecoratorInfoObserved>}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum DecoratorPlaceInsertError{MissingLiveIndex(u32),DuplicateLiveIndex(u32),InvalidLiveNiche(u8)}
/// Actual Decorators::insert + DecoMoveToStorage::update: unmark removed,
/// mark added, replace live, then insert a new global address or update an
/// existing one. The original panic domains return errors before this typed
/// owner commits; std containers are not reimplemented.
pub fn insert_decorator_source(
    storage:&mut crate::building_wall_decorator_archivist::DecoratorStorage,
    source:&AppliedDecoratorSource,
)->Result<StoredDecoratorPlace,DecoratorPlaceInsertError>{
    use crate::building_wall_decorator_archivist::{DecoratorStorageAddress,TrackedDecorators,ObservedDecoratorInfo};
    let id=source.decorator_id.0;
    if source.info.0[0x47]==2{return Err(DecoratorPlaceInsertError::InvalidLiveNiche(2));}
    let address=match source.address{DecoratorPlaceStorageAddress::Wall(w)=>DecoratorStorageAddress::Wall(w),DecoratorPlaceStorageAddress::ObservedSpecial1=>DecoratorStorageAddress::ObservedSpecial1,DecoratorPlaceStorageAddress::ObservedSpecial2=>DecoratorStorageAddress::ObservedSpecial2};
    let previous=match address{DecoratorStorageAddress::Wall(w)=>storage.walls.get(&w),DecoratorStorageAddress::ObservedSpecial1=>storage.special1.as_ref(),DecoratorStorageAddress::ObservedSpecial2=>storage.special2.as_ref()}.and_then(|d|d.items.get(&id)).cloned();
    if previous.is_some()&&!storage.id_to_address.contains_key(&id){return Err(DecoratorPlaceInsertError::MissingLiveIndex(id));}
    if previous.is_none()&&storage.id_to_address.contains_key(&id){return Err(DecoratorPlaceInsertError::DuplicateLiveIndex(id));}
    let target=match address{
        DecoratorStorageAddress::Wall(w)=>storage.walls.entry(w).or_insert_with(||TrackedDecorators{removed:storage.removed_by_address.remove(&address).unwrap_or_default(),..TrackedDecorators::default()}),
        DecoratorStorageAddress::ObservedSpecial1=>storage.special1.get_or_insert_with(||TrackedDecorators{removed:storage.removed_by_address.remove(&address).unwrap_or_default(),..TrackedDecorators::default()}),
        DecoratorStorageAddress::ObservedSpecial2=>storage.special2.get_or_insert_with(||TrackedDecorators{removed:storage.removed_by_address.remove(&address).unwrap_or_default(),..TrackedDecorators::default()}),
    };
    target.removed.remove(&id);target.added.insert(id);target.items.insert(id,ObservedDecoratorInfo(source.info.0));
    storage.id_to_address.insert(id,address);
    Ok(StoredDecoratorPlace{decorator_id:source.decorator_id,previous:previous.map(|p|DecoratorInfoObserved::from_observed(p.0))})
}
