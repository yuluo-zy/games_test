//! shader-pipeline 自有的 Speedy 类型 schema；数值尺寸由原 spirq 库计算。
//! 资源类型在原存储 schema 中是无载荷标签，不臆造 SPIR-V Image 字段。
use speedy::{Readable, Writable};
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub enum ScalarType {
    Void,
    Boolean,
    Signed(u32),
    Unsigned(u32),
    Float(u32),
}
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub struct VectorType {
    pub scalar_ty: ScalarType,
    pub nscalar: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub enum MatrixAxisOrder {
    ColumnMajor,
    RowMajor,
}
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub struct MatrixType {
    pub vec_ty: VectorType,
    pub nvec: u32,
    pub major: Option<MatrixAxisOrder>,
    pub stride: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub struct ArrayType {
    pub proto_ty: Box<Type>,
    pub nrepeat: Option<u32>,
    pub stride: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub struct StructMember {
    pub name: Option<String>,
    pub offset: u64,
    pub ty: Type,
}
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub struct StructType {
    pub name: Option<String>,
    pub members: Vec<StructMember>,
}
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub enum Type {
    Scalar(ScalarType),
    Vector(VectorType),
    Matrix(MatrixType),
    Image,
    CombinedImageSampler,
    SampledImage,
    StorageImage,
    Sampler,
    SubpassData,
    Array(ArrayType),
    Struct(StructType),
    AccelStruct,
    DeviceAddress,
    DevicePointer(Box<Type>),
}
impl ScalarType {
    pub fn to_spirq(&self) -> spirq::ty::ScalarType {
        use spirq::ty::ScalarType as S;
        match *self {
            Self::Void => S::Void,
            Self::Boolean => S::Boolean,
            Self::Signed(n) => S::Signed(n),
            Self::Unsigned(n) => S::Unsigned(n),
            Self::Float(n) => S::Float(n),
        }
    }
}
impl VectorType {
    fn to_spirq(&self) -> spirq::ty::VectorType {
        spirq::ty::VectorType {
            scalar_ty: self.scalar_ty.to_spirq(),
            nscalar: self.nscalar,
        }
    }
}
impl Type {
    /// 只转换存储 schema 保留的类型；资源标签缺少载荷，返回 None。
    pub fn to_spirq(&self) -> Option<spirq::ty::Type> {
        self.convert(false)
    }
    fn convert(&self, for_size: bool) -> Option<spirq::ty::Type> {
        use spirq::ty as s;
        Some(match self {
            Self::Scalar(x) => s::Type::Scalar(x.to_spirq()),
            Self::Vector(x) => s::Type::Vector(x.to_spirq()),
            Self::Matrix(x) => s::Type::Matrix(s::MatrixType {
                vec_ty: x.vec_ty.to_spirq(),
                nvec: x.nvec,
                major: x.major.as_ref().map(|x| match x {
                    MatrixAxisOrder::ColumnMajor => s::MatrixAxisOrder::ColumnMajor,
                    MatrixAxisOrder::RowMajor => s::MatrixAxisOrder::RowMajor,
                }),
                stride: x.stride.map(|x| x as usize),
            }),
            Self::Array(x) => s::Type::Array(s::ArrayType {
                proto_ty: Box::new(x.proto_ty.convert(for_size)?),
                nrepeat: x.nrepeat,
                stride: x.stride.map(|x| x as usize),
            }),
            Self::Struct(x) => s::Type::Struct(s::StructType {
                name: x.name.clone(),
                members: x
                    .members
                    .iter()
                    .map(|m| {
                        Some(s::StructMember {
                            name: m.name.clone(),
                            offset: m.offset as usize,
                            ty: m.ty.convert(for_size)?,
                        })
                    })
                    .collect::<Option<Vec<_>>>()?,
            }),
            Self::DeviceAddress => s::Type::DeviceAddress(),
            Self::DevicePointer(x) => s::Type::DevicePointer(s::PointerType {
                pointee_ty: Box::new(x.convert(for_size)?),
            }),
            // 尺寸查询的opaque资源统一用公开库的无尺寸Sampler载体；不对外声称该资源就是Sampler。
            _ if for_size => s::Type::Sampler(),
            _ => return None,
        })
    }
    pub fn nbyte(&self) -> Option<usize> {
        self.convert(true)?.nbyte()
    }
}
