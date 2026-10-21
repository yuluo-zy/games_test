# 合同实现补充 V1

append 同时校验 prepared_queue 与新增 prepared_family，二者只在完整 prepare/admit 成功后设值；cancel、publish、opaque/discard/preview 的原 cancel 路径同步撤销，host prepare 从 cancel 起步，因此不能借用 host 或其他 family 前缀。append 一开始撤销两项资格，失败无可发布前缀。

原 patch loop 在新私有 helper 中逐 UTF-8 字节保留，见 PATCH_LOOP_BYTE_IDENTITY.txt；原 per-patch error/依赖合并/差分切片/固定 before 计数不变。唯一算法变化是同一次 submit 的前缀保留、只处理新尾部。

共7项新增纯 CPU tests：6项 range_state（含96连续私有重叠读取前缀）、1项真实 PreparedResourceStates guard 多cell取消与重试发布。所有失败 case 明确要求发生失败，错误类别和完整文字逐边界与全前缀 prepare oracle 一致；每边界比完整 PreparedCell flags、staged dependencies、committed、容量、预算 current/peak。预算覆盖 scratch split、table max_cpu_bytes、cells.grow 边界与实际 allocator host budget。

本 agent 仅以 rustfmt 解析候选与静态检查，未 Cargo/GPU。root 需要执行既有 resource::range_state::tests 与 raw::command_buffer::native_state_submission_tests、构建、原世界同probe真实M3单帧/像素与timer on/off，随后原30帧断言原样；不得将本CPU改动标记成内存/描述符长稳通过。
