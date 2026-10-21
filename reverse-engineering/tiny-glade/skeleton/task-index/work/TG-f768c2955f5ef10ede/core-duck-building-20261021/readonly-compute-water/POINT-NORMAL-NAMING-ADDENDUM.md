# StairPoint方向字段命名勘误（2026-10-21）

此勘误只取代已冻结COMPUTE-CONTRACT.md第13行“wall-tangent Option / wall tangent”的名称，不改其字段有序值/其他compute数学；原正文和freeze-v2.json保持不变。

同owner duck复核并经本只读worker逐指令核原helpers-native-pack/node-fold-full.asm.txt：14048d0ab调用get_tangent，原返回XMM0=tx、XMM1=ty。side0在14048d0b7翻转XMM0符号，side1在14048d0ee翻转XMM1符号。14048d135把XMM1写入Point+14，14048d13b把XMM0写入Point+18。故Point的Option<Vec2>载荷实际为wall normal：side0 `(ty,-tx)`，side1 `(-ty,tx)`，不是未旋转的wall tangent。nonwall inactive值不能补造业务默认。

本compute合同只从StairPoint.position.y和width取端点高度/宽度，原点位方向payload无变化；另一个曲线tangentVec及其采样/首尾覆盖保持原义，不因本勘误改名为normal。该勘误无生产写入、无Cargo或运行验收。
