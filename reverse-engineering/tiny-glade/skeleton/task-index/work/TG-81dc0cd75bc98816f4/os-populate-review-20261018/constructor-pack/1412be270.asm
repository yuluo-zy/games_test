1412be270: push rbp
1412be271: push rsi
1412be272: sub rsp, 0xd8
1412be279: lea rbp, [rsp + 0x80]
1412be281: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412be289: mov rsi, rcx
1412be28c: lea rcx, [rbp - 0x58]
1412be290: xor r8d, r8d
1412be293: call 0x141610820
1412be298: movzx eax, byte ptr [rip + 0x237696a]
1412be29f: mov ecx, 0x28
1412be2a4: mov edx, 8
1412be2a9: call 0x140613c10
1412be2ae: test rax, rax
1412be2b1: je 0x1412be367
1412be2b7: mov rcx, qword ptr [rbp - 0x38]
1412be2bb: mov qword ptr [rax + 0x20], rcx
1412be2bf: movups xmm0, xmmword ptr [rbp - 0x58]
1412be2c3: movups xmm1, xmmword ptr [rbp - 0x48]
1412be2c7: movups xmmword ptr [rax + 0x10], xmm1
1412be2cb: movups xmmword ptr [rax], xmm0
1412be2ce: lea rcx, [rip + 0x17dfe8d]
1412be2d5: mov qword ptr [rbp], rcx
1412be2d9: mov qword ptr [rbp + 8], 0x47
1412be2e1: mov qword ptr [rbp + 0x10], rax
1412be2e5: lea rax, [rip + 0x196e56c]
1412be2ec: mov qword ptr [rbp + 0x18], rax
1412be2f0: mov qword ptr [rbp + 0x30], 0
1412be2f8: mov byte ptr [rbp + 0x46], 5
1412be2fc: mov qword ptr [rbp + 0x20], rcx
1412be300: mov qword ptr [rbp + 0x28], 0x47
1412be308: movzx eax, byte ptr [rip + 0x23768fa]
1412be30f: mov ecx, 0x40
1412be314: mov edx, 8
1412be319: call 0x140613c10
1412be31e: test rax, rax
1412be321: je 0x1412be378
1412be323: movups xmm0, xmmword ptr [rbp + 0x10]
1412be327: movups xmm1, xmmword ptr [rbp + 0x20]
1412be32b: movups xmm2, xmmword ptr [rbp + 0x30]
1412be32f: movups xmm3, xmmword ptr [rbp + 0x40]
1412be333: movups xmmword ptr [rax + 0x30], xmm3
1412be337: movups xmmword ptr [rax + 0x20], xmm2
1412be33b: movups xmmword ptr [rax + 0x10], xmm1
1412be33f: movups xmmword ptr [rax], xmm0
1412be342: movabs rdx, 0x324747c4b06c07f2
1412be34c: mov rcx, rsi
1412be34f: mov r8, rax
1412be352: call 0x140969bf0
1412be357: cmp rax, 1
1412be35b: je 0x1412be389
1412be35d: add rsp, 0xd8
1412be364: pop rsi
1412be365: pop rbp
1412be366: ret 
1412be367: mov ecx, 8
1412be36c: mov edx, 0x28
1412be371: call 0x1428d9000
1412be376: jmp 0x1412be387
1412be378: mov ecx, 8
1412be37d: mov edx, 0x40
1412be382: call 0x1428d9000
1412be387: ud2 
1412be389: mov rax, rdx
1412be38c: add rdx, 0x10
1412be390: mov rsi, rdx
1412be393: cmp qword ptr [rax + 0x18], 0x47
1412be398: jne 0x1412be416
1412be39a: mov rdx, qword ptr [rsi]
1412be39d: lea rcx, [rip + 0x17dfdbe]
1412be3a4: mov r8d, 0x47
1412be3aa: call 0x142923bf0
1412be3af: test eax, eax
1412be3b1: jne 0x1412be416
1412be3b3: lea rax, [rip + 0x196fc06]
1412be3ba: mov qword ptr [rbp - 0x30], rax
1412be3be: lea rax, [rip - 0xd8ab25]
1412be3c5: mov qword ptr [rbp - 0x28], rax
1412be3c9: mov rax, rbp
1412be3cc: mov qword ptr [rbp - 0x20], rax
1412be3d0: lea rax, [rip - 0x11b4547]
1412be3d7: mov qword ptr [rbp - 0x18], rax
1412be3db: lea rax, [rip + 0x196f246]
1412be3e2: mov qword ptr [rbp + 0x10], rax
1412be3e6: mov qword ptr [rbp + 0x18], 3
1412be3ee: mov qword ptr [rbp + 0x30], 0
1412be3f6: lea rax, [rbp - 0x30]
1412be3fa: mov qword ptr [rbp + 0x20], rax
1412be3fe: mov qword ptr [rbp + 0x28], 2
1412be406: lea rdx, [rip + 0x196f24b]
1412be40d: lea rcx, [rbp + 0x10]
1412be411: call 0x1428d9390
1412be416: lea rax, [rip + 0x196fba3]
1412be41d: mov qword ptr [rbp + 0x10], rax
1412be421: lea rax, [rip - 0xd8ab88]
1412be428: mov qword ptr [rbp + 0x18], rax
1412be42c: mov qword ptr [rbp + 0x20], rsi
1412be430: lea rax, [rip - 0x11b45a7]
1412be437: mov qword ptr [rbp + 0x28], rax
1412be43b: mov rcx, rbp
1412be43e: mov qword ptr [rbp + 0x30], rcx
1412be442: mov qword ptr [rbp + 0x38], rax
1412be446: lea rax, [rip + 0x196f11b]
1412be44d: mov qword ptr [rbp - 0x30], rax
1412be451: mov qword ptr [rbp - 0x28], 3
1412be459: mov qword ptr [rbp - 0x10], 0
1412be461: lea rax, [rbp + 0x10]
1412be465: mov qword ptr [rbp - 0x20], rax
1412be469: mov qword ptr [rbp - 0x18], 3
1412be471: lea rdx, [rip + 0x196f158]
1412be478: lea rcx, [rbp - 0x30]
1412be47c: call 0x1428d9390
1412be481: int3 
1412be482: nop word ptr cs:[rax + rax]
1412be490: mov qword ptr [rsp + 0x10], rdx
1412be495: push rbp
1412be496: push rsi
1412be497: sub rsp, 0x28
1412be49b: lea rbp, [rdx + 0x80]
1412be4a2: lea rcx, [rbp - 0x58]
1412be4a6: call 0x140be24b0
1412be4ab: nop 
1412be4ac: add rsp, 0x28
1412be4b0: pop rsi
1412be4b1: pop rbp
1412be4b2: ret 
1412be4b3: nop word ptr cs:[rax + rax]
1412be4c0: mov qword ptr [rsp + 0x10], rdx
1412be4c5: push rbp
1412be4c6: push rsi
1412be4c7: sub rsp, 0x28
1412be4cb: lea rbp, [rdx + 0x80]
1412be4d2: lea rcx, [rbp + 0x10]
1412be4d6: call 0x1400c04b0
1412be4db: nop 
1412be4dc: add rsp, 0x28
1412be4e0: pop rsi
1412be4e1: pop rbp
1412be4e2: ret 
