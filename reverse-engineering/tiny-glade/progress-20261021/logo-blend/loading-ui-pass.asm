0x140837ff0: push rbp
0x140837ff1: push r15
0x140837ff3: push r14
0x140837ff5: push r13
0x140837ff7: push r12
0x140837ff9: push rsi
0x140837ffa: push rdi
0x140837ffb: push rbx
0x140837ffc: sub rsp, 0x938
0x140838003: lea rbp, [rsp + 0x80]
0x14083800b: mov qword ptr [rbp + 0x8b0], 0xfffffffffffffffe
0x140838016: mov rbx, r9
0x140838019: mov rdi, r8
0x14083801c: mov rsi, rdx
0x14083801f: mov r14, rcx
0x140838022: lea rcx, [r8 + 4]
0x140838026: call 0x140bd5790
0x14083802b: mov dword ptr [rbp + 0x3b8], 0
0x140838035: mov qword ptr [rbp + 0x3c0], rdi
0x14083803c: mov dword ptr [rbp + 0x7a0], 2
0x140838046: mov qword ptr [rsp + 0x40], r14
0x14083804b: mov qword ptr [rsp + 0x38], rsi
0x140838050: mov qword ptr [rsp + 0x30], rax
0x140838055: lea rax, [rbp + 0x7a0]
0x14083805c: mov qword ptr [rsp + 0x28], rax
0x140838061: mov qword ptr [rsp + 0x20], 1
0x14083806a: lea rdx, [rip + 0x225ab6f]
0x140838071: lea rcx, [rbp - 0x30]
0x140838075: lea r9, [rbp + 0x3b8]
0x14083807c: mov r8d, 2
0x140838082: call 0x140867970
0x140838087: mov qword ptr [rbp + 0x8a0], rsi
0x14083808e: lea rcx, [rsi + 0x38]
0x140838092: mov byte ptr [rbp + 0x8af], 1
0x140838099: movabs rdx, 0x4bc362195b80f729
0x1408380a3: movabs r8, 0x982798ce80cbcefb
0x1408380ad: call 0x1400dd6f0
0x1408380b2: test rax, rax
0x1408380b5: mov r8, qword ptr [rbp + 0x8a0]
0x1408380bc: je 0x14083832d
0x1408380c2: cmp qword ptr [r8 + 0x218], rdx
0x1408380c9: jbe 0x14083832d
0x1408380cf: mov rax, qword ptr [r8 + 0x210]
0x1408380d6: mov rcx, qword ptr [rax + rdx*8]
0x1408380da: test rcx, rcx
0x1408380dd: je 0x14083832d
0x1408380e3: mov rax, qword ptr [r8 + 0x1e0]
0x1408380ea: not rcx
0x1408380ed: lea rcx, [rcx + rcx*2]
0x1408380f1: shl rcx, 5
0x1408380f5: cmp qword ptr [rax + rcx + 0x20], 0
0x1408380fb: je 0x14083832d
0x140838101: add rax, rcx
0x140838104: mov rax, qword ptr [rax + 0x10]
0x140838108: mov rcx, qword ptr [rax]
0x14083810b: test rcx, rcx
0x14083810e: js 0x140838117
0x140838110: cvtsi2ss xmm0, rcx
0x140838115: jmp 0x14083812c
0x140838117: mov rdx, rcx
0x14083811a: shr rdx, 1
0x14083811d: and ecx, 1
0x140838120: or rcx, rdx
0x140838123: cvtsi2ss xmm0, rcx
0x140838128: addss xmm0, xmm0
0x14083812c: cvtsi2ss xmm1, dword ptr [rax + 8]
0x140838131: divss xmm1, dword ptr [rip + 0x216f507]
0x140838139: addss xmm1, xmm0
0x14083813d: movss dword ptr [rbp + 0x898], xmm1
0x140838145: mov byte ptr [rbp + 0x89c], 1
0x14083814c: mov rdi, qword ptr [rbx + 0x1b8]
0x140838153: mov r15, qword ptr [rbx + 0x1c0]
0x14083815a: shl r15, 7
0x14083815e: add r15, rdi
0x140838161: lea rbx, [rbp + 0x3b8]
0x140838168: lea r14, [rbp + 0x7a0]
0x14083816f: cmp rdi, r15
0x140838172: je 0x1408382db
0x140838178: nop dword ptr [rax + rax]
0x140838180: mov r12, qword ptr [rdi + 0x50]
0x140838184: mov rsi, qword ptr [rdi + 0x58]
0x140838188: mov byte ptr [rbp + 0x8af], 1
0x14083818f: call 0x140be6750
0x140838194: mov r13d, eax
0x140838197: mov byte ptr [rbp + 0x8af], 1
0x14083819e: xor ecx, ecx
0x1408381a0: call 0x1400d9b20
0x1408381a5: test rax, rax
0x1408381a8: je 0x140838318
0x1408381ae: mov dword ptr [rbp + 0x848], r13d
0x1408381b5: mov word ptr [rbp + 0x850], 0
0x1408381be: mov dword ptr [rbp + 0x84c], 0x3f800000
0x1408381c8: mov qword ptr [rbp + 0x7a0], 1
0x1408381d3: mov qword ptr [rbp + 0x7a8], r12
0x1408381da: mov qword ptr [rbp + 0x840], rsi
0x1408381e1: mov qword ptr [rbp + 0x7b0], 0
0x1408381ec: mov qword ptr [rbp + 0x7b8], 4
0x1408381f7: mov qword ptr [rbp + 0x7c0], 0
0x140838202: movups xmm0, xmmword ptr [rax]
0x140838205: inc qword ptr [rax]
0x140838208: movups xmm1, xmmword ptr [rip + 0x225e521]
0x14083820f: lea rax, [rbp + 0x810]
0x140838216: movups xmmword ptr [rax + 0x10], xmm1
0x14083821a: movups xmm1, xmmword ptr [rip + 0x225e4ff]
0x140838221: movups xmmword ptr [rax], xmm1
0x140838224: movups xmmword ptr [rbp + 0x830], xmm0
0x14083822b: movabs rax, 0x100000003
0x140838235: mov qword ptr [rbp + 0x7e0], rax
0x14083823c: mov qword ptr [rbp + 0x7c8], 0
0x140838247: mov qword ptr [rbp + 0x7d0], 1
0x140838252: mov qword ptr [rbp + 0x7d8], 0
0x14083825d: mov byte ptr [rbp + 0x8af], 1
0x140838264: mov rcx, rbx
0x140838267: mov rdx, r14
0x14083826a: lea r8, [rbp - 0x30]
0x14083826e: xor r9d, r9d
0x140838271: call 0x140905be0
0x140838276: mov byte ptr [rbp + 0x8af], 1
0x14083827d: mov rcx, r14
0x140838280: mov rdx, rbx
0x140838283: lea r8, [rbp + 0x898]
0x14083828a: mov r9, rdi
0x14083828d: call 0x14090b460
0x140838292: movzx eax, byte ptr [rbp + 0x84b]
0x140838299: and al, 0x9f
0x14083829b: or al, 0x40
0x14083829d: mov byte ptr [rbp + 0x84b], al
0x1408382a3: mov rsi, qword ptr [rdi + 0x70]
0x1408382a7: mov r8d, 0xf8
0x1408382ad: mov rcx, rbx
0x1408382b0: mov rdx, r14
0x1408382b3: call 0x142923c00
0x1408382b8: mov byte ptr [rbp + 0x8af], 1
0x1408382bf: mov rcx, rbx
0x1408382c2: mov rdx, rdi
0x1408382c5: mov r8, qword ptr [rbp + 0x8a0]
0x1408382cc: call rsi
0x1408382ce: sub rdi, -0x80
0x1408382d2: cmp rdi, r15
0x1408382d5: jne 0x140838180
0x1408382db: lea rsi, [rbp + 0x3b8]
0x1408382e2: lea rdx, [rbp - 0x30]
0x1408382e6: mov r8d, 0x3e8
0x1408382ec: mov rcx, rsi
0x1408382ef: call 0x142923c00
0x1408382f4: mov byte ptr [rbp + 0x8af], 0
0x1408382fb: mov rcx, rsi
0x1408382fe: call 0x140868370
0x140838303: nop 
0x140838304: add rsp, 0x938
0x14083830b: pop rbx
0x14083830c: pop rdi
0x14083830d: pop rsi
0x14083830e: pop r12
0x140838310: pop r13
0x140838312: pop r14
0x140838314: pop r15
0x140838316: pop rbp
0x140838317: ret 
0x140838318: mov byte ptr [rbp + 0x8af], 1
0x14083831f: lea rcx, [rip + 0x240277a]
0x140838326: call 0x1428d80d0
0x14083832b: jmp 0x140838340
0x14083832d: mov byte ptr [rbp + 0x8af], 1
0x140838334: lea rcx, [rip + 0x226d89d]
0x14083833b: call 0x1428d9310
0x140838340: ud2 
0x140838342: nop word ptr cs:[rax + rax]
0x140838350: mov qword ptr [rsp + 0x10], rdx
0x140838355: push rbp
0x140838356: push r15
0x140838358: push r14
0x14083835a: push r13
0x14083835c: push r12
0x14083835e: push rsi
0x14083835f: push rdi
0x140838360: push rbx
0x140838361: sub rsp, 0x48
0x140838365: lea rbp, [rdx + 0x80]
0x14083836c: cmp byte ptr [rbp + 0x8af], 0
0x140838373: jne 0x140838386
0x140838375: add rsp, 0x48
0x140838379: pop rbx
0x14083837a: pop rdi
0x14083837b: pop rsi
0x14083837c: pop r12
0x14083837e: pop r13
0x140838380: pop r14
0x140838382: pop r15
0x140838384: pop rbp
0x140838385: ret 
0x140838386: lea rcx, [rbp - 0x30]
0x14083838a: call 0x1408923c0
0x14083838f: jmp 0x140838375
