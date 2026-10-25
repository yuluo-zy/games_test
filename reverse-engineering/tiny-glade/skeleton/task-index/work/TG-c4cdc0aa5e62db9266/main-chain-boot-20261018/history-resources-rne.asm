0x140943e10: push rsi
0x140943e11: push rdi
0x140943e12: sub rsp, 0xc8
0x140943e19: movaps xmmword ptr [rsp + 0xb0], xmm6
0x140943e21: mov qword ptr [rsp + 0x88], 0xffffffffffffffff
0x140943e2d: mov qword ptr [rsp + 0x20], 0
0x140943e36: mov qword ptr [rsp + 0x28], 8
0x140943e3f: xorps xmm6, xmm6
0x140943e42: movups xmmword ptr [rsp + 0x30], xmm6
0x140943e47: mov qword ptr [rsp + 0x40], 8
0x140943e50: mov qword ptr [rsp + 0x48], 0
0x140943e59: movabs rax, 0x8000000000000000
0x140943e63: mov qword ptr [rsp + 0x50], rax
0x140943e68: mov dword ptr [rsp + 0x90], 0
0x140943e73: movups xmmword ptr [rsp + 0x78], xmm6
0x140943e78: lea rdx, [rsp + 0x20]
0x140943e7d: call 0x1415315c0
0x140943e82: movups xmmword ptr [rsp + 0x30], xmm6
0x140943e87: mov dword ptr [rsp + 0x48], 0x3f800000
0x140943e8f: mov qword ptr [rsp + 0x20], 0
0x140943e98: mov qword ptr [rsp + 0x28], 8
0x140943ea1: mov qword ptr [rsp + 0x40], 0
0x140943eaa: lea rdx, [rsp + 0x20]
0x140943eaf: mov rcx, rax
0x140943eb2: call 0x141543570
0x140943eb7: mov rcx, rax
0x140943eba: xor edx, edx
0x140943ebc: call 0x14153e730
0x140943ec1: mov dword ptr [rsp + 0x20], 0
0x140943ec9: lea rdx, [rsp + 0x20]
0x140943ece: mov rcx, rax
0x140943ed1: call 0x14152e420
0x140943ed6: mov rsi, rax
0x140943ed9: lea rdi, [rsp + 0x20]
0x140943ede: mov rcx, rdi
0x140943ee1: call 0x14092e6e0
0x140943ee6: mov rcx, rsi
0x140943ee9: mov rdx, rdi
0x140943eec: call 0x141547990
0x140943ef1: movups xmmword ptr [rsp + 0x30], xmm6
0x140943ef6: mov qword ptr [rsp + 0x20], 0
0x140943eff: mov qword ptr [rsp + 0x28], 1
0x140943f08: lea rdx, [rsp + 0x20]
0x140943f0d: mov rcx, rax
0x140943f10: call 0x141544130
0x140943f15: xorps xmm2, xmm2
0x140943f18: mov rcx, rax
0x140943f1b: xor edx, edx
0x140943f1d: movaps xmm6, xmmword ptr [rsp + 0xb0]
0x140943f25: add rsp, 0xc8
0x140943f2c: pop rdi
0x140943f2d: pop rsi
0x140943f2e: jmp 0x14154ebb0
