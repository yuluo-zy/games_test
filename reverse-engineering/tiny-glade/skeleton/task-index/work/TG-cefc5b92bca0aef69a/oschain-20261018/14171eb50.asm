14171eb50: sub rsp, 0x38
14171eb54: xorps xmm1, xmm1
14171eb57: ucomiss xmm1, xmm0
14171eb5a: ja 0x14171ec3c
14171eb60: movd r9d, xmm0
14171eb65: mov r8d, r9d
14171eb68: shr r8d, 0x17
14171eb6c: movzx ecx, r8b
14171eb70: xor edx, edx
14171eb72: cmp cx, 0x60
14171eb76: jb 0x14171ebc3
14171eb78: mov eax, r9d
14171eb7b: and eax, 0x7fffff
14171eb80: or eax, 0x800000
14171eb85: cmp cx, 0x7f
14171eb89: jae 0x14171ebca
14171eb8b: mov eax, eax
14171eb8d: lea ecx, [r8 + 0x2a]
14171eb91: shl rax, cl
14171eb94: mov ecx, 0x3b9aca00
14171eb99: mul rcx
14171eb9c: mov rcx, rdx
14171eb9f: mov rdx, rax
14171eba2: neg rdx
14171eba5: setno dl
14171eba8: shr rax, 0x3f
14171ebac: mov r8d, ecx
14171ebaf: or r8b, dl
14171ebb2: and r8b, al
14171ebb5: movzx edx, r8b
14171ebb9: add edx, ecx
14171ebbb: cmp edx, 0x3b9aca00
14171ebc1: je 0x14171ec3c
14171ebc3: xor eax, eax
14171ebc5: add rsp, 0x38
14171ebc9: ret 
14171ebca: movzx ecx, cx
14171ebcd: cmp ecx, 0x96
14171ebd3: jae 0x14171ec26
14171ebd5: lea edx, [r8 + 1]
14171ebd9: mov cl, 0x16
14171ebdb: sub cl, r8b
14171ebde: shr eax, cl
14171ebe0: mov ecx, edx
14171ebe2: shl r9d, cl
14171ebe5: and r9d, 0x7fffff
14171ebec: imul rcx, r9, 0x3b9aca00
14171ebf3: mov r8, rcx
14171ebf6: shr r8, 0x17
14171ebfa: mov edx, ecx
14171ebfc: and edx, 0x7ffe00
14171ec02: cmp edx, 0x400000
14171ec08: setne dl
14171ec0b: shr ecx, 0x16
14171ec0e: and ecx, 1
14171ec11: mov r9d, r8d
14171ec14: or r9b, dl
14171ec17: and r9b, cl
14171ec1a: movzx edx, r9b
14171ec1e: add edx, r8d
14171ec21: add rsp, 0x38
14171ec25: ret 
14171ec26: cmp ecx, 0xbf
14171ec2c: jae 0x14171ec63
14171ec2e: mov eax, eax
14171ec30: lea ecx, [r8 + 0x2a]
14171ec34: shl rax, cl
14171ec37: add rsp, 0x38
14171ec3b: ret 
14171ec3c: mov eax, 0x3b
14171ec41: lea rcx, [rip + 0x15ab498]
14171ec48: mov qword ptr [rsp + 0x28], rcx
14171ec4d: mov qword ptr [rsp + 0x30], rax
14171ec52: lea rdx, [rip + 0x15ab557]
14171ec59: lea rcx, [rsp + 0x28]
14171ec5e: call 0x1428ce090
14171ec63: mov eax, 0x48
14171ec68: lea rcx, [rip + 0x15ab4ac]
14171ec6f: jmp 0x14171ec48
