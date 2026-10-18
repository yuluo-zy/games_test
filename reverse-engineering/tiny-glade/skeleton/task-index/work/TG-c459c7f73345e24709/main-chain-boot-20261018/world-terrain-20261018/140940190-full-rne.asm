140940190: push rbp
140940191: push rsi
140940192: push rdi
140940193: push rbx
140940194: mov eax, 0x28d8
140940199: call 0x14289aa40
14094019e: sub rsp, rax
1409401a1: lea rbp, [rsp + 0x80]
1409401a9: movaps xmmword ptr [rbp + 0x2840], xmm6
1409401b0: mov qword ptr [rbp + 0x2838], 0xfffffffffffffffe
1409401bb: mov rsi, r8
1409401be: mov rbx, rdx
1409401c1: mov rdi, rcx
1409401c4: lea rcx, [rbp + 0xbb8]
1409401cb: call 0x140936c60
1409401d0: mov byte ptr [rbp + 0x2834], 1
1409401d7: lea rcx, [rbp + 0xc98]
1409401de: mov rdx, rbx
1409401e1: call 0x14093bb10
1409401e6: movups xmm0, xmmword ptr [rip + 0x2175973]
1409401ed: movups xmmword ptr [rbp + 0xd67], xmm0
1409401f4: movups xmm0, xmmword ptr [rip + 0x2175975]
1409401fb: movups xmmword ptr [rbp + 0xd77], xmm0
140940202: mov qword ptr [rbp + 0x2800], 0
14094020d: mov byte ptr [rbp + 0x2808], 0
140940214: movups xmm0, xmmword ptr [rbp + 0xd60]
14094021b: movups xmmword ptr [rbp + 0x2809], xmm0
140940222: movups xmm0, xmmword ptr [rbp + 0xd70]
140940229: movups xmmword ptr [rbp + 0x2819], xmm0
140940230: mov rax, qword ptr [rbp + 0xd7f]
140940237: mov qword ptr [rbp + 0x2828], rax
14094023e: mov qword ptr [rbp + 0xe00], 0
140940249: mov dword ptr [rbp + 0xe08], 0
140940253: mov qword ptr [rbp + 0xe10], 0
14094025e: mov dword ptr [rbp + 0xe18], 0
140940268: mov qword ptr [rbp + 0xe20], 0
140940273: mov dword ptr [rbp + 0xe28], 0
14094027d: xorps xmm6, xmm6
140940280: movaps xmmword ptr [rbp + 0xe40], xmm6
140940287: lea rcx, [rbp + 0xe50]
14094028e: mov byte ptr [rbp + 0x2837], 1
140940295: mov byte ptr [rbp + 0x2836], 1
14094029c: mov byte ptr [rbp + 0x2835], 1
1409402a3: call 0x14173c850
1409402a8: mov dword ptr [rbp + 0xe0c], 0
1409402b2: mov dword ptr [rbp + 0xe1c], 0
1409402bc: movups xmmword ptr [rbp + 0xe2c], xmm6
1409402c3: mov dword ptr [rbp + 0xe3c], 0
1409402cd: movaps xmmword ptr [rbp + 0xd60], xmm6
1409402d4: movaps xmmword ptr [rbp + 0xd70], xmm6
1409402db: movaps xmmword ptr [rbp + 0xd80], xmm6
1409402e2: movaps xmmword ptr [rbp + 0xd90], xmm6
1409402e9: movaps xmmword ptr [rbp + 0xda0], xmm6
1409402f0: movaps xmmword ptr [rbp + 0xdb0], xmm6
1409402f7: movaps xmmword ptr [rbp + 0xdc0], xmm6
1409402fe: movaps xmmword ptr [rbp + 0xdd0], xmm6
140940305: movaps xmmword ptr [rbp + 0xde0], xmm6
14094030c: movaps xmmword ptr [rbp + 0xdf0], xmm6
140940313: mov byte ptr [rbp + 0x2837], 1
14094031a: mov byte ptr [rbp + 0x2836], 1
140940321: mov byte ptr [rbp + 0x2835], 1
140940328: lea rdx, [rbp + 0xd60]
14094032f: mov rcx, rdi
140940332: call 0x141541a30
140940337: mov rdi, rax
14094033a: mov byte ptr [rbp + 0x2837], 1
140940341: mov byte ptr [rbp + 0x2836], 1
140940348: mov byte ptr [rbp + 0x2835], 1
14094034f: xor ecx, ecx
140940351: call 0x1400d9b20
140940356: test rax, rax
140940359: je 0x140940c20
14094035f: movups xmm0, xmmword ptr [rax]
140940362: inc qword ptr [rax]
140940365: movups xmm1, xmmword ptr [rip + 0x216e0c4]
14094036c: movups xmmword ptr [rbp + 0xd78], xmm1
140940373: movups xmm1, xmmword ptr [rip + 0x216e0a6]
14094037a: movups xmmword ptr [rbp + 0xd68], xmm1
140940381: movups xmmword ptr [rbp + 0xd88], xmm0
140940388: mov qword ptr [rbp + 0xd98], 1
140940393: mov dword ptr [rbp + 0xd60], 0
14094039d: mov byte ptr [rbp + 0xda0], 0
1409403a4: mov byte ptr [rbp + 0x2837], 1
1409403ab: mov byte ptr [rbp + 0x2836], 1
1409403b2: mov byte ptr [rbp + 0x2835], 1
1409403b9: lea rdx, [rbp + 0xd60]
1409403c0: mov rcx, rdi
1409403c3: call 0x141532740
1409403c8: mov rdi, rax
1409403cb: movups xmmword ptr [rbp + 0xd80], xmm6
1409403d2: lea rcx, [rbp + 0x9b8]
1409403d9: mov r8d, 0x200
1409403df: xor edx, edx
1409403e1: call 0x142923c20
1409403e6: lea rcx, [rbp + 0xfd0]
1409403ed: mov r8d, 0x200
1409403f3: xor edx, edx
1409403f5: call 0x142923c20
1409403fa: movups xmmword ptr [rbp + 0x11f0], xmm6
140940401: lea rcx, [rbp + 0x7b4]
140940408: mov r8d, 0x200
14094040e: xor edx, edx
140940410: call 0x142923c20
140940415: lea rcx, [rbp + 0x1440]
14094041c: mov r8d, 0x200
140940422: xor edx, edx
140940424: call 0x142923c20
140940429: movups xmmword ptr [rbp + 0x1660], xmm6
140940430: lea rcx, [rbp + 0x5b0]
140940437: mov r8d, 0x200
14094043d: xor edx, edx
14094043f: call 0x142923c20
140940444: lea rcx, [rbp + 0x18b0]
14094044b: mov r8d, 0x200
140940451: xor edx, edx
140940453: call 0x142923c20
140940458: movups xmmword ptr [rbp + 0x1ad0], xmm6
14094045f: lea rcx, [rbp + 0x3ac]
140940466: mov r8d, 0x200
14094046c: xor edx, edx
14094046e: call 0x142923c20
140940473: lea rcx, [rbp + 0x1d20]
14094047a: mov r8d, 0x200
140940480: xor edx, edx
140940482: call 0x142923c20
140940487: movups xmmword ptr [rbp + 0x1f40], xmm6
14094048e: lea rcx, [rbp + 0x1a8]
140940495: mov r8d, 0x200
14094049b: xor edx, edx
14094049d: call 0x142923c20
1409404a2: lea rcx, [rbp + 0x2190]
1409404a9: mov r8d, 0x200
1409404af: xor edx, edx
1409404b1: call 0x142923c20
1409404b6: movups xmmword ptr [rbp + 0x23b0], xmm6
1409404bd: lea rcx, [rbp - 0x5c]
1409404c1: mov r8d, 0x200
1409404c7: xor edx, edx
1409404c9: call 0x142923c20
1409404ce: lea rcx, [rbp + 0x2600]
1409404d5: mov r8d, 0x200
1409404db: xor edx, edx
1409404dd: call 0x142923c20
1409404e2: mov qword ptr [rbp + 0xd60], 0
1409404ed: mov qword ptr [rbp + 0xd70], 0
1409404f8: mov qword ptr [rbp + 0xd78], 4
140940503: mov qword ptr [rbp + 0xd90], 4
14094050e: mov qword ptr [rbp + 0xd98], 0
140940519: mov dword ptr [rbp + 0xda0], 0
140940523: mov qword ptr [rbp + 0xda8], 0
14094052e: mov qword ptr [rbp + 0xdb0], 4
140940539: mov qword ptr [rbp + 0xdb8], 0
140940544: mov dword ptr [rbp + 0xdc0], 0x3f4ccccd
14094054e: mov byte ptr [rbp + 0xdc4], 0
140940555: mov dword ptr [rbp + 0xdc8], 0
14094055f: lea rcx, [rbp + 0xdcc]
140940566: lea rdx, [rbp + 0x9b4]
14094056d: mov r8d, 0x204
140940573: call 0x142923c00
140940578: mov qword ptr [rbp + 0x11d0], 0
140940583: mov qword ptr [rbp + 0x11e0], 0
14094058e: mov qword ptr [rbp + 0x11e8], 4
140940599: mov qword ptr [rbp + 0x1200], 4
1409405a4: mov qword ptr [rbp + 0x1208], 0
1409405af: mov dword ptr [rbp + 0x1210], 0
1409405b9: mov qword ptr [rbp + 0x1218], 0
1409405c4: mov qword ptr [rbp + 0x1220], 4
1409405cf: mov qword ptr [rbp + 0x1228], 0
1409405da: mov dword ptr [rbp + 0x1230], 0x3f4ccccd
1409405e4: mov byte ptr [rbp + 0x1234], 0
1409405eb: mov dword ptr [rbp + 0x1238], 0
1409405f5: lea rcx, [rbp + 0x123c]
1409405fc: lea rdx, [rbp + 0x7b0]
140940603: mov r8d, 0x204
140940609: call 0x142923c00
14094060e: mov qword ptr [rbp + 0x1640], 0
140940619: mov qword ptr [rbp + 0x1650], 0
140940624: mov qword ptr [rbp + 0x1658], 4
14094062f: mov qword ptr [rbp + 0x1670], 4
14094063a: mov qword ptr [rbp + 0x1678], 0
140940645: mov dword ptr [rbp + 0x1680], 0
14094064f: mov qword ptr [rbp + 0x1688], 0
14094065a: mov qword ptr [rbp + 0x1690], 4
140940665: mov qword ptr [rbp + 0x1698], 0
140940670: mov dword ptr [rbp + 0x16a0], 0x3f4ccccd
14094067a: mov byte ptr [rbp + 0x16a4], 0
140940681: mov dword ptr [rbp + 0x16a8], 0
14094068b: lea rcx, [rbp + 0x16ac]
140940692: lea rdx, [rbp + 0x5ac]
140940699: mov r8d, 0x204
14094069f: call 0x142923c00
1409406a4: mov qword ptr [rbp + 0x1ab0], 0
1409406af: mov qword ptr [rbp + 0x1ac0], 0
1409406ba: mov qword ptr [rbp + 0x1ac8], 4
1409406c5: mov qword ptr [rbp + 0x1ae0], 4
1409406d0: mov qword ptr [rbp + 0x1ae8], 0
1409406db: mov dword ptr [rbp + 0x1af0], 0
1409406e5: mov qword ptr [rbp + 0x1af8], 0
1409406f0: mov qword ptr [rbp + 0x1b00], 4
1409406fb: mov qword ptr [rbp + 0x1b08], 0
140940706: mov dword ptr [rbp + 0x1b10], 0x3f4ccccd
140940710: mov byte ptr [rbp + 0x1b14], 0
140940717: mov dword ptr [rbp + 0x1b18], 0
140940721: lea rcx, [rbp + 0x1b1c]
140940728: lea rdx, [rbp + 0x3a8]
14094072f: mov r8d, 0x204
140940735: call 0x142923c00
14094073a: mov qword ptr [rbp + 0x1f20], 0
140940745: mov qword ptr [rbp + 0x1f30], 0
140940750: mov qword ptr [rbp + 0x1f38], 4
14094075b: mov qword ptr [rbp + 0x1f50], 4
140940766: mov qword ptr [rbp + 0x1f58], 0
140940771: mov dword ptr [rbp + 0x1f60], 0
14094077b: mov qword ptr [rbp + 0x1f68], 0
140940786: mov qword ptr [rbp + 0x1f70], 4
140940791: mov qword ptr [rbp + 0x1f78], 0
14094079c: mov dword ptr [rbp + 0x1f80], 0x3f4ccccd
1409407a6: mov byte ptr [rbp + 0x1f84], 0
1409407ad: mov dword ptr [rbp + 0x1f88], 0
1409407b7: lea rcx, [rbp + 0x1f8c]
1409407be: lea rdx, [rbp + 0x1a4]
1409407c5: mov r8d, 0x204
1409407cb: call 0x142923c00
1409407d0: mov qword ptr [rbp + 0x2390], 0
1409407db: mov qword ptr [rbp + 0x23a0], 0
1409407e6: mov qword ptr [rbp + 0x23a8], 4
1409407f1: mov qword ptr [rbp + 0x23c0], 4
1409407fc: mov qword ptr [rbp + 0x23c8], 0
140940807: mov dword ptr [rbp + 0x23d0], 0
140940811: mov qword ptr [rbp + 0x23d8], 0
14094081c: mov qword ptr [rbp + 0x23e0], 4
140940827: mov qword ptr [rbp + 0x23e8], 0
140940832: mov dword ptr [rbp + 0x23f0], 0x3f4ccccd
14094083c: mov byte ptr [rbp + 0x23f4], 0
140940843: mov dword ptr [rbp + 0x23f8], 0
14094084d: lea rcx, [rbp + 0x23fc]
140940854: lea rdx, [rbp - 0x60]
140940858: mov r8d, 0x204
14094085e: call 0x142923c00
140940863: mov byte ptr [rbp + 0x2837], 1
14094086a: mov byte ptr [rbp + 0x2836], 1
140940871: mov byte ptr [rbp + 0x2835], 1
140940878: lea rdx, [rbp + 0xd60]
14094087f: mov rcx, rdi
140940882: call 0x14153fdb0
140940887: mov rdi, rax
14094088a: mov byte ptr [rbp + 0x2837], 1
140940891: mov byte ptr [rbp + 0x2836], 1
140940898: mov byte ptr [rbp + 0x2835], 1
14094089f: lea rcx, [rbp + 0xd60]
1409408a6: call 0x14092c920
1409408ab: mov byte ptr [rbp + 0x2837], 1
1409408b2: mov byte ptr [rbp + 0x2836], 1
1409408b9: mov byte ptr [rbp + 0x2835], 1
1409408c0: lea rdx, [rbp + 0xd60]
1409408c7: mov rcx, rdi
1409408ca: call 0x141534a80
1409408cf: mov rdi, rax
1409408d2: mov byte ptr [rbp + 0x2837], 1
1409408d9: mov byte ptr [rbp + 0x2836], 1
1409408e0: mov byte ptr [rbp + 0x2835], 1
1409408e7: lea rcx, [rbp + 0xd60]
1409408ee: call 0x140924030
1409408f3: mov byte ptr [rbp + 0x2837], 1
1409408fa: mov byte ptr [rbp + 0x2836], 1
140940901: mov byte ptr [rbp + 0x2835], 1
140940908: lea rdx, [rbp + 0xd60]
14094090f: mov rcx, rdi
140940912: call 0x141548550
140940917: mov rdi, rax
14094091a: mov byte ptr [rbp + 0x2837], 1
140940921: mov byte ptr [rbp + 0x2836], 1
140940928: mov byte ptr [rbp + 0x2835], 1
14094092f: lea rcx, [rbp + 0xd60]
140940936: call 0x140867700
14094093b: mov byte ptr [rbp + 0x2837], 1
140940942: mov byte ptr [rbp + 0x2836], 1
140940949: mov byte ptr [rbp + 0x2835], 1
140940950: lea rdx, [rbp + 0xd60]
140940957: mov rcx, rdi
14094095a: call 0x14153bfc0
14094095f: movups xmm0, xmmword ptr [rbp + 0x2800]
140940966: movups xmm1, xmmword ptr [rbp + 0x2810]
14094096d: movups xmm2, xmmword ptr [rbp + 0x2820]
140940974: movaps xmmword ptr [rbp + 0xd80], xmm2
14094097b: movaps xmmword ptr [rbp + 0xd70], xmm1
140940982: movaps xmmword ptr [rbp + 0xd60], xmm0
140940989: mov byte ptr [rbp + 0x2837], 1
140940990: mov byte ptr [rbp + 0x2836], 1
140940997: mov byte ptr [rbp + 0x2835], 0
14094099e: lea rdx, [rbp + 0xd60]
1409409a5: mov rcx, rax
1409409a8: call 0x141542f70
1409409ad: mov byte ptr [rbp + 0x2837], 1
1409409b4: mov byte ptr [rbp + 0x2836], 1
1409409bb: mov byte ptr [rbp + 0x2835], 0
1409409c2: mov rcx, rax
1409409c5: call 0x141536b40
1409409ca: mov rdi, rax
1409409cd: mov bl, byte ptr [rsi + 0xe7]
1409409d3: mov byte ptr [rbp + 0x2837], 1
1409409da: mov byte ptr [rbp + 0x2836], 1
1409409e1: mov byte ptr [rbp + 0x2835], 0
1409409e8: lea rcx, [rbp + 0xd60]
1409409ef: mov edx, ebx
1409409f1: call 0x1408ba490
1409409f6: mov byte ptr [rbp + 0x2837], 1
1409409fd: mov byte ptr [rbp + 0x2836], 1
140940a04: mov byte ptr [rbp + 0x2835], 0
140940a0b: lea rdx, [rbp + 0xd60]
140940a12: mov rcx, rdi
140940a15: call 0x141534490
140940a1a: mov rsi, rax
140940a1d: mov byte ptr [rbp + 0x2837], 1
140940a24: mov byte ptr [rbp + 0x2836], 1
140940a2b: mov byte ptr [rbp + 0x2835], 0
140940a32: lea rcx, [rbp + 0xd60]
140940a39: mov edx, ebx
140940a3b: call 0x1408bab30
140940a40: mov byte ptr [rbp + 0x2837], 1
140940a47: mov byte ptr [rbp + 0x2836], 1
140940a4e: mov byte ptr [rbp + 0x2835], 0
140940a55: lea rdx, [rbp + 0xd60]
140940a5c: mov rcx, rsi
140940a5f: call 0x14154f700
140940a64: mov rsi, rax
140940a67: lea rdi, [rbp + 0xd60]
140940a6e: lea rdx, [rbp + 0xbb8]
140940a75: mov r8d, 0xe0
140940a7b: mov rcx, rdi
140940a7e: call 0x142923c00
140940a83: mov byte ptr [rbp + 0x2837], 0
140940a8a: mov byte ptr [rbp + 0x2836], 1
140940a91: mov byte ptr [rbp + 0x2835], 0
140940a98: mov rcx, rsi
140940a9b: mov rdx, rdi
140940a9e: call 0x141546e80
140940aa3: mov rsi, rax
140940aa6: lea rdx, [rbp + 0xc98]
140940aad: mov r8d, 0xc8
140940ab3: mov rcx, rdi
140940ab6: call 0x142923c00
140940abb: mov byte ptr [rbp + 0x2837], 0
140940ac2: mov byte ptr [rbp + 0x2836], 0
140940ac9: mov byte ptr [rbp + 0x2835], 0
140940ad0: mov rcx, rsi
140940ad3: mov rdx, rdi
140940ad6: call 0x14154f130
140940adb: mov byte ptr [rbp + 0x2837], 0
140940ae2: mov byte ptr [rbp + 0x2836], 0
140940ae9: mov byte ptr [rbp + 0x2835], 0
140940af0: mov rcx, rax
140940af3: xor edx, edx
140940af5: call 0x141530aa0
140940afa: mov qword ptr [rbp + 0xd60], 0
140940b05: mov qword ptr [rbp + 0xd70], 0
140940b10: mov qword ptr [rbp + 0xd80], 0
140940b1b: mov qword ptr [rbp + 0xd90], 0
140940b26: mov qword ptr [rbp + 0xda0], 0
140940b31: mov byte ptr [rbp + 0x2837], 0
140940b38: mov byte ptr [rbp + 0x2836], 0
140940b3f: mov byte ptr [rbp + 0x2835], 0
140940b46: lea rdx, [rbp + 0xd60]
140940b4d: mov rcx, rax
140940b50: call 0x141544720
140940b55: mov rsi, rax
140940b58: call 0x140d4d740
140940b5d: mov rdi, rax
140940b60: test byte ptr [rax], 1
140940b63: je 0x140940b76
140940b65: mov rcx, rdi
140940b68: add rcx, 8
140940b6c: mov rax, qword ptr [rdi + 8]
140940b70: mov rdx, qword ptr [rdi + 0x10]
140940b74: jmp 0x140940ba6
140940b76: mov byte ptr [rbp + 0x2837], 0
140940b7d: mov byte ptr [rbp + 0x2836], 0
140940b84: mov byte ptr [rbp + 0x2835], 0
140940b8b: call 0x140d64e00
140940b90: mov rcx, rdi
140940b93: add rcx, 8
140940b97: mov qword ptr [rdi], 1
140940b9e: mov qword ptr [rdi + 8], rax
140940ba2: mov qword ptr [rdi + 0x10], rdx
140940ba6: lea r8, [rax + 1]
140940baa: mov qword ptr [rcx], r8
140940bad: movups xmm0, xmmword ptr [rip + 0x217194c]
140940bb4: movaps xmmword ptr [rbp + 0xd70], xmm0
140940bbb: movups xmm0, xmmword ptr [rip + 0x217192e]
140940bc2: movaps xmmword ptr [rbp + 0xd60], xmm0
140940bc9: mov qword ptr [rbp + 0xd80], rax
140940bd0: mov qword ptr [rbp + 0xd88], rdx
140940bd7: mov byte ptr [rbp + 0xd98], 0
140940bde: mov qword ptr [rbp + 0xd90], 0
140940be9: mov byte ptr [rbp + 0x2837], 0
140940bf0: mov byte ptr [rbp + 0x2836], 0
140940bf7: mov byte ptr [rbp + 0x2835], 0
140940bfe: lea rdx, [rbp + 0xd60]
140940c05: mov rcx, rsi
140940c08: call 0x141533320
140940c0d: movaps xmm6, xmmword ptr [rbp + 0x2840]
140940c14: add rsp, 0x28d8
140940c1b: pop rbx
140940c1c: pop rdi
140940c1d: pop rsi
140940c1e: pop rbp
140940c1f: ret 
140940c20: mov byte ptr [rbp + 0x2837], 1
140940c27: mov byte ptr [rbp + 0x2836], 1
140940c2e: mov byte ptr [rbp + 0x2835], 1
140940c35: lea rcx, [rip + 0x22f9e64]
140940c3c: call 0x1428d80d0
140940c41: ud2 
140940c43: nop word ptr cs:[rax + rax]
140940c50: mov qword ptr [rsp + 0x10], rdx
140940c55: push rbp
140940c56: push rsi
140940c57: push rdi
140940c58: push rbx
140940c59: sub rsp, 0x38
140940c5d: lea rbp, [rdx + 0x80]
140940c64: movaps xmmword ptr [rsp + 0x20], xmm6
140940c69: movzx eax, byte ptr [rbp + 0x2836]
140940c70: mov byte ptr [rbp + 0x2832], al
140940c76: movzx eax, byte ptr [rbp + 0x2837]
140940c7d: mov byte ptr [rbp + 0x2833], al
140940c83: cmp byte ptr [rbp + 0x2835], 0
140940c8a: je 0x140940cb9
140940c8c: lea rcx, [rbp + 0x2810]
140940c93: lea rdx, [rbp + 0x2830]
140940c9a: mov r8d, 0x10
140940ca0: mov r9d, 0x10
140940ca6: call 0x14004d3e0
140940cab: movaps xmm6, xmmword ptr [rsp + 0x20]
140940cb0: add rsp, 0x38
140940cb4: pop rbx
140940cb5: pop rdi
140940cb6: pop rsi
140940cb7: pop rbp
140940cb8: ret 
140940cb9: movaps xmm6, xmmword ptr [rsp + 0x20]
140940cbe: add rsp, 0x38
140940cc2: pop rbx
140940cc3: pop rdi
140940cc4: pop rsi
140940cc5: pop rbp
140940cc6: ret 
140940cc7: nop word ptr [rax + rax]
140940cd0: mov qword ptr [rsp + 0x10], rdx
140940cd5: push rbp
140940cd6: push rsi
140940cd7: push rdi
140940cd8: push rbx
140940cd9: sub rsp, 0x38
140940cdd: lea rbp, [rdx + 0x80]
140940ce4: movaps xmmword ptr [rsp + 0x20], xmm6
140940ce9: test byte ptr [rbp + 0x2832], 1
140940cf0: je 0x140940d1b
140940cf2: lea rcx, [rbp + 0xc98]
140940cf9: call 0x140608640
140940cfe: movzx eax, byte ptr [rbp + 0x2833]
140940d05: and al, 1
140940d07: mov byte ptr [rbp + 0x2834], al
140940d0d: movaps xmm6, xmmword ptr [rsp + 0x20]
140940d12: add rsp, 0x38
140940d16: pop rbx
140940d17: pop rdi
140940d18: pop rsi
140940d19: pop rbp
140940d1a: ret 
140940d1b: movzx eax, byte ptr [rbp + 0x2833]
140940d22: and al, 1
140940d24: mov byte ptr [rbp + 0x2834], al
140940d2a: movaps xmm6, xmmword ptr [rsp + 0x20]
140940d2f: add rsp, 0x38
140940d33: pop rbx
140940d34: pop rdi
140940d35: pop rsi
140940d36: pop rbp
140940d37: ret 
140940d38: nop dword ptr [rax + rax]
140940d40: mov qword ptr [rsp + 0x10], rdx
140940d45: push rbp
140940d46: push rsi
140940d47: push rdi
140940d48: push rbx
140940d49: sub rsp, 0x38
140940d4d: lea rbp, [rdx + 0x80]
140940d54: movaps xmmword ptr [rsp + 0x20], xmm6
140940d59: cmp byte ptr [rbp + 0x2834], 0
140940d60: jne 0x140940d70
140940d62: movaps xmm6, xmmword ptr [rsp + 0x20]
140940d67: add rsp, 0x38
140940d6b: pop rbx
140940d6c: pop rdi
140940d6d: pop rsi
140940d6e: pop rbp
140940d6f: ret 
140940d70: lea rcx, [rbp + 0xbb8]
140940d77: call 0x140606cc0
140940d7c: jmp 0x140940d62
