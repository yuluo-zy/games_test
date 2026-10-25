0x1409c4470: push rbp
0x1409c4471: push r15
0x1409c4473: push r14
0x1409c4475: push r13
0x1409c4477: push r12
0x1409c4479: push rsi
0x1409c447a: push rdi
0x1409c447b: push rbx
0x1409c447c: sub rsp, 0xe68
0x1409c4483: lea rbp, [rsp + 0x80]
0x1409c448b: movaps xmmword ptr [rbp + 0xdd0], xmm7
0x1409c4492: movaps xmmword ptr [rbp + 0xdc0], xmm6
0x1409c4499: mov qword ptr [rbp + 0xdb8], 0xfffffffffffffffe
0x1409c44a4: mov rdi, r8
0x1409c44a7: mov r14, rdx
0x1409c44aa: mov rsi, rcx
0x1409c44ad: call 0x140c16a00
0x1409c44b2: test al, al
0x1409c44b4: je 0x1409c4518
0x1409c44b6: mov r13, r14
0x1409c44b9: lea r14, [rip + 0x20fb583]
0x1409c44c0: mov ebx, 0x47
0x1409c44c5: mov edx, 0x47
0x1409c44ca: mov rcx, r14
0x1409c44cd: call 0x1409be680
0x1409c44d2: cmp rax, 1
0x1409c44d6: jne 0x1409c458d
0x1409c44dc: mov r9, rdx
0x1409c44df: test rdx, rdx
0x1409c44e2: je 0x1409c4521
0x1409c44e4: cmp r9, 0x47
0x1409c44e8: jae 0x1409c451f
0x1409c44ea: lea rax, [rip + 0x20fb552]
0x1409c44f1: cmp byte ptr [r9 + rax], 0xbf
0x1409c44f6: jg 0x1409c4521
0x1409c44f8: lea rax, [rip + 0x20fb611]
0x1409c44ff: mov qword ptr [rsp + 0x20], rax
0x1409c4504: lea rcx, [rip + 0x20fb538]
0x1409c450b: mov edx, 0x47
0x1409c4510: xor r8d, r8d
0x1409c4513: call 0x1428d9940
0x1409c4518: xor eax, eax
0x1409c451a: jmp 0x1409c46e6
0x1409c451f: jne 0x1409c44f8
0x1409c4521: lea r14, [rip + 0x20fb51b]
0x1409c4528: mov rcx, r14
0x1409c452b: mov rdx, r9
0x1409c452e: call 0x1409be680
0x1409c4533: cmp rax, 1
0x1409c4537: jne 0x1409c458d
0x1409c4539: mov r8, rdx
0x1409c453c: add r8, 2
0x1409c4540: je 0x1409c457b
0x1409c4542: cmp r8, 0x47
0x1409c4546: jae 0x1409c4579
0x1409c4548: lea rax, [rip + 0x20fb4f4]
0x1409c454f: cmp byte ptr [r8 + rax], 0xbf
0x1409c4554: jg 0x1409c457b
0x1409c4556: lea rax, [rip + 0x20fb5cb]
0x1409c455d: mov qword ptr [rsp + 0x20], rax
0x1409c4562: lea rcx, [rip + 0x20fb4da]
0x1409c4569: mov edx, 0x47
0x1409c456e: mov r9d, 0x47
0x1409c4574: call 0x1428d9940
0x1409c4579: jne 0x1409c4556
0x1409c457b: mov ebx, 0x45
0x1409c4580: sub rbx, rdx
0x1409c4583: lea r14, [rip + 0x20fb4b9]
0x1409c458a: add r14, r8
0x1409c458d: lea r8, [rip + 0x20fb9a0]
0x1409c4594: lea r15, [rip + 0x20fb954]
0x1409c459b: nop dword ptr [rax + rax]
0x1409c45a0: movsx eax, byte ptr [r8 - 1]
0x1409c45a5: test eax, eax
0x1409c45a7: js 0x1409c45c0
0x1409c45a9: dec r8
0x1409c45ac: cmp eax, 0x5c
0x1409c45af: jne 0x1409c4614
0x1409c45b1: jmp 0x1409c4626
0x1409c45b3: nop word ptr cs:[rax + rax]
0x1409c45c0: movzx ecx, byte ptr [r8 - 2]
0x1409c45c5: cmp cl, 0xc0
0x1409c45c8: jge 0x1409c45ed
0x1409c45ca: movzx edx, byte ptr [r8 - 3]
0x1409c45cf: cmp dl, 0xc0
0x1409c45d2: jge 0x1409c45f6
0x1409c45d4: movzx r9d, byte ptr [r8 - 4]
0x1409c45d9: add r8, -4
0x1409c45dd: and r9d, 7
0x1409c45e1: shl r9d, 6
0x1409c45e5: and edx, 0x3f
0x1409c45e8: or edx, r9d
0x1409c45eb: jmp 0x1409c45fd
0x1409c45ed: add r8, -2
0x1409c45f1: and ecx, 0x1f
0x1409c45f4: jmp 0x1409c4605
0x1409c45f6: add r8, -3
0x1409c45fa: and edx, 0xf
0x1409c45fd: shl edx, 6
0x1409c4600: and ecx, 0x3f
0x1409c4603: or ecx, edx
0x1409c4605: shl ecx, 6
0x1409c4608: and al, 0x3f
0x1409c460a: movzx eax, al
0x1409c460d: or eax, ecx
0x1409c460f: cmp eax, 0x5c
0x1409c4612: je 0x1409c4626
0x1409c4614: cmp eax, 0x2f
0x1409c4617: je 0x1409c4626
0x1409c4619: cmp r8, r15
0x1409c461c: jne 0x1409c45a0
0x1409c461e: mov r12d, 0x45
0x1409c4624: jmp 0x1409c4673
0x1409c4626: lea r15, [rip + 0x20fb8c2]
0x1409c462d: sub r8, r15
0x1409c4630: inc r8
0x1409c4633: je 0x1409c4667
0x1409c4635: cmp r8, 0x45
0x1409c4639: jae 0x1409c4665
0x1409c463b: cmp byte ptr [r8 + r15], 0xbf
0x1409c4640: jg 0x1409c4667
0x1409c4642: lea rax, [rip + 0x20fb4a7]
0x1409c4649: mov qword ptr [rsp + 0x20], rax
0x1409c464e: lea rcx, [rip + 0x20fb89a]
0x1409c4655: mov edx, 0x45
0x1409c465a: mov r9d, 0x45
0x1409c4660: call 0x1428d9940
0x1409c4665: jne 0x1409c4642
0x1409c4667: mov r12d, 0x45
0x1409c466d: sub r12, r8
0x1409c4670: add r15, r8
0x1409c4673: call 0x140c18130
0x1409c4678: mov rcx, rax
0x1409c467b: mov rax, qword ptr [rax]
0x1409c467e: cmp rax, 1
0x1409c4682: jne 0x1409c5a0b
0x1409c4688: add rcx, 8
0x1409c468c: cmp qword ptr [rcx], 0
0x1409c4690: jne 0x1409c5a2f
0x1409c4696: mov qword ptr [rcx], 0xffffffffffffffff
0x1409c469d: mov qword ptr [rbp + 0xda8], rcx
0x1409c46a4: add rcx, 8
0x1409c46a8: mov qword ptr [rsp + 0x20], r12
0x1409c46ad: mov qword ptr [rsp + 0x30], 0
0x1409c46b6: mov qword ptr [rsp + 0x28], 1
0x1409c46bf: mov rdx, r14
0x1409c46c2: mov r8, rbx
0x1409c46c5: mov r9, r15
0x1409c46c8: call 0x140c16d40
0x1409c46cd: mov rcx, qword ptr [rbp + 0xda8]
0x1409c46d4: inc qword ptr [rcx]
0x1409c46d7: mov qword ptr [rbp + 0xd78], rax
0x1409c46de: mov eax, 1
0x1409c46e3: mov r14, r13
0x1409c46e6: mov qword ptr [rbp + 0xd70], rax
0x1409c46ed: lea rbx, [r14 + 0x38]
0x1409c46f1: movabs rdx, 0x8597ee0727b461b3
0x1409c46fb: movabs r8, 0x8b297ec5ae15f15c
0x1409c4705: mov rcx, rbx
0x1409c4708: call 0x1400dd6f0
0x1409c470d: test rax, rax
0x1409c4710: je 0x1409c5ac2
0x1409c4716: cmp qword ptr [r14 + 0x218], rdx
0x1409c471d: jbe 0x1409c5ac2
0x1409c4723: mov rax, qword ptr [r14 + 0x210]
0x1409c472a: mov rcx, qword ptr [rax + rdx*8]
0x1409c472e: test rcx, rcx
0x1409c4731: je 0x1409c5ac2
0x1409c4737: mov rax, qword ptr [r14 + 0x1e0]
0x1409c473e: not rcx
0x1409c4741: lea rcx, [rcx + rcx*2]
0x1409c4745: shl rcx, 5
0x1409c4749: cmp qword ptr [rax + rcx + 0x20], 0
0x1409c474f: je 0x1409c5ac2
0x1409c4755: add rax, rcx
0x1409c4758: mov rcx, qword ptr [rax + 0x10]
0x1409c475c: lea rdx, [rip + 0x20fb7ed]
0x1409c4763: mov r8d, 5
0x1409c4769: call 0x140937aa0
0x1409c476e: mov qword ptr [rbp + 0xd90], rdx
0x1409c4775: test al, 1
0x1409c4777: je 0x1409c5b27
0x1409c477d: movabs rdx, 0x8597ee0727b461b3
0x1409c4787: movabs r8, 0x8b297ec5ae15f15c
0x1409c4791: mov rcx, rbx
0x1409c4794: call 0x1400dd6f0
0x1409c4799: test rax, rax
0x1409c479c: je 0x1409c5ad3
0x1409c47a2: cmp qword ptr [r14 + 0x218], rdx
0x1409c47a9: jbe 0x1409c5ad3
0x1409c47af: mov rax, qword ptr [r14 + 0x210]
0x1409c47b6: mov rcx, qword ptr [rax + rdx*8]
0x1409c47ba: test rcx, rcx
0x1409c47bd: je 0x1409c5ad3
0x1409c47c3: mov rax, qword ptr [r14 + 0x1e0]
0x1409c47ca: not rcx
0x1409c47cd: lea rcx, [rcx + rcx*2]
0x1409c47d1: shl rcx, 5
0x1409c47d5: cmp qword ptr [rax + rcx + 0x20], 0
0x1409c47db: je 0x1409c5ad3
0x1409c47e1: add rax, rcx
0x1409c47e4: mov rcx, qword ptr [rax + 0x10]
0x1409c47e8: lea rdx, [rip + 0x20fb799]
0x1409c47ef: mov r8d, 0x12
0x1409c47f5: call 0x140937aa0
0x1409c47fa: mov qword ptr [rbp + 0xd88], rdx
0x1409c4801: test al, 1
0x1409c4803: je 0x1409c5b35
0x1409c4809: movabs rdx, 0x864ddca98c2312a6
0x1409c4813: movabs r8, 0x27b0e9369e9a484e
0x1409c481d: mov rcx, rbx
0x1409c4820: call 0x1400dd6f0
0x1409c4825: test rax, rax
0x1409c4828: je 0x1409c5ae1
0x1409c482e: cmp qword ptr [r14 + 0x218], rdx
0x1409c4835: jbe 0x1409c5ae1
0x1409c483b: mov rax, qword ptr [r14 + 0x210]
0x1409c4842: mov rcx, qword ptr [rax + rdx*8]
0x1409c4846: test rcx, rcx
0x1409c4849: je 0x1409c5ae1
0x1409c484f: mov rax, qword ptr [r14 + 0x1e0]
0x1409c4856: not rcx
0x1409c4859: lea rcx, [rcx + rcx*2]
0x1409c485d: shl rcx, 5
0x1409c4861: cmp qword ptr [rax + rcx + 0x20], 0
0x1409c4867: je 0x1409c5ae1
0x1409c486d: add rax, rcx
0x1409c4870: mov rax, qword ptr [rax + 0x10]
0x1409c4874: mov r13d, dword ptr [rax]
0x1409c4877: mov r12d, dword ptr [rax + 4]
0x1409c487b: lea rcx, [rbp + 0x3b0]
0x1409c4882: mov rdx, r14
0x1409c4885: call 0x141659110
0x1409c488a: mov qword ptr [rbp + 0xd58], rbx
0x1409c4891: lea rcx, [rbp + 0x3b0]
0x1409c4898: mov rdx, r14
0x1409c489b: call 0x141657e90
0x1409c48a0: mov rdx, qword ptr [r14 + 0x5a0]
0x1409c48a7: mov rcx, qword ptr [rbp + 0x3b0]
0x1409c48ae: cmp rcx, rdx
0x1409c48b1: jne 0x1409c5a3b
0x1409c48b7: mov r9d, dword ptr [r14 + 0x5b4]
0x1409c48be: mov eax, dword ptr [r14 + 0x5b0]
0x1409c48c5: lea rcx, [rbp + 0xba0]
0x1409c48cc: mov dword ptr [rsp + 0x20], eax
0x1409c48d0: lea rbx, [rbp + 0x3b0]
0x1409c48d7: mov rdx, r14
0x1409c48da: mov r8, rbx
0x1409c48dd: call 0x141795c90
0x1409c48e2: lea rax, [r14 + 0xf8]
0x1409c48e9: lea rcx, [r14 + 0x1a0]
0x1409c48f0: mov qword ptr [rbp + 0xb80], r14
0x1409c48f7: mov qword ptr [rbp + 0xd68], rcx
0x1409c48fe: mov qword ptr [rbp + 0xb88], rcx
0x1409c4905: mov qword ptr [rbp + 0xd60], rax
0x1409c490c: mov qword ptr [rbp + 0xb90], rax
0x1409c4913: mov qword ptr [rbp + 0xb98], rbx
0x1409c491a: lea r15, [rbp - 0x38]
0x1409c491e: lea rbx, [rbp + 0xb80]
0x1409c4925: mov r8d, 0xb8
0x1409c492b: mov rcx, r15
0x1409c492e: mov rdx, rbx
0x1409c4931: call 0x142923c00
0x1409c4936: lea r8, [rip + 0x20fb05b]
0x1409c493d: lea rcx, [rbp + 0xd40]
0x1409c4944: mov rdx, r15
0x1409c4947: call 0x141306990
0x1409c494c: mov qword ptr [rbp + 0xd98], r14
0x1409c4953: mov byte ptr [rbp + 0xdb5], 1
0x1409c495a: lea rcx, [rbp + 0x3b0]
0x1409c4961: call 0x1405d0be0
0x1409c4966: mov rax, qword ptr [rbp + 0xd48]
0x1409c496d: mov qword ptr [rbp + 0xda0], rax
0x1409c4974: mov r14, qword ptr [rbp + 0xd50]
0x1409c497b: mov qword ptr [rbp + 0x3b0], r15
0x1409c4982: cmp r14, 2
0x1409c4986: jae 0x1409c5a4c
0x1409c498c: lea rcx, [rdi + 4]
0x1409c4990: mov byte ptr [rbp + 0xdb5], 1
0x1409c4997: call 0x140bd5790
0x1409c499c: mov dword ptr [rbp + 0x3b0], 0
0x1409c49a6: mov qword ptr [rbp + 0x3b8], rdi
0x1409c49ad: mov dword ptr [rbp + 0xb80], 2
0x1409c49b7: mov byte ptr [rbp + 0xdb5], 1
0x1409c49be: mov qword ptr [rsp + 0x40], rsi
0x1409c49c3: mov rcx, qword ptr [rbp + 0xd98]
0x1409c49ca: mov qword ptr [rsp + 0x38], rcx
0x1409c49cf: mov qword ptr [rsp + 0x30], rax
0x1409c49d4: mov qword ptr [rsp + 0x28], rbx
0x1409c49d9: mov qword ptr [rsp + 0x20], 1
0x1409c49e2: lea rdx, [rip + 0x20fb5e7]
0x1409c49e9: lea rcx, [rbp - 0x38]
0x1409c49ed: lea r15, [rbp + 0x3b0]
0x1409c49f4: mov r8d, 5
0x1409c49fa: mov r9, r15
0x1409c49fd: call 0x140867970
0x1409c4a02: cvtsi2ss xmm6, r13
0x1409c4a07: cvtsi2ss xmm7, r12
0x1409c4a0c: mov rax, qword ptr [rbp + 0xd40]
0x1409c4a13: mov qword ptr [rbp + 0xd80], rax
0x1409c4a1a: lea rax, [r14 + r14*2]
0x1409c4a1e: shl rax, 4
0x1409c4a22: mov rbx, qword ptr [rbp + 0xda0]
0x1409c4a29: add rax, rbx
0x1409c4a2c: mov qword ptr [rbp + 0xda8], rax
0x1409c4a33: cmp rbx, qword ptr [rbp + 0xda8]
0x1409c4a3a: je 0x1409c5271
0x1409c4a40: mov r14, qword ptr [rbx]
0x1409c4a43: test r14, r14
0x1409c4a46: je 0x1409c5271
0x1409c4a4c: mov rdi, qword ptr [rbx + 8]
0x1409c4a50: mov rsi, qword ptr [rbx + 0x10]
0x1409c4a54: test rsi, rsi
0x1409c4a57: je 0x1409c4e70
0x1409c4a5d: call 0x140be6750
0x1409c4a62: mov r12d, eax
0x1409c4a65: xor ecx, ecx
0x1409c4a67: call 0x1400d9b20
0x1409c4a6c: test rax, rax
0x1409c4a6f: je 0x1409c5aa0
0x1409c4a75: mov dword ptr [rbp + 0xd20], r12d
0x1409c4a7c: mov word ptr [rbp + 0xd28], 0
0x1409c4a85: mov dword ptr [rbp + 0xd24], 0x3f800000
0x1409c4a8f: mov qword ptr [rbp + 0xc78], 0
0x1409c4a9a: mov rcx, qword ptr [rbp + 0xd88]
0x1409c4aa1: mov qword ptr [rbp + 0xd18], rcx
0x1409c4aa8: mov qword ptr [rbp + 0xc88], 0
0x1409c4ab3: mov qword ptr [rbp + 0xc90], 4
0x1409c4abe: mov qword ptr [rbp + 0xc98], 0
0x1409c4ac9: movups xmm0, xmmword ptr [rax]
0x1409c4acc: inc qword ptr [rax]
0x1409c4acf: movups xmm1, xmmword ptr [rip + 0x20d1c5a]
0x1409c4ad6: lea rax, [rbp + 0xce8]
0x1409c4add: movups xmmword ptr [rax + 0x10], xmm1
0x1409c4ae1: movups xmm1, xmmword ptr [rip + 0x20d1c38]
0x1409c4ae8: movups xmmword ptr [rax], xmm1
0x1409c4aeb: movups xmmword ptr [rbp + 0xd08], xmm0
0x1409c4af2: movabs rax, 0x100000003
0x1409c4afc: mov qword ptr [rbp + 0xcb8], rax
0x1409c4b03: mov qword ptr [rbp + 0xca0], 0
0x1409c4b0e: mov qword ptr [rbp + 0xca8], 1
0x1409c4b19: mov qword ptr [rbp + 0xcb0], 0
0x1409c4b24: mov rcx, r15
0x1409c4b27: lea rdx, [rbp + 0xc78]
0x1409c4b2e: lea r8, [rbp - 0x38]
0x1409c4b32: xor r9d, r9d
0x1409c4b35: call 0x140905be0
0x1409c4b3a: cmp byte ptr [rbp + 0x48e], 8
0x1409c4b41: je 0x1409c4b79
0x1409c4b43: movss xmm3, dword ptr [r14]
0x1409c4b48: movss xmm0, dword ptr [r14 + 4]
0x1409c4b4e: mov rdx, qword ptr [rbp + 0x498]
0x1409c4b55: mov eax, 0xe8
0x1409c4b5a: add rdx, rax
0x1409c4b5d: movss dword ptr [rsp + 0x20], xmm0
0x1409c4b63: lea rcx, [rbp + 0x468]
0x1409c4b6a: movabs r8, 0x120d1fbd3e93b50f
0x1409c4b74: call 0x14096b940
0x1409c4b79: mov r8d, 0xf8
0x1409c4b7f: lea rcx, [rbp + 0xb80]
0x1409c4b86: mov rdx, r15
0x1409c4b89: call 0x142923c00
0x1409c4b8e: cmp byte ptr [rbp + 0xc5e], 8
0x1409c4b95: je 0x1409c4bce
0x1409c4b97: movss xmm3, dword ptr [r14 + 8]
0x1409c4b9d: movss xmm0, dword ptr [r14 + 0xc]
0x1409c4ba3: mov rdx, qword ptr [rbp + 0xc68]
0x1409c4baa: mov eax, 0xe8
0x1409c4baf: add rdx, rax
0x1409c4bb2: movss dword ptr [rsp + 0x20], xmm0
0x1409c4bb8: lea rcx, [rbp + 0xc38]
0x1409c4bbf: movabs r8, 0xa6fc8d382b41b9c5
0x1409c4bc9: call 0x14096b940
0x1409c4bce: mov r8d, 0xf8
0x1409c4bd4: lea rcx, [rbp + 0xa80]
0x1409c4bdb: lea r14, [rbp + 0xb80]
0x1409c4be2: mov rdx, r14
0x1409c4be5: call 0x142923c00
0x1409c4bea: cmp byte ptr [rbp + 0xb5e], 8
0x1409c4bf1: je 0x1409c4c21
0x1409c4bf3: mov rdx, qword ptr [rbp + 0xb68]
0x1409c4bfa: mov eax, 0xe8
0x1409c4bff: add rdx, rax
0x1409c4c02: movss dword ptr [rsp + 0x20], xmm7
0x1409c4c08: lea rcx, [rbp + 0xb38]
0x1409c4c0f: movabs r8, 0x69fdab9b991c4787
0x1409c4c19: movaps xmm3, xmm6
0x1409c4c1c: call 0x14096b940
0x1409c4c21: mov r8d, 0xf8
0x1409c4c27: lea rcx, [rbp + 0x890]
0x1409c4c2e: lea rdx, [rbp + 0xa80]
0x1409c4c35: call 0x142923c00
0x1409c4c3a: cmp byte ptr [rbp + 0x96e], 8
0x1409c4c41: je 0x1409c4c6d
0x1409c4c43: movss xmm3, dword ptr [rsi + 0x18]
0x1409c4c48: mov rdx, qword ptr [rbp + 0x978]
0x1409c4c4f: mov eax, 0xe8
0x1409c4c54: add rdx, rax
0x1409c4c57: lea rcx, [rbp + 0x948]
0x1409c4c5e: movabs r8, 0xeab2a5dacee822d6
0x1409c4c68: call 0x14096ba30
0x1409c4c6d: mov r8d, 0xf8
0x1409c4c73: lea rcx, [rbp + 0x798]
0x1409c4c7a: lea rdx, [rbp + 0x890]
0x1409c4c81: call 0x142923c00
0x1409c4c86: cmp byte ptr [rbp + 0x876], 8
0x1409c4c8d: je 0x1409c4cb9
0x1409c4c8f: movzx r9d, byte ptr [rsi + 0x21]
0x1409c4c94: mov rdx, qword ptr [rbp + 0x880]
0x1409c4c9b: mov eax, 0xe8
0x1409c4ca0: add rdx, rax
0x1409c4ca3: lea rcx, [rbp + 0x850]
0x1409c4caa: movabs r8, 0xd8ea64f9bd82ef1
0x1409c4cb4: call 0x14096b850
0x1409c4cb9: mov r8d, 0xf8
0x1409c4cbf: lea rcx, [rbp + 0x988]
0x1409c4cc6: lea rdx, [rbp + 0x798]
0x1409c4ccd: call 0x142923c00
0x1409c4cd2: mov rcx, qword ptr [rdi + 8]
0x1409c4cd6: mov rdx, qword ptr [rdi + 0x10]
0x1409c4cda: mov byte ptr [rbp + 0xdb3], 1
0x1409c4ce1: call 0x140c9cd80
0x1409c4ce6: mov r12, rax
0x1409c4ce9: mov dword ptr [rbp + 0x3b0], 0
0x1409c4cf3: mov dword ptr [rbp + 0x3b8], 0
0x1409c4cfd: mov dword ptr [rbp + 0x3c0], 0
0x1409c4d07: mov dword ptr [rbp + 0x3c8], 0
0x1409c4d11: mov dword ptr [rbp + 0x3d0], 0
0x1409c4d1b: mov dword ptr [rbp + 0x3d8], 0
0x1409c4d25: lea rax, [rbp + 0x3e0]
0x1409c4d2c: mov byte ptr [rax + 8], 0
0x1409c4d30: mov qword ptr [rax], 0
0x1409c4d37: mov byte ptr [rbp + 0xdb3], 1
0x1409c4d3e: mov rcx, r14
0x1409c4d41: mov rdx, r15
0x1409c4d44: mov r8d, 1
0x1409c4d4a: call 0x140bd57e0
0x1409c4d4f: mov r8d, dword ptr [rdi + 0x18]
0x1409c4d53: mov byte ptr [rbp + 0xdb3], 1
0x1409c4d5a: mov rcx, r15
0x1409c4d5d: mov rdx, r14
0x1409c4d60: call 0x140bd5960
0x1409c4d65: mov byte ptr [rbp + 0xdb3], 1
0x1409c4d6c: lea rcx, [rbp + 0x890]
0x1409c4d73: mov rdx, r15
0x1409c4d76: mov r8d, 1
0x1409c4d7c: mov r9d, 1
0x1409c4d82: call 0x140bd59a0
0x1409c4d87: mov qword ptr [rbp + 0x798], r12
0x1409c4d8e: movups xmm0, xmmword ptr [rbp + 0x890]
0x1409c4d95: movups xmm1, xmmword ptr [rbp + 0x8a0]
0x1409c4d9c: movups xmm2, xmmword ptr [rbp + 0x8b0]
0x1409c4da3: movups xmm3, xmmword ptr [rbp + 0x8bc]
0x1409c4daa: movups xmmword ptr [rbp + 0x3dc], xmm3
0x1409c4db1: movaps xmmword ptr [rbp + 0x3d0], xmm2
0x1409c4db8: movaps xmmword ptr [rbp + 0x3c0], xmm1
0x1409c4dbf: movaps xmmword ptr [rbp + 0x3b0], xmm0
0x1409c4dc6: mov qword ptr [rsp + 0x20], r15
0x1409c4dcb: lea rcx, [rbp + 0x798]
0x1409c4dd2: lea rdx, [rbp + 0x988]
0x1409c4dd9: movabs r8, 0x740686c2e74bfe87
0x1409c4de3: mov r9b, 0x1d
0x1409c4de6: call 0x140908610
0x1409c4deb: mov rsi, qword ptr [rbp + 0x988]
0x1409c4df2: mov r8d, 0xa0
0x1409c4df8: mov rcx, r14
0x1409c4dfb: lea r13, [rbp + 0x990]
0x1409c4e02: mov rdx, r13
0x1409c4e05: call 0x142923c00
0x1409c4e0a: movzx edi, byte ptr [rbp + 0xa30]
0x1409c4e11: mov rdx, r14
0x1409c4e14: movzx r14d, word ptr [rbp + 0xa31]
0x1409c4e1c: movzx r12d, byte ptr [rbp + 0xa33]
0x1409c4e24: movups xmm0, xmmword ptr [r13 + 0xa4]
0x1409c4e2c: movups xmm1, xmmword ptr [r13 + 0xb4]
0x1409c4e34: movups xmm2, xmmword ptr [r13 + 0xc4]
0x1409c4e3c: movups xmm3, xmmword ptr [r13 + 0xd4]
0x1409c4e44: movaps xmmword ptr [rbp + 0xa80], xmm0
0x1409c4e4b: movaps xmmword ptr [rbp + 0xa90], xmm1
0x1409c4e52: movaps xmmword ptr [rbp + 0xaa0], xmm2
0x1409c4e59: movaps xmmword ptr [rbp + 0xab0], xmm3
0x1409c4e60: movups xmm0, xmmword ptr [r13 + 0xe0]
0x1409c4e68: jmp 0x1409c51c7
0x1409c4e6d: nop dword ptr [rax]
0x1409c4e70: mov rsi, qword ptr [rbx + 0x18]
0x1409c4e74: mov r13, qword ptr [rbx + 0x28]
0x1409c4e78: call 0x140be6750
0x1409c4e7d: mov r12d, eax
0x1409c4e80: xor ecx, ecx
0x1409c4e82: call 0x1400d9b20
0x1409c4e87: test rax, rax
0x1409c4e8a: je 0x1409c5ab1
0x1409c4e90: mov dword ptr [rbp + 0xd20], r12d
0x1409c4e97: mov word ptr [rbp + 0xd28], 0
0x1409c4ea0: mov dword ptr [rbp + 0xd24], 0x3f800000
0x1409c4eaa: mov qword ptr [rbp + 0xc78], 0
0x1409c4eb5: mov rcx, qword ptr [rbp + 0xd90]
0x1409c4ebc: mov qword ptr [rbp + 0xd18], rcx
0x1409c4ec3: mov qword ptr [rbp + 0xc88], 0
0x1409c4ece: mov qword ptr [rbp + 0xc90], 4
0x1409c4ed9: mov qword ptr [rbp + 0xc98], 0
0x1409c4ee4: movups xmm0, xmmword ptr [rax]
0x1409c4ee7: inc qword ptr [rax]
0x1409c4eea: movups xmm1, xmmword ptr [rip + 0x20d183f]
0x1409c4ef1: lea rax, [rbp + 0xce8]
0x1409c4ef8: movups xmmword ptr [rax + 0x10], xmm1
0x1409c4efc: movups xmm1, xmmword ptr [rip + 0x20d181d]
0x1409c4f03: movups xmmword ptr [rax], xmm1
0x1409c4f06: movups xmmword ptr [rbp + 0xd08], xmm0
0x1409c4f0d: movabs rax, 0x100000003
0x1409c4f17: mov qword ptr [rbp + 0xcb8], rax
0x1409c4f1e: mov qword ptr [rbp + 0xca0], 0
0x1409c4f29: mov qword ptr [rbp + 0xca8], 1
0x1409c4f34: mov qword ptr [rbp + 0xcb0], 0
0x1409c4f3f: mov rcx, r15
0x1409c4f42: lea rdx, [rbp + 0xc78]
0x1409c4f49: lea r8, [rbp - 0x38]
0x1409c4f4d: xor r9d, r9d
0x1409c4f50: call 0x140905be0
0x1409c4f55: cmp byte ptr [rbp + 0x48e], 8
0x1409c4f5c: je 0x1409c4f94
0x1409c4f5e: movss xmm3, dword ptr [r14]
0x1409c4f63: movss xmm0, dword ptr [r14 + 4]
0x1409c4f69: mov rdx, qword ptr [rbp + 0x498]
0x1409c4f70: mov eax, 0xe8
0x1409c4f75: add rdx, rax
0x1409c4f78: movss dword ptr [rsp + 0x20], xmm0
0x1409c4f7e: lea rcx, [rbp + 0x468]
0x1409c4f85: movabs r8, 0x120d1fbd3e93b50f
0x1409c4f8f: call 0x14096b940
0x1409c4f94: mov r8d, 0xf8
0x1409c4f9a: lea rcx, [rbp + 0xb80]
0x1409c4fa1: mov rdx, r15
0x1409c4fa4: call 0x142923c00
0x1409c4fa9: cmp byte ptr [rbp + 0xc5e], 8
0x1409c4fb0: je 0x1409c4fe9
0x1409c4fb2: movss xmm3, dword ptr [r14 + 8]
0x1409c4fb8: movss xmm0, dword ptr [r14 + 0xc]
0x1409c4fbe: mov rdx, qword ptr [rbp + 0xc68]
0x1409c4fc5: mov eax, 0xe8
0x1409c4fca: add rdx, rax
0x1409c4fcd: movss dword ptr [rsp + 0x20], xmm0
0x1409c4fd3: lea rcx, [rbp + 0xc38]
0x1409c4fda: movabs r8, 0xa6fc8d382b41b9c5
0x1409c4fe4: call 0x14096b940
0x1409c4fe9: mov r8d, 0xf8
0x1409c4fef: lea rcx, [rbp + 0xa80]
0x1409c4ff6: lea r14, [rbp + 0xb80]
0x1409c4ffd: mov rdx, r14
0x1409c5000: call 0x142923c00
0x1409c5005: cmp byte ptr [rbp + 0xb5e], 8
0x1409c500c: je 0x1409c503c
0x1409c500e: mov rdx, qword ptr [rbp + 0xb68]
0x1409c5015: mov eax, 0xe8
0x1409c501a: add rdx, rax
0x1409c501d: movss dword ptr [rsp + 0x20], xmm7
0x1409c5023: lea rcx, [rbp + 0xb38]
0x1409c502a: movabs r8, 0x69fdab9b991c4787
0x1409c5034: movaps xmm3, xmm6
0x1409c5037: call 0x14096b940
0x1409c503c: mov r8d, 0xf8
0x1409c5042: lea rcx, [rbp + 0x890]
0x1409c5049: lea rdx, [rbp + 0xa80]
0x1409c5050: call 0x142923c00
0x1409c5055: cmp byte ptr [rbp + 0x96e], 8
0x1409c505c: je 0x1409c508a
0x1409c505e: test r13, r13
0x1409c5061: setne r9b
0x1409c5065: mov rdx, qword ptr [rbp + 0x978]
0x1409c506c: mov eax, 0xe8
0x1409c5071: add rdx, rax
0x1409c5074: lea rcx, [rbp + 0x948]
0x1409c507b: movabs r8, 0x35594af7455f3925
0x1409c5085: call 0x14096b850
0x1409c508a: mov r8d, 0xf8
0x1409c5090: lea rcx, [rbp + 0x798]
0x1409c5097: lea rdx, [rbp + 0x890]
0x1409c509e: call 0x142923c00
0x1409c50a3: test rsi, rsi
0x1409c50a6: je 0x1409c50b8
0x1409c50a8: movzx r9d, byte ptr [rsi + 2]
0x1409c50ad: cmp byte ptr [rbp + 0x876], 8
0x1409c50b4: jne 0x1409c50c4
0x1409c50b6: jmp 0x1409c50e9
0x1409c50b8: xor r9d, r9d
0x1409c50bb: cmp byte ptr [rbp + 0x876], 8
0x1409c50c2: je 0x1409c50e9
0x1409c50c4: mov rdx, qword ptr [rbp + 0x880]
0x1409c50cb: mov eax, 0xe8
0x1409c50d0: add rdx, rax
0x1409c50d3: lea rcx, [rbp + 0x850]
0x1409c50da: movabs r8, 0x4c5803d8a5526c67
0x1409c50e4: call 0x14096b850
0x1409c50e9: mov r8d, 0xf8
0x1409c50ef: lea rsi, [rbp + 0x988]
0x1409c50f6: mov rcx, rsi
0x1409c50f9: lea rdx, [rbp + 0x798]
0x1409c5100: call 0x142923c00
0x1409c5105: mov rcx, qword ptr [rdi + 8]
0x1409c5109: mov rdx, qword ptr [rdi + 0x10]
0x1409c510d: mov byte ptr [rbp + 0xdb1], 1
0x1409c5114: call 0x140c9cd80
0x1409c5119: mov qword ptr [rbp + 0x890], rax
0x1409c5120: mov dword ptr [rbp + 0x3b0], 2
0x1409c512a: mov qword ptr [rsp + 0x20], r15
0x1409c512f: lea rcx, [rbp + 0x890]
0x1409c5136: mov rdx, rsi
0x1409c5139: movabs r8, 0x740686c2e74bfe87
0x1409c5143: mov r9b, 0x1d
0x1409c5146: call 0x140908610
0x1409c514b: mov rsi, qword ptr [rbp + 0x988]
0x1409c5152: mov r8d, 0xa0
0x1409c5158: mov rcx, r14
0x1409c515b: lea rdx, [rbp + 0x990]
0x1409c5162: call 0x142923c00
0x1409c5167: movzx edi, byte ptr [rbp + 0xa30]
0x1409c516e: mov rdx, r14
0x1409c5171: movzx r14d, word ptr [rbp + 0xa31]
0x1409c5179: movzx r12d, byte ptr [rbp + 0xa33]
0x1409c5181: lea rax, [rbp + 0x990]
0x1409c5188: movups xmm0, xmmword ptr [rax + 0xa4]
0x1409c518f: movups xmm1, xmmword ptr [rax + 0xb4]
0x1409c5196: movups xmm2, xmmword ptr [rax + 0xc4]
0x1409c519d: movups xmm3, xmmword ptr [rax + 0xd4]
0x1409c51a4: movaps xmmword ptr [rbp + 0xa80], xmm0
0x1409c51ab: movaps xmmword ptr [rbp + 0xa90], xmm1
0x1409c51b2: movaps xmmword ptr [rbp + 0xaa0], xmm2
0x1409c51b9: movaps xmmword ptr [rbp + 0xab0], xmm3
0x1409c51c0: movups xmm0, xmmword ptr [rax + 0xe0]
0x1409c51c7: movups xmmword ptr [rbp + 0xabc], xmm0
0x1409c51ce: and dil, 0x7f
0x1409c51d2: and r12b, 0x9f
0x1409c51d6: or r12b, 0x40
0x1409c51da: mov qword ptr [rbp + 0x3b0], rsi
0x1409c51e1: mov r8d, 0xa0
0x1409c51e7: lea rcx, [rbp + 0x3b8]
0x1409c51ee: call 0x142923c00
0x1409c51f3: mov byte ptr [rbp + 0x458], dil
0x1409c51fa: mov word ptr [rbp + 0x459], r14w
0x1409c5202: mov byte ptr [rbp + 0x45b], r12b
0x1409c5209: movaps xmm0, xmmword ptr [rbp + 0xa80]
0x1409c5210: movaps xmm1, xmmword ptr [rbp + 0xa90]
0x1409c5217: movaps xmm2, xmmword ptr [rbp + 0xaa0]
0x1409c521e: movaps xmm3, xmmword ptr [rbp + 0xab0]
0x1409c5225: lea rax, [rbp + 0x3e0]
0x1409c522c: movups xmmword ptr [rax + 0x7c], xmm0
0x1409c5230: movups xmmword ptr [rax + 0x8c], xmm1
0x1409c5237: movups xmmword ptr [rax + 0x9c], xmm2
0x1409c523e: movups xmmword ptr [rax + 0xac], xmm3
0x1409c5245: movups xmm0, xmmword ptr [rbp + 0xabc]
0x1409c524c: movups xmmword ptr [rax + 0xb8], xmm0
0x1409c5253: mov edx, 6
0x1409c5258: mov rcx, r15
0x1409c525b: call 0x140906a90
0x1409c5260: add rbx, 0x30
0x1409c5264: cmp rbx, qword ptr [rbp + 0xda8]
0x1409c526b: jne 0x1409c4a40
0x1409c5271: mov rax, qword ptr [rbp + 0xd80]
0x1409c5278: test rax, rax
0x1409c527b: je 0x1409c5297
0x1409c527d: shl rax, 4
0x1409c5281: lea rdx, [rax + rax*2]
0x1409c5285: mov r8d, 8
0x1409c528b: mov rcx, qword ptr [rbp + 0xda0]
0x1409c5292: call 0x140613c20
0x1409c5297: mov byte ptr [rbp + 0xdb2], 1
0x1409c529e: lea rcx, [rbp + 0x3b0]
0x1409c52a5: mov rdi, qword ptr [rbp + 0xd98]
0x1409c52ac: mov rdx, rdi
0x1409c52af: call 0x141659850
0x1409c52b4: mov rbx, qword ptr [rbp + 0xd58]
0x1409c52bb: lea rsi, [rbp + 0x3b0]
0x1409c52c2: mov rcx, rsi
0x1409c52c5: mov rdx, rdi
0x1409c52c8: call 0x141657e90
0x1409c52cd: mov rdx, qword ptr [rdi + 0x5a0]
0x1409c52d4: mov rcx, qword ptr [rbp + 0x3b0]
0x1409c52db: cmp rcx, rdx
0x1409c52de: jne 0x1409c5a7e
0x1409c52e4: mov eax, dword ptr [rdi + 0x5b0]
0x1409c52ea: mov rax, qword ptr [rbp + 0x4a8]
0x1409c52f1: mov rcx, qword ptr [rbp + 0x4b0]
0x1409c52f8: lea rcx, [rax + rcx*4]
0x1409c52fc: movzx edx, byte ptr [rbp + 0x4b8]
0x1409c5303: xorps xmm0, xmm0
0x1409c5306: movups xmmword ptr [rbp + 0xbd8], xmm0
0x1409c530d: movups xmmword ptr [rbp + 0xbc8], xmm0
0x1409c5314: mov qword ptr [rbp + 0xbe8], 0
0x1409c531f: mov qword ptr [rbp + 0xb80], rdi
0x1409c5326: mov r8, qword ptr [rbp + 0xd68]
0x1409c532d: mov qword ptr [rbp + 0xb88], r8
0x1409c5334: mov r8, qword ptr [rbp + 0xd60]
0x1409c533b: mov qword ptr [rbp + 0xb90], r8
0x1409c5342: mov qword ptr [rbp + 0xb98], rsi
0x1409c5349: mov qword ptr [rbp + 0xba0], rax
0x1409c5350: mov qword ptr [rbp + 0xba8], rcx
0x1409c5357: mov qword ptr [rbp + 0xbb0], 8
0x1409c5362: mov qword ptr [rbp + 0xbb8], 0
0x1409c536d: mov qword ptr [rbp + 0xbc0], 8
0x1409c5378: mov byte ptr [rbp + 0xbf0], dl
0x1409c537e: mov eax, dword ptr [rbp + 0xa80]
0x1409c5384: mov ecx, dword ptr [rbp + 0xa83]
0x1409c538a: mov dword ptr [rbp + 0xbf1], eax
0x1409c5390: mov dword ptr [rbp + 0xbf4], ecx
0x1409c5396: lea r8, [rip + 0x20fa5fb]
0x1409c539d: lea rcx, [rbp + 0xc78]
0x1409c53a4: lea rdx, [rbp + 0xb80]
0x1409c53ab: call 0x141305880
0x1409c53b0: mov byte ptr [rbp + 0xdb7], 1
0x1409c53b7: mov byte ptr [rbp + 0xdb6], 1
0x1409c53be: mov rcx, rsi
0x1409c53c1: call 0x1405d0be0
0x1409c53c6: mov byte ptr [rbp + 0xdb7], 1
0x1409c53cd: mov byte ptr [rbp + 0xdb6], 1
0x1409c53d4: movabs rdx, 0x8597ee0727b461b3
0x1409c53de: movabs r8, 0x8b297ec5ae15f15c
0x1409c53e8: mov rcx, rbx
0x1409c53eb: call 0x1400dd6f0
0x1409c53f0: test rax, rax
0x1409c53f3: je 0x1409c5aef
0x1409c53f9: cmp qword ptr [rdi + 0x218], rdx
0x1409c5400: jbe 0x1409c5aef
0x1409c5406: mov rax, qword ptr [rdi + 0x210]
0x1409c540d: mov rcx, qword ptr [rax + rdx*8]
0x1409c5411: test rcx, rcx
0x1409c5414: je 0x1409c5aef
0x1409c541a: mov rax, qword ptr [rdi + 0x1e0]
0x1409c5421: not rcx
0x1409c5424: lea rcx, [rcx + rcx*2]
0x1409c5428: shl rcx, 5
0x1409c542c: cmp qword ptr [rax + rcx + 0x20], 0
0x1409c5432: je 0x1409c5aef
0x1409c5438: add rax, rcx
0x1409c543b: mov rcx, qword ptr [rax + 0x10]
0x1409c543f: mov byte ptr [rbp + 0xdb7], 1
0x1409c5446: mov byte ptr [rbp + 0xdb6], 1
0x1409c544d: lea rdx, [rip + 0x20fab9c]
0x1409c5454: mov r8d, 8
0x1409c545a: call 0x140937aa0
0x1409c545f: mov qword ptr [rbp + 0xd88], rdx
0x1409c5466: test al, 1
0x1409c5468: je 0x1409c5b43
0x1409c546e: mov byte ptr [rbp + 0xdb7], 1
0x1409c5475: mov byte ptr [rbp + 0xdb6], 1
0x1409c547c: movabs rdx, 0x4bc362195b80f729
0x1409c5486: movabs r8, 0x982798ce80cbcefb
0x1409c5490: mov rcx, rbx
0x1409c5493: call 0x1400dd6f0
0x1409c5498: test rax, rax
0x1409c549b: je 0x1409c5b0b
0x1409c54a1: cmp qword ptr [rdi + 0x218], rdx
0x1409c54a8: jbe 0x1409c5b0b
0x1409c54ae: mov rax, qword ptr [rdi + 0x210]
0x1409c54b5: mov rax, qword ptr [rax + rdx*8]
0x1409c54b9: test rax, rax
0x1409c54bc: je 0x1409c5b0b
0x1409c54c2: mov rcx, qword ptr [rdi + 0x1e0]
0x1409c54c9: not rax
0x1409c54cc: lea rax, [rax + rax*2]
0x1409c54d0: shl rax, 5
0x1409c54d4: cmp qword ptr [rcx + rax + 0x20], 0
0x1409c54da: je 0x1409c5b0b
0x1409c54e0: mov rax, qword ptr [rbp + 0xc78]
0x1409c54e7: mov qword ptr [rbp + 0xd90], rax
0x1409c54ee: mov r15, qword ptr [rbp + 0xc80]
0x1409c54f5: mov rax, qword ptr [rbp + 0xc88]
0x1409c54fc: shl rax, 4
0x1409c5500: add rax, r15
0x1409c5503: mov qword ptr [rbp + 0xda8], rax
0x1409c550a: lea r13, [rbp + 0x7a0]
0x1409c5511: lea r12, [rbp + 0xb80]
0x1409c5518: mov qword ptr [rbp + 0xd98], r15
0x1409c551f: cmp r15, qword ptr [rbp + 0xda8]
0x1409c5526: je 0x1409c5931
0x1409c552c: nop dword ptr [rax]
0x1409c5530: mov rbx, qword ptr [r15]
0x1409c5533: mov r14, qword ptr [r15 + 8]
0x1409c5537: call 0x140be6750
0x1409c553c: mov edi, eax
0x1409c553e: xor ecx, ecx
0x1409c5540: call 0x1400d9b20
0x1409c5545: test rax, rax
0x1409c5548: je 0x1409c5a8f
0x1409c554e: mov dword ptr [rbp + 0xa30], edi
0x1409c5554: mov word ptr [rbp + 0xa38], 0
0x1409c555d: mov dword ptr [rbp + 0xa34], 0x3f800000
0x1409c5567: mov qword ptr [rbp + 0x988], 0
0x1409c5572: mov rcx, qword ptr [rbp + 0xd88]
0x1409c5579: mov qword ptr [rbp + 0xa28], rcx
0x1409c5580: mov qword ptr [rbp + 0x998], 0
0x1409c558b: mov qword ptr [rbp + 0x9a0], 4
0x1409c5596: mov qword ptr [rbp + 0x9a8], 0
0x1409c55a1: movups xmm0, xmmword ptr [rax]
0x1409c55a4: inc qword ptr [rax]
0x1409c55a7: movups xmm1, xmmword ptr [rip + 0x20d1182]
0x1409c55ae: lea rax, [rbp + 0x9f8]
0x1409c55b5: movups xmmword ptr [rax + 0x10], xmm1
0x1409c55b9: movups xmm1, xmmword ptr [rip + 0x20d1160]
0x1409c55c0: movups xmmword ptr [rax], xmm1
0x1409c55c3: movups xmmword ptr [rbp + 0xa18], xmm0
0x1409c55ca: movabs rax, 0x100000003
0x1409c55d4: mov qword ptr [rbp + 0x9c8], rax
0x1409c55db: mov qword ptr [rbp + 0x9b0], 0
0x1409c55e6: mov qword ptr [rbp + 0x9b8], 1
0x1409c55f1: mov qword ptr [rbp + 0x9c0], 0
0x1409c55fc: mov rcx, rsi
0x1409c55ff: lea rdx, [rbp + 0x988]
0x1409c5606: lea r8, [rbp - 0x38]
0x1409c560a: xor r9d, r9d
0x1409c560d: call 0x140905be0
0x1409c5612: cmp byte ptr [rbp + 0x48e], 8
0x1409c5619: je 0x1409c5646
0x1409c561b: mov rdx, qword ptr [rbp + 0x498]
0x1409c5622: mov eax, 0xe8
0x1409c5627: add rdx, rax
0x1409c562a: lea rcx, [rbp + 0x468]
0x1409c5631: movabs r8, 0xc5ab673fb08651f5
0x1409c563b: mov r9d, 1
0x1409c5641: call 0x14096b570
0x1409c5646: mov r8d, 0xf8
0x1409c564c: mov rcx, r12
0x1409c564f: mov rdx, rsi
0x1409c5652: call 0x142923c00
0x1409c5657: cmp byte ptr [rbp + 0xc5e], 8
0x1409c565e: je 0x1409c569c
0x1409c5660: mov rdx, qword ptr [rbp + 0xc68]
0x1409c5667: mov eax, 0xe8
0x1409c566c: add rdx, rax
0x1409c566f: mov eax, dword ptr [rbx + 0x10]
0x1409c5672: mov dword ptr [rbp + 0x3b8], eax
0x1409c5678: mov rax, qword ptr [rbx + 8]
0x1409c567c: mov qword ptr [rbp + 0x3b0], rax
0x1409c5683: lea rcx, [rbp + 0xc38]
0x1409c568a: movabs r8, 0x120d1fbd3e93b50f
0x1409c5694: mov r9, rsi
0x1409c5697: call 0x14096b660
0x1409c569c: mov r8d, 0xf8
0x1409c56a2: lea rcx, [rbp + 0xa80]
0x1409c56a9: mov rdx, r12
0x1409c56ac: call 0x142923c00
0x1409c56b1: cmp byte ptr [rbp + 0xb5e], 8
0x1409c56b8: je 0x1409c56ee
0x1409c56ba: movss xmm3, dword ptr [rbx]
0x1409c56be: movss xmm0, dword ptr [rbx + 4]
0x1409c56c3: mov rdx, qword ptr [rbp + 0xb68]
0x1409c56ca: mov eax, 0xe8
0x1409c56cf: add rdx, rax
0x1409c56d2: movss dword ptr [rsp + 0x20], xmm0
0x1409c56d8: lea rcx, [rbp + 0xb38]
0x1409c56df: movabs r8, 0xa6fc8d382b41b9c5
0x1409c56e9: call 0x14096b940
0x1409c56ee: mov r8d, 0xf8
0x1409c56f4: lea rcx, [rbp + 0x890]
0x1409c56fb: lea rdx, [rbp + 0xa80]
0x1409c5702: call 0x142923c00
0x1409c5707: cmp byte ptr [rbp + 0x96e], 8
0x1409c570e: je 0x1409c573e
0x1409c5710: mov rdx, qword ptr [rbp + 0x978]
0x1409c5717: mov eax, 0xe8
0x1409c571c: add rdx, rax
0x1409c571f: movss dword ptr [rsp + 0x20], xmm7
0x1409c5725: lea rcx, [rbp + 0x948]
0x1409c572c: movabs r8, 0x69fdab9b991c4787
0x1409c5736: movaps xmm3, xmm6
0x1409c5739: call 0x14096b940
0x1409c573e: mov r8d, 0xf8
0x1409c5744: lea rcx, [rbp + 0x798]
0x1409c574b: lea rdx, [rbp + 0x890]
0x1409c5752: call 0x142923c00
0x1409c5757: mov rcx, qword ptr [r14 + 8]
0x1409c575b: mov rdx, qword ptr [r14 + 0x10]
0x1409c575f: mov byte ptr [rbp + 0xdb4], 1
0x1409c5766: call 0x140c9cd80
0x1409c576b: mov rdi, rax
0x1409c576e: mov dword ptr [rbp + 0x3b0], 0
0x1409c5778: mov dword ptr [rbp + 0x3b8], 0
0x1409c5782: mov dword ptr [rbp + 0x3c0], 0
0x1409c578c: mov dword ptr [rbp + 0x3c8], 0
0x1409c5796: mov dword ptr [rbp + 0x3d0], 0
0x1409c57a0: mov dword ptr [rbp + 0x3d8], 0
0x1409c57aa: lea rax, [rbp + 0x3e0]
0x1409c57b1: mov byte ptr [rax + 8], 0
0x1409c57b5: mov qword ptr [rax], 0
0x1409c57bc: mov byte ptr [rbp + 0xdb4], 1
0x1409c57c3: mov rcx, r12
0x1409c57c6: mov rdx, rsi
0x1409c57c9: mov r8d, 1
0x1409c57cf: call 0x140bd57e0
0x1409c57d4: mov r8d, dword ptr [r14 + 0x18]
0x1409c57d8: mov byte ptr [rbp + 0xdb4], 1
0x1409c57df: mov rcx, rsi
0x1409c57e2: mov rdx, r12
0x1409c57e5: call 0x140bd5960
0x1409c57ea: mov byte ptr [rbp + 0xdb4], 1
0x1409c57f1: mov rcx, r12
0x1409c57f4: mov rdx, rsi
0x1409c57f7: mov r8d, 1
0x1409c57fd: mov r9d, 1
0x1409c5803: call 0x140bd59a0
0x1409c5808: mov qword ptr [rbp + 0xa80], rdi
0x1409c580f: movups xmm0, xmmword ptr [rbp + 0xb80]
0x1409c5816: movups xmm1, xmmword ptr [rbp + 0xb90]
0x1409c581d: movups xmm2, xmmword ptr [rbp + 0xba0]
0x1409c5824: movups xmm3, xmmword ptr [rbp + 0xbac]
0x1409c582b: movups xmmword ptr [rbp + 0x3dc], xmm3
0x1409c5832: movaps xmmword ptr [rbp + 0x3d0], xmm2
0x1409c5839: movaps xmmword ptr [rbp + 0x3c0], xmm1
0x1409c5840: movaps xmmword ptr [rbp + 0x3b0], xmm0
0x1409c5847: mov qword ptr [rsp + 0x20], rsi
0x1409c584c: lea rcx, [rbp + 0xa80]
0x1409c5853: lea rdx, [rbp + 0x798]
0x1409c585a: movabs r8, 0x740686c2e74bfe87
0x1409c5864: mov r9b, 0x1d
0x1409c5867: call 0x140908610
0x1409c586c: mov rdi, qword ptr [rbp + 0x798]
0x1409c5873: mov r8d, 0xa0
0x1409c5879: lea rcx, [rbp + 0x3b8]
0x1409c5880: mov rdx, r13
0x1409c5883: call 0x142923c00
0x1409c5888: movzx eax, byte ptr [rbp + 0x840]
0x1409c588f: movzx ecx, word ptr [rbp + 0x841]
0x1409c5896: movzx edx, byte ptr [rbp + 0x843]
0x1409c589d: movups xmm0, xmmword ptr [r13 + 0xa4]
0x1409c58a5: movups xmm1, xmmword ptr [r13 + 0xb4]
0x1409c58ad: movups xmm2, xmmword ptr [r13 + 0xc4]
0x1409c58b5: movups xmm3, xmmword ptr [r13 + 0xd4]
0x1409c58bd: lea r8, [rbp + 0x3e0]
0x1409c58c4: movups xmmword ptr [r8 + 0x7c], xmm0
0x1409c58c9: movups xmmword ptr [r8 + 0x8c], xmm1
0x1409c58d1: movups xmmword ptr [r8 + 0x9c], xmm2
0x1409c58d9: movups xmmword ptr [r8 + 0xac], xmm3
0x1409c58e1: movups xmm0, xmmword ptr [r13 + 0xe0]
0x1409c58e9: movups xmmword ptr [r8 + 0xb8], xmm0
0x1409c58f1: and al, 0x7f
0x1409c58f3: and dl, 0x9f
0x1409c58f6: or dl, 0x40
0x1409c58f9: mov qword ptr [rbp + 0x3b0], rdi
0x1409c5900: mov byte ptr [rbp + 0x458], al
0x1409c5906: mov word ptr [rbp + 0x459], cx
0x1409c590d: mov byte ptr [rbp + 0x45b], dl
0x1409c5913: mov edx, 6
0x1409c5918: mov rcx, rsi
0x1409c591b: call 0x140906a90
0x1409c5920: add r15, 0x10
0x1409c5924: cmp r15, qword ptr [rbp + 0xda8]
0x1409c592b: jne 0x1409c5530
0x1409c5931: mov rdx, qword ptr [rbp + 0xd90]
0x1409c5938: test rdx, rdx
0x1409c593b: je 0x1409c5953
0x1409c593d: shl rdx, 4
0x1409c5941: mov r8d, 8
0x1409c5947: mov rcx, qword ptr [rbp + 0xd98]
0x1409c594e: call 0x140613c20
0x1409c5953: lea rsi, [rbp + 0x3b0]
0x1409c595a: lea rdx, [rbp - 0x38]
0x1409c595e: mov r8d, 0x3e8
0x1409c5964: mov rcx, rsi
0x1409c5967: call 0x142923c00
0x1409c596c: mov byte ptr [rbp + 0xdb7], 0
0x1409c5973: mov byte ptr [rbp + 0xdb6], 0
0x1409c597a: mov rcx, rsi
0x1409c597d: call 0x140868370
0x1409c5982: cmp qword ptr [rbp + 0xd70], 0
0x1409c598a: je 0x1409c59d3
0x1409c598c: call 0x140c18130
0x1409c5991: mov rcx, rax
0x1409c5994: mov rax, qword ptr [rax]
0x1409c5997: cmp rax, 1
0x1409c599b: jne 0x1409c59f5
0x1409c599d: add rcx, 8
0x1409c59a1: cmp qword ptr [rcx], 0
0x1409c59a5: jne 0x1409c5a2f
0x1409c59ab: mov qword ptr [rcx], 0xffffffffffffffff
0x1409c59b2: mov qword ptr [rbp + 0xda8], rcx
0x1409c59b9: add rcx, 8
0x1409c59bd: mov rdx, qword ptr [rbp + 0xd78]
0x1409c59c4: call 0x140c170f0
0x1409c59c9: mov rax, qword ptr [rbp + 0xda8]
0x1409c59d0: inc qword ptr [rax]
0x1409c59d3: movaps xmm6, xmmword ptr [rbp + 0xdc0]
0x1409c59da: movaps xmm7, xmmword ptr [rbp + 0xdd0]
0x1409c59e1: add rsp, 0xe68
0x1409c59e8: pop rbx
0x1409c59e9: pop rdi
0x1409c59ea: pop rsi
0x1409c59eb: pop r12
0x1409c59ed: pop r13
0x1409c59ef: pop r14
0x1409c59f1: pop r15
0x1409c59f3: pop rbp
0x1409c59f4: ret 
0x1409c59f5: test rax, rax
0x1409c59f8: jne 0x1409c5a23
0x1409c59fa: xor edx, edx
0x1409c59fc: call 0x1428f87d0
0x1409c5a01: mov rcx, rax
0x1409c5a04: test rax, rax
0x1409c5a07: jne 0x1409c59a1
0x1409c5a09: jmp 0x1409c5a23
0x1409c5a0b: test rax, rax
0x1409c5a0e: jne 0x1409c5a23
0x1409c5a10: xor edx, edx
0x1409c5a12: call 0x1428f87d0
0x1409c5a17: mov rcx, rax
0x1409c5a1a: test rax, rax
0x1409c5a1d: jne 0x1409c468c
0x1409c5a23: lea rcx, [rip + 0x2275076]
0x1409c5a2a: call 0x1428d80d0
0x1409c5a2f: lea rcx, [rip + 0x227513a]
0x1409c5a36: call 0x1428d9250
0x1409c5a3b: lea r8, [rip + 0x22bbea6]
0x1409c5a42: call 0x1428cf200
0x1409c5a47: jmp 0x1409c5b5d
0x1409c5a4c: cmp r14, 0x15
0x1409c5a50: jae 0x1409c5b5f
0x1409c5a56: mov byte ptr [rbp + 0xdb5], 1
0x1409c5a5d: lea r9, [rbp + 0x3b0]
0x1409c5a64: mov r8d, 1
0x1409c5a6a: mov rcx, qword ptr [rbp + 0xda0]
0x1409c5a71: mov rdx, r14
0x1409c5a74: call 0x1413386d0
0x1409c5a79: jmp 0x1409c498c
0x1409c5a7e: lea r8, [rip + 0x22bbe63]
0x1409c5a85: call 0x1428cf200
0x1409c5a8a: jmp 0x1409c5b5d
0x1409c5a8f: lea rcx, [rip + 0x227500a]
0x1409c5a96: call 0x1428d80d0
0x1409c5a9b: jmp 0x1409c5b5d
0x1409c5aa0: lea rcx, [rip + 0x2274ff9]
0x1409c5aa7: call 0x1428d80d0
0x1409c5aac: jmp 0x1409c5b5d
0x1409c5ab1: lea rcx, [rip + 0x2274fe8]
0x1409c5ab8: call 0x1428d80d0
0x1409c5abd: jmp 0x1409c5b5d
0x1409c5ac2: lea rcx, [rip + 0x20fa46f]
0x1409c5ac9: call 0x1428d9310
0x1409c5ace: jmp 0x1409c5b5d
0x1409c5ad3: lea rcx, [rip + 0x20fa496]
0x1409c5ada: call 0x1428d9310
0x1409c5adf: jmp 0x1409c5b5d
0x1409c5ae1: lea rcx, [rip + 0x20fa4d0]
0x1409c5ae8: call 0x1428d9310
0x1409c5aed: jmp 0x1409c5b5d
0x1409c5aef: mov byte ptr [rbp + 0xdb7], 1
0x1409c5af6: mov byte ptr [rbp + 0xdb6], 1
0x1409c5afd: lea rcx, [rip + 0x20fa4d4]
0x1409c5b04: call 0x1428d9310
0x1409c5b09: jmp 0x1409c5b5d
0x1409c5b0b: mov byte ptr [rbp + 0xdb7], 1
0x1409c5b12: mov byte ptr [rbp + 0xdb6], 1
0x1409c5b19: lea rcx, [rip + 0x20e00b8]
0x1409c5b20: call 0x1428d9310
0x1409c5b25: jmp 0x1409c5b5d
0x1409c5b27: lea rcx, [rip + 0x20fa42a]
0x1409c5b2e: call 0x1428d9310
0x1409c5b33: jmp 0x1409c5b5d
0x1409c5b35: lea rcx, [rip + 0x20fa464]
0x1409c5b3c: call 0x1428d9310
0x1409c5b41: jmp 0x1409c5b5d
0x1409c5b43: mov byte ptr [rbp + 0xdb7], 1
0x1409c5b4a: mov byte ptr [rbp + 0xdb6], 1
0x1409c5b51: lea rcx, [rip + 0x20fa4a0]
0x1409c5b58: call 0x1428d9310
0x1409c5b5d: ud2 
0x1409c5b5f: mov byte ptr [rbp + 0xdb5], 1
0x1409c5b66: lea r8, [rbp + 0x3b0]
0x1409c5b6d: mov rcx, qword ptr [rbp + 0xda0]
0x1409c5b74: mov rdx, r14
0x1409c5b77: call 0x140b4be30
0x1409c5b7c: jmp 0x1409c498c
0x1409c5b81: nop word ptr cs:[rax + rax]
0x1409c5b90: mov qword ptr [rsp + 0x10], rdx
0x1409c5b95: push rbp
0x1409c5b96: push r15
0x1409c5b98: push r14
0x1409c5b9a: push r13
0x1409c5b9c: push r12
0x1409c5b9e: push rsi
0x1409c5b9f: push rdi
0x1409c5ba0: push rbx
0x1409c5ba1: sub rsp, 0x68
0x1409c5ba5: lea rbp, [rdx + 0x80]
0x1409c5bac: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c5bb1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c5bb6: mov rax, qword ptr [rbp + 0xda8]
0x1409c5bbd: inc qword ptr [rax]
0x1409c5bc0: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5bc5: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5bca: add rsp, 0x68
0x1409c5bce: pop rbx
0x1409c5bcf: pop rdi
0x1409c5bd0: pop rsi
0x1409c5bd1: pop r12
0x1409c5bd3: pop r13
0x1409c5bd5: pop r14
0x1409c5bd7: pop r15
0x1409c5bd9: pop rbp
0x1409c5bda: ret 
0x1409c5bdb: nop dword ptr [rax + rax]
0x1409c5be0: mov qword ptr [rsp + 0x10], rdx
0x1409c5be5: push rbp
0x1409c5be6: push r15
0x1409c5be8: push r14
0x1409c5bea: push r13
0x1409c5bec: push r12
0x1409c5bee: push rsi
0x1409c5bef: push rdi
0x1409c5bf0: push rbx
0x1409c5bf1: sub rsp, 0x68
0x1409c5bf5: lea rbp, [rdx + 0x80]
0x1409c5bfc: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c5c01: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c5c06: cmp qword ptr [rbp + 0xd70], 0
0x1409c5c0e: je 0x1409c5c3f
0x1409c5c10: lea rax, [rbp + 0xd78]
0x1409c5c17: mov qword ptr [rbp + 0xd38], rax
0x1409c5c1e: lea rax, [rbp + 0xd38]
0x1409c5c25: mov qword ptr [rbp + 0xd30], rax
0x1409c5c2c: lea rcx, [rip + 0x20f9e5d]
0x1409c5c33: lea rdx, [rbp + 0xd30]
0x1409c5c3a: call 0x14132f2d0
0x1409c5c3f: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5c44: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5c49: add rsp, 0x68
0x1409c5c4d: pop rbx
0x1409c5c4e: pop rdi
0x1409c5c4f: pop rsi
0x1409c5c50: pop r12
0x1409c5c52: pop r13
0x1409c5c54: pop r14
0x1409c5c56: pop r15
0x1409c5c58: pop rbp
0x1409c5c59: ret 
0x1409c5c5a: nop word ptr [rax + rax]
0x1409c5c60: mov qword ptr [rsp + 0x10], rdx
0x1409c5c65: push rbp
0x1409c5c66: push r15
0x1409c5c68: push r14
0x1409c5c6a: push r13
0x1409c5c6c: push r12
0x1409c5c6e: push rsi
0x1409c5c6f: push rdi
0x1409c5c70: push rbx
0x1409c5c71: sub rsp, 0x68
0x1409c5c75: lea rbp, [rdx + 0x80]
0x1409c5c7c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c5c81: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c5c86: lea rcx, [rbp + 0x3b0]
0x1409c5c8d: call 0x1405d0be0
0x1409c5c92: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5c97: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5c9c: add rsp, 0x68
0x1409c5ca0: pop rbx
0x1409c5ca1: pop rdi
0x1409c5ca2: pop rsi
0x1409c5ca3: pop r12
0x1409c5ca5: pop r13
0x1409c5ca7: pop r14
0x1409c5ca9: pop r15
0x1409c5cab: pop rbp
0x1409c5cac: ret 
0x1409c5cad: nop dword ptr [rax]
0x1409c5cb0: mov qword ptr [rsp + 0x10], rdx
0x1409c5cb5: push rbp
0x1409c5cb6: push r15
0x1409c5cb8: push r14
0x1409c5cba: push r13
0x1409c5cbc: push r12
0x1409c5cbe: push rsi
0x1409c5cbf: push rdi
0x1409c5cc0: push rbx
0x1409c5cc1: sub rsp, 0x68
0x1409c5cc5: lea rbp, [rdx + 0x80]
0x1409c5ccc: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c5cd1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c5cd6: mov rax, qword ptr [rbp + 0xd80]
0x1409c5cdd: test rax, rax
0x1409c5ce0: je 0x1409c5cfc
0x1409c5ce2: shl rax, 4
0x1409c5ce6: lea rdx, [rax + rax*2]
0x1409c5cea: mov r8d, 8
0x1409c5cf0: mov rcx, qword ptr [rbp + 0xda0]
0x1409c5cf7: call 0x140613c20
0x1409c5cfc: mov byte ptr [rbp + 0xdb2], 1
0x1409c5d03: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5d08: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5d0d: add rsp, 0x68
0x1409c5d11: pop rbx
0x1409c5d12: pop rdi
0x1409c5d13: pop rsi
0x1409c5d14: pop r12
0x1409c5d16: pop r13
0x1409c5d18: pop r14
0x1409c5d1a: pop r15
0x1409c5d1c: pop rbp
0x1409c5d1d: ret 
0x1409c5d1e: nop 
0x1409c5d20: mov qword ptr [rsp + 0x10], rdx
0x1409c5d25: push rbp
0x1409c5d26: push r15
0x1409c5d28: push r14
0x1409c5d2a: push r13
0x1409c5d2c: push r12
0x1409c5d2e: push rsi
0x1409c5d2f: push rdi
0x1409c5d30: push rbx
0x1409c5d31: sub rsp, 0x68
0x1409c5d35: lea rbp, [rdx + 0x80]
0x1409c5d3c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c5d41: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c5d46: lea rcx, [rbp + 0x3b0]
0x1409c5d4d: call 0x1405d0be0
0x1409c5d52: mov byte ptr [rbp + 0xdb2], 1
0x1409c5d59: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5d5e: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5d63: add rsp, 0x68
0x1409c5d67: pop rbx
0x1409c5d68: pop rdi
0x1409c5d69: pop rsi
0x1409c5d6a: pop r12
0x1409c5d6c: pop r13
0x1409c5d6e: pop r14
0x1409c5d70: pop r15
0x1409c5d72: pop rbp
0x1409c5d73: ret 
0x1409c5d74: nop word ptr cs:[rax + rax]
0x1409c5d80: mov qword ptr [rsp + 0x10], rdx
0x1409c5d85: push rbp
0x1409c5d86: push r15
0x1409c5d88: push r14
0x1409c5d8a: push r13
0x1409c5d8c: push r12
0x1409c5d8e: push rsi
0x1409c5d8f: push rdi
0x1409c5d90: push rbx
0x1409c5d91: sub rsp, 0x68
0x1409c5d95: lea rbp, [rdx + 0x80]
0x1409c5d9c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c5da1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c5da6: mov rdx, qword ptr [rbp + 0xd90]
0x1409c5dad: test rdx, rdx
0x1409c5db0: je 0x1409c5dc8
0x1409c5db2: shl rdx, 4
0x1409c5db6: mov r8d, 8
0x1409c5dbc: mov rcx, qword ptr [rbp + 0xd98]
0x1409c5dc3: call 0x140613c20
0x1409c5dc8: mov byte ptr [rbp + 0xdb7], 0
0x1409c5dcf: mov byte ptr [rbp + 0xdb6], 1
0x1409c5dd6: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5ddb: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5de0: add rsp, 0x68
0x1409c5de4: pop rbx
0x1409c5de5: pop rdi
0x1409c5de6: pop rsi
0x1409c5de7: pop r12
0x1409c5de9: pop r13
0x1409c5deb: pop r14
0x1409c5ded: pop r15
0x1409c5def: pop rbp
0x1409c5df0: ret 
0x1409c5df1: nop word ptr cs:[rax + rax]
0x1409c5e00: mov qword ptr [rsp + 0x10], rdx
0x1409c5e05: push rbp
0x1409c5e06: push r15
0x1409c5e08: push r14
0x1409c5e0a: push r13
0x1409c5e0c: push r12
0x1409c5e0e: push rsi
0x1409c5e0f: push rdi
0x1409c5e10: push rbx
0x1409c5e11: sub rsp, 0x68
0x1409c5e15: lea rbp, [rdx + 0x80]
0x1409c5e1c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c5e21: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c5e26: mov rax, qword ptr [rbp + 0xda8]
0x1409c5e2d: inc qword ptr [rax]
0x1409c5e30: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5e35: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5e3a: add rsp, 0x68
0x1409c5e3e: pop rbx
0x1409c5e3f: pop rdi
0x1409c5e40: pop rsi
0x1409c5e41: pop r12
0x1409c5e43: pop r13
0x1409c5e45: pop r14
0x1409c5e47: pop r15
0x1409c5e49: pop rbp
0x1409c5e4a: ret 
0x1409c5e4b: nop dword ptr [rax + rax]
0x1409c5e50: mov qword ptr [rsp + 0x10], rdx
0x1409c5e55: push rbp
0x1409c5e56: push r15
0x1409c5e58: push r14
0x1409c5e5a: push r13
0x1409c5e5c: push r12
0x1409c5e5e: push rsi
0x1409c5e5f: push rdi
0x1409c5e60: push rbx
0x1409c5e61: sub rsp, 0x68
0x1409c5e65: lea rbp, [rdx + 0x80]
0x1409c5e6c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c5e71: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c5e76: cmp byte ptr [rbp + 0xdb4], 0
0x1409c5e7d: je 0x1409c5ea6
0x1409c5e7f: lea rcx, [rbp + 0x798]
0x1409c5e86: call 0x1408935c0
0x1409c5e8b: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5e90: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5e95: add rsp, 0x68
0x1409c5e99: pop rbx
0x1409c5e9a: pop rdi
0x1409c5e9b: pop rsi
0x1409c5e9c: pop r12
0x1409c5e9e: pop r13
0x1409c5ea0: pop r14
0x1409c5ea2: pop r15
0x1409c5ea4: pop rbp
0x1409c5ea5: ret 
0x1409c5ea6: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5eab: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5eb0: add rsp, 0x68
0x1409c5eb4: pop rbx
0x1409c5eb5: pop rdi
0x1409c5eb6: pop rsi
0x1409c5eb7: pop r12
0x1409c5eb9: pop r13
0x1409c5ebb: pop r14
0x1409c5ebd: pop r15
0x1409c5ebf: pop rbp
0x1409c5ec0: ret 
0x1409c5ec1: nop word ptr cs:[rax + rax]
0x1409c5ed0: mov qword ptr [rsp + 0x10], rdx
0x1409c5ed5: push rbp
0x1409c5ed6: push r15
0x1409c5ed8: push r14
0x1409c5eda: push r13
0x1409c5edc: push r12
0x1409c5ede: push rsi
0x1409c5edf: push rdi
0x1409c5ee0: push rbx
0x1409c5ee1: sub rsp, 0x68
0x1409c5ee5: lea rbp, [rdx + 0x80]
0x1409c5eec: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c5ef1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c5ef6: movzx ebx, byte ptr [rbp + 0xdb6]
0x1409c5efd: cmp byte ptr [rbp + 0xdb7], 0
0x1409c5f04: je 0x1409c5f4c
0x1409c5f06: mov rdx, qword ptr [rbp + 0xc78]
0x1409c5f0d: test rdx, rdx
0x1409c5f10: je 0x1409c5f28
0x1409c5f12: mov rcx, qword ptr [rbp + 0xc80]
0x1409c5f19: shl rdx, 4
0x1409c5f1d: mov r8d, 8
0x1409c5f23: call 0x140613c20
0x1409c5f28: and bl, 1
0x1409c5f2b: mov byte ptr [rbp + 0xdb2], bl
0x1409c5f31: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5f36: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5f3b: add rsp, 0x68
0x1409c5f3f: pop rbx
0x1409c5f40: pop rdi
0x1409c5f41: pop rsi
0x1409c5f42: pop r12
0x1409c5f44: pop r13
0x1409c5f46: pop r14
0x1409c5f48: pop r15
0x1409c5f4a: pop rbp
0x1409c5f4b: ret 
0x1409c5f4c: and bl, 1
0x1409c5f4f: mov byte ptr [rbp + 0xdb2], bl
0x1409c5f55: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5f5a: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5f5f: add rsp, 0x68
0x1409c5f63: pop rbx
0x1409c5f64: pop rdi
0x1409c5f65: pop rsi
0x1409c5f66: pop r12
0x1409c5f68: pop r13
0x1409c5f6a: pop r14
0x1409c5f6c: pop r15
0x1409c5f6e: pop rbp
0x1409c5f6f: ret 
0x1409c5f70: mov qword ptr [rsp + 0x10], rdx
0x1409c5f75: push rbp
0x1409c5f76: push r15
0x1409c5f78: push r14
0x1409c5f7a: push r13
0x1409c5f7c: push r12
0x1409c5f7e: push rsi
0x1409c5f7f: push rdi
0x1409c5f80: push rbx
0x1409c5f81: sub rsp, 0x68
0x1409c5f85: lea rbp, [rdx + 0x80]
0x1409c5f8c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c5f91: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c5f96: cmp byte ptr [rbp + 0xdb3], 0
0x1409c5f9d: je 0x1409c5fc6
0x1409c5f9f: lea rcx, [rbp + 0x988]
0x1409c5fa6: call 0x1408935c0
0x1409c5fab: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5fb0: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5fb5: add rsp, 0x68
0x1409c5fb9: pop rbx
0x1409c5fba: pop rdi
0x1409c5fbb: pop rsi
0x1409c5fbc: pop r12
0x1409c5fbe: pop r13
0x1409c5fc0: pop r14
0x1409c5fc2: pop r15
0x1409c5fc4: pop rbp
0x1409c5fc5: ret 
0x1409c5fc6: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c5fcb: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c5fd0: add rsp, 0x68
0x1409c5fd4: pop rbx
0x1409c5fd5: pop rdi
0x1409c5fd6: pop rsi
0x1409c5fd7: pop r12
0x1409c5fd9: pop r13
0x1409c5fdb: pop r14
0x1409c5fdd: pop r15
0x1409c5fdf: pop rbp
0x1409c5fe0: ret 
0x1409c5fe1: nop word ptr cs:[rax + rax]
0x1409c5ff0: mov qword ptr [rsp + 0x10], rdx
0x1409c5ff5: push rbp
0x1409c5ff6: push r15
0x1409c5ff8: push r14
0x1409c5ffa: push r13
0x1409c5ffc: push r12
0x1409c5ffe: push rsi
0x1409c5fff: push rdi
0x1409c6000: push rbx
0x1409c6001: sub rsp, 0x68
0x1409c6005: lea rbp, [rdx + 0x80]
0x1409c600c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6011: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6016: cmp byte ptr [rbp + 0xdb1], 0
0x1409c601d: je 0x1409c6046
0x1409c601f: lea rcx, [rbp + 0x988]
0x1409c6026: call 0x1408935c0
0x1409c602b: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c6030: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c6035: add rsp, 0x68
0x1409c6039: pop rbx
0x1409c603a: pop rdi
0x1409c603b: pop rsi
0x1409c603c: pop r12
0x1409c603e: pop r13
0x1409c6040: pop r14
0x1409c6042: pop r15
0x1409c6044: pop rbp
0x1409c6045: ret 
0x1409c6046: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c604b: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c6050: add rsp, 0x68
0x1409c6054: pop rbx
0x1409c6055: pop rdi
0x1409c6056: pop rsi
0x1409c6057: pop r12
0x1409c6059: pop r13
0x1409c605b: pop r14
0x1409c605d: pop r15
0x1409c605f: pop rbp
0x1409c6060: ret 
0x1409c6061: nop word ptr cs:[rax + rax]
0x1409c6070: mov qword ptr [rsp + 0x10], rdx
0x1409c6075: push rbp
0x1409c6076: push r15
0x1409c6078: push r14
0x1409c607a: push r13
0x1409c607c: push r12
0x1409c607e: push rsi
0x1409c607f: push rdi
0x1409c6080: push rbx
0x1409c6081: sub rsp, 0x68
0x1409c6085: lea rbp, [rdx + 0x80]
0x1409c608c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6091: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6096: cmp byte ptr [rbp + 0xdb2], 0
0x1409c609d: je 0x1409c60ca
0x1409c609f: lea rcx, [rbp - 0x38]
0x1409c60a3: call 0x1408923c0
0x1409c60a8: mov byte ptr [rbp + 0xdb5], 0
0x1409c60af: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c60b4: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c60b9: add rsp, 0x68
0x1409c60bd: pop rbx
0x1409c60be: pop rdi
0x1409c60bf: pop rsi
0x1409c60c0: pop r12
0x1409c60c2: pop r13
0x1409c60c4: pop r14
0x1409c60c6: pop r15
0x1409c60c8: pop rbp
0x1409c60c9: ret 
0x1409c60ca: mov byte ptr [rbp + 0xdb5], 0
0x1409c60d1: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c60d6: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c60db: add rsp, 0x68
0x1409c60df: pop rbx
0x1409c60e0: pop rdi
0x1409c60e1: pop rsi
0x1409c60e2: pop r12
0x1409c60e4: pop r13
0x1409c60e6: pop r14
0x1409c60e8: pop r15
0x1409c60ea: pop rbp
0x1409c60eb: ret 
0x1409c60ec: nop dword ptr [rax]
0x1409c60f0: mov qword ptr [rsp + 0x10], rdx
0x1409c60f5: push rbp
0x1409c60f6: push r15
0x1409c60f8: push r14
0x1409c60fa: push r13
0x1409c60fc: push r12
0x1409c60fe: push rsi
0x1409c60ff: push rdi
0x1409c6100: push rbx
0x1409c6101: sub rsp, 0x68
0x1409c6105: lea rbp, [rdx + 0x80]
0x1409c610c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6111: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6116: cmp byte ptr [rbp + 0xdb5], 0
0x1409c611d: je 0x1409c6160
0x1409c611f: mov rax, qword ptr [rbp + 0xd40]
0x1409c6126: test rax, rax
0x1409c6129: je 0x1409c6145
0x1409c612b: mov rcx, qword ptr [rbp + 0xd48]
0x1409c6132: shl rax, 4
0x1409c6136: lea rdx, [rax + rax*2]
0x1409c613a: mov r8d, 8
0x1409c6140: call 0x140613c20
0x1409c6145: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c614a: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c614f: add rsp, 0x68
0x1409c6153: pop rbx
0x1409c6154: pop rdi
0x1409c6155: pop rsi
0x1409c6156: pop r12
0x1409c6158: pop r13
0x1409c615a: pop r14
0x1409c615c: pop r15
0x1409c615e: pop rbp
0x1409c615f: ret 
0x1409c6160: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c6165: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c616a: add rsp, 0x68
0x1409c616e: pop rbx
0x1409c616f: pop rdi
0x1409c6170: pop rsi
0x1409c6171: pop r12
0x1409c6173: pop r13
0x1409c6175: pop r14
0x1409c6177: pop r15
0x1409c6179: pop rbp
0x1409c617a: ret 
0x1409c617b: nop dword ptr [rax + rax]
0x1409c6180: mov qword ptr [rsp + 0x10], rdx
0x1409c6185: push rbp
0x1409c6186: push r15
0x1409c6188: push r14
0x1409c618a: push r13
0x1409c618c: push r12
0x1409c618e: push rsi
0x1409c618f: push rdi
0x1409c6190: push rbx
0x1409c6191: sub rsp, 0x68
0x1409c6195: lea rbp, [rdx + 0x80]
0x1409c619c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c61a1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c61a6: lea rcx, [rbp + 0x3b0]
0x1409c61ad: call 0x1408935c0
0x1409c61b2: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c61b7: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c61bc: add rsp, 0x68
0x1409c61c0: pop rbx
0x1409c61c1: pop rdi
0x1409c61c2: pop rsi
0x1409c61c3: pop r12
0x1409c61c5: pop r13
0x1409c61c7: pop r14
0x1409c61c9: pop r15
0x1409c61cb: pop rbp
0x1409c61cc: ret 
0x1409c61cd: nop dword ptr [rax]
0x1409c61d0: mov qword ptr [rsp + 0x10], rdx
0x1409c61d5: push rbp
0x1409c61d6: push r15
0x1409c61d8: push r14
0x1409c61da: push r13
0x1409c61dc: push r12
0x1409c61de: push rsi
0x1409c61df: push rdi
0x1409c61e0: push rbx
0x1409c61e1: sub rsp, 0x68
0x1409c61e5: lea rbp, [rdx + 0x80]
0x1409c61ec: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c61f1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c61f6: lea rcx, [rbp + 0xb80]
0x1409c61fd: call 0x1408935c0
0x1409c6202: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c6207: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c620c: add rsp, 0x68
0x1409c6210: pop rbx
0x1409c6211: pop rdi
0x1409c6212: pop rsi
0x1409c6213: pop r12
0x1409c6215: pop r13
0x1409c6217: pop r14
0x1409c6219: pop r15
0x1409c621b: pop rbp
0x1409c621c: ret 
0x1409c621d: nop dword ptr [rax]
0x1409c6220: mov qword ptr [rsp + 0x10], rdx
0x1409c6225: push rbp
0x1409c6226: push r15
0x1409c6228: push r14
0x1409c622a: push r13
0x1409c622c: push r12
0x1409c622e: push rsi
0x1409c622f: push rdi
0x1409c6230: push rbx
0x1409c6231: sub rsp, 0x68
0x1409c6235: lea rbp, [rdx + 0x80]
0x1409c623c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6241: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6246: lea rcx, [rbp + 0xa80]
0x1409c624d: call 0x1408935c0
0x1409c6252: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c6257: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c625c: add rsp, 0x68
0x1409c6260: pop rbx
0x1409c6261: pop rdi
0x1409c6262: pop rsi
0x1409c6263: pop r12
0x1409c6265: pop r13
0x1409c6267: pop r14
0x1409c6269: pop r15
0x1409c626b: pop rbp
0x1409c626c: ret 
0x1409c626d: nop dword ptr [rax]
0x1409c6270: mov qword ptr [rsp + 0x10], rdx
0x1409c6275: push rbp
0x1409c6276: push r15
0x1409c6278: push r14
0x1409c627a: push r13
0x1409c627c: push r12
0x1409c627e: push rsi
0x1409c627f: push rdi
0x1409c6280: push rbx
0x1409c6281: sub rsp, 0x68
0x1409c6285: lea rbp, [rdx + 0x80]
0x1409c628c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6291: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6296: lea rcx, [rbp + 0x890]
0x1409c629d: call 0x1408935c0
0x1409c62a2: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c62a7: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c62ac: add rsp, 0x68
0x1409c62b0: pop rbx
0x1409c62b1: pop rdi
0x1409c62b2: pop rsi
0x1409c62b3: pop r12
0x1409c62b5: pop r13
0x1409c62b7: pop r14
0x1409c62b9: pop r15
0x1409c62bb: pop rbp
0x1409c62bc: ret 
0x1409c62bd: nop dword ptr [rax]
0x1409c62c0: mov qword ptr [rsp + 0x10], rdx
0x1409c62c5: push rbp
0x1409c62c6: push r15
0x1409c62c8: push r14
0x1409c62ca: push r13
0x1409c62cc: push r12
0x1409c62ce: push rsi
0x1409c62cf: push rdi
0x1409c62d0: push rbx
0x1409c62d1: sub rsp, 0x68
0x1409c62d5: lea rbp, [rdx + 0x80]
0x1409c62dc: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c62e1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c62e6: mov byte ptr [rbp + 0xdb4], 0
0x1409c62ed: lea rcx, [rbp + 0x798]
0x1409c62f4: call 0x1408935c0
0x1409c62f9: mov byte ptr [rbp + 0xdb4], 0
0x1409c6300: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c6305: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c630a: add rsp, 0x68
0x1409c630e: pop rbx
0x1409c630f: pop rdi
0x1409c6310: pop rsi
0x1409c6311: pop r12
0x1409c6313: pop r13
0x1409c6315: pop r14
0x1409c6317: pop r15
0x1409c6319: pop rbp
0x1409c631a: ret 
0x1409c631b: nop dword ptr [rax + rax]
0x1409c6320: mov qword ptr [rsp + 0x10], rdx
0x1409c6325: push rbp
0x1409c6326: push r15
0x1409c6328: push r14
0x1409c632a: push r13
0x1409c632c: push r12
0x1409c632e: push rsi
0x1409c632f: push rdi
0x1409c6330: push rbx
0x1409c6331: sub rsp, 0x68
0x1409c6335: lea rbp, [rdx + 0x80]
0x1409c633c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6341: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6346: lea rcx, [rbp + 0x3b0]
0x1409c634d: call 0x1408935c0
0x1409c6352: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c6357: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c635c: add rsp, 0x68
0x1409c6360: pop rbx
0x1409c6361: pop rdi
0x1409c6362: pop rsi
0x1409c6363: pop r12
0x1409c6365: pop r13
0x1409c6367: pop r14
0x1409c6369: pop r15
0x1409c636b: pop rbp
0x1409c636c: ret 
0x1409c636d: nop dword ptr [rax]
0x1409c6370: mov qword ptr [rsp + 0x10], rdx
0x1409c6375: push rbp
0x1409c6376: push r15
0x1409c6378: push r14
0x1409c637a: push r13
0x1409c637c: push r12
0x1409c637e: push rsi
0x1409c637f: push rdi
0x1409c6380: push rbx
0x1409c6381: sub rsp, 0x68
0x1409c6385: lea rbp, [rdx + 0x80]
0x1409c638c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6391: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6396: lea rcx, [rbp + 0xb80]
0x1409c639d: call 0x1408935c0
0x1409c63a2: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c63a7: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c63ac: add rsp, 0x68
0x1409c63b0: pop rbx
0x1409c63b1: pop rdi
0x1409c63b2: pop rsi
0x1409c63b3: pop r12
0x1409c63b5: pop r13
0x1409c63b7: pop r14
0x1409c63b9: pop r15
0x1409c63bb: pop rbp
0x1409c63bc: ret 
0x1409c63bd: nop dword ptr [rax]
0x1409c63c0: mov qword ptr [rsp + 0x10], rdx
0x1409c63c5: push rbp
0x1409c63c6: push r15
0x1409c63c8: push r14
0x1409c63ca: push r13
0x1409c63cc: push r12
0x1409c63ce: push rsi
0x1409c63cf: push rdi
0x1409c63d0: push rbx
0x1409c63d1: sub rsp, 0x68
0x1409c63d5: lea rbp, [rdx + 0x80]
0x1409c63dc: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c63e1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c63e6: lea rcx, [rbp + 0xa80]
0x1409c63ed: call 0x1408935c0
0x1409c63f2: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c63f7: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c63fc: add rsp, 0x68
0x1409c6400: pop rbx
0x1409c6401: pop rdi
0x1409c6402: pop rsi
0x1409c6403: pop r12
0x1409c6405: pop r13
0x1409c6407: pop r14
0x1409c6409: pop r15
0x1409c640b: pop rbp
0x1409c640c: ret 
0x1409c640d: nop dword ptr [rax]
0x1409c6410: mov qword ptr [rsp + 0x10], rdx
0x1409c6415: push rbp
0x1409c6416: push r15
0x1409c6418: push r14
0x1409c641a: push r13
0x1409c641c: push r12
0x1409c641e: push rsi
0x1409c641f: push rdi
0x1409c6420: push rbx
0x1409c6421: sub rsp, 0x68
0x1409c6425: lea rbp, [rdx + 0x80]
0x1409c642c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6431: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6436: lea rcx, [rbp + 0x890]
0x1409c643d: call 0x1408935c0
0x1409c6442: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c6447: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c644c: add rsp, 0x68
0x1409c6450: pop rbx
0x1409c6451: pop rdi
0x1409c6452: pop rsi
0x1409c6453: pop r12
0x1409c6455: pop r13
0x1409c6457: pop r14
0x1409c6459: pop r15
0x1409c645b: pop rbp
0x1409c645c: ret 
0x1409c645d: nop dword ptr [rax]
0x1409c6460: mov qword ptr [rsp + 0x10], rdx
0x1409c6465: push rbp
0x1409c6466: push r15
0x1409c6468: push r14
0x1409c646a: push r13
0x1409c646c: push r12
0x1409c646e: push rsi
0x1409c646f: push rdi
0x1409c6470: push rbx
0x1409c6471: sub rsp, 0x68
0x1409c6475: lea rbp, [rdx + 0x80]
0x1409c647c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6481: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6486: lea rcx, [rbp + 0x798]
0x1409c648d: call 0x1408935c0
0x1409c6492: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c6497: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c649c: add rsp, 0x68
0x1409c64a0: pop rbx
0x1409c64a1: pop rdi
0x1409c64a2: pop rsi
0x1409c64a3: pop r12
0x1409c64a5: pop r13
0x1409c64a7: pop r14
0x1409c64a9: pop r15
0x1409c64ab: pop rbp
0x1409c64ac: ret 
0x1409c64ad: nop dword ptr [rax]
0x1409c64b0: mov qword ptr [rsp + 0x10], rdx
0x1409c64b5: push rbp
0x1409c64b6: push r15
0x1409c64b8: push r14
0x1409c64ba: push r13
0x1409c64bc: push r12
0x1409c64be: push rsi
0x1409c64bf: push rdi
0x1409c64c0: push rbx
0x1409c64c1: sub rsp, 0x68
0x1409c64c5: lea rbp, [rdx + 0x80]
0x1409c64cc: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c64d1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c64d6: mov byte ptr [rbp + 0xdb3], 0
0x1409c64dd: lea rcx, [rbp + 0x988]
0x1409c64e4: call 0x1408935c0
0x1409c64e9: mov byte ptr [rbp + 0xdb3], 0
0x1409c64f0: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c64f5: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c64fa: add rsp, 0x68
0x1409c64fe: pop rbx
0x1409c64ff: pop rdi
0x1409c6500: pop rsi
0x1409c6501: pop r12
0x1409c6503: pop r13
0x1409c6505: pop r14
0x1409c6507: pop r15
0x1409c6509: pop rbp
0x1409c650a: ret 
0x1409c650b: nop dword ptr [rax + rax]
0x1409c6510: mov qword ptr [rsp + 0x10], rdx
0x1409c6515: push rbp
0x1409c6516: push r15
0x1409c6518: push r14
0x1409c651a: push r13
0x1409c651c: push r12
0x1409c651e: push rsi
0x1409c651f: push rdi
0x1409c6520: push rbx
0x1409c6521: sub rsp, 0x68
0x1409c6525: lea rbp, [rdx + 0x80]
0x1409c652c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6531: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6536: lea rcx, [rbp + 0x3b0]
0x1409c653d: call 0x1408935c0
0x1409c6542: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c6547: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c654c: add rsp, 0x68
0x1409c6550: pop rbx
0x1409c6551: pop rdi
0x1409c6552: pop rsi
0x1409c6553: pop r12
0x1409c6555: pop r13
0x1409c6557: pop r14
0x1409c6559: pop r15
0x1409c655b: pop rbp
0x1409c655c: ret 
0x1409c655d: nop dword ptr [rax]
0x1409c6560: mov qword ptr [rsp + 0x10], rdx
0x1409c6565: push rbp
0x1409c6566: push r15
0x1409c6568: push r14
0x1409c656a: push r13
0x1409c656c: push r12
0x1409c656e: push rsi
0x1409c656f: push rdi
0x1409c6570: push rbx
0x1409c6571: sub rsp, 0x68
0x1409c6575: lea rbp, [rdx + 0x80]
0x1409c657c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6581: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6586: lea rcx, [rbp + 0xb80]
0x1409c658d: call 0x1408935c0
0x1409c6592: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c6597: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c659c: add rsp, 0x68
0x1409c65a0: pop rbx
0x1409c65a1: pop rdi
0x1409c65a2: pop rsi
0x1409c65a3: pop r12
0x1409c65a5: pop r13
0x1409c65a7: pop r14
0x1409c65a9: pop r15
0x1409c65ab: pop rbp
0x1409c65ac: ret 
0x1409c65ad: nop dword ptr [rax]
0x1409c65b0: mov qword ptr [rsp + 0x10], rdx
0x1409c65b5: push rbp
0x1409c65b6: push r15
0x1409c65b8: push r14
0x1409c65ba: push r13
0x1409c65bc: push r12
0x1409c65be: push rsi
0x1409c65bf: push rdi
0x1409c65c0: push rbx
0x1409c65c1: sub rsp, 0x68
0x1409c65c5: lea rbp, [rdx + 0x80]
0x1409c65cc: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c65d1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c65d6: lea rcx, [rbp + 0xa80]
0x1409c65dd: call 0x1408935c0
0x1409c65e2: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c65e7: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c65ec: add rsp, 0x68
0x1409c65f0: pop rbx
0x1409c65f1: pop rdi
0x1409c65f2: pop rsi
0x1409c65f3: pop r12
0x1409c65f5: pop r13
0x1409c65f7: pop r14
0x1409c65f9: pop r15
0x1409c65fb: pop rbp
0x1409c65fc: ret 
0x1409c65fd: nop dword ptr [rax]
0x1409c6600: mov qword ptr [rsp + 0x10], rdx
0x1409c6605: push rbp
0x1409c6606: push r15
0x1409c6608: push r14
0x1409c660a: push r13
0x1409c660c: push r12
0x1409c660e: push rsi
0x1409c660f: push rdi
0x1409c6610: push rbx
0x1409c6611: sub rsp, 0x68
0x1409c6615: lea rbp, [rdx + 0x80]
0x1409c661c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6621: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6626: lea rcx, [rbp + 0x890]
0x1409c662d: call 0x1408935c0
0x1409c6632: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c6637: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c663c: add rsp, 0x68
0x1409c6640: pop rbx
0x1409c6641: pop rdi
0x1409c6642: pop rsi
0x1409c6643: pop r12
0x1409c6645: pop r13
0x1409c6647: pop r14
0x1409c6649: pop r15
0x1409c664b: pop rbp
0x1409c664c: ret 
0x1409c664d: nop dword ptr [rax]
0x1409c6650: mov qword ptr [rsp + 0x10], rdx
0x1409c6655: push rbp
0x1409c6656: push r15
0x1409c6658: push r14
0x1409c665a: push r13
0x1409c665c: push r12
0x1409c665e: push rsi
0x1409c665f: push rdi
0x1409c6660: push rbx
0x1409c6661: sub rsp, 0x68
0x1409c6665: lea rbp, [rdx + 0x80]
0x1409c666c: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c6671: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c6676: lea rcx, [rbp + 0x798]
0x1409c667d: call 0x1408935c0
0x1409c6682: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c6687: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c668c: add rsp, 0x68
0x1409c6690: pop rbx
0x1409c6691: pop rdi
0x1409c6692: pop rsi
0x1409c6693: pop r12
0x1409c6695: pop r13
0x1409c6697: pop r14
0x1409c6699: pop r15
0x1409c669b: pop rbp
0x1409c669c: ret 
0x1409c669d: nop dword ptr [rax]
0x1409c66a0: mov qword ptr [rsp + 0x10], rdx
0x1409c66a5: push rbp
0x1409c66a6: push r15
0x1409c66a8: push r14
0x1409c66aa: push r13
0x1409c66ac: push r12
0x1409c66ae: push rsi
0x1409c66af: push rdi
0x1409c66b0: push rbx
0x1409c66b1: sub rsp, 0x68
0x1409c66b5: lea rbp, [rdx + 0x80]
0x1409c66bc: movaps xmmword ptr [rsp + 0x40], xmm7
0x1409c66c1: movaps xmmword ptr [rsp + 0x50], xmm6
0x1409c66c6: mov byte ptr [rbp + 0xdb1], 0
0x1409c66cd: lea rcx, [rbp + 0x988]
0x1409c66d4: call 0x1408935c0
0x1409c66d9: mov byte ptr [rbp + 0xdb1], 0
0x1409c66e0: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1409c66e5: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1409c66ea: add rsp, 0x68
0x1409c66ee: pop rbx
0x1409c66ef: pop rdi
0x1409c66f0: pop rsi
0x1409c66f1: pop r12
0x1409c66f3: pop r13
0x1409c66f5: pop r14
0x1409c66f7: pop r15
0x1409c66f9: pop rbp
0x1409c66fa: ret 
