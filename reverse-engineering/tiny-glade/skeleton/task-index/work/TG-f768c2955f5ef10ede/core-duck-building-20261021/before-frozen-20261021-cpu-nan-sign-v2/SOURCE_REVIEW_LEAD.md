# NaN原同ISA精确断言增量独审

通过v2；v1无条件mask撤回不可应用。实际当前767→535716d3，只测试中x86_64保原两条完整ffc00000断言，非x86只放开原算术NaN signbit，仍精确lower31=7fc00000，所有rawmetadata/阈值/业务算序与其它断言不变。独立临时basename cwd patch -p1 -F0成功，最终hash精确。未Cargo运行，root实际重跑；原同ISA差分范围不被放宽。
