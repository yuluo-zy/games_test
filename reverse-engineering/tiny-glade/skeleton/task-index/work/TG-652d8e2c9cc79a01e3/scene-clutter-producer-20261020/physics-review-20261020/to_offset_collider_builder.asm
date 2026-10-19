0x1408d0390 56 push rsi
0x1408d0391 57 push rdi
0x1408d0392 4881eca8000000 sub rsp, 0xa8
0x1408d0399 4889d7 mov rdi, rdx
0x1408d039c 498b5020 mov rdx, qword ptr [r8 + 0x20]
0x1408d03a0 4d8b4028 mov r8, qword ptr [r8 + 0x28]
0x1408d03a4 f048ff02 lock inc qword ptr [rdx]
0x1408d03a8 0f8ed1000000 jle 0x1408d047f
0x1408d03ae 4889ce mov rsi, rcx
0x1408d03b1 488d4c2420 lea rcx, [rsp + 0x20]
0x1408d03b6 e8e52b2c00 call 0x140b92fa0
0x1408d03bb c684249600000001 mov byte ptr [rsp + 0x96], 1
0x1408d03c3 488b442460 mov rax, qword ptr [rsp + 0x60]
0x1408d03c8 48894660 mov qword ptr [rsi + 0x60], rax
0x1408d03cc 0f28442420 movaps xmm0, xmmword ptr [rsp + 0x20]
0x1408d03d1 0f284c2430 movaps xmm1, xmmword ptr [rsp + 0x30]
0x1408d03d6 0f28542440 movaps xmm2, xmmword ptr [rsp + 0x40]
0x1408d03db 0f285c2450 movaps xmm3, xmmword ptr [rsp + 0x50]
0x1408d03e0 0f295e50 movaps xmmword ptr [rsi + 0x50], xmm3
0x1408d03e4 0f295640 movaps xmmword ptr [rsi + 0x40], xmm2
0x1408d03e8 0f294e30 movaps xmmword ptr [rsi + 0x30], xmm1
0x1408d03ec 0f294620 movaps xmmword ptr [rsi + 0x20], xmm0
0x1408d03f0 0f10842484000000 movups xmm0, xmmword ptr [rsp + 0x84]
0x1408d03f8 0f118684000000 movups xmmword ptr [rsi + 0x84], xmm0
0x1408d03ff 8b842490000000 mov eax, dword ptr [rsp + 0x90]
0x1408d0406 898690000000 mov dword ptr [rsi + 0x90], eax
0x1408d040c 0fb7842494000000 movzx eax, word ptr [rsp + 0x94]
0x1408d0414 66898694000000 mov word ptr [rsi + 0x94], ax
0x1408d041b 0fb6842496000000 movzx eax, byte ptr [rsp + 0x96]
0x1408d0423 888696000000 mov byte ptr [rsi + 0x96], al
0x1408d0429 488b842497000000 mov rax, qword ptr [rsp + 0x97]
0x1408d0431 48898697000000 mov qword ptr [rsi + 0x97], rax
0x1408d0438 0fb684249f000000 movzx eax, byte ptr [rsp + 0x9f]
0x1408d0440 88869f000000 mov byte ptr [rsi + 0x9f], al
0x1408d0446 f30f104718 movss xmm0, dword ptr [rdi + 0x18]
0x1408d044b 0f280f movaps xmm1, xmmword ptr [rdi]
0x1408d044e 0f114e68 movups xmmword ptr [rsi + 0x68], xmm1
0x1408d0452 f20f104f10 movsd xmm1, qword ptr [rdi + 0x10]
0x1408d0457 f20f114e78 movsd qword ptr [rsi + 0x78], xmm1
0x1408d045c f30f118680000000 movss dword ptr [rsi + 0x80], xmm0
0x1408d0464 0f2807 movaps xmm0, xmmword ptr [rdi]
0x1408d0467 0f284f10 movaps xmm1, xmmword ptr [rdi + 0x10]
0x1408d046b 0f2906 movaps xmmword ptr [rsi], xmm0
0x1408d046e 0f294e10 movaps xmmword ptr [rsi + 0x10], xmm1
0x1408d0472 4889f0 mov rax, rsi
0x1408d0475 4881c4a8000000 add rsp, 0xa8
0x1408d047c 5f pop rdi
0x1408d047d 5e pop rsi
0x1408d047e c3 ret 
0x1408d047f 0f0b ud2 
