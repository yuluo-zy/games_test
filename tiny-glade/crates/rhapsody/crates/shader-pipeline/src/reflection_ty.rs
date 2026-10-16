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

#[cfg(test)]
mod nbyte_tests {
    //! `Type::nbyte`（原 0x1420bda00，15 项跳表 0x142fd8f84 + 标量/向量子表）的逐 case 对拍。
    //! 跳表逐项由 `.rne` 读出：0 Scalar、1 Vector、2 Matrix、3..8 opaque 资源（None）、9 Array、
    //! 10 Struct（默认）、11 AccelStruct（None）、12/13 地址（Some(8)）。下面的参考模型逐条抄机器码：
    //!
    //! * Vector（0x1420bdaeb）：`scalar_nbyte(tag, payload) * nscalar`，其中 Void→0、Boolean→1、
    //!   Signed/Unsigned/Float→payload（`[rcx+0xc]`）。
    //! * Matrix（0x1420bdaa0）：`stride 为 Some ? stride : scalar_nbyte * nscalar`，再乘 `nvec`
    //!   （`imul rdx,rax` 最后一步）。
    //! * Array（0x1420bdb09）：`stride.unwrap_or(0) * nrepeat.unwrap_or(0)`（两个 Option 标签字节
    //!   `cmp byte [rcx+8],0` / `[rcx+0x18]`，None 取 0）。
    //! * Struct（0x1420bda47）：空 → Some(0)；否则最后一个成员 `offset + nbyte(ty).unwrap_or(0)`。
    use super::*;

    fn scalar_mc(s: &ScalarType) -> usize {
        match s {
            ScalarType::Void => 0,
            ScalarType::Boolean => 1,
            ScalarType::Signed(n) | ScalarType::Unsigned(n) | ScalarType::Float(n) => *n as usize,
        }
    }

    fn mc(t: &Type) -> Option<usize> {
        match t {
            Type::Scalar(s) => Some(scalar_mc(s)),
            Type::Vector(v) => Some(scalar_mc(&v.scalar_ty) * v.nscalar as usize),
            Type::Matrix(m) => {
                let stride = m
                    .stride
                    .map(|x| x as usize)
                    .unwrap_or(scalar_mc(&m.vec_ty.scalar_ty) * m.vec_ty.nscalar as usize);
                Some(stride * m.nvec as usize)
            }
            Type::Array(a) => {
                Some(a.stride.unwrap_or(0) as usize * a.nrepeat.unwrap_or(0) as usize)
            }
            Type::Struct(s) => Some(match s.members.last() {
                None => 0,
                Some(m) => m.offset as usize + mc(&m.ty).unwrap_or(0),
            }),
            Type::DeviceAddress | Type::DevicePointer(_) => Some(8),
            _ => None,
        }
    }

    fn scalars() -> Vec<ScalarType> {
        vec![
            ScalarType::Void,
            ScalarType::Boolean,
            ScalarType::Signed(4),
            ScalarType::Unsigned(8),
            ScalarType::Float(2),
            ScalarType::Float(4),
        ]
    }

    #[test]
    fn literal_vectors() {
        let v = |s, n| {
            Type::Vector(VectorType {
                scalar_ty: s,
                nscalar: n,
            })
        };
        assert_eq!(v(ScalarType::Float(4), 3).nbyte(), Some(12));
        assert_eq!(v(ScalarType::Boolean, 4).nbyte(), Some(4));
        assert_eq!(v(ScalarType::Void, 4).nbyte(), Some(0));
        let m = |stride| {
            Type::Matrix(MatrixType {
                vec_ty: VectorType {
                    scalar_ty: ScalarType::Float(4),
                    nscalar: 4,
                },
                nvec: 4,
                major: Some(MatrixAxisOrder::ColumnMajor),
                stride,
            })
        };
        assert_eq!(m(None).nbyte(), Some(64));
        assert_eq!(m(Some(32)).nbyte(), Some(128));
        let arr = |nrepeat, stride| {
            Type::Array(ArrayType {
                proto_ty: Box::new(Type::Scalar(ScalarType::Float(4))),
                nrepeat,
                stride,
            })
        };
        assert_eq!(arr(Some(5), Some(16)).nbyte(), Some(80));
        assert_eq!(arr(None, Some(16)).nbyte(), Some(0));
        assert_eq!(arr(Some(5), None).nbyte(), Some(0));
        assert_eq!(Type::Sampler.nbyte(), None);
        assert_eq!(Type::AccelStruct.nbyte(), None);
        assert_eq!(Type::DeviceAddress.nbyte(), Some(8));
    }

    #[test]
    fn vector_matrix_array_grid_matches_machine_code_model() {
        for s in scalars() {
            for n in [1u32, 2, 3, 4] {
                let vec_ty = VectorType {
                    scalar_ty: s.clone(),
                    nscalar: n,
                };
                let t = Type::Vector(vec_ty.clone());
                assert_eq!(t.nbyte(), mc(&t), "vector {s:?}x{n}");
                for nvec in [1u32, 2, 4] {
                    for stride in [None, Some(16u64), Some(0)] {
                        let t = Type::Matrix(MatrixType {
                            vec_ty: vec_ty.clone(),
                            nvec,
                            major: None,
                            stride,
                        });
                        assert_eq!(t.nbyte(), mc(&t), "matrix {s:?}x{n}x{nvec} {stride:?}");
                    }
                }
                for nrepeat in [None, Some(0u32), Some(3)] {
                    for stride in [None, Some(0u64), Some(16)] {
                        let t = Type::Array(ArrayType {
                            proto_ty: Box::new(Type::Vector(vec_ty.clone())),
                            nrepeat,
                            stride,
                        });
                        assert_eq!(t.nbyte(), mc(&t), "array {s:?}x{n} {nrepeat:?} {stride:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn struct_last_member_offset_plus_size() {
        let member = |offset, ty| StructMember {
            name: None,
            offset,
            ty,
        };
        let st = |members| {
            Type::Struct(StructType {
                name: None,
                members,
            })
        };
        assert_eq!(st(vec![]).nbyte(), Some(0));
        let t = st(vec![
            member(0, Type::Scalar(ScalarType::Float(4))),
            member(
                16,
                Type::Vector(VectorType {
                    scalar_ty: ScalarType::Float(4),
                    nscalar: 3,
                }),
            ),
        ]);
        assert_eq!(t.nbyte(), Some(28));
        assert_eq!(t.nbyte(), mc(&t));
    }

    /// 末尾成员无尺寸（opaque）：原版 `test cl,1; cmovne` 取 0，结果仍是 `Some(offset)`。
    #[test]
    fn struct_with_unsized_last_member_is_offset_only() {
        let t = Type::Struct(StructType {
            name: None,
            members: vec![StructMember {
                name: None,
                offset: 24,
                ty: Type::Sampler,
            }],
        });
        assert_eq!(t.nbyte(), Some(24));
    }
}
