0x1408d9590 55 push rbp
0x1408d9591 4156 push r14
0x1408d9593 56 push rsi
0x1408d9594 57 push rdi
0x1408d9595 53 push rbx
0x1408d9596 4881ece0010000 sub rsp, 0x1e0
0x1408d959d 488dac2480000000 lea rbp, [rsp + 0x80]
0x1408d95a5 48c78558010000feffffff mov qword ptr [rbp + 0x158], 0xfffffffffffffffe
0x1408d95b0 4c89c3 mov rbx, r8
0x1408d95b3 4889d7 mov rdi, rdx
0x1408d95b6 4889ce mov rsi, rcx
0x1408d95b9 410fb64010 movzx eax, byte ptr [r8 + 0x10]
0x1408d95be 4869c090030000 imul rax, rax, 0x390
0x1408d95c5 4c8d3402 lea r14, [rdx + rax]
0x1408d95c9 4983c620 add r14, 0x20
0x1408d95cd 458b4008 mov r8d, dword ptr [r8 + 8]
0x1408d95d1 448b4b0c mov r9d, dword ptr [rbx + 0xc]
0x1408d95d5 488d82e83c0000 lea rax, [rdx + 0x3ce8]
0x1408d95dc 488d8a803d0000 lea rcx, [rdx + 0x3d80]
0x1408d95e3 48894c2428 mov qword ptr [rsp + 0x28], rcx
0x1408d95e8 4889442420 mov qword ptr [rsp + 0x20], rax
0x1408d95ed c644243000 mov byte ptr [rsp + 0x30], 0
0x1408d95f2 488d4dc0 lea rcx, [rbp - 0x40]
0x1408d95f6 4c89f2 mov rdx, r14
0x1408d95f9 e8a2a02b00 call 0x140b936a0
0x1408d95fe 837d4402 cmp dword ptr [rbp + 0x44], 2
0x1408d9602 0f84b7010000 je 0x1408d97bf
0x1408d9608 488d4d70 lea rcx, [rbp + 0x70]
0x1408d960c 488d55c0 lea rdx, [rbp - 0x40]
0x1408d9610 41b8b0000000 mov r8d, 0xb0
0x1408d9616 e8e5a50402 call 0x142923c00
0x1408d961b 8b4304 mov eax, dword ptr [rbx + 4]
0x1408d961e 48394710 cmp qword ptr [rdi + 0x10], rax
0x1408d9622 0f8689010000 jbe 0x1408d97b1
0x1408d9628 488b4f08 mov rcx, qword ptr [rdi + 8]
0x1408d962c 4885c9 test rcx, rcx
0x1408d962f 0f847c010000 je 0x1408d97b1
0x1408d9635 8b13 mov edx, dword ptr [rbx]
0x1408d9637 4c8d0c80 lea r9, [rax + rax*4]
0x1408d963b 423954c920 cmp dword ptr [rcx + r9*8 + 0x20], edx
0x1408d9640 0f856b010000 jne 0x1408d97b1
0x1408d9646 460fb604c9 movzx r8d, byte ptr [rcx + r9*8]
0x1408d964b 420f1044c910 movups xmm0, xmmword ptr [rcx + r9*8 + 0x10]
0x1408d9651 0f11853f010000 movups xmmword ptr [rbp + 0x13f], xmm0
0x1408d9658 420f1044c901 movups xmm0, xmmword ptr [rcx + r9*8 + 1]
0x1408d965e 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
0x1408d9665 448b5718 mov r10d, dword ptr [rdi + 0x18]
0x1408d9669 468914c9 mov dword ptr [rcx + r9*8], r10d
0x1408d966d 894718 mov dword ptr [rdi + 0x18], eax
0x1408d9670 ff4f1c dec dword ptr [rdi + 0x1c]
0x1408d9673 ffc2 inc edx
0x1408d9675 428954c920 mov dword ptr [rcx + r9*8 + 0x20], edx
0x1408d967a 4180f811 cmp r8b, 0x11
0x1408d967e 0f842d010000 je 0x1408d97b1
0x1408d9684 448845c0 mov byte ptr [rbp - 0x40], r8b
0x1408d9688 0f288530010000 movaps xmm0, xmmword ptr [rbp + 0x130]
0x1408d968f 0f1145c1 movups xmmword ptr [rbp - 0x3f], xmm0
0x1408d9693 0f10853f010000 movups xmm0, xmmword ptr [rbp + 0x13f]
0x1408d969a 0f1145d0 movups xmmword ptr [rbp - 0x30], xmm0
0x1408d969e 41c6868803000001 mov byte ptr [r14 + 0x388], 1
0x1408d96a6 488d8d30010000 lea rcx, [rbp + 0x130]
0x1408d96ad 488d5570 lea rdx, [rbp + 0x70]
0x1408d96b1 e8aa982b00 call 0x140b92f60
0x1408d96b6 4981c688010000 add r14, 0x188
0x1408d96bd f30f108530010000 movss xmm0, dword ptr [rbp + 0x130]
0x1408d96c5 f30f108d44010000 movss xmm1, dword ptr [rbp + 0x144]
0x1408d96cd f30f118520010000 movss dword ptr [rbp + 0x120], xmm0
0x1408d96d5 f20f108538010000 movsd xmm0, qword ptr [rbp + 0x138]
0x1408d96dd f20f118524010000 movsd qword ptr [rbp + 0x124], xmm0
0x1408d96e5 f30f118d2c010000 movss dword ptr [rbp + 0x12c], xmm1
0x1408d96ed 488d8d20010000 lea rcx, [rbp + 0x120]
0x1408d96f4 4c89f2 mov rdx, r14
0x1408d96f7 e8e4512000 call 0x140ade8e0
0x1408d96fc 4883c308 add rbx, 8
0x1408d9700 8b4308 mov eax, dword ptr [rbx + 8]
0x1408d9703 89853c010000 mov dword ptr [rbp + 0x13c], eax
0x1408d9709 488b03 mov rax, qword ptr [rbx]
0x1408d970c 48898534010000 mov qword ptr [rbp + 0x134], rax
0x1408d9713 4881c7b03c0000 add rdi, 0x3cb0
0x1408d971a c7853001000001000000 mov dword ptr [rbp + 0x130], 1
0x1408d9724 488d9530010000 lea rdx, [rbp + 0x130]
0x1408d972b 4889f9 mov rcx, rdi
0x1408d972e e8cda4b700 call 0x141453c00
0x1408d9733 f30f1085bc000000 movss xmm0, dword ptr [rbp + 0xbc]
0x1408d973b 0f108da4000000 movups xmm1, xmmword ptr [rbp + 0xa4]
0x1408d9742 f20f1095b4000000 movsd xmm2, qword ptr [rbp + 0xb4]
0x1408d974a f20f115610 movsd qword ptr [rsi + 0x10], xmm2
0x1408d974f f30f114618 movss dword ptr [rsi + 0x18], xmm0
0x1408d9754 0f1045c0 movups xmm0, xmmword ptr [rbp - 0x40]
0x1408d9758 0f1055d0 movups xmm2, xmmword ptr [rbp - 0x30]
0x1408d975c 0f114620 movups xmmword ptr [rsi + 0x20], xmm0
0x1408d9760 0f115630 movups xmmword ptr [rsi + 0x30], xmm2
0x1408d9764 0f290e movaps xmmword ptr [rsi], xmm1
0x1408d9767 488b8580000000 mov rax, qword ptr [rbp + 0x80]
0x1408d976e f048ff08 lock dec qword ptr [rax]
0x1408d9772 750c jne 0x1408d9780
0x1408d9774 488d8d80000000 lea rcx, [rbp + 0x80]
0x1408d977b e85062d7ff call 0x14064f9d0
0x1408d9780 83bd9000000002 cmp dword ptr [rbp + 0x90], 2
0x1408d9787 7217 jb 0x1408d97a0
0x1408d9789 488b8d98000000 mov rcx, qword ptr [rbp + 0x98]
0x1408d9790 ba2c000000 mov edx, 0x2c
0x1408d9795 41b804000000 mov r8d, 4
0x1408d979b e880a4d3ff call 0x140613c20
0x1408d97a0 4889f0 mov rax, rsi
0x1408d97a3 4881c4e0010000 add rsp, 0x1e0
0x1408d97aa 5b pop rbx
0x1408d97ab 5f pop rdi
0x1408d97ac 5e pop rsi
0x1408d97ad 415e pop r14
0x1408d97af 5d pop rbp
0x1408d97b0 c3 ret
0x1408d97b1 488d0df8aa1c02 lea rcx, [rip + 0x21caaf8]
0x1408d97b8 e853fbff01 call 0x1428d9310
0x1408d97bd 0f0b ud2
0x1408d97bf 488d0d8aaa1c02 lea rcx, [rip + 0x21caa8a]
0x1408d97c6 4c8d05cbaa1c02 lea r8, [rip + 0x21caacb]
0x1408d97cd ba45000000 mov edx, 0x45
0x1408d97d2 e859fbff01 call 0x1428d9330
0x1408d97d7 cc int3
0x1408d97d8 0f1f840000000000 nop dword ptr [rax + rax]
0x1408d97e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408d97e5 55 push rbp
0x1408d97e6 4156 push r14
0x1408d97e8 56 push rsi
0x1408d97e9 57 push rdi
0x1408d97ea 53 push rbx
0x1408d97eb 4883ec40 sub rsp, 0x40
0x1408d97ef 488daa80000000 lea rbp, [rdx + 0x80]
0x1408d97f6 488b45d8 mov rax, qword ptr [rbp - 0x28]
0x1408d97fa f048ff08 lock dec qword ptr [rax]
0x1408d97fe 7509 jne 0x1408d9809
0x1408d9800 488d4dd8 lea rcx, [rbp - 0x28]
0x1408d9804 e8172e2800 call 0x140b5c620
0x1408d9809 90 nop
0x1408d980a 4883c440 add rsp, 0x40
0x1408d980e 5b pop rbx
0x1408d980f 5f pop rdi
0x1408d9810 5e pop rsi
0x1408d9811 415e pop r14
0x1408d9813 5d pop rbp
0x1408d9814 c3 ret
0x1408d9815 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1408d9820 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408d9825 55 push rbp
0x1408d9826 4156 push r14
0x1408d9828 56 push rsi
0x1408d9829 57 push rdi
0x1408d982a 53 push rbx
0x1408d982b 4883ec40 sub rsp, 0x40
0x1408d982f 488daa80000000 lea rbp, [rdx + 0x80]
0x1408d9836 83bd9000000002 cmp dword ptr [rbp + 0x90], 2
0x1408d983d 7217 jb 0x1408d9856
0x1408d983f 488b8d98000000 mov rcx, qword ptr [rbp + 0x98]
0x1408d9846 ba2c000000 mov edx, 0x2c
0x1408d984b 41b804000000 mov r8d, 4
0x1408d9851 e8caa3d3ff call 0x140613c20
0x1408d9856 90 nop
0x1408d9857 4883c440 add rsp, 0x40
0x1408d985b 5b pop rbx
0x1408d985c 5f pop rdi
0x1408d985d 5e pop rsi
0x1408d985e 415e pop r14
0x1408d9860 5d pop rbp
0x1408d9861 c3 ret
0x1408d9862 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1408d9870 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408d9875 55 push rbp
0x1408d9876 4156 push r14
0x1408d9878 56 push rsi
0x1408d9879 57 push rdi
0x1408d987a 53 push rbx
0x1408d987b 4883ec40 sub rsp, 0x40
0x1408d987f 488daa80000000 lea rbp, [rdx + 0x80]
0x1408d9886 488d4d70 lea rcx, [rbp + 0x70]
0x1408d988a e811acd1ff call 0x1405f44a0
0x1408d988f 90 nop
0x1408d9890 4883c440 add rsp, 0x40
0x1408d9894 5b pop rbx
0x1408d9895 5f pop rdi
0x1408d9896 5e pop rsi
0x1408d9897 415e pop r14
0x1408d9899 5d pop rbp
0x1408d989a c3 ret
