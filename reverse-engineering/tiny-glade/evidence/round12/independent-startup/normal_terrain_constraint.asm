140949dc0: 56 push rsi
140949dc1: 4883ec60 sub rsp, 0x60
140949dc5: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140949dcb: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
140949dd1: 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140949dd6: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
140949ddb: 4889ce mov rsi, rcx
140949dde: f30f104140 movss xmm0, dword ptr [rcx + 0x40]
140949de3: f30f10514c movss xmm2, dword ptr [rcx + 0x4c]
140949de8: f30f58d0 addss xmm2, xmm0
140949dec: 488b4a20 mov rcx, qword ptr [rdx + 0x20]
140949df0: 0f28fa movaps xmm7, xmm2
140949df3: f30f5cf8 subss xmm7, xmm0
140949df7: f2440f104638 movsd xmm8, qword ptr [rsi + 0x38]
140949dfd: f30f107644 movss xmm6, dword ptr [rsi + 0x44]
140949e02: f3410f58f0 addss xmm6, xmm8
140949e07: f3450f16c8 movshdup xmm9, xmm8
140949e0c: f3440f584e48 addss xmm9, dword ptr [rsi + 0x48]
140949e12: 0f28ce movaps xmm1, xmm6
140949e15: e826f80900 call 0x1409e9640
140949e1a: f30f584668 addss xmm0, dword ptr [rsi + 0x68]
140949e1f: 410f28c9 movaps xmm1, xmm9
140949e23: f3410fc2c903 cmpunordss xmm1, xmm9
140949e29: 0f28d1 movaps xmm2, xmm1
140949e2c: 0f54d0 andps xmm2, xmm0
140949e2f: f3410f5fc1 maxss xmm0, xmm9
140949e34: 0f55c8 andnps xmm1, xmm0
140949e37: 0f56ca orps xmm1, xmm2
140949e3a: 0f14f1 unpcklps xmm6, xmm1
140949e3d: 410f5cf0 subps xmm6, xmm8
140949e41: 0f137644 movlps qword ptr [rsi + 0x44], xmm6
140949e45: f30f117e4c movss dword ptr [rsi + 0x4c], xmm7
140949e4a: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
140949e4f: 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140949e54: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140949e5a: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140949e60: 4883c460 add rsp, 0x60
140949e64: 5e pop rsi
140949e65: c3 ret 
