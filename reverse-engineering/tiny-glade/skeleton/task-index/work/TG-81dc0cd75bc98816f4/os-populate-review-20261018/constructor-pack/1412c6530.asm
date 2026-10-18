1412c6530: push rbp
1412c6531: push rsi
1412c6532: sub rsp, 0xd8
1412c6539: lea rbp, [rsp + 0x80]
1412c6541: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c6549: mov rsi, rcx
1412c654c: lea rcx, [rbp - 0x58]
1412c6550: xor r8d, r8d
1412c6553: call 0x141614d30
1412c6558: movzx eax, byte ptr [rip + 0x236e6aa]
1412c655f: mov ecx, 0x28
1412c6564: mov edx, 8
1412c6569: call 0x140613c10
1412c656e: test rax, rax
1412c6571: je 0x1412c6627
1412c6577: mov rcx, qword ptr [rbp - 0x38]
1412c657b: mov qword ptr [rax + 0x20], rcx
1412c657f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c6583: movups xmm1, xmmword ptr [rbp - 0x48]
1412c6587: movups xmmword ptr [rax + 0x10], xmm1
1412c658b: movups xmmword ptr [rax], xmm0
1412c658e: lea rcx, [rip + 0x17d7d6e]
1412c6595: mov qword ptr [rbp], rcx
1412c6599: mov qword ptr [rbp + 8], 0x4e
1412c65a1: mov qword ptr [rbp + 0x10], rax
1412c65a5: lea rax, [rip + 0x1966e0c]
1412c65ac: mov qword ptr [rbp + 0x18], rax
1412c65b0: mov qword ptr [rbp + 0x30], 0
1412c65b8: mov byte ptr [rbp + 0x46], 5
1412c65bc: mov qword ptr [rbp + 0x20], rcx
1412c65c0: mov qword ptr [rbp + 0x28], 0x4e
1412c65c8: movzx eax, byte ptr [rip + 0x236e63a]
1412c65cf: mov ecx, 0x40
1412c65d4: mov edx, 8
1412c65d9: call 0x140613c10
1412c65de: test rax, rax
1412c65e1: je 0x1412c6638
1412c65e3: movups xmm0, xmmword ptr [rbp + 0x10]
1412c65e7: movups xmm1, xmmword ptr [rbp + 0x20]
1412c65eb: movups xmm2, xmmword ptr [rbp + 0x30]
1412c65ef: movups xmm3, xmmword ptr [rbp + 0x40]
1412c65f3: movups xmmword ptr [rax + 0x30], xmm3
1412c65f7: movups xmmword ptr [rax + 0x20], xmm2
1412c65fb: movups xmmword ptr [rax + 0x10], xmm1
1412c65ff: movups xmmword ptr [rax], xmm0
1412c6602: movabs rdx, 0xf4a562f8a5a7ece7
1412c660c: mov rcx, rsi
1412c660f: mov r8, rax
1412c6612: call 0x140969bf0
1412c6617: cmp rax, 1
1412c661b: je 0x1412c6649
1412c661d: add rsp, 0xd8
1412c6624: pop rsi
1412c6625: pop rbp
1412c6626: ret 
1412c6627: mov ecx, 8
1412c662c: mov edx, 0x28
1412c6631: call 0x1428d9000
1412c6636: jmp 0x1412c6647
1412c6638: mov ecx, 8
1412c663d: mov edx, 0x40
1412c6642: call 0x1428d9000
1412c6647: ud2 
1412c6649: mov rax, rdx
1412c664c: add rdx, 0x10
1412c6650: mov rsi, rdx
1412c6653: cmp qword ptr [rax + 0x18], 0x4e
1412c6658: jne 0x1412c66d6
1412c665a: mov rdx, qword ptr [rsi]
1412c665d: lea rcx, [rip + 0x17d7c9f]
1412c6664: mov r8d, 0x4e
1412c666a: call 0x142923bf0
1412c666f: test eax, eax
1412c6671: jne 0x1412c66d6
1412c6673: lea rax, [rip + 0x19675fe]
1412c667a: mov qword ptr [rbp - 0x30], rax
1412c667e: lea rax, [rip - 0xd92de5]
1412c6685: mov qword ptr [rbp - 0x28], rax
1412c6689: mov rax, rbp
1412c668c: mov qword ptr [rbp - 0x20], rax
1412c6690: lea rax, [rip - 0x11bc807]
1412c6697: mov qword ptr [rbp - 0x18], rax
1412c669b: lea rax, [rip + 0x1966f86]
1412c66a2: mov qword ptr [rbp + 0x10], rax
1412c66a6: mov qword ptr [rbp + 0x18], 3
1412c66ae: mov qword ptr [rbp + 0x30], 0
1412c66b6: lea rax, [rbp - 0x30]
1412c66ba: mov qword ptr [rbp + 0x20], rax
1412c66be: mov qword ptr [rbp + 0x28], 2
1412c66c6: lea rdx, [rip + 0x1966f8b]
1412c66cd: lea rcx, [rbp + 0x10]
1412c66d1: call 0x1428d9390
1412c66d6: lea rax, [rip + 0x196759b]
1412c66dd: mov qword ptr [rbp + 0x10], rax
1412c66e1: lea rax, [rip - 0xd92e48]
1412c66e8: mov qword ptr [rbp + 0x18], rax
1412c66ec: mov qword ptr [rbp + 0x20], rsi
1412c66f0: lea rax, [rip - 0x11bc867]
1412c66f7: mov qword ptr [rbp + 0x28], rax
1412c66fb: mov rcx, rbp
1412c66fe: mov qword ptr [rbp + 0x30], rcx
1412c6702: mov qword ptr [rbp + 0x38], rax
1412c6706: lea rax, [rip + 0x1966e5b]
1412c670d: mov qword ptr [rbp - 0x30], rax
1412c6711: mov qword ptr [rbp - 0x28], 3
1412c6719: mov qword ptr [rbp - 0x10], 0
1412c6721: lea rax, [rbp + 0x10]
1412c6725: mov qword ptr [rbp - 0x20], rax
1412c6729: mov qword ptr [rbp - 0x18], 3
1412c6731: lea rdx, [rip + 0x1966e98]
1412c6738: lea rcx, [rbp - 0x30]
1412c673c: call 0x1428d9390
1412c6741: int3 
1412c6742: nop word ptr cs:[rax + rax]
1412c6750: mov qword ptr [rsp + 0x10], rdx
1412c6755: push rbp
1412c6756: push rsi
1412c6757: sub rsp, 0x28
1412c675b: lea rbp, [rdx + 0x80]
1412c6762: lea rcx, [rbp - 0x58]
1412c6766: call 0x140be24b0
1412c676b: nop 
1412c676c: add rsp, 0x28
1412c6770: pop rsi
1412c6771: pop rbp
1412c6772: ret 
1412c6773: nop word ptr cs:[rax + rax]
1412c6780: mov qword ptr [rsp + 0x10], rdx
1412c6785: push rbp
1412c6786: push rsi
1412c6787: sub rsp, 0x28
1412c678b: lea rbp, [rdx + 0x80]
1412c6792: lea rcx, [rbp + 0x10]
1412c6796: call 0x1400c04b0
1412c679b: nop 
1412c679c: add rsp, 0x28
1412c67a0: pop rsi
1412c67a1: pop rbp
1412c67a2: ret 
