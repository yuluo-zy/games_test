141736760: push rbp
141736761: push rsi
141736762: push rdi
141736763: push rbx
141736764: sub rsp, 0x5f8
14173676b: lea rbp, [rsp + 0x80]
141736773: mov qword ptr [rbp + 0x570], 0xfffffffffffffffe
14173677e: mov rsi, rcx
141736781: movzx eax, byte ptr [rip + 0x1efe481]
141736788: mov ecx, 0x4d0
14173678d: mov edx, 8
141736792: call 0x140613c10
141736797: test rax, rax
14173679a: je 0x141736ddb
1417367a0: mov qword ptr [rbp + 0x568], rax
1417367a7: xorps xmm0, xmm0
1417367aa: movups xmmword ptr [rbp + 0x448], xmm0
1417367b1: movups xmmword ptr [rbp + 0x460], xmm0
1417367b8: movups xmmword ptr [rbp + 0x478], xmm0
1417367bf: movups xmmword ptr [rbp + 0x490], xmm0
1417367c6: movups xmmword ptr [rbp + 0x4a8], xmm0
1417367cd: mov dword ptr [rbp + 0x4b8], 0
1417367d7: movups xmmword ptr [rbp + 0x4e0], xmm0
1417367de: movups xmmword ptr [rbp + 0x4f8], xmm0
1417367e5: movups xmmword ptr [rbp + 0x510], xmm0
1417367ec: movups xmmword ptr [rbp + 0x528], xmm0
1417367f3: movups xmmword ptr [rbp + 0x540], xmm0
1417367fa: mov dword ptr [rbp + 0x550], 0
141736804: mov qword ptr [rbp + 0x400], 0
14173680f: mov qword ptr [rbp + 0x428], 0
14173681a: mov qword ptr [rbp + 0x430], 8
141736825: mov qword ptr [rbp + 0x438], 0
141736830: mov qword ptr [rbp + 0x440], 0x10
14173683b: mov qword ptr [rbp + 0x458], 0x10
141736846: mov qword ptr [rbp + 0x470], 0x10
141736851: mov qword ptr [rbp + 0x488], 0x10
14173685c: mov qword ptr [rbp + 0x4a0], 0x10
141736867: movabs rbx, 0x8000000000000000
141736871: mov qword ptr [rbp + 0x4c0], rbx
141736878: lea rax, [rip + 0x1596cdc]
14173687f: mov qword ptr [rbp + 0x4c8], rax
141736886: mov qword ptr [rbp + 0x4d0], 0x39
141736891: mov qword ptr [rbp + 0x4d8], 0x10
14173689c: mov qword ptr [rbp + 0x4f0], 0x10
1417368a7: mov qword ptr [rbp + 0x508], 0x10
1417368b2: mov qword ptr [rbp + 0x520], 0x10
1417368bd: mov qword ptr [rbp + 0x538], 0x10
1417368c8: mov dword ptr [rbp + 0x558], 0
1417368d2: mov word ptr [rbp + 0x55c], 1
1417368db: mov dword ptr [rbp + 0x560], 0
1417368e5: movzx eax, byte ptr [rip + 0x1efe31d]
1417368ec: mov ecx, 0x168
1417368f1: mov edx, 8
1417368f6: call 0x140613c10
1417368fb: test rax, rax
1417368fe: je 0x141736dea
141736904: mov rdi, rax
141736907: lea rdx, [rbp + 0x400]
14173690e: mov r8d, 0x168
141736914: mov rcx, rax
141736917: call 0x142923c00
14173691c: lea r8, [rip + 0x1596965]
141736923: lea rcx, [rbp + 0x390]
14173692a: mov rdx, rdi
14173692d: call 0x140ce4b10
141736932: lea rcx, [rbp + 0x320]
141736939: call 0x1417cce60
14173693e: lea rcx, [rbp + 0x2b0]
141736945: call 0x1417407f0
14173694a: lea rcx, [rbp + 0x240]
141736951: call 0x1408c6af0
141736956: lea rcx, [rbp + 0x1d0]
14173695d: call 0x140a91810
141736962: lea rcx, [rbp + 0x160]
141736969: call 0x140935640
14173696e: lea rcx, [rbp + 0xf0]
141736975: call 0x140a11fe0
14173697a: lea rcx, [rbp + 0x80]
141736981: call 0x140ac9150
141736986: lea rcx, [rbp + 0x10]
14173698a: call 0x1417e2c20
14173698f: lea rcx, [rbp - 0x60]
141736993: call 0x1409853c0
141736998: lea rcx, [rbp + 0x400]
14173699f: call 0x14092af20
1417369a4: movups xmm0, xmmword ptr [rbp + 0x3f0]
1417369ab: mov rax, qword ptr [rbp + 0x568]
1417369b2: movups xmmword ptr [rax + 0x60], xmm0
1417369b6: movups xmm0, xmmword ptr [rbp + 0x3e0]
1417369bd: movups xmmword ptr [rax + 0x50], xmm0
1417369c1: movups xmm0, xmmword ptr [rbp + 0x3d0]
1417369c8: movups xmmword ptr [rax + 0x40], xmm0
1417369cc: movups xmm0, xmmword ptr [rbp + 0x390]
1417369d3: movups xmm1, xmmword ptr [rbp + 0x3a0]
1417369da: movups xmm2, xmmword ptr [rbp + 0x3b0]
1417369e1: movups xmm3, xmmword ptr [rbp + 0x3c0]
1417369e8: movups xmmword ptr [rax + 0x30], xmm3
1417369ec: movups xmmword ptr [rax + 0x20], xmm2
1417369f0: movups xmmword ptr [rax + 0x10], xmm1
1417369f4: movups xmmword ptr [rax], xmm0
1417369f7: movups xmm0, xmmword ptr [rbp + 0x320]
1417369fe: movups xmm1, xmmword ptr [rbp + 0x330]
141736a05: movups xmm2, xmmword ptr [rbp + 0x340]
141736a0c: movups xmm3, xmmword ptr [rbp + 0x350]
141736a13: movups xmmword ptr [rax + 0x70], xmm0
141736a17: movups xmmword ptr [rax + 0x80], xmm1
141736a1e: movups xmmword ptr [rax + 0x90], xmm2
141736a25: movups xmmword ptr [rax + 0xa0], xmm3
141736a2c: movups xmm0, xmmword ptr [rbp + 0x360]
141736a33: movups xmmword ptr [rax + 0xb0], xmm0
141736a3a: movups xmm0, xmmword ptr [rbp + 0x370]
141736a41: movups xmmword ptr [rax + 0xc0], xmm0
141736a48: movups xmm0, xmmword ptr [rbp + 0x380]
141736a4f: movups xmmword ptr [rax + 0xd0], xmm0
141736a56: movups xmm0, xmmword ptr [rbp + 0x310]
141736a5d: movups xmmword ptr [rax + 0x140], xmm0
141736a64: movups xmm0, xmmword ptr [rbp + 0x300]
141736a6b: movups xmmword ptr [rax + 0x130], xmm0
141736a72: movups xmm0, xmmword ptr [rbp + 0x2f0]
141736a79: movups xmmword ptr [rax + 0x120], xmm0
141736a80: movups xmm0, xmmword ptr [rbp + 0x2b0]
141736a87: movups xmm1, xmmword ptr [rbp + 0x2c0]
141736a8e: movups xmm2, xmmword ptr [rbp + 0x2d0]
141736a95: movups xmm3, xmmword ptr [rbp + 0x2e0]
141736a9c: movups xmmword ptr [rax + 0x110], xmm3
141736aa3: movups xmmword ptr [rax + 0x100], xmm2
141736aaa: movups xmmword ptr [rax + 0xf0], xmm1
141736ab1: movups xmmword ptr [rax + 0xe0], xmm0
141736ab8: movups xmm0, xmmword ptr [rbp + 0x240]
141736abf: movups xmm1, xmmword ptr [rbp + 0x250]
141736ac6: movups xmm2, xmmword ptr [rbp + 0x260]
141736acd: movups xmm3, xmmword ptr [rbp + 0x270]
141736ad4: movups xmmword ptr [rax + 0x150], xmm0
141736adb: movups xmmword ptr [rax + 0x160], xmm1
141736ae2: movups xmmword ptr [rax + 0x170], xmm2
141736ae9: movups xmmword ptr [rax + 0x180], xmm3
141736af0: movups xmm0, xmmword ptr [rbp + 0x280]
141736af7: movups xmmword ptr [rax + 0x190], xmm0
141736afe: movups xmm0, xmmword ptr [rbp + 0x290]
141736b05: movups xmmword ptr [rax + 0x1a0], xmm0
141736b0c: movups xmm0, xmmword ptr [rbp + 0x2a0]
141736b13: movups xmmword ptr [rax + 0x1b0], xmm0
141736b1a: movups xmm0, xmmword ptr [rbp + 0x1d0]
141736b21: movups xmm1, xmmword ptr [rbp + 0x1e0]
141736b28: movups xmm2, xmmword ptr [rbp + 0x1f0]
141736b2f: movups xmm3, xmmword ptr [rbp + 0x200]
141736b36: movups xmmword ptr [rax + 0x1c0], xmm0
141736b3d: movups xmmword ptr [rax + 0x1d0], xmm1
141736b44: movups xmmword ptr [rax + 0x1e0], xmm2
141736b4b: movups xmmword ptr [rax + 0x1f0], xmm3
141736b52: movups xmm0, xmmword ptr [rbp + 0x210]
141736b59: movups xmmword ptr [rax + 0x200], xmm0
141736b60: movups xmm0, xmmword ptr [rbp + 0x230]
141736b67: movups xmmword ptr [rax + 0x220], xmm0
141736b6e: movups xmm0, xmmword ptr [rbp + 0x220]
141736b75: movups xmmword ptr [rax + 0x210], xmm0
141736b7c: movups xmm0, xmmword ptr [rbp + 0x1c0]
141736b83: movups xmmword ptr [rax + 0x290], xmm0
141736b8a: movups xmm0, xmmword ptr [rbp + 0x1b0]
141736b91: movups xmmword ptr [rax + 0x280], xmm0
141736b98: movups xmm0, xmmword ptr [rbp + 0x1a0]
141736b9f: movups xmmword ptr [rax + 0x270], xmm0
141736ba6: movups xmm0, xmmword ptr [rbp + 0x160]
141736bad: movups xmm1, xmmword ptr [rbp + 0x170]
141736bb4: movups xmm2, xmmword ptr [rbp + 0x180]
141736bbb: movups xmm3, xmmword ptr [rbp + 0x190]
141736bc2: movups xmmword ptr [rax + 0x260], xmm3
141736bc9: movups xmmword ptr [rax + 0x250], xmm2
141736bd0: movups xmmword ptr [rax + 0x240], xmm1
141736bd7: movups xmmword ptr [rax + 0x230], xmm0
141736bde: movups xmm0, xmmword ptr [rbp + 0xf0]
141736be5: movups xmm1, xmmword ptr [rbp + 0x100]
141736bec: movups xmm2, xmmword ptr [rbp + 0x110]
141736bf3: movups xmm3, xmmword ptr [rbp + 0x120]
141736bfa: movups xmmword ptr [rax + 0x2a0], xmm0
141736c01: movups xmmword ptr [rax + 0x2b0], xmm1
141736c08: movups xmmword ptr [rax + 0x2c0], xmm2
141736c0f: movups xmmword ptr [rax + 0x2d0], xmm3
141736c16: movups xmm0, xmmword ptr [rbp + 0x130]
141736c1d: movups xmmword ptr [rax + 0x2e0], xmm0
141736c24: movups xmm0, xmmword ptr [rbp + 0x140]
141736c2b: movups xmmword ptr [rax + 0x2f0], xmm0
141736c32: movups xmm0, xmmword ptr [rbp + 0x150]
141736c39: movups xmmword ptr [rax + 0x300], xmm0
141736c40: movups xmm0, xmmword ptr [rbp + 0x80]
141736c47: movups xmm1, xmmword ptr [rbp + 0x90]
141736c4e: movups xmm2, xmmword ptr [rbp + 0xa0]
141736c55: movups xmm3, xmmword ptr [rbp + 0xb0]
141736c5c: movups xmmword ptr [rax + 0x310], xmm0
141736c63: movups xmmword ptr [rax + 0x320], xmm1
141736c6a: movups xmmword ptr [rax + 0x330], xmm2
141736c71: movups xmmword ptr [rax + 0x340], xmm3
141736c78: movups xmm0, xmmword ptr [rbp + 0xc0]
141736c7f: movups xmmword ptr [rax + 0x350], xmm0
141736c86: movups xmm0, xmmword ptr [rbp + 0xd0]
141736c8d: movups xmmword ptr [rax + 0x360], xmm0
141736c94: movups xmm0, xmmword ptr [rbp + 0xe0]
141736c9b: movups xmmword ptr [rax + 0x370], xmm0
141736ca2: movups xmm0, xmmword ptr [rbp + 0x10]
141736ca6: movups xmm1, xmmword ptr [rbp + 0x20]
141736caa: movups xmm2, xmmword ptr [rbp + 0x30]
141736cae: movups xmm3, xmmword ptr [rbp + 0x40]
141736cb2: movups xmmword ptr [rax + 0x380], xmm0
141736cb9: movups xmmword ptr [rax + 0x390], xmm1
141736cc0: movups xmmword ptr [rax + 0x3a0], xmm2
141736cc7: movups xmmword ptr [rax + 0x3b0], xmm3
141736cce: movups xmm0, xmmword ptr [rbp + 0x50]
141736cd2: movups xmmword ptr [rax + 0x3c0], xmm0
141736cd9: movups xmm0, xmmword ptr [rbp + 0x60]
141736cdd: movups xmmword ptr [rax + 0x3d0], xmm0
141736ce4: movups xmm0, xmmword ptr [rbp + 0x70]
141736ce8: movups xmmword ptr [rax + 0x3e0], xmm0
141736cef: movups xmm0, xmmword ptr [rbp - 0x60]
141736cf3: movups xmm1, xmmword ptr [rbp - 0x50]
141736cf7: movups xmm2, xmmword ptr [rbp - 0x40]
141736cfb: movups xmm3, xmmword ptr [rbp - 0x30]
141736cff: movups xmmword ptr [rax + 0x3f0], xmm0
141736d06: movups xmmword ptr [rax + 0x400], xmm1
141736d0d: movups xmmword ptr [rax + 0x410], xmm2
141736d14: movups xmmword ptr [rax + 0x420], xmm3
141736d1b: movups xmm0, xmmword ptr [rbp - 0x20]
141736d1f: movups xmmword ptr [rax + 0x430], xmm0
141736d26: movups xmm0, xmmword ptr [rbp - 0x10]
141736d2a: movups xmmword ptr [rax + 0x440], xmm0
141736d31: movups xmm0, xmmword ptr [rbp]
141736d35: movups xmmword ptr [rax + 0x450], xmm0
141736d3c: movups xmm0, xmmword ptr [rbp + 0x400]
141736d43: movups xmm1, xmmword ptr [rbp + 0x410]
141736d4a: movups xmm2, xmmword ptr [rbp + 0x420]
141736d51: movups xmm3, xmmword ptr [rbp + 0x430]
141736d58: movups xmmword ptr [rax + 0x460], xmm0
141736d5f: movups xmmword ptr [rax + 0x470], xmm1
141736d66: movups xmmword ptr [rax + 0x480], xmm2
141736d6d: movups xmmword ptr [rax + 0x490], xmm3
141736d74: movups xmm0, xmmword ptr [rbp + 0x440]
141736d7b: movups xmmword ptr [rax + 0x4a0], xmm0
141736d82: movups xmm0, xmmword ptr [rbp + 0x450]
141736d89: movups xmmword ptr [rax + 0x4b0], xmm0
141736d90: movups xmm0, xmmword ptr [rbp + 0x460]
141736d97: movups xmmword ptr [rax + 0x4c0], xmm0
141736d9e: mov qword ptr [rsi + 8], 0xb
141736da6: mov qword ptr [rsi + 0x10], rax
141736daa: mov qword ptr [rsi + 0x18], 0xb
141736db2: mov qword ptr [rsi + 0x20], 0
141736dba: mov qword ptr [rsi + 0x28], 8
141736dc2: xorps xmm0, xmm0
141736dc5: movups xmmword ptr [rsi + 0x30], xmm0
141736dc9: mov qword ptr [rsi], rbx
141736dcc: mov rax, rsi
141736dcf: add rsp, 0x5f8
141736dd6: pop rbx
141736dd7: pop rdi
141736dd8: pop rsi
141736dd9: pop rbp
141736dda: ret 
141736ddb: mov ecx, 8
141736de0: mov edx, 0x4d0
141736de5: call 0x1428d9000
141736dea: mov ecx, 8
141736def: mov edx, 0x168
141736df4: call 0x1428d9000
141736df9: ud2 
141736dfb: nop dword ptr [rax + rax]
141736e00: mov qword ptr [rsp + 0x10], rdx
141736e05: push rbp
141736e06: push rsi
141736e07: push rdi
141736e08: push rbx
141736e09: sub rsp, 0x28
141736e0d: lea rbp, [rdx + 0x80]
141736e14: lea rcx, [rbp + 0x400]
141736e1b: call 0x1405e6f10
141736e20: nop 
141736e21: add rsp, 0x28
141736e25: pop rbx
141736e26: pop rdi
141736e27: pop rsi
141736e28: pop rbp
141736e29: ret 
141736e2a: nop word ptr [rax + rax]
141736e30: mov qword ptr [rsp + 0x10], rdx
141736e35: push rbp
141736e36: push rsi
141736e37: push rdi
141736e38: push rbx
141736e39: sub rsp, 0x28
141736e3d: lea rbp, [rdx + 0x80]
141736e44: mov edx, 0x4d0
141736e49: mov r8d, 8
141736e4f: mov rcx, qword ptr [rbp + 0x568]
141736e56: call 0x140613c20
141736e5b: nop 
141736e5c: add rsp, 0x28
141736e60: pop rbx
141736e61: pop rdi
141736e62: pop rsi
141736e63: pop rbp
141736e64: ret 
141736e65: nop word ptr cs:[rax + rax]
141736e70: mov qword ptr [rsp + 0x10], rdx
141736e75: push rbp
141736e76: push rsi
141736e77: push rdi
141736e78: push rbx
141736e79: sub rsp, 0x28
141736e7d: lea rbp, [rdx + 0x80]
141736e84: lea rcx, [rbp + 0x390]
141736e8b: call 0x140029880
141736e90: nop 
141736e91: add rsp, 0x28
141736e95: pop rbx
141736e96: pop rdi
141736e97: pop rsi
141736e98: pop rbp
141736e99: ret 
141736e9a: nop word ptr [rax + rax]
141736ea0: mov qword ptr [rsp + 0x10], rdx
141736ea5: push rbp
141736ea6: push rsi
141736ea7: push rdi
141736ea8: push rbx
141736ea9: sub rsp, 0x28
141736ead: lea rbp, [rdx + 0x80]
141736eb4: lea rcx, [rbp + 0x320]
141736ebb: call 0x140029880
141736ec0: nop 
141736ec1: add rsp, 0x28
141736ec5: pop rbx
141736ec6: pop rdi
141736ec7: pop rsi
141736ec8: pop rbp
141736ec9: ret 
141736eca: nop word ptr [rax + rax]
141736ed0: mov qword ptr [rsp + 0x10], rdx
141736ed5: push rbp
141736ed6: push rsi
141736ed7: push rdi
141736ed8: push rbx
141736ed9: sub rsp, 0x28
141736edd: lea rbp, [rdx + 0x80]
141736ee4: lea rcx, [rbp + 0x2b0]
141736eeb: call 0x140029880
141736ef0: nop 
141736ef1: add rsp, 0x28
141736ef5: pop rbx
141736ef6: pop rdi
141736ef7: pop rsi
141736ef8: pop rbp
141736ef9: ret 
141736efa: nop word ptr [rax + rax]
141736f00: mov qword ptr [rsp + 0x10], rdx
141736f05: push rbp
141736f06: push rsi
141736f07: push rdi
141736f08: push rbx
141736f09: sub rsp, 0x28
141736f0d: lea rbp, [rdx + 0x80]
141736f14: lea rcx, [rbp + 0x240]
141736f1b: call 0x140029880
141736f20: nop 
141736f21: add rsp, 0x28
141736f25: pop rbx
141736f26: pop rdi
141736f27: pop rsi
141736f28: pop rbp
141736f29: ret 
141736f2a: nop word ptr [rax + rax]
141736f30: mov qword ptr [rsp + 0x10], rdx
141736f35: push rbp
141736f36: push rsi
141736f37: push rdi
141736f38: push rbx
141736f39: sub rsp, 0x28
141736f3d: lea rbp, [rdx + 0x80]
141736f44: lea rcx, [rbp + 0x1d0]
141736f4b: call 0x140029880
141736f50: nop 
141736f51: add rsp, 0x28
141736f55: pop rbx
141736f56: pop rdi
141736f57: pop rsi
141736f58: pop rbp
141736f59: ret 
141736f5a: nop word ptr [rax + rax]
141736f60: mov qword ptr [rsp + 0x10], rdx
141736f65: push rbp
141736f66: push rsi
141736f67: push rdi
141736f68: push rbx
141736f69: sub rsp, 0x28
141736f6d: lea rbp, [rdx + 0x80]
141736f74: lea rcx, [rbp + 0x160]
141736f7b: call 0x140029880
141736f80: nop 
141736f81: add rsp, 0x28
141736f85: pop rbx
141736f86: pop rdi
141736f87: pop rsi
141736f88: pop rbp
141736f89: ret 
141736f8a: nop word ptr [rax + rax]
141736f90: mov qword ptr [rsp + 0x10], rdx
141736f95: push rbp
141736f96: push rsi
141736f97: push rdi
141736f98: push rbx
141736f99: sub rsp, 0x28
141736f9d: lea rbp, [rdx + 0x80]
141736fa4: lea rcx, [rbp + 0xf0]
141736fab: call 0x140029880
141736fb0: nop 
141736fb1: add rsp, 0x28
141736fb5: pop rbx
141736fb6: pop rdi
141736fb7: pop rsi
141736fb8: pop rbp
141736fb9: ret 
141736fba: nop word ptr [rax + rax]
141736fc0: mov qword ptr [rsp + 0x10], rdx
141736fc5: push rbp
141736fc6: push rsi
141736fc7: push rdi
141736fc8: push rbx
141736fc9: sub rsp, 0x28
141736fcd: lea rbp, [rdx + 0x80]
141736fd4: lea rcx, [rbp + 0x80]
141736fdb: call 0x140029880
141736fe0: nop 
141736fe1: add rsp, 0x28
141736fe5: pop rbx
141736fe6: pop rdi
141736fe7: pop rsi
141736fe8: pop rbp
141736fe9: ret 
141736fea: nop word ptr [rax + rax]
141736ff0: mov qword ptr [rsp + 0x10], rdx
141736ff5: push rbp
141736ff6: push rsi
141736ff7: push rdi
141736ff8: push rbx
141736ff9: sub rsp, 0x28
141736ffd: lea rbp, [rdx + 0x80]
141737004: lea rcx, [rbp + 0x10]
141737008: call 0x140029880
14173700d: nop 
14173700e: add rsp, 0x28
141737012: pop rbx
141737013: pop rdi
141737014: pop rsi
141737015: pop rbp
141737016: ret 
141737017: nop word ptr [rax + rax]
141737020: mov qword ptr [rsp + 0x10], rdx
141737025: push rbp
141737026: push rsi
141737027: push rdi
141737028: push rbx
141737029: sub rsp, 0x28
14173702d: lea rbp, [rdx + 0x80]
141737034: lea rcx, [rbp - 0x60]
141737038: call 0x140029880
14173703d: nop 
14173703e: add rsp, 0x28
141737042: pop rbx
141737043: pop rdi
141737044: pop rsi
141737045: pop rbp
141737046: ret 
