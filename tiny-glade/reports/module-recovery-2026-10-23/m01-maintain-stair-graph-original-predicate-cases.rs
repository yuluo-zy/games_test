// 原RNE有界分支及原is_decorator_cull_allowed；输入/范围见同名JSON。
pub const ORIGINAL_STAIR_CULL_PREDICATES: &[(u32,bool,bool)] = &[
    (0,false,false),
    (0,true,false),
    (1,false,false),
    (1,true,true),
    (2,false,false),
    (2,true,false),
    (3,false,false),
    (3,true,false),
    (7,false,false),
    (7,true,false),
    (8,false,false),
    (8,true,false),
    (1024,false,false),
    (1024,true,false),
];
