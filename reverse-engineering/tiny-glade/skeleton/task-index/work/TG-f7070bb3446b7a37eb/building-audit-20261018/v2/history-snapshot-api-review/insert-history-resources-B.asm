140943e10: 56 push rsi
140943e11: 57 push rdi
140943e12: 4881ecc8000000 sub rsp, 0xc8
140943e19: 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
140943e21: 48c7842488000000ffffffff mov qword ptr [rsp + 0x88], 0xffffffffffffffff
140943e2d: 48c744242000000000 mov qword ptr [rsp + 0x20], 0
140943e36: 48c744242808000000 mov qword ptr [rsp + 0x28], 8
140943e3f: 0f57f6 xorps xmm6, xmm6
140943e42: 0f11742430 movups xmmword ptr [rsp + 0x30], xmm6
140943e47: 48c744244008000000 mov qword ptr [rsp + 0x40], 8
140943e50: 48c744244800000000 mov qword ptr [rsp + 0x48], 0
140943e59: 48b80000000000000080 movabs rax, 0x8000000000000000
140943e63: 4889442450 mov qword ptr [rsp + 0x50], rax
140943e68: c784249000000000000000 mov dword ptr [rsp + 0x90], 0
140943e73: 0f11742478 movups xmmword ptr [rsp + 0x78], xmm6
140943e78: 488d542420 lea rdx, [rsp + 0x20]
140943e7d: e83ed7be00 call 0x1415315c0
140943e82: 0f11742430 movups xmmword ptr [rsp + 0x30], xmm6
140943e87: c74424480000803f mov dword ptr [rsp + 0x48], 0x3f800000
140943e8f: 48c744242000000000 mov qword ptr [rsp + 0x20], 0
140943e98: 48c744242808000000 mov qword ptr [rsp + 0x28], 8
140943ea1: 48c744244000000000 mov qword ptr [rsp + 0x40], 0
140943eaa: 488d542420 lea rdx, [rsp + 0x20]
140943eaf: 4889c1 mov rcx, rax
140943eb2: e8b9f6bf00 call 0x141543570
140943eb7: 4889c1 mov rcx, rax
140943eba: 31d2 xor edx, edx
140943ebc: e86fa8bf00 call 0x14153e730
140943ec1: c744242000000000 mov dword ptr [rsp + 0x20], 0
140943ec9: 488d542420 lea rdx, [rsp + 0x20]
140943ece: 4889c1 mov rcx, rax
140943ed1: e84aa5be00 call 0x14152e420
140943ed6: 4889c6 mov rsi, rax
140943ed9: 488d7c2420 lea rdi, [rsp + 0x20]
140943ede: 4889f9 mov rcx, rdi
140943ee1: e8faa7feff call 0x14092e6e0
140943ee6: 4889f1 mov rcx, rsi
140943ee9: 4889fa mov rdx, rdi
140943eec: e89f3ac000 call 0x141547990
140943ef1: 0f11742430 movups xmmword ptr [rsp + 0x30], xmm6
140943ef6: 48c744242000000000 mov qword ptr [rsp + 0x20], 0
140943eff: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
140943f08: 488d542420 lea rdx, [rsp + 0x20]
140943f0d: 4889c1 mov rcx, rax
140943f10: e81b02c000 call 0x141544130
140943f15: 0f57d2 xorps xmm2, xmm2
140943f18: 4889c1 mov rcx, rax
140943f1b: 31d2 xor edx, edx
140943f1d: 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
140943f25: 4881c4c8000000 add rsp, 0xc8
140943f2c: 5f pop rdi
140943f2d: 5e pop rsi
140943f2e: e97dacc000 jmp 0x14154ebb0
