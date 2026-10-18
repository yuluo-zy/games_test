140a83300: push rbp
140a83301: push r15
140a83303: push r14
140a83305: push r13
140a83307: push r12
140a83309: push rsi
140a8330a: push rdi
140a8330b: push rbx
140a8330c: sub rsp, 0x438
140a83313: lea rbp, [rsp + 0x80]
140a8331b: movaps xmmword ptr [rbp + 0x3a0], xmm7
140a83322: movaps xmmword ptr [rbp + 0x390], xmm6
140a83329: mov qword ptr [rbp + 0x388], 0xfffffffffffffffe
140a83334: movss xmm1, dword ptr [r8 + 0x30]
140a8333a: xorps xmm0, xmm0
140a8333d: ucomiss xmm1, xmm0
140a83340: mov qword ptr [rbp + 0x350], r8
140a83347: jne 0x140a8334f
140a83349: jnp 0x140a8393c
140a8334f: movss xmm2, dword ptr [rip + 0x1f49d11]
140a83357: divss xmm2, xmm1
140a8335b: xorps xmm3, xmm3
140a8335e: maxss xmm3, xmm2
140a83362: movss xmm1, dword ptr [rip + 0x1ea259a]
140a8336a: movaps xmm6, xmm1
140a8336d: minss xmm6, xmm3
140a83371: movss dword ptr [rbp + 0xb0], xmm6
140a83379: ucomiss xmm6, xmm0
140a8337c: jb 0x140a83960
140a83382: ucomiss xmm1, xmm6
140a83385: jb 0x140a83960
140a8338b: ucomiss xmm6, xmm6
140a8338e: jp 0x140a839cf
140a83394: mov edi, r9d
140a83397: mov r13, rdx
140a8339a: mov qword ptr [rbp + 0x348], rcx
140a833a1: mov byte ptr [rbp + 0x387], 1
140a833a8: lea rax, [rip + 0x2054a39]
140a833af: mov qword ptr [rsp + 0x20], rax
140a833b4: lea rcx, [rbp + 0x1a8]
140a833bb: xorps xmm2, xmm2
140a833be: mov rdx, r8
140a833c1: movaps xmm3, xmm6
140a833c4: mov rsi, r8
140a833c7: call 0x141466bb0
140a833cc: mov dword ptr [rbp + 0x378], edi
140a833d2: movzx r8d, byte ptr [rsi + 0x64]
140a833d7: movzx r9d, byte ptr [rsi + 0x65]
140a833dc: movzx ecx, byte ptr [rsi + 0x68]
140a833e0: movzx edx, byte ptr [rsi + 0x69]
140a833e4: movzx r10d, byte ptr [rsi + 0x66]
140a833e9: movups xmm7, xmmword ptr [rsi + 0x54]
140a833ed: movzx esi, byte ptr [rsi + 0x67]
140a833f1: mov rax, qword ptr [rbp + 0x1d8]
140a833f8: mov qword ptr [rbp + 0x2e0], rax
140a833ff: movups xmm0, xmmword ptr [rbp + 0x1a8]
140a83406: movups xmm1, xmmword ptr [rbp + 0x1b8]
140a8340d: movups xmm2, xmmword ptr [rbp + 0x1c8]
140a83414: movaps xmmword ptr [rbp + 0x2d0], xmm2
140a8341b: movaps xmmword ptr [rbp + 0x2c0], xmm1
140a83422: movaps xmmword ptr [rbp + 0x2b0], xmm0
140a83429: mov dword ptr [rbp + 0x2e8], 2
140a83433: mov byte ptr [rbp + 0x37f], r8b
140a8343a: mov byte ptr [rbp + 0x314], r8b
140a83441: mov byte ptr [rbp + 0x368], cl
140a83447: mov byte ptr [rbp + 0x318], cl
140a8344d: mov byte ptr [rbp + 0x380], r9b
140a83454: mov byte ptr [rbp + 0x315], r9b
140a8345b: mov byte ptr [rbp + 0x360], dl
140a83461: mov byte ptr [rbp + 0x319], dl
140a83467: mov byte ptr [rbp + 0x381], r10b
140a8346e: mov byte ptr [rbp + 0x316], r10b
140a83475: movups xmmword ptr [rbp + 0x304], xmm7
140a8347c: mov byte ptr [rbp + 0x317], sil
140a83483: mov rdi, qword ptr [rbp + 0x2c0]
140a8348a: lea r14, [rdi*8]
140a83492: xor r12d, r12d
140a83495: mov rax, rdi
140a83498: shr rax, 0x3d
140a8349c: jne 0x140a839f3
140a834a2: movabs rbx, 0x7ffffffffffffffc
140a834ac: cmp r14, rbx
140a834af: ja 0x140a839f3
140a834b5: mov r15, qword ptr [rbp + 0x2b8]
140a834bc: test r14, r14
140a834bf: mov qword ptr [rbp + 0x340], r13
140a834c6: mov byte ptr [rbp + 0x382], sil
140a834cd: je 0x140a834fb
140a834cf: movzx eax, byte ptr [rip + 0x2bb1733]
140a834d6: mov r12d, 4
140a834dc: mov edx, 4
140a834e1: mov rcx, r14
140a834e4: call 0x140613c10
140a834e9: test rax, rax
140a834ec: je 0x140a839f3
140a834f2: mov qword ptr [rbp + 0x370], rdi
140a834f9: jmp 0x140a8350b
140a834fb: mov eax, 4
140a83500: mov qword ptr [rbp + 0x370], 0
140a8350b: mov qword ptr [rbp + 0x358], rax
140a83512: mov rcx, rax
140a83515: mov rdx, r15
140a83518: mov r8, r14
140a8351b: call 0x142923c00
140a83520: mov rsi, qword ptr [rbp + 0x2d8]
140a83527: lea r14, [rsi*4]
140a8352f: xor r13d, r13d
140a83532: mov rax, rsi
140a83535: shr rax, 0x3e
140a83539: jne 0x140a83a11
140a8353f: cmp r14, rbx
140a83542: ja 0x140a83a11
140a83548: mov r15, qword ptr [rbp + 0x2d0]
140a8354f: test r14, r14
140a83552: je 0x140a8357f
140a83554: movzx eax, byte ptr [rip + 0x2bb16ae]
140a8355b: mov r13d, 4
140a83561: mov edx, 4
140a83566: mov rcx, r14
140a83569: call 0x140613c10
140a8356e: test rax, rax
140a83571: je 0x140a83a11
140a83577: mov r12, rax
140a8357a: mov rbx, rsi
140a8357d: jmp 0x140a83587
140a8357f: mov r12d, 4
140a83585: xor ebx, ebx
140a83587: mov rcx, r12
140a8358a: mov rdx, r15
140a8358d: mov r8, r14
140a83590: call 0x142923c00
140a83595: movss xmm0, dword ptr [rbp + 0x2e0]
140a8359d: mov rax, qword ptr [rbp + 0x370]
140a835a4: mov qword ptr [rbp + 0xb8], rax
140a835ab: mov rax, qword ptr [rbp + 0x358]
140a835b2: mov qword ptr [rbp + 0xc0], rax
140a835b9: mov qword ptr [rbp + 0xc8], rdi
140a835c0: mov qword ptr [rbp + 0xd0], rbx
140a835c7: mov qword ptr [rbp + 0xd8], r12
140a835ce: mov qword ptr [rbp + 0xe0], rsi
140a835d5: mov rcx, qword ptr [rbp + 0x340]
140a835dc: mov qword ptr [rbp + 0x128], rcx
140a835e3: movlps qword ptr [rbp + 0x130], xmm7
140a835ea: movss dword ptr [rbp + 0xe8], xmm0
140a835f2: mov dword ptr [rbp + 0xf0], 2
140a835fc: movups xmmword ptr [rbp + 0x10c], xmm7
140a83603: movzx eax, byte ptr [rbp + 0x37f]
140a8360a: mov byte ptr [rbp + 0x11c], al
140a83610: movzx eax, byte ptr [rbp + 0x380]
140a83617: mov byte ptr [rbp + 0x11d], al
140a8361d: movzx eax, byte ptr [rbp + 0x381]
140a83624: mov byte ptr [rbp + 0x11e], al
140a8362a: movzx eax, byte ptr [rbp + 0x382]
140a83631: mov byte ptr [rbp + 0x11f], al
140a83637: movzx eax, byte ptr [rbp + 0x368]
140a8363e: mov byte ptr [rbp + 0x120], al
140a83644: movzx eax, byte ptr [rbp + 0x360]
140a8364b: mov byte ptr [rbp + 0x121], al
140a83651: mov eax, dword ptr [rbp + 0x378]
140a83657: mov dword ptr [rbp + 0x138], eax
140a8365d: movabs rax, 0x8000000000000000
140a83667: mov qword ptr [rbp + 0xb0], rax
140a8366e: movaps xmm0, xmmword ptr [rbp + 0x310]
140a83675: movaps xmmword ptr [rbp + 0x10], xmm0
140a83679: movaps xmm0, xmmword ptr [rbp + 0x300]
140a83680: movaps xmmword ptr [rbp], xmm0
140a83684: movaps xmm0, xmmword ptr [rbp + 0x2f0]
140a8368b: movaps xmmword ptr [rbp - 0x10], xmm0
140a8368f: movaps xmm0, xmmword ptr [rbp + 0x2b0]
140a83696: movaps xmm1, xmmword ptr [rbp + 0x2c0]
140a8369d: movaps xmm2, xmmword ptr [rbp + 0x2d0]
140a836a4: movaps xmm3, xmmword ptr [rbp + 0x2e0]
140a836ab: movaps xmmword ptr [rbp - 0x20], xmm3
140a836af: movaps xmmword ptr [rbp - 0x30], xmm2
140a836b3: movaps xmmword ptr [rbp - 0x40], xmm1
140a836b7: movaps xmmword ptr [rbp - 0x50], xmm0
140a836bb: mov rax, qword ptr [rbp + 0x350]
140a836c2: movups xmm0, xmmword ptr [rax]
140a836c5: movups xmm1, xmmword ptr [rax + 0x10]
140a836c9: movups xmm2, xmmword ptr [rax + 0x20]
140a836cd: movups xmm3, xmmword ptr [rax + 0x30]
140a836d1: movaps xmmword ptr [rbp + 0x20], xmm0
140a836d5: movaps xmmword ptr [rbp + 0x30], xmm1
140a836d9: movaps xmmword ptr [rbp + 0x40], xmm2
140a836dd: movaps xmmword ptr [rbp + 0x50], xmm3
140a836e1: movups xmm0, xmmword ptr [rax + 0x40]
140a836e5: movaps xmmword ptr [rbp + 0x60], xmm0
140a836e9: movups xmm0, xmmword ptr [rax + 0x50]
140a836ed: movaps xmmword ptr [rbp + 0x70], xmm0
140a836f1: movups xmm0, xmmword ptr [rax + 0x60]
140a836f5: movaps xmmword ptr [rbp + 0x80], xmm0
140a836fc: mov qword ptr [rbp + 0x90], rcx
140a83703: mov qword ptr [rbp + 0x98], 0
140a8370e: movss dword ptr [rbp + 0xa0], xmm6
140a83716: movzx eax, byte ptr [rip + 0x2bb14ec]
140a8371d: mov ecx, 0x30
140a83722: mov edx, 8
140a83727: call 0x140613c10
140a8372c: test rax, rax
140a8372f: je 0x140a83a28
140a83735: mov rsi, rax
140a83738: movzx eax, byte ptr [rip + 0x2bb14ca]
140a8373f: mov ecx, 8
140a83744: mov edx, 8
140a83749: call 0x140613c10
140a8374e: test rax, rax
140a83751: sete bl
140a83754: je 0x140a83a4a
140a8375a: mov r14, rax
140a8375d: lea rcx, [rbp + 0x1b8]
140a83764: lea rdx, [rbp + 0xb0]
140a8376b: mov r8d, 0xf8
140a83771: call 0x142923c00
140a83776: mov qword ptr [rbp + 0x1a8], 1
140a83781: mov qword ptr [rbp + 0x1b0], 1
140a8378c: movzx eax, byte ptr [rip + 0x2bb1476]
140a83793: mov ecx, 0x108
140a83798: mov edx, 8
140a8379d: call 0x140613c10
140a837a2: test rax, rax
140a837a5: je 0x140a83a6f
140a837ab: mov rdi, rax
140a837ae: lea rdx, [rbp + 0x1a8]
140a837b5: mov r8d, 0x108
140a837bb: mov rcx, rax
140a837be: call 0x142923c00
140a837c3: mov qword ptr [r14], rdi
140a837c6: mov qword ptr [rbp + 0x328], 1
140a837d1: mov qword ptr [rbp + 0x330], r14
140a837d8: mov qword ptr [rbp + 0x338], 1
140a837e3: movzx eax, byte ptr [rip + 0x2bb141f]
140a837ea: mov ecx, 8
140a837ef: mov edx, 8
140a837f4: call 0x140613c10
140a837f9: test rax, rax
140a837fc: sete r15b
140a83800: je 0x140a83a94
140a83806: mov r14, rax
140a83809: lea rcx, [rbp + 0x1b8]
140a83810: lea rdx, [rbp - 0x50]
140a83814: mov r8d, 0xf8
140a8381a: call 0x142923c00
140a8381f: mov qword ptr [rbp + 0x1a8], 1
140a8382a: mov qword ptr [rbp + 0x1b0], 1
140a83835: movzx eax, byte ptr [rip + 0x2bb13cd]
140a8383c: mov ecx, 0x108
140a83841: mov edx, 8
140a83846: call 0x140613c10
140a8384b: test rax, rax
140a8384e: je 0x140a83ab9
140a83854: mov rdi, rax
140a83857: lea rdx, [rbp + 0x1a8]
140a8385e: mov r8d, 0x108
140a83864: mov rcx, rax
140a83867: call 0x142923c00
140a8386c: mov qword ptr [r14], rdi
140a8386f: mov rax, qword ptr [rbp + 0x338]
140a83876: mov qword ptr [rsi + 0x10], rax
140a8387a: movups xmm0, xmmword ptr [rbp + 0x328]
140a83881: movups xmmword ptr [rsi], xmm0
140a83884: mov qword ptr [rsi + 0x18], 1
140a8388c: mov qword ptr [rsi + 0x20], r14
140a83890: mov qword ptr [rsi + 0x28], 1
140a83898: mov rax, rsi
140a8389b: add rax, 0x30
140a8389f: mov qword ptr [rbp + 0x1a8], rsi
140a838a6: mov qword ptr [rbp + 0x1b0], rsi
140a838ad: mov qword ptr [rbp + 0x1b8], 2
140a838b8: mov qword ptr [rbp + 0x1c0], rax
140a838bf: mov byte ptr [rbp + 0x385], 0
140a838c6: mov byte ptr [rbp + 0x384], 0
140a838cd: lea rcx, [rbp + 0x1a8]
140a838d4: call 0x141696f50
140a838d9: mov qword ptr [rbp + 0x1b0], rax
140a838e0: mov qword ptr [rbp + 0x1b8], rdx
140a838e7: mov qword ptr [rbp + 0x1a8], 7
140a838f2: mov byte ptr [rbp + 0x385], 0
140a838f9: mov byte ptr [rbp + 0x384], 0
140a83900: lea r8, [rip + 0x20544f9]
140a83907: lea rdx, [rbp + 0x1a8]
140a8390e: mov rcx, qword ptr [rbp + 0x348]
140a83915: call 0x140a84ec0
140a8391a: movaps xmm6, xmmword ptr [rbp + 0x390]
140a83921: movaps xmm7, xmmword ptr [rbp + 0x3a0]
140a83928: add rsp, 0x438
140a8392f: pop rbx
140a83930: pop rdi
140a83931: pop rsi
140a83932: pop r12
140a83934: pop r13
140a83936: pop r14
140a83938: pop r15
140a8393a: pop rbp
140a8393b: ret 
140a8393c: mov byte ptr [rbp + 0x387], 1
140a83943: lea rcx, [rip + 0x20543fe]
140a8394a: lea r8, [rip + 0x2054467]
140a83951: mov edx, 0x33
140a83956: call 0x1428d9430
140a8395b: jmp 0x140a83ae3
140a83960: lea rax, [rbp + 0xb0]
140a83967: mov qword ptr [rbp - 0x50], rax
140a8396b: lea rax, [rip + 0x308f9e]
140a83972: mov qword ptr [rbp - 0x48], rax
140a83976: lea rax, [rip + 0x2021a43]
140a8397d: mov qword ptr [rbp + 0x1a8], rax
140a83984: mov qword ptr [rbp + 0x1b0], 1
140a8398f: mov qword ptr [rbp + 0x1c8], 0
140a8399a: lea rax, [rbp - 0x50]
140a8399e: mov qword ptr [rbp + 0x1b8], rax
140a839a5: mov qword ptr [rbp + 0x1c0], 1
140a839b0: mov byte ptr [rbp + 0x387], 1
140a839b7: lea rdx, [rip + 0x2054412]
