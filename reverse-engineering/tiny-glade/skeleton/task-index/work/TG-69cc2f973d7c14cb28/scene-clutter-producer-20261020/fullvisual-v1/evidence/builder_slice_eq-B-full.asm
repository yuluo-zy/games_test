0x1412a92a0 4157 push r15
0x1412a92a2 4156 push r14
0x1412a92a4 4155 push r13
0x1412a92a6 4154 push r12
0x1412a92a8 56 push rsi
0x1412a92a9 57 push rdi
0x1412a92aa 55 push rbp
0x1412a92ab 53 push rbx
0x1412a92ac 4883ec28 sub rsp, 0x28
0x1412a92b0 4c39ca cmp rdx, r9
0x1412a92b3 0f8556080000 jne 0x1412a9b0f
0x1412a92b9 4885d2 test rdx, rdx
0x1412a92bc 0f8449080000 je 0x1412a9b0b
0x1412a92c2 4c89c6 mov rsi, r8
0x1412a92c5 4889cf mov rdi, rcx
0x1412a92c8 498d9850010000 lea rbx, [r8 + 0x150]
0x1412a92cf 4d8d7028 lea r14, [r8 + 0x28]
0x1412a92d3 4883c608 add rsi, 8
0x1412a92d7 4c8db950010000 lea r15, [rcx + 0x150]
0x1412a92de 4c8d6128 lea r12, [rcx + 0x28]
0x1412a92e2 4883c708 add rdi, 8
0x1412a92e6 4531ed xor r13d, r13d
0x1412a92e9 f3430f10442f80 movss xmm0, dword ptr [r15 + r13 - 0x80]
0x1412a92f0 420f2e442b80 ucomiss xmm0, dword ptr [rbx + r13 - 0x80]
0x1412a92f6 0f8513080000 jne 0x1412a9b0f
0x1412a92fc 0f8a0d080000 jp 0x1412a9b0f
0x1412a9302 f3430f10442f84 movss xmm0, dword ptr [r15 + r13 - 0x7c]
0x1412a9309 420f2e442b84 ucomiss xmm0, dword ptr [rbx + r13 - 0x7c]
0x1412a930f 0f85fa070000 jne 0x1412a9b0f
0x1412a9315 0f8af4070000 jp 0x1412a9b0f
0x1412a931b f3430f10442f88 movss xmm0, dword ptr [r15 + r13 - 0x78]
0x1412a9322 420f2e442b88 ucomiss xmm0, dword ptr [rbx + r13 - 0x78]
0x1412a9328 0f85e1070000 jne 0x1412a9b0f
0x1412a932e 0f8adb070000 jp 0x1412a9b0f
0x1412a9334 420f28842b70ffffff movaps xmm0, xmmword ptr [rbx + r13 - 0x90]
0x1412a933d 430fc2842f70ffffff00 cmpeqps xmm0, xmmword ptr [r15 + r13 - 0x90]
0x1412a9347 0f50c0 movmskps eax, xmm0
0x1412a934a 83f80f cmp eax, 0xf
0x1412a934d 0f85bc070000 jne 0x1412a9b0f
0x1412a9353 f3430f10442f8c movss xmm0, dword ptr [r15 + r13 - 0x74]
0x1412a935a 420f2e442b8c ucomiss xmm0, dword ptr [rbx + r13 - 0x74]
0x1412a9360 0f85a9070000 jne 0x1412a9b0f
0x1412a9366 0f8aa3070000 jp 0x1412a9b0f
0x1412a936c f3430f10442f90 movss xmm0, dword ptr [r15 + r13 - 0x70]
0x1412a9373 420f2e442b90 ucomiss xmm0, dword ptr [rbx + r13 - 0x70]
0x1412a9379 0f8590070000 jne 0x1412a9b0f
0x1412a937f 0f8a8a070000 jp 0x1412a9b0f
0x1412a9385 f3430f10442f94 movss xmm0, dword ptr [r15 + r13 - 0x6c]
0x1412a938c 420f2e442b94 ucomiss xmm0, dword ptr [rbx + r13 - 0x6c]
0x1412a9392 0f8577070000 jne 0x1412a9b0f
0x1412a9398 0f8a71070000 jp 0x1412a9b0f
0x1412a939e 430fb6442ffe movzx eax, byte ptr [r15 + r13 - 2]
0x1412a93a4 423a442bfe cmp al, byte ptr [rbx + r13 - 2]
0x1412a93a9 0f8560070000 jne 0x1412a9b0f
0x1412a93af 420fb6442ba0 movzx eax, byte ptr [rbx + r13 - 0x60]
0x1412a93b5 43f6442fa001 test byte ptr [r15 + r13 - 0x60], 1
0x1412a93bb 7418 je 0x1412a93d5
0x1412a93bd 428b4c2ba4 mov ecx, dword ptr [rbx + r13 - 0x5c]
0x1412a93c2 43394c2fa4 cmp dword ptr [r15 + r13 - 0x5c], ecx
0x1412a93c7 0f94c1 sete cl
0x1412a93ca 20c8 and al, cl
0x1412a93cc 3c01 cmp al, 1
0x1412a93ce 740d je 0x1412a93dd
0x1412a93d0 e93a070000 jmp 0x1412a9b0f
0x1412a93d5 a801 test al, 1
0x1412a93d7 0f8532070000 jne 0x1412a9b0f
0x1412a93dd f3430f10442ff0 movss xmm0, dword ptr [r15 + r13 - 0x10]
0x1412a93e4 420f2e442bf0 ucomiss xmm0, dword ptr [rbx + r13 - 0x10]
0x1412a93ea 0f851f070000 jne 0x1412a9b0f
0x1412a93f0 0f8a19070000 jp 0x1412a9b0f
0x1412a93f6 f3430f10442ff4 movss xmm0, dword ptr [r15 + r13 - 0xc]
0x1412a93fd 420f2e442bf4 ucomiss xmm0, dword ptr [rbx + r13 - 0xc]
0x1412a9403 0f8506070000 jne 0x1412a9b0f
0x1412a9409 0f8a00070000 jp 0x1412a9b0f
0x1412a940f f3430f10442ff8 movss xmm0, dword ptr [r15 + r13 - 8]
0x1412a9416 420f2e442bf8 ucomiss xmm0, dword ptr [rbx + r13 - 8]
0x1412a941c 0f85ed060000 jne 0x1412a9b0f
0x1412a9422 0f8ae7060000 jp 0x1412a9b0f
0x1412a9428 4f8b842f58ffffff mov r8, qword ptr [r15 + r13 - 0xa8]
0x1412a9430 4e3b842b58ffffff cmp r8, qword ptr [rbx + r13 - 0xa8]
0x1412a9438 0f85d1060000 jne 0x1412a9b0f
0x1412a943e 4889d5 mov rbp, rdx
0x1412a9441 4a8b942b50ffffff mov rdx, qword ptr [rbx + r13 - 0xb0]
0x1412a9449 4b8b8c2f50ffffff mov rcx, qword ptr [r15 + r13 - 0xb0]
0x1412a9451 e89aa76701 call 0x142923bf0
0x1412a9456 85c0 test eax, eax
0x1412a9458 0f85b1060000 jne 0x1412a9b0f
0x1412a945e 420fb6842bb0feffff movzx eax, byte ptr [rbx + r13 - 0x150]
0x1412a9467 4383bc2fb0feffff01 cmp dword ptr [r15 + r13 - 0x150], 1
0x1412a9470 751e jne 0x1412a9490
0x1412a9472 a801 test al, 1
0x1412a9474 0f8495060000 je 0x1412a9b0f
0x1412a947a 4a8d0c2f lea rcx, [rdi + r13]
0x1412a947e 4a8d142e lea rdx, [rsi + r13]
0x1412a9482 e8e9389fff call 0x140c9cd70
0x1412a9487 84c0 test al, al
0x1412a9489 750d jne 0x1412a9498
0x1412a948b e97f060000 jmp 0x1412a9b0f
0x1412a9490 a801 test al, 1
0x1412a9492 0f8577060000 jne 0x1412a9b0f
0x1412a9498 4f8b842f68ffffff mov r8, qword ptr [r15 + r13 - 0x98]
0x1412a94a0 4e3b842b68ffffff cmp r8, qword ptr [rbx + r13 - 0x98]
0x1412a94a8 0f8561060000 jne 0x1412a9b0f
0x1412a94ae 4a8b942b60ffffff mov rdx, qword ptr [rbx + r13 - 0xa0]
0x1412a94b6 4b8b8c2f60ffffff mov rcx, qword ptr [r15 + r13 - 0xa0]
0x1412a94be e82da76701 call 0x142923bf0
0x1412a94c3 85c0 test eax, eax
0x1412a94c5 0f8544060000 jne 0x1412a9b0f
0x1412a94cb 420fb6842be0feffff movzx eax, byte ptr [rbx + r13 - 0x120]
0x1412a94d4 43f6842fe0feffff01 test byte ptr [r15 + r13 - 0x120], 1
0x1412a94dd 7469 je 0x1412a9548
0x1412a94df a801 test al, 1
0x1412a94e1 0f8428060000 je 0x1412a9b0f
0x1412a94e7 f3430f10842fe4feffff movss xmm0, dword ptr [r15 + r13 - 0x11c]
0x1412a94f1 420f2e842be4feffff ucomiss xmm0, dword ptr [rbx + r13 - 0x11c]
0x1412a94fa 0f850f060000 jne 0x1412a9b0f
0x1412a9500 0f8a09060000 jp 0x1412a9b0f
0x1412a9506 f3430f10842fe8feffff movss xmm0, dword ptr [r15 + r13 - 0x118]
0x1412a9510 420f2e842be8feffff ucomiss xmm0, dword ptr [rbx + r13 - 0x118]
0x1412a9519 0f85f0050000 jne 0x1412a9b0f
0x1412a951f 0f8aea050000 jp 0x1412a9b0f
0x1412a9525 4889ea mov rdx, rbp
0x1412a9528 f3430f10842fecfeffff movss xmm0, dword ptr [r15 + r13 - 0x114]
0x1412a9532 420f2e842becfeffff ucomiss xmm0, dword ptr [rbx + r13 - 0x114]
0x1412a953b 0f85ce050000 jne 0x1412a9b0f
0x1412a9541 7b10 jnp 0x1412a9553
0x1412a9543 e9c7050000 jmp 0x1412a9b0f
0x1412a9548 a801 test al, 1
0x1412a954a 4889ea mov rdx, rbp
0x1412a954d 0f85bc050000 jne 0x1412a9b0f
0x1412a9553 420fb6842bf0feffff movzx eax, byte ptr [rbx + r13 - 0x110]
0x1412a955c 43f6842ff0feffff01 test byte ptr [r15 + r13 - 0x110], 1
0x1412a9565 0f84e0010000 je 0x1412a974b
0x1412a956b a801 test al, 1
0x1412a956d 0f849c050000 je 0x1412a9b0f
0x1412a9573 f3430f10842f0cffffff movss xmm0, dword ptr [r15 + r13 - 0xf4]
0x1412a957d 420f2e842b0cffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xf4]
0x1412a9586 0f8583050000 jne 0x1412a9b0f
0x1412a958c 0f8a7d050000 jp 0x1412a9b0f
0x1412a9592 f3430f10842f10ffffff movss xmm0, dword ptr [r15 + r13 - 0xf0]
0x1412a959c 420f2e842b10ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xf0]
0x1412a95a5 0f8564050000 jne 0x1412a9b0f
0x1412a95ab 0f8a5e050000 jp 0x1412a9b0f
0x1412a95b1 f3430f10842f14ffffff movss xmm0, dword ptr [r15 + r13 - 0xec]
0x1412a95bb 420f2e842b14ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xec]
0x1412a95c4 0f8545050000 jne 0x1412a9b0f
0x1412a95ca 0f8a3f050000 jp 0x1412a9b0f
0x1412a95d0 f3430f10842ff4feffff movss xmm0, dword ptr [r15 + r13 - 0x10c]
0x1412a95da 31c0 xor eax, eax
0x1412a95dc 420f2e842bf4feffff ucomiss xmm0, dword ptr [rbx + r13 - 0x10c]
0x1412a95e5 0f8526050000 jne 0x1412a9b11
0x1412a95eb 0f8a20050000 jp 0x1412a9b11
0x1412a95f1 f3430f10842ff8feffff movss xmm0, dword ptr [r15 + r13 - 0x108]
0x1412a95fb 420f2e842bf8feffff ucomiss xmm0, dword ptr [rbx + r13 - 0x108]
0x1412a9604 0f8507050000 jne 0x1412a9b11
0x1412a960a 0f8a01050000 jp 0x1412a9b11
0x1412a9610 f3430f10842f18ffffff movss xmm0, dword ptr [r15 + r13 - 0xe8]
0x1412a961a 420f2e842b18ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xe8]
0x1412a9623 0f85e6040000 jne 0x1412a9b0f
0x1412a9629 0f8ae0040000 jp 0x1412a9b0f
0x1412a962f f3430f10842f1cffffff movss xmm0, dword ptr [r15 + r13 - 0xe4]
0x1412a9639 420f2e842b1cffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xe4]
0x1412a9642 0f85c7040000 jne 0x1412a9b0f
0x1412a9648 0f8ac1040000 jp 0x1412a9b0f
0x1412a964e f3430f10842f20ffffff movss xmm0, dword ptr [r15 + r13 - 0xe0]
0x1412a9658 420f2e842b20ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xe0]
0x1412a9661 0f85a8040000 jne 0x1412a9b0f
0x1412a9667 0f8aa2040000 jp 0x1412a9b0f
0x1412a966d f3430f10842ffcfeffff movss xmm0, dword ptr [r15 + r13 - 0x104]
0x1412a9677 31c0 xor eax, eax
0x1412a9679 420f2e842bfcfeffff ucomiss xmm0, dword ptr [rbx + r13 - 0x104]
0x1412a9682 0f8589040000 jne 0x1412a9b11
0x1412a9688 0f8a83040000 jp 0x1412a9b11
0x1412a968e f3430f10842f00ffffff movss xmm0, dword ptr [r15 + r13 - 0x100]
0x1412a9698 420f2e842b00ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0x100]
0x1412a96a1 0f856a040000 jne 0x1412a9b11
0x1412a96a7 0f8a64040000 jp 0x1412a9b11
0x1412a96ad f3430f10842f24ffffff movss xmm0, dword ptr [r15 + r13 - 0xdc]
0x1412a96b7 420f2e842b24ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xdc]
0x1412a96c0 0f8549040000 jne 0x1412a9b0f
0x1412a96c6 0f8a43040000 jp 0x1412a9b0f
0x1412a96cc f3430f10842f28ffffff movss xmm0, dword ptr [r15 + r13 - 0xd8]
0x1412a96d6 420f2e842b28ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xd8]
0x1412a96df 0f852a040000 jne 0x1412a9b0f
0x1412a96e5 0f8a24040000 jp 0x1412a9b0f
0x1412a96eb f3430f10842f2cffffff movss xmm0, dword ptr [r15 + r13 - 0xd4]
0x1412a96f5 420f2e842b2cffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xd4]
0x1412a96fe 0f850b040000 jne 0x1412a9b0f
0x1412a9704 0f8a05040000 jp 0x1412a9b0f
0x1412a970a f3430f10842f04ffffff movss xmm0, dword ptr [r15 + r13 - 0xfc]
0x1412a9714 31c0 xor eax, eax
0x1412a9716 420f2e842b04ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xfc]
0x1412a971f 0f85ec030000 jne 0x1412a9b11
0x1412a9725 0f8ae6030000 jp 0x1412a9b11
0x1412a972b f3430f10842f08ffffff movss xmm0, dword ptr [r15 + r13 - 0xf8]
0x1412a9735 420f2e842b08ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xf8]
0x1412a973e 0f85cd030000 jne 0x1412a9b11
0x1412a9744 7b0d jnp 0x1412a9753
0x1412a9746 e9c6030000 jmp 0x1412a9b11
0x1412a974b a801 test al, 1
0x1412a974d 0f85bc030000 jne 0x1412a9b0f
0x1412a9753 470fb6442fc4 movzx r8d, byte ptr [r15 + r13 - 0x3c]
0x1412a9759 4180f802 cmp r8b, 2
0x1412a975d 0f94c0 sete al
0x1412a9760 420fb64c2bc4 movzx ecx, byte ptr [rbx + r13 - 0x3c]
0x1412a9766 80f902 cmp cl, 2
0x1412a9769 410f94c1 sete r9b
0x1412a976d 4108c1 or r9b, al
0x1412a9770 7416 je 0x1412a9788
0x1412a9772 31c0 xor eax, eax
0x1412a9774 4180f802 cmp r8b, 2
0x1412a9778 0f8593030000 jne 0x1412a9b11
0x1412a977e 80f902 cmp cl, 2
0x1412a9781 747b je 0x1412a97fe
0x1412a9783 e989030000 jmp 0x1412a9b11
0x1412a9788 438b442fb0 mov eax, dword ptr [r15 + r13 - 0x50]
0x1412a978d 423b442bb0 cmp eax, dword ptr [rbx + r13 - 0x50]
0x1412a9792 0f8577030000 jne 0x1412a9b0f
0x1412a9798 438b442fb4 mov eax, dword ptr [r15 + r13 - 0x4c]
0x1412a979d 423b442bb4 cmp eax, dword ptr [rbx + r13 - 0x4c]
0x1412a97a2 0f8567030000 jne 0x1412a9b0f
0x1412a97a8 f3430f10442fb8 movss xmm0, dword ptr [r15 + r13 - 0x48]
0x1412a97af 420f2e442bb8 ucomiss xmm0, dword ptr [rbx + r13 - 0x48]
0x1412a97b5 0f8554030000 jne 0x1412a9b0f
0x1412a97bb 0f8a4e030000 jp 0x1412a9b0f
0x1412a97c1 f3430f10442fbc movss xmm0, dword ptr [r15 + r13 - 0x44]
0x1412a97c8 420f2e442bbc ucomiss xmm0, dword ptr [rbx + r13 - 0x44]
0x1412a97ce 0f853b030000 jne 0x1412a9b0f
0x1412a97d4 0f8a35030000 jp 0x1412a9b0f
0x1412a97da 31c0 xor eax, eax
0x1412a97dc 4138c8 cmp r8b, cl
0x1412a97df 0f852c030000 jne 0x1412a9b11
0x1412a97e5 f3430f10442fc0 movss xmm0, dword ptr [r15 + r13 - 0x40]
0x1412a97ec 420f2e442bc0 ucomiss xmm0, dword ptr [rbx + r13 - 0x40]
0x1412a97f2 0f8519030000 jne 0x1412a9b11
0x1412a97f8 0f8a13030000 jp 0x1412a9b11
0x1412a97fe 420fb6842b30ffffff movzx eax, byte ptr [rbx + r13 - 0xd0]
0x1412a9807 43f6842f30ffffff01 test byte ptr [r15 + r13 - 0xd0], 1
0x1412a9810 745d je 0x1412a986f
0x1412a9812 a801 test al, 1
0x1412a9814 0f84f5020000 je 0x1412a9b0f
0x1412a981a f3430f10842f34ffffff movss xmm0, dword ptr [r15 + r13 - 0xcc]
0x1412a9824 420f2e842b34ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xcc]
0x1412a982d 0f85dc020000 jne 0x1412a9b0f
0x1412a9833 0f8ad6020000 jp 0x1412a9b0f
0x1412a9839 f3430f10842f38ffffff movss xmm0, dword ptr [r15 + r13 - 0xc8]
0x1412a9843 420f2e842b38ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xc8]
0x1412a984c 0f85bd020000 jne 0x1412a9b0f
0x1412a9852 0f8ab7020000 jp 0x1412a9b0f
0x1412a9858 438b842f3cffffff mov eax, dword ptr [r15 + r13 - 0xc4]
0x1412a9860 423b842b3cffffff cmp eax, dword ptr [rbx + r13 - 0xc4]
0x1412a9868 740d je 0x1412a9877
0x1412a986a e9a0020000 jmp 0x1412a9b0f
0x1412a986f a801 test al, 1
0x1412a9871 0f8598020000 jne 0x1412a9b0f
0x1412a9877 420fb6842bc0feffff movzx eax, byte ptr [rbx + r13 - 0x140]
0x1412a9880 43f6842fc0feffff01 test byte ptr [r15 + r13 - 0x140], 1
0x1412a9889 741e je 0x1412a98a9
0x1412a988b 4a8b8c2bc8feffff mov rcx, qword ptr [rbx + r13 - 0x138]
0x1412a9893 4b398c2fc8feffff cmp qword ptr [r15 + r13 - 0x138], rcx
0x1412a989b 0f94c1 sete cl
0x1412a989e 20c8 and al, cl
0x1412a98a0 3c01 cmp al, 1
0x1412a98a2 740d je 0x1412a98b1
0x1412a98a4 e966020000 jmp 0x1412a9b0f
0x1412a98a9 a801 test al, 1
0x1412a98ab 0f855e020000 jne 0x1412a9b0f
0x1412a98b1 430fb6442fff movzx eax, byte ptr [r15 + r13 - 1]
0x1412a98b7 423a442bff cmp al, byte ptr [rbx + r13 - 1]
0x1412a98bc 0f854d020000 jne 0x1412a9b0f
0x1412a98c2 470fb6442ffc movzx r8d, byte ptr [r15 + r13 - 4]
0x1412a98c8 420fb64c2bfc movzx ecx, byte ptr [rbx + r13 - 4]
0x1412a98ce 4180f802 cmp r8b, 2
0x1412a98d2 0f94c0 sete al
0x1412a98d5 80f902 cmp cl, 2
0x1412a98d8 410f94c1 sete r9b
0x1412a98dc 4108c1 or r9b, al
0x1412a98df 7416 je 0x1412a98f7
0x1412a98e1 31c0 xor eax, eax
0x1412a98e3 4180f802 cmp r8b, 2
0x1412a98e7 0f8524020000 jne 0x1412a9b11
0x1412a98ed 80f902 cmp cl, 2
0x1412a98f0 7421 je 0x1412a9913
0x1412a98f2 e91a020000 jmp 0x1412a9b11
0x1412a98f7 31c0 xor eax, eax
0x1412a98f9 4138c8 cmp r8b, cl
0x1412a98fc 0f850f020000 jne 0x1412a9b11
0x1412a9902 420fb64c2bfd movzx ecx, byte ptr [rbx + r13 - 3]
0x1412a9908 43384c2ffd cmp byte ptr [r15 + r13 - 3], cl
0x1412a990d 0f85fe010000 jne 0x1412a9b11
0x1412a9913 420fb6442ba8 movzx eax, byte ptr [rbx + r13 - 0x58]
0x1412a9919 43f6442fa801 test byte ptr [r15 + r13 - 0x58], 1
0x1412a991f 7418 je 0x1412a9939
0x1412a9921 428b4c2bac mov ecx, dword ptr [rbx + r13 - 0x54]
0x1412a9926 43394c2fac cmp dword ptr [r15 + r13 - 0x54], ecx
0x1412a992b 0f94c1 sete cl
0x1412a992e 20c8 and al, cl
0x1412a9930 3c01 cmp al, 1
0x1412a9932 740d je 0x1412a9941
0x1412a9934 e9d6010000 jmp 0x1412a9b0f
0x1412a9939 a801 test al, 1
0x1412a993b 0f85ce010000 jne 0x1412a9b0f
0x1412a9941 470fb6442fec movzx r8d, byte ptr [r15 + r13 - 0x14]
0x1412a9947 4180f802 cmp r8b, 2
0x1412a994b 0f94c0 sete al
0x1412a994e 420fb64c2bec movzx ecx, byte ptr [rbx + r13 - 0x14]
0x1412a9954 80f902 cmp cl, 2
0x1412a9957 410f94c1 sete r9b
0x1412a995b 4108c1 or r9b, al
0x1412a995e 7416 je 0x1412a9976
0x1412a9960 31c0 xor eax, eax
0x1412a9962 4180f802 cmp r8b, 2
0x1412a9966 0f85a5010000 jne 0x1412a9b11
0x1412a996c 80f902 cmp cl, 2
0x1412a996f 746b je 0x1412a99dc
0x1412a9971 e99b010000 jmp 0x1412a9b11
0x1412a9976 438b442fdc mov eax, dword ptr [r15 + r13 - 0x24]
0x1412a997b 423b442bdc cmp eax, dword ptr [rbx + r13 - 0x24]
0x1412a9980 0f8589010000 jne 0x1412a9b0f
0x1412a9986 f3430f10442fe0 movss xmm0, dword ptr [r15 + r13 - 0x20]
0x1412a998d 420f2e442be0 ucomiss xmm0, dword ptr [rbx + r13 - 0x20]
0x1412a9993 0f8576010000 jne 0x1412a9b0f
0x1412a9999 0f8a70010000 jp 0x1412a9b0f
0x1412a999f f3430f10442fe4 movss xmm0, dword ptr [r15 + r13 - 0x1c]
0x1412a99a6 420f2e442be4 ucomiss xmm0, dword ptr [rbx + r13 - 0x1c]
0x1412a99ac 0f855d010000 jne 0x1412a9b0f
0x1412a99b2 0f8a57010000 jp 0x1412a9b0f
0x1412a99b8 31c0 xor eax, eax
0x1412a99ba 4138c8 cmp r8b, cl
0x1412a99bd 0f854e010000 jne 0x1412a9b11
0x1412a99c3 f3430f10442fe8 movss xmm0, dword ptr [r15 + r13 - 0x18]
0x1412a99ca 420f2e442be8 ucomiss xmm0, dword ptr [rbx + r13 - 0x18]
0x1412a99d0 0f853b010000 jne 0x1412a9b11
0x1412a99d6 0f8a35010000 jp 0x1412a9b11
0x1412a99dc 420fb6842b40ffffff movzx eax, byte ptr [rbx + r13 - 0xc0]
0x1412a99e5 43f6842f40ffffff01 test byte ptr [r15 + r13 - 0xc0], 1
0x1412a99ee 7466 je 0x1412a9a56
0x1412a99f0 a801 test al, 1
0x1412a99f2 0f8417010000 je 0x1412a9b0f
0x1412a99f8 f3430f10842f44ffffff movss xmm0, dword ptr [r15 + r13 - 0xbc]
0x1412a9a02 420f2e842b44ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xbc]
0x1412a9a0b 0f85fe000000 jne 0x1412a9b0f
0x1412a9a11 0f8af8000000 jp 0x1412a9b0f
0x1412a9a17 f3430f10842f48ffffff movss xmm0, dword ptr [r15 + r13 - 0xb8]
0x1412a9a21 420f2e842b48ffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xb8]
0x1412a9a2a 0f85df000000 jne 0x1412a9b0f
0x1412a9a30 0f8ad9000000 jp 0x1412a9b0f
0x1412a9a36 f3430f10842f4cffffff movss xmm0, dword ptr [r15 + r13 - 0xb4]
0x1412a9a40 420f2e842b4cffffff ucomiss xmm0, dword ptr [rbx + r13 - 0xb4]
0x1412a9a49 0f85c0000000 jne 0x1412a9b0f
0x1412a9a4f 7b0d jnp 0x1412a9a5e
0x1412a9a51 e9b9000000 jmp 0x1412a9b0f
0x1412a9a56 a801 test al, 1
0x1412a9a58 0f85b1000000 jne 0x1412a9b0f
0x1412a9a5e 420fb6842bd0feffff movzx eax, byte ptr [rbx + r13 - 0x130]
0x1412a9a67 4383bc2fd0feffff01 cmp dword ptr [r15 + r13 - 0x130], 1
0x1412a9a70 751e jne 0x1412a9a90
0x1412a9a72 a801 test al, 1
0x1412a9a74 0f8495000000 je 0x1412a9b0f
0x1412a9a7a 4b8d0c2c lea rcx, [r12 + r13]
0x1412a9a7e 4b8d142e lea rdx, [r14 + r13]
0x1412a9a82 e8e9329fff call 0x140c9cd70
0x1412a9a87 4889ea mov rdx, rbp
0x1412a9a8a 84c0 test al, al
0x1412a9a8c 7506 jne 0x1412a9a94
0x1412a9a8e eb7f jmp 0x1412a9b0f
0x1412a9a90 a801 test al, 1
0x1412a9a92 757b jne 0x1412a9b0f
0x1412a9a94 420fb6042b movzx eax, byte ptr [rbx + r13]
0x1412a9a99 433a042f cmp al, byte ptr [r15 + r13]
0x1412a9a9d 7570 jne 0x1412a9b0f
0x1412a9a9f 420fb6442bc8 movzx eax, byte ptr [rbx + r13 - 0x38]
0x1412a9aa5 43f6442fc801 test byte ptr [r15 + r13 - 0x38], 1
0x1412a9aab 744a je 0x1412a9af7
0x1412a9aad a801 test al, 1
0x1412a9aaf 745e je 0x1412a9b0f
0x1412a9ab1 f3430f10442fcc movss xmm0, dword ptr [r15 + r13 - 0x34]
0x1412a9ab8 420f2e442bcc ucomiss xmm0, dword ptr [rbx + r13 - 0x34]
0x1412a9abe 754f jne 0x1412a9b0f
0x1412a9ac0 7a4d jp 0x1412a9b0f
0x1412a9ac2 f3430f10442fd0 movss xmm0, dword ptr [r15 + r13 - 0x30]
0x1412a9ac9 420f2e442bd0 ucomiss xmm0, dword ptr [rbx + r13 - 0x30]
0x1412a9acf 753e jne 0x1412a9b0f
0x1412a9ad1 7a3c jp 0x1412a9b0f
0x1412a9ad3 f3430f10442fd4 movss xmm0, dword ptr [r15 + r13 - 0x2c]
0x1412a9ada 420f2e442bd4 ucomiss xmm0, dword ptr [rbx + r13 - 0x2c]
0x1412a9ae0 752d jne 0x1412a9b0f
0x1412a9ae2 7a2b jp 0x1412a9b0f
0x1412a9ae4 f3430f10442fd8 movss xmm0, dword ptr [r15 + r13 - 0x28]
0x1412a9aeb 420f2e442bd8 ucomiss xmm0, dword ptr [rbx + r13 - 0x28]
0x1412a9af1 751c jne 0x1412a9b0f
0x1412a9af3 7b06 jnp 0x1412a9afb
0x1412a9af5 eb18 jmp 0x1412a9b0f
0x1412a9af7 a801 test al, 1
0x1412a9af9 7514 jne 0x1412a9b0f
0x1412a9afb 4981c560010000 add r13, 0x160
0x1412a9b02 48ffca dec rdx
0x1412a9b05 0f85def7ffff jne 0x1412a92e9
0x1412a9b0b b001 mov al, 1
0x1412a9b0d eb02 jmp 0x1412a9b11
0x1412a9b0f 31c0 xor eax, eax
0x1412a9b11 4883c428 add rsp, 0x28
0x1412a9b15 5b pop rbx
0x1412a9b16 5d pop rbp
0x1412a9b17 5f pop rdi
0x1412a9b18 5e pop rsi
0x1412a9b19 415c pop r12
0x1412a9b1b 415d pop r13
0x1412a9b1d 415e pop r14
0x1412a9b1f 415f pop r15
0x1412a9b21 c3 ret 
