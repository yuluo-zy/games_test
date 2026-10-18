1409438a0: push rbp
1409438a1: push r14
1409438a3: push rsi
1409438a4: push rdi
1409438a5: push rbx
1409438a6: sub rsp, 0x420
1409438ad: lea rbp, [rsp + 0x80]
1409438b5: mov qword ptr [rbp + 0x398], 0xfffffffffffffffe
1409438c0: mov rsi, rcx
1409438c3: lea r8, [rip + 0x216f076]
1409438ca: lea rcx, [rbp + 0x10]
1409438ce: mov dl, 0xe
1409438d0: call 0x141738630
1409438d5: lea rcx, [rbp + 0xf0]
1409438dc: call 0x141737fb0
1409438e1: mov byte ptr [rbp + 0x2b0], 0x23
1409438e8: mov rcx, qword ptr [rip + 0x2ce8a81]
1409438ef: lea r8, [rip + 0x231c46a]
1409438f6: lea rdx, [rbp + 0x2b0]
1409438fd: call 0x140b218a0
140943902: mov r8, rdx
140943905: lea rcx, [rbp + 0xf0]
14094390c: mov rdx, rax
14094390f: call 0x141453000
140943914: movups xmm0, xmmword ptr [rbp + 0x150]
14094391b: movaps xmmword ptr [rbp + 0x380], xmm0
140943922: movups xmm0, xmmword ptr [rbp + 0x140]
140943929: movaps xmmword ptr [rbp + 0x370], xmm0
140943930: movups xmm0, xmmword ptr [rbp + 0x130]
140943937: movaps xmmword ptr [rbp + 0x360], xmm0
14094393e: movups xmm0, xmmword ptr [rbp + 0xf0]
140943945: movups xmm1, xmmword ptr [rbp + 0x100]
14094394c: movups xmm2, xmmword ptr [rbp + 0x110]
140943953: movups xmm3, xmmword ptr [rbp + 0x120]
14094395a: movaps xmmword ptr [rbp + 0x350], xmm3
140943961: movaps xmmword ptr [rbp + 0x340], xmm2
140943968: movaps xmmword ptr [rbp + 0x330], xmm1
14094396f: movaps xmmword ptr [rbp + 0x320], xmm0
140943976: lea r8, [rip + 0x216efdb]
14094397d: lea rcx, [rbp - 0x60]
140943981: mov dl, 0x28
140943983: call 0x140ae3740
140943988: lea rcx, [rbp + 0xf0]
14094398f: call 0x14098ee10
140943994: movups xmm0, xmmword ptr [rbp + 0x150]
14094399b: movaps xmmword ptr [rbp + 0x310], xmm0
1409439a2: movups xmm0, xmmword ptr [rbp + 0x140]
1409439a9: movaps xmmword ptr [rbp + 0x300], xmm0
1409439b0: movups xmm0, xmmword ptr [rbp + 0x130]
1409439b7: movaps xmmword ptr [rbp + 0x2f0], xmm0
1409439be: movups xmm0, xmmword ptr [rbp + 0xf0]
1409439c5: movups xmm1, xmmword ptr [rbp + 0x100]
1409439cc: movups xmm2, xmmword ptr [rbp + 0x110]
1409439d3: movups xmm3, xmmword ptr [rbp + 0x120]
1409439da: movaps xmmword ptr [rbp + 0x2e0], xmm3
1409439e1: movaps xmmword ptr [rbp + 0x2d0], xmm2
1409439e8: movaps xmmword ptr [rbp + 0x2c0], xmm1
1409439ef: movaps xmmword ptr [rbp + 0x2b0], xmm0
1409439f6: lea rcx, [rbp + 0x80]
1409439fd: lea rdx, [rbp + 0x2b0]
140943a04: call 0x141737ae0
140943a09: mov byte ptr [rbp + 0x2b0], 0x29
140943a10: mov rcx, qword ptr [rip + 0x2ce8959]
140943a17: lea r8, [rip + 0x231c342]
140943a1e: lea rdi, [rbp + 0x2b0]
140943a25: mov rdx, rdi
140943a28: call 0x140b218a0
140943a2d: mov r8, rdx
140943a30: lea rbx, [rbp + 0x80]
140943a37: mov rcx, rbx
140943a3a: mov rdx, rax
140943a3d: call 0x141453000
140943a42: movups xmm0, xmmword ptr [rbp + 0xe0]
140943a49: movaps xmmword ptr [rbp + 0x2a0], xmm0
140943a50: movups xmm0, xmmword ptr [rbp + 0xd0]
140943a57: movaps xmmword ptr [rbp + 0x290], xmm0
140943a5e: movups xmm0, xmmword ptr [rbp + 0xc0]
140943a65: movaps xmmword ptr [rbp + 0x280], xmm0
140943a6c: movups xmm0, xmmword ptr [rbp + 0x80]
140943a73: movups xmm1, xmmword ptr [rbp + 0x90]
140943a7a: movups xmm2, xmmword ptr [rbp + 0xa0]
140943a81: movups xmm3, xmmword ptr [rbp + 0xb0]
140943a88: movaps xmmword ptr [rbp + 0x270], xmm3
140943a8f: movaps xmmword ptr [rbp + 0x260], xmm2
140943a96: movaps xmmword ptr [rbp + 0x250], xmm1
140943a9d: movaps xmmword ptr [rbp + 0x240], xmm0
140943aa4: movups xmm0, xmmword ptr [rbp + 0x10]
140943aa8: movups xmm1, xmmword ptr [rbp + 0x20]
140943aac: movups xmm2, xmmword ptr [rbp + 0x30]
140943ab0: movups xmm3, xmmword ptr [rbp + 0x40]
140943ab4: movaps xmmword ptr [rbp + 0xf0], xmm0
140943abb: movaps xmmword ptr [rbp + 0x100], xmm1
140943ac2: movaps xmmword ptr [rbp + 0x110], xmm2
140943ac9: movaps xmmword ptr [rbp + 0x120], xmm3
140943ad0: movups xmm0, xmmword ptr [rbp + 0x50]
140943ad4: movaps xmmword ptr [rbp + 0x130], xmm0
140943adb: movups xmm0, xmmword ptr [rbp + 0x60]
140943adf: movaps xmmword ptr [rbp + 0x140], xmm0
140943ae6: movups xmm0, xmmword ptr [rbp + 0x70]
140943aea: movaps xmmword ptr [rbp + 0x150], xmm0
140943af1: movaps xmm0, xmmword ptr [rbp + 0x380]
140943af8: movaps xmmword ptr [rbp + 0x1c0], xmm0
140943aff: movaps xmm0, xmmword ptr [rbp + 0x370]
140943b06: movaps xmmword ptr [rbp + 0x1b0], xmm0
140943b0d: movaps xmm0, xmmword ptr [rbp + 0x360]
140943b14: movaps xmmword ptr [rbp + 0x1a0], xmm0
140943b1b: movaps xmm0, xmmword ptr [rbp + 0x320]
140943b22: movaps xmm1, xmmword ptr [rbp + 0x330]
140943b29: movaps xmm2, xmmword ptr [rbp + 0x340]
140943b30: movaps xmm3, xmmword ptr [rbp + 0x350]
140943b37: movaps xmmword ptr [rbp + 0x190], xmm3
140943b3e: movaps xmmword ptr [rbp + 0x180], xmm2
140943b45: movaps xmmword ptr [rbp + 0x170], xmm1
140943b4c: movaps xmmword ptr [rbp + 0x160], xmm0
140943b53: movups xmm0, xmmword ptr [rbp]
140943b57: movaps xmmword ptr [rbp + 0x230], xmm0
140943b5e: movups xmm0, xmmword ptr [rbp - 0x10]
140943b62: movaps xmmword ptr [rbp + 0x220], xmm0
140943b69: movups xmm0, xmmword ptr [rbp - 0x20]
140943b6d: movaps xmmword ptr [rbp + 0x210], xmm0
140943b74: movups xmm0, xmmword ptr [rbp - 0x60]
140943b78: movups xmm1, xmmword ptr [rbp - 0x50]
140943b7c: movups xmm2, xmmword ptr [rbp - 0x40]
140943b80: movups xmm3, xmmword ptr [rbp - 0x30]
140943b84: movaps xmmword ptr [rbp + 0x200], xmm3
140943b8b: movaps xmmword ptr [rbp + 0x1f0], xmm2
140943b92: movaps xmmword ptr [rbp + 0x1e0], xmm1
140943b99: movaps xmmword ptr [rbp + 0x1d0], xmm0
