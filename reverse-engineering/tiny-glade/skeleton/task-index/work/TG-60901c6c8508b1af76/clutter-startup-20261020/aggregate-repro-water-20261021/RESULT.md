# 18源聚合机械可重现检查

本报告仅机械重放；我写过其中schema，不作为独立业务审查。

- manifest实际18源，19份ordered patch。所有production当前before hash与manifest吻合；原字节复制到/private/tmp/tg-canonical-repro-water-20261021-dvsvxsxh/production-before。
- 按每份patch真实header选-p0/-p1，以/usr/bin/patch -N -f -F0在/private/tmp/tg-canonical-repro-water-20261021-dvsvxsxh/isolated隔离副本按序应用，逐返回码、最终SHA和candidate完整bytes核对。
- 聚合结果：18/18通过。详细returncode/路径/每步after/patchhash见REPRODUCIBILITY-REPORT.json。
- 单独核已审helper720bdfb6：通过；它尚未收入aggregate，但cottage parent要求配对。
- metadata ready_sources=17，实际files=18，计数过期。
- 未Cargo、未format、未生产写；检查前后18源生产bytes相同。
