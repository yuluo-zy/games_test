0x1408260d0 55 push rbp
0x1408260d1 4157 push r15
0x1408260d3 4156 push r14
0x1408260d5 4155 push r13
0x1408260d7 4154 push r12
0x1408260d9 56 push rsi
0x1408260da 57 push rdi
0x1408260db 53 push rbx
0x1408260dc 4881ec38010000 sub rsp, 0x138
0x1408260e3 488dac2480000000 lea rbp, [rsp + 0x80]
0x1408260eb 440f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm8
0x1408260f3 660f7fbd90000000 movdqa xmmword ptr [rbp + 0x90], xmm7
0x1408260fb 660f7fb580000000 movdqa xmmword ptr [rbp + 0x80], xmm6
0x140826103 48c74578feffffff mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
0x14082610b 4889d3 mov rbx, rdx
0x14082610e 4889ce mov rsi, rcx
0x140826111 e8ea083f00 call 0x140c16a00
0x140826116 84c0 test al, al
0x140826118 745f je 0x140826179
0x14082611a 4c8d353cb32602 lea r14, [rip + 0x226b33c]
0x140826121 bf2d000000 mov edi, 0x2d
0x140826126 ba2d000000 mov edx, 0x2d
0x14082612b 4c89f1 mov rcx, r14
0x14082612e e88deaffff call 0x140824bc0
0x140826133 4883f801 cmp rax, 1
0x140826137 0f85b1000000 jne 0x1408261ee
0x14082613d 4989d1 mov r9, rdx
0x140826140 4885d2 test rdx, rdx
0x140826143 743d je 0x140826182
0x140826145 4983f92d cmp r9, 0x2d
0x140826149 7335 jae 0x140826180
0x14082614b 488d050bb32602 lea rax, [rip + 0x226b30b]
0x140826152 41803c01bf cmp byte ptr [r9 + rax], 0xbf
0x140826157 7f29 jg 0x140826182
0x140826159 488d0530b42602 lea rax, [rip + 0x226b430]
0x140826160 4889442420 mov qword ptr [rsp + 0x20], rax
0x140826165 488d0df1b22602 lea rcx, [rip + 0x226b2f1]
0x14082616c ba2d000000 mov edx, 0x2d
0x140826171 4531c0 xor r8d, r8d
0x140826174 e8c7370b02 call 0x1428d9940
0x140826179 31c0 xor eax, eax
0x14082617b e9ba010000 jmp 0x14082633a
0x140826180 75d7 jne 0x140826159
0x140826182 4c8d35d4b22602 lea r14, [rip + 0x226b2d4]
0x140826189 4c89f1 mov rcx, r14
0x14082618c 4c89ca mov rdx, r9
0x14082618f e82ceaffff call 0x140824bc0
0x140826194 4883f801 cmp rax, 1
0x140826198 7554 jne 0x1408261ee
0x14082619a 4989d0 mov r8, rdx
0x14082619d 4983c002 add r8, 2
0x1408261a1 7439 je 0x1408261dc
0x1408261a3 4983f82d cmp r8, 0x2d
0x1408261a7 7331 jae 0x1408261da
0x1408261a9 488d05adb22602 lea rax, [rip + 0x226b2ad]
0x1408261b0 41803c00bf cmp byte ptr [r8 + rax], 0xbf
0x1408261b5 7f25 jg 0x1408261dc
0x1408261b7 488d05eab32602 lea rax, [rip + 0x226b3ea]
0x1408261be 4889442420 mov qword ptr [rsp + 0x20], rax
0x1408261c3 488d0d93b22602 lea rcx, [rip + 0x226b293]
0x1408261ca ba2d000000 mov edx, 0x2d
0x1408261cf 41b92d000000 mov r9d, 0x2d
0x1408261d5 e866370b02 call 0x1428d9940
0x1408261da 75db jne 0x1408261b7
0x1408261dc bf2b000000 mov edi, 0x2b
0x1408261e1 4829d7 sub rdi, rdx
0x1408261e4 4c8d3572b22602 lea r14, [rip + 0x226b272]
0x1408261eb 4d01c6 add r14, r8
0x1408261ee 4c8d0584b42602 lea r8, [rip + 0x226b484]
0x1408261f5 4c8d3d54b42602 lea r15, [rip + 0x226b454]
0x1408261fc 0f1f4000 nop dword ptr [rax]
0x140826200 410fbe40ff movsx eax, byte ptr [r8 - 1]
0x140826205 85c0 test eax, eax
0x140826207 7817 js 0x140826220
0x140826209 49ffc8 dec r8
0x14082620c 83f85c cmp eax, 0x5c
0x14082620f 7563 jne 0x140826274
0x140826211 eb73 jmp 0x140826286
0x140826213 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140826220 410fb648fe movzx ecx, byte ptr [r8 - 2]
0x140826225 80f9c0 cmp cl, 0xc0
0x140826228 7d23 jge 0x14082624d
0x14082622a 410fb650fd movzx edx, byte ptr [r8 - 3]
0x14082622f 80fac0 cmp dl, 0xc0
0x140826232 7d22 jge 0x140826256
0x140826234 450fb648fc movzx r9d, byte ptr [r8 - 4]
0x140826239 4983c0fc add r8, -4
0x14082623d 4183e107 and r9d, 7
0x140826241 41c1e106 shl r9d, 6
0x140826245 83e23f and edx, 0x3f
0x140826248 4409ca or edx, r9d
0x14082624b eb10 jmp 0x14082625d
0x14082624d 4983c0fe add r8, -2
0x140826251 83e11f and ecx, 0x1f
0x140826254 eb0f jmp 0x140826265
0x140826256 4983c0fd add r8, -3
0x14082625a 83e20f and edx, 0xf
0x14082625d c1e206 shl edx, 6
0x140826260 83e13f and ecx, 0x3f
0x140826263 09d1 or ecx, edx
0x140826265 c1e106 shl ecx, 6
0x140826268 243f and al, 0x3f
0x14082626a 0fb6c0 movzx eax, al
0x14082626d 09c8 or eax, ecx
0x14082626f 83f85c cmp eax, 0x5c
0x140826272 7412 je 0x140826286
0x140826274 83f82f cmp eax, 0x2f
0x140826277 740d je 0x140826286
0x140826279 4d39f8 cmp r8, r15
0x14082627c 7582 jne 0x140826200
0x14082627e 41bc29000000 mov r12d, 0x29
0x140826284 eb4d jmp 0x1408262d3
0x140826286 4c8d3dc3b32602 lea r15, [rip + 0x226b3c3]
0x14082628d 4d29f8 sub r8, r15
0x140826290 49ffc0 inc r8
0x140826293 7432 je 0x1408262c7
0x140826295 4983f829 cmp r8, 0x29
0x140826299 732a jae 0x1408262c5
0x14082629b 43803c38bf cmp byte ptr [r8 + r15], 0xbf
0x1408262a0 7f25 jg 0x1408262c7
0x1408262a2 488d05c7b22602 lea rax, [rip + 0x226b2c7]
0x1408262a9 4889442420 mov qword ptr [rsp + 0x20], rax
0x1408262ae 488d0d9bb32602 lea rcx, [rip + 0x226b39b]
0x1408262b5 ba29000000 mov edx, 0x29
0x1408262ba 41b929000000 mov r9d, 0x29
0x1408262c0 e87b360b02 call 0x1428d9940
0x1408262c5 75db jne 0x1408262a2
0x1408262c7 41bc29000000 mov r12d, 0x29
0x1408262cd 4d29c4 sub r12, r8
0x1408262d0 4d01c7 add r15, r8
0x1408262d3 e8581e3f00 call 0x140c18130
0x1408262d8 4889c1 mov rcx, rax
0x1408262db 488b00 mov rax, qword ptr [rax]
0x1408262de 4883f801 cmp rax, 1
0x1408262e2 0f850f090000 jne 0x140826bf7
0x1408262e8 4883c108 add rcx, 8
0x1408262ec 48833900 cmp qword ptr [rcx], 0
0x1408262f0 0f8525090000 jne 0x140826c1b
0x1408262f6 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1408262fd 48894d60 mov qword ptr [rbp + 0x60], rcx
0x140826301 4883c108 add rcx, 8
0x140826305 4c89642420 mov qword ptr [rsp + 0x20], r12
0x14082630a 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x140826313 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x14082631c 4c89f2 mov rdx, r14
0x14082631f 4989f8 mov r8, rdi
0x140826322 4d89f9 mov r9, r15
0x140826325 e8160a3f00 call 0x140c16d40
0x14082632a 488b4d60 mov rcx, qword ptr [rbp + 0x60]
0x14082632e 48ff01 inc qword ptr [rcx]
0x140826331 488945d8 mov qword ptr [rbp - 0x28], rax
0x140826335 b801000000 mov eax, 1
0x14082633a 488945d0 mov qword ptr [rbp - 0x30], rax
0x14082633e 8b461c mov eax, dword ptr [rsi + 0x1c]
0x140826341 488b4e10 mov rcx, qword ptr [rsi + 0x10]
0x140826345 8901 mov dword ptr [rcx], eax
0x140826347 488b16 mov rdx, qword ptr [rsi]
0x14082634a 4889e9 mov rcx, rbp
0x14082634d 488955c8 mov qword ptr [rbp - 0x38], rdx
0x140826351 e8da821f00 call 0x140a1e630
0x140826356 0fb64500 movzx eax, byte ptr [rbp]
0x14082635a 884573 mov byte ptr [rbp + 0x73], al
0x14082635d 4c8b6d08 mov r13, qword ptr [rbp + 8]
0x140826361 488b5510 mov rdx, qword ptr [rbp + 0x10]
0x140826365 4c8b4518 mov r8, qword ptr [rbp + 0x18]
0x140826369 4c8b4d20 mov r9, qword ptr [rbp + 0x20]
0x14082636d 4c8b6528 mov r12, qword ptr [rbp + 0x28]
0x140826371 0fb74538 movzx eax, word ptr [rbp + 0x38]
0x140826375 488b4d40 mov rcx, qword ptr [rbp + 0x40]
0x140826379 48894de8 mov qword ptr [rbp - 0x18], rcx
0x14082637d 8b4b1c mov ecx, dword ptr [rbx + 0x1c]
0x140826380 894d60 mov dword ptr [rbp + 0x60], ecx
0x140826383 488b33 mov rsi, qword ptr [rbx]
0x140826386 488b4b10 mov rcx, qword ptr [rbx + 0x10]
0x14082638a 48894de0 mov qword ptr [rbp - 0x20], rcx
0x14082638e 660f6f35ea6c1a02 movdqa xmm6, xmmword ptr [rip + 0x21a6cea]
0x140826396 660f6f3df26c1a02 movdqa xmm7, xmmword ptr [rip + 0x21a6cf2]
0x14082639e 450f57c0 xorps xmm8, xmm8
0x1408263a2 eb1f jmp 0x1408263c3
0x1408263a4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1408263b0 8b456c mov eax, dword ptr [rbp + 0x6c]
0x1408263b3 4c8b6df8 mov r13, qword ptr [rbp - 8]
0x1408263b7 488b5550 mov rdx, qword ptr [rbp + 0x50]
0x1408263bb 4c8b4558 mov r8, qword ptr [rbp + 0x58]
0x1408263bf 4c8b4df0 mov r9, qword ptr [rbp - 0x10]
0x1408263c3 4883fa02 cmp rdx, 2
0x1408263c7 0f84e6000000 je 0x1408264b3
0x1408263cd 4d85c9 test r9, r9
0x1408263d0 0f8498000000 je 0x14082646e
0x1408263d6 48837de800 cmp qword ptr [rbp - 0x18], 0
0x1408263db 0f84c2000000 je 0x1408264a3
0x1408263e1 6685c0 test ax, ax
0x1408263e4 48895550 mov qword ptr [rbp + 0x50], rdx
0x1408263e8 4c894558 mov qword ptr [rbp + 0x58], r8
0x1408263ec 4c896df8 mov qword ptr [rbp - 8], r13
0x1408263f0 740e je 0x140826400
0x1408263f2 8d48ff lea ecx, [rax - 1]
0x1408263f5 21c1 and ecx, eax
0x1408263f7 894d6c mov dword ptr [rbp + 0x6c], ecx
0x1408263fa eb30 jmp 0x14082642c
0x1408263fc 0f1f4000 nop dword ptr [rax]
0x140826400 66410f6f0424 movdqa xmm0, xmmword ptr [r12]
0x140826406 660fd7c0 pmovmskb eax, xmm0
0x14082640a 4981c180f6ffff add r9, -0x980
0x140826411 4983c410 add r12, 0x10
0x140826415 3dffff0000 cmp eax, 0xffff
0x14082641a 74e4 je 0x140826400
0x14082641c b9feffffff mov ecx, 0xfffffffe
0x140826421 29c1 sub ecx, eax
0x140826423 f7d0 not eax
0x140826425 89c2 mov edx, eax
0x140826427 21ca and edx, ecx
0x140826429 89556c mov dword ptr [rbp + 0x6c], edx
0x14082642c f30fbcc0 tzcnt eax, eax
0x140826430 48f7d8 neg rax
0x140826433 4869c098000000 imul rax, rax, 0x98
0x14082643a 4c894df0 mov qword ptr [rbp - 0x10], r9
0x14082643e 4889c7 mov rdi, rax
0x140826441 498d0c01 lea rcx, [r9 + rax]
0x140826445 4881c168ffffff add rcx, -0x98
0x14082644c e84f5e1f00 call 0x140a1c2a0
0x140826451 48ff4de8 dec qword ptr [rbp - 0x18]
0x140826455 4883f803 cmp rax, 3
0x140826459 0f8589040000 jne 0x1408268e8
0x14082645f 8b456c mov eax, dword ptr [rbp + 0x6c]
0x140826462 4c8b6df8 mov r13, qword ptr [rbp - 8]
0x140826466 488b5550 mov rdx, qword ptr [rbp + 0x50]
0x14082646a 4c8b4558 mov r8, qword ptr [rbp + 0x58]
0x14082646e f6c201 test dl, 1
0x140826471 743d je 0x1408264b0
0x140826473 41b900000000 mov r9d, 0
0x140826479 b900000000 mov ecx, 0
0x14082647e 4d85c0 test r8, r8
0x140826481 7433 je 0x1408264b6
0x140826483 48895550 mov qword ptr [rbp + 0x50], rdx
0x140826487 4c896df8 mov qword ptr [rbp - 8], r13
0x14082648b 4c89c2 mov rdx, r8
0x14082648e 89456c mov dword ptr [rbp + 0x6c], eax
0x140826491 48c745f000000000 mov qword ptr [rbp - 0x10], 0
0x140826499 48c7455800000000 mov qword ptr [rbp + 0x58], 0
0x1408264a1 eb44 jmp 0x1408264e7
0x1408264a3 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x1408264ab f6c201 test dl, 1
0x1408264ae 75c3 jne 0x140826473
0x1408264b0 4531c9 xor r9d, r9d
0x1408264b3 4c89c1 mov rcx, r8
0x1408264b6 4d85ed test r13, r13
0x1408264b9 0f95c2 setne dl
0x1408264bc 225573 and dl, byte ptr [rbp + 0x73]
0x1408264bf 80fa01 cmp dl, 1
0x1408264c2 0f8533040000 jne 0x1408268fb
0x1408264c8 4c894df0 mov qword ptr [rbp - 0x10], r9
0x1408264cc ba02000000 mov edx, 2
0x1408264d1 48895550 mov qword ptr [rbp + 0x50], rdx
0x1408264d5 4c89ea mov rdx, r13
0x1408264d8 89456c mov dword ptr [rbp + 0x6c], eax
0x1408264db 48894d58 mov qword ptr [rbp + 0x58], rcx
0x1408264df 48c745f800000000 mov qword ptr [rbp - 8], 0
0x1408264e7 4889e9 mov rcx, rbp
0x1408264ea 4989d5 mov r13, rdx
0x1408264ed e83e4b1f00 call 0x140a1b030
0x1408264f2 488b7d00 mov rdi, qword ptr [rbp]
0x1408264f6 4c8b7d08 mov r15, qword ptr [rbp + 8]
0x1408264fa 0fb74518 movzx eax, word ptr [rbp + 0x18]
0x1408264fe 488b5d20 mov rbx, qword ptr [rbp + 0x20]
0x140826502 eb0f jmp 0x140826513
0x140826504 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140826510 4489f0 mov eax, r14d
0x140826513 4883eb01 sub rbx, 1
0x140826517 0f82d3010000 jb 0x1408266f0
0x14082651d 6685c0 test ax, ax
0x140826520 741e je 0x140826540
0x140826522 4885ff test rdi, rdi
0x140826525 0f84c5010000 je 0x1408266f0
0x14082652b 448d70ff lea r14d, [rax - 1]
0x14082652f 4121c6 and r14d, eax
0x140826532 eb33 jmp 0x140826567
0x140826534 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140826540 66410f6f07 movdqa xmm0, xmmword ptr [r15]
0x140826545 660fd7c0 pmovmskb eax, xmm0
0x140826549 4883c7c0 add rdi, -0x40
0x14082654d 4983c710 add r15, 0x10
0x140826551 3dffff0000 cmp eax, 0xffff
0x140826556 74e8 je 0x140826540
0x140826558 b9feffffff mov ecx, 0xfffffffe
0x14082655d 29c1 sub ecx, eax
0x14082655f f7d0 not eax
0x140826561 4189c6 mov r14d, eax
0x140826564 4121ce and r14d, ecx
0x140826567 f30fbcc0 tzcnt eax, eax
0x14082656b c1e002 shl eax, 2
0x14082656e 4889f9 mov rcx, rdi
0x140826571 4829c1 sub rcx, rax
0x140826574 8b41fc mov eax, dword ptr [rcx - 4]
0x140826577 894574 mov dword ptr [rbp + 0x74], eax
0x14082657a 8b4560 mov eax, dword ptr [rbp + 0x60]
0x14082657d 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140826581 8901 mov dword ptr [rcx], eax
0x140826583 f30f6f4620 movdqu xmm0, xmmword ptr [rsi + 0x20]
0x140826588 660f70c844 pshufd xmm1, xmm0, 0x44
0x14082658d 660fefce pxor xmm1, xmm6
0x140826591 660f7f4d00 movdqa xmmword ptr [rbp], xmm1
0x140826596 660f70c8ee pshufd xmm1, xmm0, 0xee
0x14082659b 660fefcf pxor xmm1, xmm7
0x14082659f 660f7f4d10 movdqa xmmword ptr [rbp + 0x10], xmm1
0x1408265a4 660f7f4520 movdqa xmmword ptr [rbp + 0x20], xmm0
0x1408265a9 488d4530 lea rax, [rbp + 0x30]
0x1408265ad 440f2900 movaps xmmword ptr [rax], xmm8
0x1408265b1 48c7401000000000 mov qword ptr [rax + 0x10], 0
0x1408265b9 8b4574 mov eax, dword ptr [rbp + 0x74]
0x1408265bc 894548 mov dword ptr [rbp + 0x48], eax
0x1408265bf 41b804000000 mov r8d, 4
0x1408265c5 4889e9 mov rcx, rbp
0x1408265c8 488d5548 lea rdx, [rbp + 0x48]
0x1408265cc e88ffb8dff call 0x140106160
0x1408265d1 4c8b4d30 mov r9, qword ptr [rbp + 0x30]
0x1408265d5 49c1e138 shl r9, 0x38
0x1408265d9 4c0b4d38 or r9, qword ptr [rbp + 0x38]
0x1408265dd 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x1408265e1 488b4518 mov rax, qword ptr [rbp + 0x18]
0x1408265e5 4c31c8 xor rax, r9
0x1408265e8 488b5500 mov rdx, qword ptr [rbp]
0x1408265ec 4801ca add rdx, rcx
0x1408265ef 4c8b4508 mov r8, qword ptr [rbp + 8]
0x1408265f3 48c1c10d rol rcx, 0xd
0x1408265f7 4901c0 add r8, rax
0x1408265fa 4831d1 xor rcx, rdx
0x1408265fd 48c1c010 rol rax, 0x10
0x140826601 4c31c0 xor rax, r8
0x140826604 48c1c220 rol rdx, 0x20
0x140826608 4901c8 add r8, rcx
0x14082660b 4801c2 add rdx, rax
0x14082660e 48c1c111 rol rcx, 0x11
0x140826612 4c31c1 xor rcx, r8
0x140826615 48c1c015 rol rax, 0x15
0x140826619 49c1c020 rol r8, 0x20
0x14082661d 4831d0 xor rax, rdx
0x140826620 4c31ca xor rdx, r9
0x140826623 4981f0ff000000 xor r8, 0xff
0x14082662a 4801ca add rdx, rcx
0x14082662d 4901c0 add r8, rax
0x140826630 48c1c10d rol rcx, 0xd
0x140826634 4831d1 xor rcx, rdx
0x140826637 48c1c010 rol rax, 0x10
0x14082663b 48c1c220 rol rdx, 0x20
0x14082663f 4c31c0 xor rax, r8
0x140826642 4901c8 add r8, rcx
0x140826645 48c1c111 rol rcx, 0x11
0x140826649 4801c2 add rdx, rax
0x14082664c 4c31c1 xor rcx, r8
0x14082664f 48c1c015 rol rax, 0x15
0x140826653 4831d0 xor rax, rdx
0x140826656 49c1c020 rol r8, 0x20
0x14082665a 4801ca add rdx, rcx
0x14082665d 4901c0 add r8, rax
0x140826660 48c1c10d rol rcx, 0xd
0x140826664 4831d1 xor rcx, rdx
0x140826667 48c1c010 rol rax, 0x10
0x14082666b 48c1c220 rol rdx, 0x20
0x14082666f 4c31c0 xor rax, r8
0x140826672 4901c8 add r8, rcx
0x140826675 48c1c111 rol rcx, 0x11
0x140826679 4801c2 add rdx, rax
0x14082667c 4c31c1 xor rcx, r8
0x14082667f 48c1c015 rol rax, 0x15
0x140826683 4831d0 xor rax, rdx
0x140826686 49c1c020 rol r8, 0x20
0x14082668a 4801ca add rdx, rcx
0x14082668d 4901c0 add r8, rax
0x140826690 48c1c10d rol rcx, 0xd
0x140826694 4831d1 xor rcx, rdx
0x140826697 48c1c010 rol rax, 0x10
0x14082669b 4c31c0 xor rax, r8
0x14082669e 4901c8 add r8, rcx
0x1408266a1 48c1c111 rol rcx, 0x11
0x1408266a5 48c1c015 rol rax, 0x15
0x1408266a9 4c89c2 mov rdx, r8
0x1408266ac 48c1c220 rol rdx, 0x20
0x1408266b0 4831ca xor rdx, rcx
0x1408266b3 4831c2 xor rdx, rax
0x1408266b6 4c31c2 xor rdx, r8
0x1408266b9 4889f1 mov rcx, rsi
0x1408266bc 4c8d4574 lea r8, [rbp + 0x74]
0x1408266c0 e85be1a300 call 0x141264820
0x1408266c5 48895500 mov qword ptr [rbp], rdx
0x1408266c9 4885d2 test rdx, rdx
0x1408266cc 0f843efeffff je 0x140826510
0x1408266d2 f048ff0a lock dec qword ptr [rdx]
0x1408266d6 0f8534feffff jne 0x140826510
0x1408266dc 4889e9 mov rcx, rbp
0x1408266df e8bc4a0000 call 0x14082b1a0
0x1408266e4 e927feffff jmp 0x140826510
0x1408266e9 0f1f8000000000 nop dword ptr [rax]
0x1408266f0 4889e9 mov rcx, rbp
0x1408266f3 4c89ea mov rdx, r13
0x1408266f6 e8054a1f00 call 0x140a1b100
0x1408266fb 488b7d00 mov rdi, qword ptr [rbp]
0x1408266ff 488b5d08 mov rbx, qword ptr [rbp + 8]
0x140826703 0fb74518 movzx eax, word ptr [rbp + 0x18]
0x140826707 4c8b7d20 mov r15, qword ptr [rbp + 0x20]
0x14082670b eb06 jmp 0x140826713
0x14082670d 0f1f00 nop dword ptr [rax]
0x140826710 4489f0 mov eax, r14d
0x140826713 4983ef01 sub r15, 1
0x140826717 0f8293fcffff jb 0x1408263b0
0x14082671d 6685c0 test ax, ax
0x140826720 741e je 0x140826740
0x140826722 448d70ff lea r14d, [rax - 1]
0x140826726 4121c6 and r14d, eax
0x140826729 4885ff test rdi, rdi
0x14082672c 7538 jne 0x140826766
0x14082672e e97dfcffff jmp 0x1408263b0
0x140826733 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140826740 660f6f03 movdqa xmm0, xmmword ptr [rbx]
0x140826744 660fd7c0 pmovmskb eax, xmm0
0x140826748 4883c7c0 add rdi, -0x40
0x14082674c 4883c310 add rbx, 0x10
0x140826750 3dffff0000 cmp eax, 0xffff
0x140826755 74e9 je 0x140826740
0x140826757 b9feffffff mov ecx, 0xfffffffe
0x14082675c 29c1 sub ecx, eax
0x14082675e f7d0 not eax
0x140826760 4189c6 mov r14d, eax
0x140826763 4121ce and r14d, ecx
0x140826766 f30fbcc0 tzcnt eax, eax
0x14082676a c1e002 shl eax, 2
0x14082676d 4889f9 mov rcx, rdi
0x140826770 4829c1 sub rcx, rax
0x140826773 8b41fc mov eax, dword ptr [rcx - 4]
0x140826776 894574 mov dword ptr [rbp + 0x74], eax
0x140826779 8b4560 mov eax, dword ptr [rbp + 0x60]
0x14082677c 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140826780 8901 mov dword ptr [rcx], eax
0x140826782 f30f6f4620 movdqu xmm0, xmmword ptr [rsi + 0x20]
0x140826787 660f70c844 pshufd xmm1, xmm0, 0x44
0x14082678c 660fefce pxor xmm1, xmm6
0x140826790 660f7f4d00 movdqa xmmword ptr [rbp], xmm1
0x140826795 660f70c8ee pshufd xmm1, xmm0, 0xee
0x14082679a 660fefcf pxor xmm1, xmm7
0x14082679e 660f7f4d10 movdqa xmmword ptr [rbp + 0x10], xmm1
0x1408267a3 660f7f4520 movdqa xmmword ptr [rbp + 0x20], xmm0
0x1408267a8 488d4530 lea rax, [rbp + 0x30]
0x1408267ac 440f2900 movaps xmmword ptr [rax], xmm8
0x1408267b0 48c7401000000000 mov qword ptr [rax + 0x10], 0
0x1408267b8 8b4574 mov eax, dword ptr [rbp + 0x74]
0x1408267bb 894548 mov dword ptr [rbp + 0x48], eax
0x1408267be 41b804000000 mov r8d, 4
0x1408267c4 4889e9 mov rcx, rbp
0x1408267c7 488d5548 lea rdx, [rbp + 0x48]
0x1408267cb e890f98dff call 0x140106160
0x1408267d0 4c8b4d30 mov r9, qword ptr [rbp + 0x30]
0x1408267d4 49c1e138 shl r9, 0x38
0x1408267d8 4c0b4d38 or r9, qword ptr [rbp + 0x38]
0x1408267dc 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x1408267e0 488b4518 mov rax, qword ptr [rbp + 0x18]
0x1408267e4 4c31c8 xor rax, r9
0x1408267e7 488b5500 mov rdx, qword ptr [rbp]
0x1408267eb 4801ca add rdx, rcx
0x1408267ee 4c8b4508 mov r8, qword ptr [rbp + 8]
0x1408267f2 48c1c10d rol rcx, 0xd
0x1408267f6 4901c0 add r8, rax
0x1408267f9 4831d1 xor rcx, rdx
0x1408267fc 48c1c010 rol rax, 0x10
0x140826800 4c31c0 xor rax, r8
0x140826803 48c1c220 rol rdx, 0x20
0x140826807 4901c8 add r8, rcx
0x14082680a 4801c2 add rdx, rax
0x14082680d 48c1c111 rol rcx, 0x11
0x140826811 4c31c1 xor rcx, r8
0x140826814 48c1c015 rol rax, 0x15
0x140826818 49c1c020 rol r8, 0x20
0x14082681c 4831d0 xor rax, rdx
0x14082681f 4c31ca xor rdx, r9
0x140826822 4981f0ff000000 xor r8, 0xff
0x140826829 4801ca add rdx, rcx
0x14082682c 4901c0 add r8, rax
0x14082682f 48c1c10d rol rcx, 0xd
0x140826833 4831d1 xor rcx, rdx
0x140826836 48c1c010 rol rax, 0x10
0x14082683a 48c1c220 rol rdx, 0x20
0x14082683e 4c31c0 xor rax, r8
0x140826841 4901c8 add r8, rcx
0x140826844 48c1c111 rol rcx, 0x11
0x140826848 4801c2 add rdx, rax
0x14082684b 4c31c1 xor rcx, r8
0x14082684e 48c1c015 rol rax, 0x15
0x140826852 4831d0 xor rax, rdx
0x140826855 49c1c020 rol r8, 0x20
0x140826859 4801ca add rdx, rcx
0x14082685c 4901c0 add r8, rax
0x14082685f 48c1c10d rol rcx, 0xd
0x140826863 4831d1 xor rcx, rdx
0x140826866 48c1c010 rol rax, 0x10
0x14082686a 48c1c220 rol rdx, 0x20
0x14082686e 4c31c0 xor rax, r8
0x140826871 4901c8 add r8, rcx
0x140826874 48c1c111 rol rcx, 0x11
0x140826878 4801c2 add rdx, rax
0x14082687b 4c31c1 xor rcx, r8
0x14082687e 48c1c015 rol rax, 0x15
0x140826882 4831d0 xor rax, rdx
0x140826885 49c1c020 rol r8, 0x20
0x140826889 4801ca add rdx, rcx
0x14082688c 4901c0 add r8, rax
0x14082688f 48c1c10d rol rcx, 0xd
0x140826893 4831d1 xor rcx, rdx
0x140826896 48c1c010 rol rax, 0x10
0x14082689a 4c31c0 xor rax, r8
0x14082689d 4901c8 add r8, rcx
0x1408268a0 48c1c111 rol rcx, 0x11
0x1408268a4 48c1c015 rol rax, 0x15
0x1408268a8 4c89c2 mov rdx, r8
0x1408268ab 48c1c220 rol rdx, 0x20
0x1408268af 4831ca xor rdx, rcx
0x1408268b2 4831c2 xor rdx, rax
0x1408268b5 4c31c2 xor rdx, r8
0x1408268b8 4889f1 mov rcx, rsi
0x1408268bb 4c8d4574 lea r8, [rbp + 0x74]
0x1408268bf e85cdfa300 call 0x141264820
0x1408268c4 48895500 mov qword ptr [rbp], rdx
0x1408268c8 4885d2 test rdx, rdx
0x1408268cb 0f843ffeffff je 0x140826710
0x1408268d1 f048ff0a lock dec qword ptr [rdx]
0x1408268d5 0f8535feffff jne 0x140826710
0x1408268db 4889e9 mov rcx, rbp
0x1408268de e8bd480000 call 0x14082b1a0
0x1408268e3 e928feffff jmp 0x140826710
0x1408268e8 4889fa mov rdx, rdi
0x1408268eb 480355f0 add rdx, qword ptr [rbp - 0x10]
0x1408268ef 4881c270ffffff add rdx, -0x90
0x1408268f6 e9ecfbffff jmp 0x1408264e7
0x1408268fb 4889e9 mov rcx, rbp
0x1408268fe 488b55c8 mov rdx, qword ptr [rbp - 0x38]
0x140826902 e8d97d1f00 call 0x140a1e6e0
0x140826907 488b5d00 mov rbx, qword ptr [rbp]
0x14082690b 4c8b7d08 mov r15, qword ptr [rbp + 8]
0x14082690f 0fb74518 movzx eax, word ptr [rbp + 0x18]
0x140826913 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x140826917 450f57c0 xorps xmm8, xmm8
0x14082691b eb0a jmp 0x140826927
0x14082691d 0f1f00 nop dword ptr [rax]
0x140826920 8b4558 mov eax, dword ptr [rbp + 0x58]
0x140826923 488b4d50 mov rcx, qword ptr [rbp + 0x50]
0x140826927 4883e901 sub rcx, 1
0x14082692b 0f8241020000 jb 0x140826b72
0x140826931 6685c0 test ax, ax
0x140826934 48894d50 mov qword ptr [rbp + 0x50], rcx
0x140826938 7416 je 0x140826950
0x14082693a 4885db test rbx, rbx
0x14082693d 0f842f020000 je 0x140826b72
0x140826943 8d48ff lea ecx, [rax - 1]
0x140826946 21c1 and ecx, eax
0x140826948 894d58 mov dword ptr [rbp + 0x58], ecx
0x14082694b eb2e jmp 0x14082697b
0x14082694d 0f1f00 nop dword ptr [rax]
0x140826950 66410f6f07 movdqa xmm0, xmmword ptr [r15]
0x140826955 660fd7c0 pmovmskb eax, xmm0
0x140826959 4881c300fcffff add rbx, -0x400
0x140826960 4983c710 add r15, 0x10
0x140826964 3dffff0000 cmp eax, 0xffff
0x140826969 74e5 je 0x140826950
0x14082696b b9feffffff mov ecx, 0xfffffffe
0x140826970 29c1 sub ecx, eax
0x140826972 f7d0 not eax
0x140826974 89c2 mov edx, eax
0x140826976 21ca and edx, ecx
0x140826978 895558 mov dword ptr [rbp + 0x58], edx
0x14082697b f30fbcc0 tzcnt eax, eax
0x14082697f c1e006 shl eax, 6
0x140826982 4889d9 mov rcx, rbx
0x140826985 4829c1 sub rcx, rax
0x140826988 488b79d0 mov rdi, qword ptr [rcx - 0x30]
0x14082698c 4c8b69e8 mov r13, qword ptr [rcx - 0x18]
0x140826990 660f6f07 movdqa xmm0, xmmword ptr [rdi]
0x140826994 66440fd7f0 pmovmskb r14d, xmm0
0x140826999 41f7d6 not r14d
0x14082699c 4c8d6710 lea r12, [rdi + 0x10]
0x1408269a0 eb18 jmp 0x1408269ba
0x1408269a2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1408269b0 418d46ff lea eax, [r14 - 1]
0x1408269b4 4421f0 and eax, r14d
0x1408269b7 4189c6 mov r14d, eax
0x1408269ba 4983ed01 sub r13, 1
0x1408269be 0f825cffffff jb 0x140826920
0x1408269c4 664585f6 test r14w, r14w
0x1408269c8 7525 jne 0x1408269ef
0x1408269ca 660f1f440000 nop word ptr [rax + rax]
0x1408269d0 66410f6f0424 movdqa xmm0, xmmword ptr [r12]
0x1408269d6 66440fd7f0 pmovmskb r14d, xmm0
0x1408269db 4883c7c0 add rdi, -0x40
0x1408269df 4983c410 add r12, 0x10
0x1408269e3 4181feffff0000 cmp r14d, 0xffff
0x1408269ea 74e4 je 0x1408269d0
0x1408269ec 41f7d6 not r14d
0x1408269ef f3410fbcc6 tzcnt eax, r14d
0x1408269f4 c1e002 shl eax, 2
0x1408269f7 4889f9 mov rcx, rdi
0x1408269fa 4829c1 sub rcx, rax
0x1408269fd 8b41fc mov eax, dword ptr [rcx - 4]
0x140826a00 894574 mov dword ptr [rbp + 0x74], eax
0x140826a03 8b4560 mov eax, dword ptr [rbp + 0x60]
0x140826a06 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140826a0a 8901 mov dword ptr [rcx], eax
0x140826a0c f30f6f4620 movdqu xmm0, xmmword ptr [rsi + 0x20]
0x140826a11 660f70c844 pshufd xmm1, xmm0, 0x44
0x140826a16 660fefce pxor xmm1, xmm6
0x140826a1a 660f7f4d00 movdqa xmmword ptr [rbp], xmm1
0x140826a1f 660f70c8ee pshufd xmm1, xmm0, 0xee
0x140826a24 660fefcf pxor xmm1, xmm7
0x140826a28 660f7f4d10 movdqa xmmword ptr [rbp + 0x10], xmm1
0x140826a2d 660f7f4520 movdqa xmmword ptr [rbp + 0x20], xmm0
0x140826a32 488d4530 lea rax, [rbp + 0x30]
0x140826a36 440f2900 movaps xmmword ptr [rax], xmm8
0x140826a3a 48c7401000000000 mov qword ptr [rax + 0x10], 0
0x140826a42 8b4574 mov eax, dword ptr [rbp + 0x74]
0x140826a45 894548 mov dword ptr [rbp + 0x48], eax
0x140826a48 41b804000000 mov r8d, 4
0x140826a4e 4889e9 mov rcx, rbp
0x140826a51 488d5548 lea rdx, [rbp + 0x48]
0x140826a55 e806f78dff call 0x140106160
0x140826a5a 4c8b4d30 mov r9, qword ptr [rbp + 0x30]
0x140826a5e 49c1e138 shl r9, 0x38
0x140826a62 4c0b4d38 or r9, qword ptr [rbp + 0x38]
0x140826a66 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x140826a6a 488b4518 mov rax, qword ptr [rbp + 0x18]
0x140826a6e 4c31c8 xor rax, r9
0x140826a71 488b5500 mov rdx, qword ptr [rbp]
0x140826a75 4801ca add rdx, rcx
0x140826a78 4c8b4508 mov r8, qword ptr [rbp + 8]
0x140826a7c 48c1c10d rol rcx, 0xd
0x140826a80 4901c0 add r8, rax
0x140826a83 4831d1 xor rcx, rdx
0x140826a86 48c1c010 rol rax, 0x10
0x140826a8a 4c31c0 xor rax, r8
0x140826a8d 48c1c220 rol rdx, 0x20
0x140826a91 4901c8 add r8, rcx
0x140826a94 4801c2 add rdx, rax
0x140826a97 48c1c111 rol rcx, 0x11
0x140826a9b 4c31c1 xor rcx, r8
0x140826a9e 48c1c015 rol rax, 0x15
0x140826aa2 49c1c020 rol r8, 0x20
0x140826aa6 4831d0 xor rax, rdx
0x140826aa9 4c31ca xor rdx, r9
0x140826aac 4981f0ff000000 xor r8, 0xff
0x140826ab3 4801ca add rdx, rcx
0x140826ab6 4901c0 add r8, rax
0x140826ab9 48c1c10d rol rcx, 0xd
0x140826abd 4831d1 xor rcx, rdx
0x140826ac0 48c1c010 rol rax, 0x10
0x140826ac4 48c1c220 rol rdx, 0x20
0x140826ac8 4c31c0 xor rax, r8
0x140826acb 4901c8 add r8, rcx
0x140826ace 48c1c111 rol rcx, 0x11
0x140826ad2 4801c2 add rdx, rax
0x140826ad5 4c31c1 xor rcx, r8
0x140826ad8 48c1c015 rol rax, 0x15
0x140826adc 4831d0 xor rax, rdx
0x140826adf 49c1c020 rol r8, 0x20
0x140826ae3 4801ca add rdx, rcx
0x140826ae6 4901c0 add r8, rax
0x140826ae9 48c1c10d rol rcx, 0xd
0x140826aed 4831d1 xor rcx, rdx
0x140826af0 48c1c010 rol rax, 0x10
0x140826af4 48c1c220 rol rdx, 0x20
0x140826af8 4c31c0 xor rax, r8
0x140826afb 4901c8 add r8, rcx
0x140826afe 48c1c111 rol rcx, 0x11
0x140826b02 4801c2 add rdx, rax
0x140826b05 4c31c1 xor rcx, r8
0x140826b08 48c1c015 rol rax, 0x15
0x140826b0c 4831d0 xor rax, rdx
0x140826b0f 49c1c020 rol r8, 0x20
0x140826b13 4801ca add rdx, rcx
0x140826b16 4901c0 add r8, rax
0x140826b19 48c1c10d rol rcx, 0xd
0x140826b1d 4831d1 xor rcx, rdx
0x140826b20 48c1c010 rol rax, 0x10
0x140826b24 4c31c0 xor rax, r8
0x140826b27 4901c8 add r8, rcx
0x140826b2a 48c1c111 rol rcx, 0x11
0x140826b2e 48c1c015 rol rax, 0x15
0x140826b32 4c89c2 mov rdx, r8
0x140826b35 48c1c220 rol rdx, 0x20
0x140826b39 4831ca xor rdx, rcx
0x140826b3c 4831c2 xor rdx, rax
0x140826b3f 4c31c2 xor rdx, r8
0x140826b42 4889f1 mov rcx, rsi
0x140826b45 4c8d4574 lea r8, [rbp + 0x74]
0x140826b49 e8d2dca300 call 0x141264820
0x140826b4e 48895500 mov qword ptr [rbp], rdx
0x140826b52 4885d2 test rdx, rdx
0x140826b55 0f8455feffff je 0x1408269b0
0x140826b5b f048ff0a lock dec qword ptr [rdx]
0x140826b5f 0f854bfeffff jne 0x1408269b0
0x140826b65 4889e9 mov rcx, rbp
0x140826b68 e833460000 call 0x14082b1a0
0x140826b6d e93efeffff jmp 0x1408269b0
0x140826b72 48837dd000 cmp qword ptr [rbp - 0x30], 0
0x140826b77 743e je 0x140826bb7
0x140826b79 e8b2153f00 call 0x140c18130
0x140826b7e 4889c1 mov rcx, rax
0x140826b81 488b00 mov rax, qword ptr [rax]
0x140826b84 4883f801 cmp rax, 1
0x140826b88 7557 jne 0x140826be1
0x140826b8a 4883c108 add rcx, 8
0x140826b8e 48833900 cmp qword ptr [rcx], 0
0x140826b92 0f8583000000 jne 0x140826c1b
0x140826b98 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x140826b9f 48894d60 mov qword ptr [rbp + 0x60], rcx
0x140826ba3 4883c108 add rcx, 8
0x140826ba7 488b55d8 mov rdx, qword ptr [rbp - 0x28]
0x140826bab e840053f00 call 0x140c170f0
0x140826bb0 488b4560 mov rax, qword ptr [rbp + 0x60]
0x140826bb4 48ff00 inc qword ptr [rax]
0x140826bb7 0f28b580000000 movaps xmm6, xmmword ptr [rbp + 0x80]
0x140826bbe 0f28bd90000000 movaps xmm7, xmmword ptr [rbp + 0x90]
0x140826bc5 440f2885a0000000 movaps xmm8, xmmword ptr [rbp + 0xa0]
0x140826bcd 4881c438010000 add rsp, 0x138
0x140826bd4 5b pop rbx
0x140826bd5 5f pop rdi
0x140826bd6 5e pop rsi
0x140826bd7 415c pop r12
0x140826bd9 415d pop r13
0x140826bdb 415e pop r14
0x140826bdd 415f pop r15
0x140826bdf 5d pop rbp
0x140826be0 c3 ret 
0x140826be1 4885c0 test rax, rax
0x140826be4 7529 jne 0x140826c0f
0x140826be6 31d2 xor edx, edx
0x140826be8 e893b90c02 call 0x1428f2580
0x140826bed 4889c1 mov rcx, rax
0x140826bf0 4885c0 test rax, rax
0x140826bf3 7599 jne 0x140826b8e
0x140826bf5 eb18 jmp 0x140826c0f
0x140826bf7 4885c0 test rax, rax
0x140826bfa 7513 jne 0x140826c0f
0x140826bfc 31d2 xor edx, edx
0x140826bfe e87db90c02 call 0x1428f2580
0x140826c03 4889c1 mov rcx, rax
0x140826c06 4885c0 test rax, rax
0x140826c09 0f85ddf6ffff jne 0x1408262ec
0x140826c0f 488d0db2a73f02 lea rcx, [rip + 0x23fa7b2]
0x140826c16 e8b5140b02 call 0x1428d80d0
0x140826c1b 488d0d16a83f02 lea rcx, [rip + 0x23fa816]
0x140826c22 e829260b02 call 0x1428d9250
0x140826c27 cc int3 
0x140826c28 0f1f840000000000 nop dword ptr [rax + rax]
0x140826c30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140826c35 55 push rbp
0x140826c36 4157 push r15
0x140826c38 4156 push r14
0x140826c3a 4155 push r13
0x140826c3c 4154 push r12
0x140826c3e 56 push rsi
0x140826c3f 57 push rdi
0x140826c40 53 push rbx
0x140826c41 4883ec68 sub rsp, 0x68
0x140826c45 488daa80000000 lea rbp, [rdx + 0x80]
0x140826c4c 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x140826c52 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
0x140826c58 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x140826c5e 488b4560 mov rax, qword ptr [rbp + 0x60]
0x140826c62 48ff00 inc qword ptr [rax]
0x140826c65 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140826c6a 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x140826c6f 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x140826c75 4883c468 add rsp, 0x68
0x140826c79 5b pop rbx
0x140826c7a 5f pop rdi
0x140826c7b 5e pop rsi
0x140826c7c 415c pop r12
0x140826c7e 415d pop r13
0x140826c80 415e pop r14
0x140826c82 415f pop r15
0x140826c84 5d pop rbp
0x140826c85 c3 ret 
0x140826c86 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140826c90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140826c95 55 push rbp
0x140826c96 4157 push r15
0x140826c98 4156 push r14
0x140826c9a 4155 push r13
0x140826c9c 4154 push r12
0x140826c9e 56 push rsi
0x140826c9f 57 push rdi
0x140826ca0 53 push rbx
0x140826ca1 4883ec68 sub rsp, 0x68
0x140826ca5 488daa80000000 lea rbp, [rdx + 0x80]
0x140826cac 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x140826cb2 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
0x140826cb8 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x140826cbe 48837dd000 cmp qword ptr [rbp - 0x30], 0
0x140826cc3 7420 je 0x140826ce5
0x140826cc5 488d45d8 lea rax, [rbp - 0x28]
0x140826cc9 488945c0 mov qword ptr [rbp - 0x40], rax
0x140826ccd 488d45c0 lea rax, [rbp - 0x40]
0x140826cd1 48894548 mov qword ptr [rbp + 0x48], rax
0x140826cd5 488d0d34a82602 lea rcx, [rip + 0x226a834]
0x140826cdc 488d5548 lea rdx, [rbp + 0x48]
0x140826ce0 e8db0ea300 call 0x141257bc0
0x140826ce5 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140826cea 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x140826cef 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x140826cf5 4883c468 add rsp, 0x68
0x140826cf9 5b pop rbx
0x140826cfa 5f pop rdi
0x140826cfb 5e pop rsi
0x140826cfc 415c pop r12
0x140826cfe 415d pop r13
0x140826d00 415e pop r14
0x140826d02 415f pop r15
0x140826d04 5d pop rbp
0x140826d05 c3 ret 
0x140826d06 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140826d10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140826d15 55 push rbp
0x140826d16 4157 push r15
0x140826d18 4156 push r14
0x140826d1a 4155 push r13
0x140826d1c 4154 push r12
0x140826d1e 56 push rsi
0x140826d1f 57 push rdi
0x140826d20 53 push rbx
0x140826d21 4883ec68 sub rsp, 0x68
0x140826d25 488daa80000000 lea rbp, [rdx + 0x80]
0x140826d2c 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
0x140826d32 660f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm7
0x140826d38 660f7f742450 movdqa xmmword ptr [rsp + 0x50], xmm6
0x140826d3e 488b4560 mov rax, qword ptr [rbp + 0x60]
0x140826d42 48ff00 inc qword ptr [rax]
0x140826d45 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140826d4a 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
0x140826d4f 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
0x140826d55 4883c468 add rsp, 0x68
0x140826d59 5b pop rbx
0x140826d5a 5f pop rdi
0x140826d5b 5e pop rsi
0x140826d5c 415c pop r12
0x140826d5e 415d pop r13
0x140826d60 415e pop r14
0x140826d62 415f pop r15
0x140826d64 5d pop rbp
0x140826d65 c3 ret 
