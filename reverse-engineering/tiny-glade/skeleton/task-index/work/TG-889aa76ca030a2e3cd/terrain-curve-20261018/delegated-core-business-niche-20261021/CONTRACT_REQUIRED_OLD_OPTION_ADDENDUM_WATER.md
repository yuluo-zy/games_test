# 原 old Option 必要补充（water 独审，lead 已接受）

本补充不修改原 CONTRACT.md；与它一同约束候选。全部原证复用既有五原完整包，无新增采集。

- merge第三循环：`14155b678` 调HashMap insert，`14155b67d`比较真实返回区RBP+b0+47=f7；旧None进入`14155b6a8`的d2s insert，旧Some经`14155b520/b52e`的d2s update。必须保留此前d2s.remove成功守卫（b618/b61d）、removed.remove(b651)、modified.insert(b65d)、map.insert(b678)顺序。当前无条件update通常因前面已remove而同值，但本次恢复该明确分支，不依赖隐含条件删除原门。
- split目标子墙桶：`140a2013e`实际call `14155afe0`，输入R9=RBP+30，返回区RCX=RBP+1a0，d2s操作参数位于caller stack20。故child bucket.insert也保存旧Option，None d2s.insert，Some d2s.update；不按新info.state，不强写或mask原flags。侧分派/几何公式不在本次补充范围。
- tracked慢路径新项是另一条原inline：`14125720c`先map.insert，`141257226` removed.remove，`141257232` modified.insert。这个次序与通用Decorators::insert不同，现生产对应顺序正确；不要为复用新通用insert把它重排。原Drain与尾部d2s insert保持合同。

原范围：`TG609/clutter-startup-20261020/storage-niche-audit-20261021/native-pack/merge-full.asm.txt`、`wall-split-full.asm.txt`、`tracked-clone-d2s-full.asm.txt`。source必须同时满足本补充与原合同；此文是具体原证审查要求，不是新增生产writer或实现。
