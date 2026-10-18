0x1403276f0 55 push rbp
0x1403276f1 56 push rsi
0x1403276f2 4881ec38010000 sub rsp, 0x138
0x1403276f9 488dac2480000000 lea rbp, [rsp + 0x80]
0x140327701 48c785b0000000feffffff mov qword ptr [rbp + 0xb0], 0xfffffffffffffffe
0x14032770c 488995a0000000 mov qword ptr [rbp + 0xa0], rdx
0x140327713 4889ce mov rsi, rcx
0x140327716 4883c138 add rcx, 0x38
0x14032771a c685af00000001 mov byte ptr [rbp + 0xaf], 1
0x140327721 48ba84a72b9651d21b8e movabs rdx, 0x8e1bd251962ba784
0x14032772b 49b89b52e6c07423786a movabs r8, 0x6a782374c0e6529b
0x140327735 e8b65fdbff call 0x1400dd6f0
0x14032773a 4885c0 test rax, rax
0x14032773d 0f841d010000 je 0x140327860
0x140327743 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x14032774a 0f8610010000 jbe 0x140327860
0x140327750 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x140327757 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x14032775b 4885c9 test rcx, rcx
0x14032775e 0f84fc000000 je 0x140327860
0x140327764 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x14032776b 48f7d1 not rcx
0x14032776e 488d0c49 lea rcx, [rcx + rcx*2]
0x140327772 48c1e105 shl rcx, 5
0x140327776 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x14032777c 0f84de000000 je 0x140327860
0x140327782 4801c8 add rax, rcx
0x140327785 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140327789 8b96b0050000 mov edx, dword ptr [rsi + 0x5b0]
0x14032778f 89505c mov dword ptr [rax + 0x5c], edx
0x140327792 488b85a0000000 mov rax, qword ptr [rbp + 0xa0]
0x140327799 0f104060 movups xmm0, xmmword ptr [rax + 0x60]
0x14032779d 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
0x1403277a1 0f104050 movups xmm0, xmmword ptr [rax + 0x50]
0x1403277a5 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
0x1403277a9 0f104040 movups xmm0, xmmword ptr [rax + 0x40]
0x1403277ad 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
0x1403277b1 0f1000 movups xmm0, xmmword ptr [rax]
0x1403277b4 0f104810 movups xmm1, xmmword ptr [rax + 0x10]
0x1403277b8 0f105020 movups xmm2, xmmword ptr [rax + 0x20]
0x1403277bc 0f105830 movups xmm3, xmmword ptr [rax + 0x30]
0x1403277c0 0f295d40 movaps xmmword ptr [rbp + 0x40], xmm3
0x1403277c4 0f295530 movaps xmmword ptr [rbp + 0x30], xmm2
0x1403277c8 0f294d20 movaps xmmword ptr [rbp + 0x20], xmm1
0x1403277cc 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x1403277d0 c685ae00000001 mov byte ptr [rbp + 0xae], 1
0x1403277d7 e8a4ade4ff call 0x140172580
0x1403277dc 0f284570 movaps xmm0, xmmword ptr [rbp + 0x70]
0x1403277e0 0f294500 movaps xmmword ptr [rbp], xmm0
0x1403277e4 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
0x1403277e8 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x1403277ec 0f284550 movaps xmm0, xmmword ptr [rbp + 0x50]
0x1403277f0 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1403277f4 0f284510 movaps xmm0, xmmword ptr [rbp + 0x10]
0x1403277f8 0f284d20 movaps xmm1, xmmword ptr [rbp + 0x20]
0x1403277fc 0f285530 movaps xmm2, xmmword ptr [rbp + 0x30]
0x140327800 0f285d40 movaps xmm3, xmmword ptr [rbp + 0x40]
0x140327804 0f295dd0 movaps xmmword ptr [rbp - 0x30], xmm3
0x140327808 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
0x14032780c 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x140327810 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x140327814 c685ae00000000 mov byte ptr [rbp + 0xae], 0
0x14032781b 488d8d80000000 lea rcx, [rbp + 0x80]
0x140327822 4c8d45a0 lea r8, [rbp - 0x60]
0x140327826 4889c2 mov rdx, rax
0x140327829 4531c9 xor r9d, r9d
0x14032782c e8af66e4ff call 0x14016dee0
0x140327831 488b9580000000 mov rdx, qword ptr [rbp + 0x80]
0x140327838 4885d2 test rdx, rdx
0x14032783b 7416 je 0x140327853
0x14032783d 488b8d88000000 mov rcx, qword ptr [rbp + 0x88]
0x140327844 48c1e204 shl rdx, 4
0x140327848 41b808000000 mov r8d, 8
0x14032784e e8cdc32e00 call 0x140613c20
0x140327853 4889f0 mov rax, rsi
0x140327856 4881c438010000 add rsp, 0x138
0x14032785d 5e pop rsi
0x14032785e 5d pop rbp
0x14032785f c3 ret 
0x140327860 488d05d1696902 lea rax, [rip + 0x26969d1]
0x140327867 48894510 mov qword ptr [rbp + 0x10], rax
0x14032786b 48c7451827000000 mov qword ptr [rbp + 0x18], 0x27
0x140327873 488d4510 lea rax, [rbp + 0x10]
0x140327877 48898580000000 mov qword ptr [rbp + 0x80], rax
0x14032787e 488d050b26deff lea rax, [rip - 0x21d9f5]
0x140327885 48898588000000 mov qword ptr [rbp + 0x88], rax
0x14032788c 488d05d56a6902 lea rax, [rip + 0x2696ad5]
0x140327893 488945a0 mov qword ptr [rbp - 0x60], rax
0x140327897 48c745a802000000 mov qword ptr [rbp - 0x58], 2
0x14032789f 48c745c000000000 mov qword ptr [rbp - 0x40], 0
0x1403278a7 488d8580000000 lea rax, [rbp + 0x80]
0x1403278ae 488945b0 mov qword ptr [rbp - 0x50], rax
0x1403278b2 48c745b801000000 mov qword ptr [rbp - 0x48], 1
0x1403278ba c685af00000001 mov byte ptr [rbp + 0xaf], 1
0x1403278c1 488d1528696902 lea rdx, [rip + 0x2696928]
0x1403278c8 488d4da0 lea rcx, [rbp - 0x60]
0x1403278cc e8bf1a5b02 call 0x1428d9390
0x1403278d1 0f0b ud2 
0x1403278d3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1403278e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1403278e5 55 push rbp
0x1403278e6 56 push rsi
0x1403278e7 4883ec28 sub rsp, 0x28
0x1403278eb 488daa80000000 lea rbp, [rdx + 0x80]
0x1403278f2 80bdae00000000 cmp byte ptr [rbp + 0xae], 0
0x1403278f9 7410 je 0x14032790b
0x1403278fb c685af00000000 mov byte ptr [rbp + 0xaf], 0
0x140327902 488d4d10 lea rcx, [rbp + 0x10]
0x140327906 e8751fd0ff call 0x140029880
0x14032790b c685af00000000 mov byte ptr [rbp + 0xaf], 0
0x140327912 4883c428 add rsp, 0x28
0x140327916 5e pop rsi
0x140327917 5d pop rbp
0x140327918 c3 ret 
0x140327919 0f1f8000000000 nop dword ptr [rax]
0x140327920 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140327925 55 push rbp
0x140327926 56 push rsi
0x140327927 4883ec28 sub rsp, 0x28
0x14032792b 488daa80000000 lea rbp, [rdx + 0x80]
0x140327932 80bdaf00000000 cmp byte ptr [rbp + 0xaf], 0
0x140327939 7507 jne 0x140327942
0x14032793b 4883c428 add rsp, 0x28
0x14032793f 5e pop rsi
0x140327940 5d pop rbp
0x140327941 c3 ret 
0x140327942 488b8da0000000 mov rcx, qword ptr [rbp + 0xa0]
0x140327949 e8321fd0ff call 0x140029880
0x14032794e ebeb jmp 0x14032793b
