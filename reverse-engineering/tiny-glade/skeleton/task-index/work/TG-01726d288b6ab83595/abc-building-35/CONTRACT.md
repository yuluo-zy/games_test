# 有界候选合同

目标C；A/B参照。原body同码仅支持本批观察范围。完整作者ABI、枚举和容器布局未知；语义Rust接口不宣称原内存表示。

独立恢复前合同见 /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/abc-building-35-2026-10-12/contract-review.md；本批过滤/寄存器补证见support-pack-A/B/C。

依赖：std/Vec与glam使用现成API；已冻结上批类型通过assembly借用，禁止另写公共库内部。未接生产Cargo，锁与feature适配由后续整合负责。

## 每函数观察与边界

### utils::geometry::aabb::Aabb2::width

- observed / ABC：原指令见contracts-observations.json键`utils::geometry::aabb::Aabb2::width`及mapping中native_instances的evidence；具体VA见本文件源码注释。
- Rust：/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs:7。输入/输出、数值顺序、失败边界以该函数中文注释与独立合同为范围。
- inferred：字段语义名、借用视图/泛型接口是候选API，不是完整原类型的ABI证明。
- unknown：原始完整签名、合法输入producer、跨版本消费者接入；PDB调用者列表在mapping的indexed_callers/physical_reference_callers_rva，未恢复这些caller不能声称业务闭环。
- 副作用：仅保留该叶体的返回/字段赋值；没有原证据的通知、分配、清理或历史推进不自行增加。父owner的增改删、传播、回放另待接入。

### utils::geometry::aabb::Aabb2::height

- observed / ABC：原指令见contracts-observations.json键`utils::geometry::aabb::Aabb2::height`及mapping中native_instances的evidence；具体VA见本文件源码注释。
- Rust：/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs:14。输入/输出、数值顺序、失败边界以该函数中文注释与独立合同为范围。
- inferred：字段语义名、借用视图/泛型接口是候选API，不是完整原类型的ABI证明。
- unknown：原始完整签名、合法输入producer、跨版本消费者接入；PDB调用者列表在mapping的indexed_callers/physical_reference_callers_rva，未恢复这些caller不能声称业务闭环。
- 副作用：仅保留该叶体的返回/字段赋值；没有原证据的通知、分配、清理或历史推进不自行增加。父owner的增改删、传播、回放另待接入。

### utils::geometry::aabb::Aabb2::contains

- observed / ABC：原指令见contracts-observations.json键`utils::geometry::aabb::Aabb2::contains`及mapping中native_instances的evidence；具体VA见本文件源码注释。
- Rust：/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs:21。输入/输出、数值顺序、失败边界以该函数中文注释与独立合同为范围。
- inferred：字段语义名、借用视图/泛型接口是候选API，不是完整原类型的ABI证明。
- unknown：原始完整签名、合法输入producer、跨版本消费者接入；PDB调用者列表在mapping的indexed_callers/physical_reference_callers_rva，未恢复这些caller不能声称业务闭环。
- 副作用：仅保留该叶体的返回/字段赋值；没有原证据的通知、分配、清理或历史推进不自行增加。父owner的增改删、传播、回放另待接入。

