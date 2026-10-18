1412c4700: push rbp
1412c4701: push rsi
1412c4702: sub rsp, 0xd8
1412c4709: lea rbp, [rsp + 0x80]
1412c4711: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c4719: mov rsi, rcx
1412c471c: lea rcx, [rbp - 0x58]
1412c4720: xor r8d, r8d
1412c4723: call 0x14161a1f0
1412c4728: mov qword ptr [rbp - 0x60], 0
1412c4730: movzx eax, byte ptr [rip + 0x23704d2]
1412c4737: mov ecx, 0x30
1412c473c: mov edx, 8
1412c4741: call 0x140613c10
1412c4746: test rax, rax
1412c4749: je 0x1412c47ff
1412c474f: movups xmm0, xmmword ptr [rbp - 0x60]
1412c4753: movups xmm1, xmmword ptr [rbp - 0x50]
1412c4757: movups xmm2, xmmword ptr [rbp - 0x40]
1412c475b: movups xmmword ptr [rax + 0x20], xmm2
1412c475f: movups xmmword ptr [rax + 0x10], xmm1
1412c4763: movups xmmword ptr [rax], xmm0
1412c4766: lea rcx, [rip + 0x17da53a]
1412c476d: mov qword ptr [rbp], rcx
1412c4771: mov qword ptr [rbp + 8], 0x54
1412c4779: mov qword ptr [rbp + 0x10], rax
1412c477d: lea rax, [rip + 0x1968994]
1412c4784: mov qword ptr [rbp + 0x18], rax
1412c4788: mov qword ptr [rbp + 0x30], 0
1412c4790: mov byte ptr [rbp + 0x46], 5
1412c4794: mov qword ptr [rbp + 0x20], rcx
1412c4798: mov qword ptr [rbp + 0x28], 0x54
1412c47a0: movzx eax, byte ptr [rip + 0x2370462]
1412c47a7: mov ecx, 0x40
1412c47ac: mov edx, 8
1412c47b1: call 0x140613c10
1412c47b6: test rax, rax
1412c47b9: je 0x1412c4810
1412c47bb: movups xmm0, xmmword ptr [rbp + 0x10]
1412c47bf: movups xmm1, xmmword ptr [rbp + 0x20]
1412c47c3: movups xmm2, xmmword ptr [rbp + 0x30]
1412c47c7: movups xmm3, xmmword ptr [rbp + 0x40]
1412c47cb: movups xmmword ptr [rax + 0x30], xmm3
1412c47cf: movups xmmword ptr [rax + 0x20], xmm2
1412c47d3: movups xmmword ptr [rax + 0x10], xmm1
1412c47d7: movups xmmword ptr [rax], xmm0
1412c47da: movabs rdx, 0xae37cbcafbe88da4
1412c47e4: mov rcx, rsi
1412c47e7: mov r8, rax
1412c47ea: call 0x140969bf0
1412c47ef: cmp rax, 1
1412c47f3: je 0x1412c4821
1412c47f5: add rsp, 0xd8
1412c47fc: pop rsi
1412c47fd: pop rbp
1412c47fe: ret 
1412c47ff: mov ecx, 8
1412c4804: mov edx, 0x30
1412c4809: call 0x1428d9000
1412c480e: jmp 0x1412c481f
1412c4810: mov ecx, 8
1412c4815: mov edx, 0x40
1412c481a: call 0x1428d9000
1412c481f: ud2 
1412c4821: mov rax, rdx
1412c4824: add rdx, 0x10
1412c4828: mov rsi, rdx
1412c482b: cmp qword ptr [rax + 0x18], 0x54
1412c4830: jne 0x1412c48ae
1412c4832: mov rdx, qword ptr [rsi]
1412c4835: lea rcx, [rip + 0x17da46b]
1412c483c: mov r8d, 0x54
1412c4842: call 0x142923bf0
1412c4847: test eax, eax
1412c4849: jne 0x1412c48ae
1412c484b: lea rax, [rip + 0x1968e96]
1412c4852: mov qword ptr [rbp - 0x30], rax
1412c4856: lea rax, [rip - 0xd90fbd]
1412c485d: mov qword ptr [rbp - 0x28], rax
1412c4861: mov rax, rbp
1412c4864: mov qword ptr [rbp - 0x20], rax
1412c4868: lea rax, [rip - 0x11ba9df]
1412c486f: mov qword ptr [rbp - 0x18], rax
1412c4873: lea rax, [rip + 0x1968dae]
1412c487a: mov qword ptr [rbp + 0x10], rax
1412c487e: mov qword ptr [rbp + 0x18], 3
1412c4886: mov qword ptr [rbp + 0x30], 0
1412c488e: lea rax, [rbp - 0x30]
1412c4892: mov qword ptr [rbp + 0x20], rax
1412c4896: mov qword ptr [rbp + 0x28], 2
1412c489e: lea rdx, [rip + 0x1968db3]
1412c48a5: lea rcx, [rbp + 0x10]
1412c48a9: call 0x1428d9390
1412c48ae: lea rax, [rip + 0x1968e33]
1412c48b5: mov qword ptr [rbp + 0x10], rax
1412c48b9: lea rax, [rip - 0xd91020]
1412c48c0: mov qword ptr [rbp + 0x18], rax
1412c48c4: mov qword ptr [rbp + 0x20], rsi
1412c48c8: lea rax, [rip - 0x11baa3f]
1412c48cf: mov qword ptr [rbp + 0x28], rax
1412c48d3: mov rcx, rbp
1412c48d6: mov qword ptr [rbp + 0x30], rcx
1412c48da: mov qword ptr [rbp + 0x38], rax
1412c48de: lea rax, [rip + 0x1968c83]
1412c48e5: mov qword ptr [rbp - 0x30], rax
1412c48e9: mov qword ptr [rbp - 0x28], 3
1412c48f1: mov qword ptr [rbp - 0x10], 0
1412c48f9: lea rax, [rbp + 0x10]
1412c48fd: mov qword ptr [rbp - 0x20], rax
1412c4901: mov qword ptr [rbp - 0x18], 3
1412c4909: lea rdx, [rip + 0x1968cc0]
1412c4910: lea rcx, [rbp - 0x30]
1412c4914: call 0x1428d9390
1412c4919: int3 
1412c491a: nop word ptr [rax + rax]
1412c4920: mov qword ptr [rsp + 0x10], rdx
1412c4925: push rbp
1412c4926: push rsi
1412c4927: sub rsp, 0x28
1412c492b: lea rbp, [rdx + 0x80]
1412c4932: lea rcx, [rbp - 0x58]
1412c4936: call 0x140be24b0
1412c493b: nop 
1412c493c: add rsp, 0x28
1412c4940: pop rsi
1412c4941: pop rbp
1412c4942: ret 
1412c4943: nop word ptr cs:[rax + rax]
1412c4950: mov qword ptr [rsp + 0x10], rdx
1412c4955: push rbp
1412c4956: push rsi
1412c4957: sub rsp, 0x28
1412c495b: lea rbp, [rdx + 0x80]
1412c4962: lea rcx, [rbp + 0x10]
1412c4966: call 0x1400c04b0
1412c496b: nop 
1412c496c: add rsp, 0x28
1412c4970: pop rsi
1412c4971: pop rbp
1412c4972: ret 
