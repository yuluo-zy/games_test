0x140b67620 55 push rbp
0x140b67621 56 push rsi
0x140b67622 4881ec38010000 sub rsp, 0x138
0x140b67629 488dac2480000000 lea rbp, [rsp + 0x80]
0x140b67631 48c785b0000000feffffff mov qword ptr [rbp + 0xb0], 0xfffffffffffffffe
0x140b6763c 488995a0000000 mov qword ptr [rbp + 0xa0], rdx
0x140b67643 4889ce mov rsi, rcx
0x140b67646 4883c138 add rcx, 0x38
0x140b6764a c685af00000001 mov byte ptr [rbp + 0xaf], 1
0x140b67651 48ba84a72b9651d21b8e movabs rdx, 0x8e1bd251962ba784
0x140b6765b 49b89b52e6c07423786a movabs r8, 0x6a782374c0e6529b
0x140b67665 e8866057ff call 0x1400dd6f0
0x140b6766a 4885c0 test rax, rax
0x140b6766d 0f841d010000 je 0x140b67790
0x140b67673 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x140b6767a 0f8610010000 jbe 0x140b67790
0x140b67680 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x140b67687 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140b6768b 4885c9 test rcx, rcx
0x140b6768e 0f84fc000000 je 0x140b67790
0x140b67694 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x140b6769b 48f7d1 not rcx
0x140b6769e 488d0c49 lea rcx, [rcx + rcx*2]
0x140b676a2 48c1e105 shl rcx, 5
0x140b676a6 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140b676ac 0f84de000000 je 0x140b67790
0x140b676b2 4801c8 add rax, rcx
0x140b676b5 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140b676b9 8b96b0050000 mov edx, dword ptr [rsi + 0x5b0]
0x140b676bf 89505c mov dword ptr [rax + 0x5c], edx
0x140b676c2 488b85a0000000 mov rax, qword ptr [rbp + 0xa0]
0x140b676c9 0f104060 movups xmm0, xmmword ptr [rax + 0x60]
0x140b676cd 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
0x140b676d1 0f104050 movups xmm0, xmmword ptr [rax + 0x50]
0x140b676d5 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
0x140b676d9 0f104040 movups xmm0, xmmword ptr [rax + 0x40]
0x140b676dd 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
0x140b676e1 0f1000 movups xmm0, xmmword ptr [rax]
0x140b676e4 0f104810 movups xmm1, xmmword ptr [rax + 0x10]
0x140b676e8 0f105020 movups xmm2, xmmword ptr [rax + 0x20]
0x140b676ec 0f105830 movups xmm3, xmmword ptr [rax + 0x30]
0x140b676f0 0f295d40 movaps xmmword ptr [rbp + 0x40], xmm3
0x140b676f4 0f295530 movaps xmmword ptr [rbp + 0x30], xmm2
0x140b676f8 0f294d20 movaps xmmword ptr [rbp + 0x20], xmm1
0x140b676fc 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x140b67700 c685ae00000001 mov byte ptr [rbp + 0xae], 1
0x140b67707 e8e49cb300 call 0x1416a13f0
0x140b6770c 0f284570 movaps xmm0, xmmword ptr [rbp + 0x70]
0x140b67710 0f294500 movaps xmmword ptr [rbp], xmm0
0x140b67714 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
0x140b67718 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x140b6771c 0f284550 movaps xmm0, xmmword ptr [rbp + 0x50]
0x140b67720 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x140b67724 0f284510 movaps xmm0, xmmword ptr [rbp + 0x10]
0x140b67728 0f284d20 movaps xmm1, xmmword ptr [rbp + 0x20]
0x140b6772c 0f285530 movaps xmm2, xmmword ptr [rbp + 0x30]
0x140b67730 0f285d40 movaps xmm3, xmmword ptr [rbp + 0x40]
0x140b67734 0f295dd0 movaps xmmword ptr [rbp - 0x30], xmm3
0x140b67738 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
0x140b6773c 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x140b67740 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x140b67744 c685ae00000000 mov byte ptr [rbp + 0xae], 0
0x140b6774b 488d8d80000000 lea rcx, [rbp + 0x80]
0x140b67752 4c8d45a0 lea r8, [rbp - 0x60]
0x140b67756 4889c2 mov rdx, rax
0x140b67759 4531c9 xor r9d, r9d
0x140b6775c e8cf75b300 call 0x14169ed30
0x140b67761 488b9580000000 mov rdx, qword ptr [rbp + 0x80]
0x140b67768 4885d2 test rdx, rdx
0x140b6776b 7416 je 0x140b67783
0x140b6776d 488b8d88000000 mov rcx, qword ptr [rbp + 0x88]
0x140b67774 48c1e204 shl rdx, 4
0x140b67778 41b808000000 mov r8d, 8
0x140b6777e e89dc4aaff call 0x140613c20
0x140b67783 4889f0 mov rax, rsi
0x140b67786 4881c438010000 add rsp, 0x138
0x140b6778d 5e pop rsi
0x140b6778e 5d pop rbp
0x140b6778f c3 ret 
0x140b67790 488d0529b5f801 lea rax, [rip + 0x1f8b529]
0x140b67797 48894510 mov qword ptr [rbp + 0x10], rax
0x140b6779b 48c7451827000000 mov qword ptr [rbp + 0x18], 0x27
0x140b677a3 488d4510 lea rax, [rbp + 0x10]
0x140b677a7 48898580000000 mov qword ptr [rbp + 0x80], rax
0x140b677ae 488d05db265aff lea rax, [rip - 0xa5d925]
0x140b677b5 48898588000000 mov qword ptr [rbp + 0x88], rax
0x140b677bc 488d052db6f801 lea rax, [rip + 0x1f8b62d]
0x140b677c3 488945a0 mov qword ptr [rbp - 0x60], rax
0x140b677c7 48c745a802000000 mov qword ptr [rbp - 0x58], 2
0x140b677cf 48c745c000000000 mov qword ptr [rbp - 0x40], 0
0x140b677d7 488d8580000000 lea rax, [rbp + 0x80]
0x140b677de 488945b0 mov qword ptr [rbp - 0x50], rax
0x140b677e2 48c745b801000000 mov qword ptr [rbp - 0x48], 1
0x140b677ea c685af00000001 mov byte ptr [rbp + 0xaf], 1
0x140b677f1 488d15b0b4f801 lea rdx, [rip + 0x1f8b4b0]
0x140b677f8 488d4da0 lea rcx, [rbp - 0x60]
0x140b677fc e88f1bd701 call 0x1428d9390
0x140b67801 0f0b ud2 
0x140b67803 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140b67810 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b67815 55 push rbp
0x140b67816 56 push rsi
0x140b67817 4883ec28 sub rsp, 0x28
0x140b6781b 488daa80000000 lea rbp, [rdx + 0x80]
0x140b67822 80bdae00000000 cmp byte ptr [rbp + 0xae], 0
0x140b67829 7410 je 0x140b6783b
0x140b6782b c685af00000000 mov byte ptr [rbp + 0xaf], 0
0x140b67832 488d4d10 lea rcx, [rbp + 0x10]
0x140b67836 e845204cff call 0x140029880
0x140b6783b c685af00000000 mov byte ptr [rbp + 0xaf], 0
0x140b67842 4883c428 add rsp, 0x28
0x140b67846 5e pop rsi
0x140b67847 5d pop rbp
0x140b67848 c3 ret 
0x140b67849 0f1f8000000000 nop dword ptr [rax]
0x140b67850 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b67855 55 push rbp
0x140b67856 56 push rsi
0x140b67857 4883ec28 sub rsp, 0x28
0x140b6785b 488daa80000000 lea rbp, [rdx + 0x80]
0x140b67862 80bdaf00000000 cmp byte ptr [rbp + 0xaf], 0
0x140b67869 7507 jne 0x140b67872
0x140b6786b 4883c428 add rsp, 0x28
0x140b6786f 5e pop rsi
0x140b67870 5d pop rbp
0x140b67871 c3 ret 
0x140b67872 488b8da0000000 mov rcx, qword ptr [rbp + 0xa0]
0x140b67879 e802204cff call 0x140029880
0x140b6787e ebeb jmp 0x140b6786b
