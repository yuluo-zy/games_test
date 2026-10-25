# Dispatcher 必要类型投影 V1

原 TG2a claim 保持 clutter-startup-20261020；root延续授权 TG69 必要callee，不接管其它UI族，不改生产/Cargo/GPU。仅恢复 validate_blueprints 原140827285..2d6内联 decorator_ty。

原完整539B入口与六项有界表已由 TG69/window-validation-root-contract-v1/NATIVE_BYTE_PROOF_V1.json 绑定发行 B。原128位header精确 clear(6,0)选择 None；其它判别选择活动union类型投影。table142a91c00六项分别取+69,+5c,+60,+5c,+5c,None。不能对现Rust重构struct执行裸offset读取，ty字段也不能伪称统一原+5c。

现保用户语义结构：唯一 None 是完整(state6,payload0)；真实owned Move(state2,payload0)/Rotate(state3,payload0)直接读request.ty，避免过期投影；其它变体用已有显式ty安全投影。字段注释明确每个其它原variant writer须写入真实当前选中type，这不是默认type回退，也不宣称未恢复place/height producer完整。现Default零tag属于原其它variant并非None；不给它无证清空。clear仍按原16B写state6/payload0并清owned载荷。

两个未跑源码测试：exact clear门/非零第二字判别及其它header保活动type；真实Move/Rotate typed setter的owned字段优先、clear后无stale图标。无新的活动type算法，不改 dispatch 主体、注册和其它函数；原无关source字节保持。

限制：这是当前重构架构的必要semantic accessor，不是完整原Dispatcher union恢复声明。验证consumer可以在具备实际selectedtype的caller下可调用，但完整UI/place host仍不mount。
