# gen_stair_floors build Err 日志勘误（覆盖旧合同静默错误结论）

绑定 FLOOR_MESH_FULL_CONTRACT.md SHA cf7f03cd34d3ae0fbde4ea7760c5b345726098b060c994bf413725e3dfd26c69。Tree 独立读取原9864+1454全部2253指令后发现原C遗漏的log副作用；旧“build Err 静默、不log”无效。

原1407dd949搬Err→dd980取全局14362c3a8间接level filter→dd98a test非零才进入日志（Error level1）；dd99a设置1407cef30 Display formatter；dd9c5的142a88570/51是target `system_decorator::decorator_visual::stairs_assemble`，不能误当message。142a88548是format piece数组，首项ptr142a88528、len27，实际 `Error spawning stair mesh: `，后接Err的Display。dda68调用140d4a330，PDB825:1760/48B `log::__private_api::impl$2::log`。然后才进入Err析构和局部Vec清理；日志不把Err改panic，不spawn实体，不改变已有entities prefix。

现代同源使用现log crate canonical `log::error!(target: "system_decorator::decorator_visual::stairs_assemble", "Error spawning stair mesh: {}", error)`，保Error全局level过滤与Display；不造自有logger/cache/去重，不使用Debug。原allocation错误panic沿原上游，不变为此Err分支。

同时明确raise callback按原每分量 `point + (Vec3::Y * height)`，X/Z包含+0乘height；旧合同的Vec3文字表示方向乘法而非直接literal忽略0*height。法线normal是先raw cross normalize_or_zero后取负，失败负零保留。其余strip/Nani/Arc合同待Tree完整独审结论，不由此勘误宣称source完工。
