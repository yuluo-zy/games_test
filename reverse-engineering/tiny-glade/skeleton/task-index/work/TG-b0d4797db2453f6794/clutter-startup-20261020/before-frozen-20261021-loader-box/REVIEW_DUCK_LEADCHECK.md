# Duck reviewer 独立复核：canonical box入口

2026-10-21；core_business_lead 授权。live show仍旧owner clutter-startup-20261020，无claim变动。只新增本review，未改candidate/production，未运行Cargo/rustc/GPU/原程序。

**结论：冻结候选452094d4d7ccf7bf5a0969d3ed5a73fe13eaebaeb97251b58850fd73668451cd在原B dimensions→真实12 triangles范围内静态通过；无阻止整合的具体反例。** before801d2e6c3120022044edd33fa5341675ca1aba08e10ff09a15affdeb4c3468ce。原完整1302B outer、690B map、558B collect identity-bound pack均ok，不外推C/原执行。候选变化使本记录失效。

- 真实caller1407bb602的XMM1/2/3为width/height/depth，stack20为Transform，候选修正原公开inner错误签名；原3处测试调用仅迁到private mapper，没有删除历史域。loader使用此canonical入口，未复制box表或增建新生产文件。
- 独立对照原outer140ada14b..2bc的实际stack stores与已有24点lift，六面vertex顺序吻合；140ada2c5..457的36usize literal完整吻合。候选保留24个面局部顶点而非8角点重排，负半通过乘-.5，保负/零/非有限输入，不插finite/abs/normalize/default保护。
- 原map14147854c..563、5a3..5ba保(yY+xX)→zZ+previous→previous+translation；第三点1414785fa..611最后为translation+previous。候选明确同加法分组并保第三点顺序，不用Mat4::transform_point3、mul_add、f64或投影divide。
- 原bounds检查vertex24及chunk[1]/[2]失败保持。canonical外层固定36/3/24，内部异常不被捕获成空Vec。复用原权威Transform.compute_matrix，不造raw-layout镜像。glam依赖及assert feature边界仍依旧合同，未据source复用声称原fork数值机器码等价。
- 新测试覆盖真实12个三角/非均匀SRT、反射/零/非有限传播、短chunk失败；未运行。NaN传播检查不等于跨平台NaN payload逐位保证，当前Rust/glam代码接受范围仍为合同明示边界。

配对loader SHA7eacff3ac5c9839abaa99017bb504eee6011be3e0accb406477633ea19b302d8；schema/outline及root唯一PreStartup注册由其owner整合。本helper本身不完成装饰业务全部producer/consumer。
