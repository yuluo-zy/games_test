0x140b66830 55 push rbp
0x140b66831 56 push rsi
0x140b66832 53 push rbx
0x140b66833 4881ecd0000000 sub rsp, 0xd0
0x140b6683a 488dac2480000000 lea rbp, [rsp + 0x80]
0x140b66842 48c74548feffffff mov qword ptr [rbp + 0x48], 0xfffffffffffffffe
0x140b6684a 4c894538 mov qword ptr [rbp + 0x38], r8
0x140b6684e 88d3 mov bl, dl
0x140b66850 4889ce mov rsi, rcx
0x140b66853 4883c138 add rcx, 0x38
0x140b66857 c6454701 mov byte ptr [rbp + 0x47], 1
0x140b6685b 48ba84a72b9651d21b8e movabs rdx, 0x8e1bd251962ba784
0x140b66865 49b89b52e6c07423786a movabs r8, 0x6a782374c0e6529b
0x140b6686f e87c6e57ff call 0x1400dd6f0
0x140b66874 4885c0 test rax, rax
0x140b66877 0f84a3000000 je 0x140b66920
0x140b6687d 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x140b66884 0f8696000000 jbe 0x140b66920
0x140b6688a 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x140b66891 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140b66895 4885c9 test rcx, rcx
0x140b66898 0f8482000000 je 0x140b66920
0x140b6689e 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x140b668a5 48f7d1 not rcx
0x140b668a8 488d0c49 lea rcx, [rcx + rcx*2]
0x140b668ac 48c1e105 shl rcx, 5
0x140b668b0 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140b668b6 7468 je 0x140b66920
0x140b668b8 4801c8 add rax, rcx
0x140b668bb 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140b668bf 8b96b0050000 mov edx, dword ptr [rsi + 0x5b0]
0x140b668c5 89505c mov dword ptr [rax + 0x5c], edx
0x140b668c8 488b4538 mov rax, qword ptr [rbp + 0x38]
0x140b668cc 0f104060 movups xmm0, xmmword ptr [rax + 0x60]
0x140b668d0 0f294500 movaps xmmword ptr [rbp], xmm0
0x140b668d4 0f104050 movups xmm0, xmmword ptr [rax + 0x50]
0x140b668d8 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x140b668dc 0f104040 movups xmm0, xmmword ptr [rax + 0x40]
0x140b668e0 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x140b668e4 0f1000 movups xmm0, xmmword ptr [rax]
0x140b668e7 0f104810 movups xmm1, xmmword ptr [rax + 0x10]
0x140b668eb 0f105020 movups xmm2, xmmword ptr [rax + 0x20]
0x140b668ef 0f105830 movups xmm3, xmmword ptr [rax + 0x30]
0x140b668f3 0f295dd0 movaps xmmword ptr [rbp - 0x30], xmm3
0x140b668f7 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
0x140b668fb 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x140b668ff 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x140b66903 c6454700 mov byte ptr [rbp + 0x47], 0
0x140b66907 4c8d45a0 lea r8, [rbp - 0x60]
0x140b6690b 89da mov edx, ebx
0x140b6690d e8de99b300 call 0x1416a02f0
0x140b66912 4889f0 mov rax, rsi
0x140b66915 4881c4d0000000 add rsp, 0xd0
0x140b6691c 5b pop rbx
0x140b6691d 5e pop rsi
0x140b6691e 5d pop rbp
0x140b6691f c3 ret 
0x140b66920 488d0599c3f801 lea rax, [rip + 0x1f8c399]
0x140b66927 48894518 mov qword ptr [rbp + 0x18], rax
0x140b6692b 48c7452027000000 mov qword ptr [rbp + 0x20], 0x27
0x140b66933 488d4518 lea rax, [rbp + 0x18]
0x140b66937 48894528 mov qword ptr [rbp + 0x28], rax
0x140b6693b 488d054e355aff lea rax, [rip - 0xa5cab2]
0x140b66942 48894530 mov qword ptr [rbp + 0x30], rax
0x140b66946 488d05a3c4f801 lea rax, [rip + 0x1f8c4a3]
0x140b6694d 488945a0 mov qword ptr [rbp - 0x60], rax
0x140b66951 48c745a802000000 mov qword ptr [rbp - 0x58], 2
0x140b66959 48c745c000000000 mov qword ptr [rbp - 0x40], 0
0x140b66961 488d4528 lea rax, [rbp + 0x28]
0x140b66965 488945b0 mov qword ptr [rbp - 0x50], rax
0x140b66969 48c745b801000000 mov qword ptr [rbp - 0x48], 1
0x140b66971 c6454701 mov byte ptr [rbp + 0x47], 1
0x140b66975 488d152cc3f801 lea rdx, [rip + 0x1f8c32c]
0x140b6697c 488d4da0 lea rcx, [rbp - 0x60]
0x140b66980 e80b2ad701 call 0x1428d9390
0x140b66985 0f0b ud2 
0x140b66987 660f1f840000000000 nop word ptr [rax + rax]
0x140b66990 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b66995 55 push rbp
0x140b66996 56 push rsi
0x140b66997 53 push rbx
0x140b66998 4883ec20 sub rsp, 0x20
0x140b6699c 488daa80000000 lea rbp, [rdx + 0x80]
0x140b669a3 807d4700 cmp byte ptr [rbp + 0x47], 0
0x140b669a7 7508 jne 0x140b669b1
0x140b669a9 4883c420 add rsp, 0x20
0x140b669ad 5b pop rbx
0x140b669ae 5e pop rsi
0x140b669af 5d pop rbp
0x140b669b0 c3 ret 
0x140b669b1 488b4d38 mov rcx, qword ptr [rbp + 0x38]
0x140b669b5 e8c62e4cff call 0x140029880
0x140b669ba ebed jmp 0x140b669a9
