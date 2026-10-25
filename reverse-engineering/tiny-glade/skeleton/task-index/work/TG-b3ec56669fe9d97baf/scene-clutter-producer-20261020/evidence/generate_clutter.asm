0x1407e3370 55 push rbp
0x1407e3371 4157 push r15
0x1407e3373 4156 push r14
0x1407e3375 4155 push r13
0x1407e3377 4154 push r12
0x1407e3379 56 push rsi
0x1407e337a 57 push rdi
0x1407e337b 53 push rbx
0x1407e337c 4881ec58040000 sub rsp, 0x458
0x1407e3383 488dac2480000000 lea rbp, [rsp + 0x80]
0x1407e338b 66440f7fbdc0030000 movdqa xmmword ptr [rbp + 0x3c0], xmm15
0x1407e3394 440f29b5b0030000 movaps xmmword ptr [rbp + 0x3b0], xmm14
0x1407e339c 66440f7fada0030000 movdqa xmmword ptr [rbp + 0x3a0], xmm13
0x1407e33a5 66440f7fa590030000 movdqa xmmword ptr [rbp + 0x390], xmm12
0x1407e33ae 66440f7f9d80030000 movdqa xmmword ptr [rbp + 0x380], xmm11
0x1407e33b7 66440f7f9570030000 movdqa xmmword ptr [rbp + 0x370], xmm10
0x1407e33c0 66440f7f8d60030000 movdqa xmmword ptr [rbp + 0x360], xmm9
0x1407e33c9 66440f7f8550030000 movdqa xmmword ptr [rbp + 0x350], xmm8
0x1407e33d2 660f7fbd40030000 movdqa xmmword ptr [rbp + 0x340], xmm7
0x1407e33da 660f7fb530030000 movdqa xmmword ptr [rbp + 0x330], xmm6
0x1407e33e2 48c78528030000feffffff mov qword ptr [rbp + 0x328], 0xfffffffffffffffe
0x1407e33ed 4c89ce mov rsi, r9
0x1407e33f0 4d89c6 mov r14, r8
0x1407e33f3 4989d5 mov r13, rdx
0x1407e33f6 4889cf mov rdi, rcx
0x1407e33f9 e802364300 call 0x140c16a00
0x1407e33fe 84c0 test al, al
0x1407e3400 745d je 0x1407e345f
0x1407e3402 41bc51000000 mov r12d, 0x51
0x1407e3408 b951000000 mov ecx, 0x51
0x1407e340d e82efbffff call 0x1407e2f40
0x1407e3412 488d1da75a2a02 lea rbx, [rip + 0x22a5aa7]
0x1407e3419 4883f801 cmp rax, 1
0x1407e341d 0f85a8000000 jne 0x1407e34cb
0x1407e3423 4989d1 mov r9, rdx
0x1407e3426 4885d2 test rdx, rdx
0x1407e3429 743d je 0x1407e3468
0x1407e342b 4983f951 cmp r9, 0x51
0x1407e342f 7335 jae 0x1407e3466
0x1407e3431 488d05885a2a02 lea rax, [rip + 0x22a5a88]
0x1407e3438 41803c01bf cmp byte ptr [r9 + rax], 0xbf
0x1407e343d 7f29 jg 0x1407e3468
0x1407e343f 488d05525b2a02 lea rax, [rip + 0x22a5b52]
0x1407e3446 4889442420 mov qword ptr [rsp + 0x20], rax
0x1407e344b 488d0d6e5a2a02 lea rcx, [rip + 0x22a5a6e]
0x1407e3452 ba51000000 mov edx, 0x51
0x1407e3457 4531c0 xor r8d, r8d
0x1407e345a e8e1640f02 call 0x1428d9940
0x1407e345f 31c0 xor eax, eax
0x1407e3461 e9c4010000 jmp 0x1407e362a
0x1407e3466 75d7 jne 0x1407e343f
0x1407e3468 4c89c9 mov rcx, r9
0x1407e346b e8d0faffff call 0x1407e2f40
0x1407e3470 4883f801 cmp rax, 1
0x1407e3474 7555 jne 0x1407e34cb
0x1407e3476 4989d0 mov r8, rdx
0x1407e3479 4983c002 add r8, 2
0x1407e347d 7439 je 0x1407e34b8
0x1407e347f 4983f851 cmp r8, 0x51
0x1407e3483 7331 jae 0x1407e34b6
0x1407e3485 488d05345a2a02 lea rax, [rip + 0x22a5a34]
0x1407e348c 41803c00bf cmp byte ptr [r8 + rax], 0xbf
0x1407e3491 7f25 jg 0x1407e34b8
0x1407e3493 488d05165b2a02 lea rax, [rip + 0x22a5b16]
0x1407e349a 4889442420 mov qword ptr [rsp + 0x20], rax
0x1407e349f 488d0d1a5a2a02 lea rcx, [rip + 0x22a5a1a]
0x1407e34a6 ba51000000 mov edx, 0x51
0x1407e34ab 41b951000000 mov r9d, 0x51
0x1407e34b1 e88a640f02 call 0x1428d9940
0x1407e34b6 75db jne 0x1407e3493
0x1407e34b8 41bc4f000000 mov r12d, 0x4f
0x1407e34be 4929d4 sub r12, rdx
0x1407e34c1 488d1df8592a02 lea rbx, [rip + 0x22a59f8]
0x1407e34c8 4c01c3 add rbx, r8
0x1407e34cb 4c89ad08030000 mov qword ptr [rbp + 0x308], r13
0x1407e34d2 4c8d05495b2a02 lea r8, [rip + 0x22a5b49]
0x1407e34d9 4c8d2d005b2a02 lea r13, [rip + 0x22a5b00]
0x1407e34e0 410fbe40ff movsx eax, byte ptr [r8 - 1]
0x1407e34e5 85c0 test eax, eax
0x1407e34e7 7817 js 0x1407e3500
0x1407e34e9 49ffc8 dec r8
0x1407e34ec 83f85c cmp eax, 0x5c
0x1407e34ef 7563 jne 0x1407e3554
0x1407e34f1 eb73 jmp 0x1407e3566
0x1407e34f3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1407e3500 410fb648fe movzx ecx, byte ptr [r8 - 2]
0x1407e3505 80f9c0 cmp cl, 0xc0
0x1407e3508 7d23 jge 0x1407e352d
0x1407e350a 410fb650fd movzx edx, byte ptr [r8 - 3]
0x1407e350f 80fac0 cmp dl, 0xc0
0x1407e3512 7d22 jge 0x1407e3536
0x1407e3514 450fb648fc movzx r9d, byte ptr [r8 - 4]
0x1407e3519 4983c0fc add r8, -4
0x1407e351d 4183e107 and r9d, 7
0x1407e3521 41c1e106 shl r9d, 6
0x1407e3525 83e23f and edx, 0x3f
0x1407e3528 4409ca or edx, r9d
0x1407e352b eb10 jmp 0x1407e353d
0x1407e352d 4983c0fe add r8, -2
0x1407e3531 83e11f and ecx, 0x1f
0x1407e3534 eb0f jmp 0x1407e3545
0x1407e3536 4983c0fd add r8, -3
0x1407e353a 83e20f and edx, 0xf
0x1407e353d c1e206 shl edx, 6
0x1407e3540 83e13f and ecx, 0x3f
0x1407e3543 09d1 or ecx, edx
0x1407e3545 c1e106 shl ecx, 6
0x1407e3548 243f and al, 0x3f
0x1407e354a 0fb6c0 movzx eax, al
0x1407e354d 09c8 or eax, ecx
0x1407e354f 83f85c cmp eax, 0x5c
0x1407e3552 7412 je 0x1407e3566
0x1407e3554 83f82f cmp eax, 0x2f
0x1407e3557 740d je 0x1407e3566
0x1407e3559 4d39e8 cmp r8, r13
0x1407e355c 7582 jne 0x1407e34e0
0x1407e355e 41bf42000000 mov r15d, 0x42
0x1407e3564 eb4d jmp 0x1407e35b3
0x1407e3566 4c8d2d735a2a02 lea r13, [rip + 0x22a5a73]
0x1407e356d 4d29e8 sub r8, r13
0x1407e3570 49ffc0 inc r8
0x1407e3573 7432 je 0x1407e35a7
0x1407e3575 4983f842 cmp r8, 0x42
0x1407e3579 732a jae 0x1407e35a5
0x1407e357b 43803c28bf cmp byte ptr [r8 + r13], 0xbf
0x1407e3580 7f25 jg 0x1407e35a7
0x1407e3582 488d05ef592a02 lea rax, [rip + 0x22a59ef]
0x1407e3589 4889442420 mov qword ptr [rsp + 0x20], rax
0x1407e358e 488d0d4b5a2a02 lea rcx, [rip + 0x22a5a4b]
0x1407e3595 ba42000000 mov edx, 0x42
0x1407e359a 41b942000000 mov r9d, 0x42
0x1407e35a0 e89b630f02 call 0x1428d9940
0x1407e35a5 75db jne 0x1407e3582
0x1407e35a7 41bf42000000 mov r15d, 0x42
0x1407e35ad 4d29c7 sub r15, r8
0x1407e35b0 4d01c5 add r13, r8
0x1407e35b3 e8784b4300 call 0x140c18130
0x1407e35b8 4889c1 mov rcx, rax
0x1407e35bb 488b00 mov rax, qword ptr [rax]
0x1407e35be 4883f801 cmp rax, 1
0x1407e35c2 0f859d0e0000 jne 0x1407e4465
0x1407e35c8 4883c108 add rcx, 8
0x1407e35cc 48833900 cmp qword ptr [rcx], 0
0x1407e35d0 0f858b0f0000 jne 0x1407e4561
0x1407e35d6 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1407e35dd 48898df0020000 mov qword ptr [rbp + 0x2f0], rcx
0x1407e35e4 4883c108 add rcx, 8
0x1407e35e8 4c897c2420 mov qword ptr [rsp + 0x20], r15
0x1407e35ed 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x1407e35f6 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x1407e35ff 4889da mov rdx, rbx
0x1407e3602 4d89e0 mov r8, r12
0x1407e3605 4d89e9 mov r9, r13
0x1407e3608 e833374300 call 0x140c16d40
0x1407e360d 488b8df0020000 mov rcx, qword ptr [rbp + 0x2f0]
0x1407e3614 48ff01 inc qword ptr [rcx]
0x1407e3617 48898558020000 mov qword ptr [rbp + 0x258], rax
0x1407e361e b801000000 mov eax, 1
0x1407e3623 4c8bad08030000 mov r13, qword ptr [rbp + 0x308]
0x1407e362a 48898550020000 mov qword ptr [rbp + 0x250], rax
0x1407e3631 488b17 mov rdx, qword ptr [rdi]
0x1407e3634 488d8dc0010000 lea rcx, [rbp + 0x1c0]
0x1407e363b e870af2300 call 0x140a1e5b0
0x1407e3640 488b8550040000 mov rax, qword ptr [rbp + 0x450]
0x1407e3647 488b8d40040000 mov rcx, qword ptr [rbp + 0x440]
0x1407e364e 0fb695c0010000 movzx edx, byte ptr [rbp + 0x1c0]
0x1407e3655 88951f030000 mov byte ptr [rbp + 0x31f], dl
0x1407e365b 488b95c8010000 mov rdx, qword ptr [rbp + 0x1c8]
0x1407e3662 48899560020000 mov qword ptr [rbp + 0x260], rdx
0x1407e3669 4c8b85d0010000 mov r8, qword ptr [rbp + 0x1d0]
0x1407e3670 488b95d8010000 mov rdx, qword ptr [rbp + 0x1d8]
0x1407e3677 488995f8020000 mov qword ptr [rbp + 0x2f8], rdx
0x1407e367e 4c8bbde0010000 mov r15, qword ptr [rbp + 0x1e0]
0x1407e3685 488b16 mov rdx, qword ptr [rsi]
0x1407e3688 48899530010000 mov qword ptr [rbp + 0x130], rdx
0x1407e368f 488b11 mov rdx, qword ptr [rcx]
0x1407e3692 48899560010000 mov qword ptr [rbp + 0x160], rdx
0x1407e3699 488b4908 mov rcx, qword ptr [rcx + 8]
0x1407e369d 48898d68020000 mov qword ptr [rbp + 0x268], rcx
0x1407e36a4 418b4e1c mov ecx, dword ptr [r14 + 0x1c]
0x1407e36a8 898d8c020000 mov dword ptr [rbp + 0x28c], ecx
0x1407e36ae 498b0e mov rcx, qword ptr [r14]
0x1407e36b1 48898d40010000 mov qword ptr [rbp + 0x140], rcx
0x1407e36b8 498b4e10 mov rcx, qword ptr [r14 + 0x10]
0x1407e36bc 48898d48010000 mov qword ptr [rbp + 0x148], rcx
0x1407e36c3 498b4d00 mov rcx, qword ptr [r13]
0x1407e36c7 48898d50010000 mov qword ptr [rbp + 0x150], rcx
0x1407e36ce 488b00 mov rax, qword ptr [rax]
0x1407e36d1 48898558010000 mov qword ptr [rbp + 0x158], rax
0x1407e36d8 660f76f6 pcmpeqd xmm6, xmm6
0x1407e36dc 440f28357cd61402 movaps xmm14, xmmword ptr [rip + 0x214d67c]
0x1407e36e4 4c8bb5e8010000 mov r14, qword ptr [rbp + 0x1e8]
0x1407e36eb 440fb795f8010000 movzx r10d, word ptr [rbp + 0x1f8]
0x1407e36f3 4c8b8d00020000 mov r9, qword ptr [rbp + 0x200]
0x1407e36fa 488d8dc0010000 lea rcx, [rbp + 0x1c0]
0x1407e3701 660f6e3dfb211402 movd xmm7, dword ptr [rip + 0x21421fb]
0x1407e3709 f3440f7e3d1e562a02 movq xmm15, qword ptr [rip + 0x22a561e]
0x1407e3712 66440f6e2d25562a02 movd xmm13, dword ptr [rip + 0x22a5625]
0x1407e371b f3440f7e1dfc552a02 movq xmm11, qword ptr [rip + 0x22a55fc]
0x1407e3724 f3440f7e15b3552a02 movq xmm10, qword ptr [rip + 0x22a55b3]
0x1407e372d 66440f6e05ba552a02 movd xmm8, dword ptr [rip + 0x22a55ba]
0x1407e3736 f3440f7e2581552a02 movq xmm12, qword ptr [rip + 0x22a5581]
0x1407e373f 66440f6e0d88552a02 movd xmm9, dword ptr [rip + 0x22a5588]
0x1407e3748 4c8985d8020000 mov qword ptr [rbp + 0x2d8], r8
0x1407e374f 4983f802 cmp r8, 2
0x1407e3753 0f8427010000 je 0x1407e3880
0x1407e3759 4d85ff test r15, r15
0x1407e375c 0f84a7000000 je 0x1407e3809
0x1407e3762 4d85c9 test r9, r9
0x1407e3765 0f84e8000000 je 0x1407e3853
0x1407e376b 664585d2 test r10w, r10w
0x1407e376f 4c898d00030000 mov qword ptr [rbp + 0x300], r9
0x1407e3776 7418 je 0x1407e3790
0x1407e3778 4889cf mov rdi, rcx
0x1407e377b 418d42ff lea eax, [r10 - 1]
0x1407e377f 4421d0 and eax, r10d
0x1407e3782 898518030000 mov dword ptr [rbp + 0x318], eax
0x1407e3788 eb3d jmp 0x1407e37c7
0x1407e378a 660f1f440000 nop word ptr [rax + rax]
0x1407e3790 66410f6f06 movdqa xmm0, xmmword ptr [r14]
0x1407e3795 66440fd7d0 pmovmskb r10d, xmm0
0x1407e379a 4981c780f6ffff add r15, -0x980
0x1407e37a1 4983c610 add r14, 0x10
0x1407e37a5 4181faffff0000 cmp r10d, 0xffff
0x1407e37ac 74e2 je 0x1407e3790
0x1407e37ae 4889cf mov rdi, rcx
0x1407e37b1 b8feffffff mov eax, 0xfffffffe
0x1407e37b6 4429d0 sub eax, r10d
0x1407e37b9 41f7d2 not r10d
0x1407e37bc 4489d1 mov ecx, r10d
0x1407e37bf 21c1 and ecx, eax
0x1407e37c1 898d18030000 mov dword ptr [rbp + 0x318], ecx
0x1407e37c7 f3410fbcc2 tzcnt eax, r10d
0x1407e37cc 48f7d8 neg rax
0x1407e37cf 4869c098000000 imul rax, rax, 0x98
0x1407e37d6 4889c6 mov rsi, rax
0x1407e37d9 498d0c07 lea rcx, [r15 + rax]
0x1407e37dd 4881c168ffffff add rcx, -0x98
0x1407e37e4 e8b78a2300 call 0x140a1c2a0
0x1407e37e9 48ff8d00030000 dec qword ptr [rbp + 0x300]
0x1407e37f0 4883f803 cmp rax, 3
0x1407e37f4 756e jne 0x1407e3864
0x1407e37f6 8b8518030000 mov eax, dword ptr [rbp + 0x318]
0x1407e37fc 4189c2 mov r10d, eax
0x1407e37ff 4c8b8d00030000 mov r9, qword ptr [rbp + 0x300]
0x1407e3806 4889f9 mov rcx, rdi
0x1407e3809 f685d802000001 test byte ptr [rbp + 0x2d8], 1
0x1407e3810 744d je 0x1407e385f
0x1407e3812 b801000000 mov eax, 1
0x1407e3817 41bf00000000 mov r15d, 0
0x1407e381d 41b800000000 mov r8d, 0
0x1407e3823 488b95f8020000 mov rdx, qword ptr [rbp + 0x2f8]
0x1407e382a 4885d2 test rdx, rdx
0x1407e382d 7458 je 0x1407e3887
0x1407e382f 4c898d00030000 mov qword ptr [rbp + 0x300], r9
0x1407e3836 4589d0 mov r8d, r10d
0x1407e3839 44898518030000 mov dword ptr [rbp + 0x318], r8d
0x1407e3840 4531ff xor r15d, r15d
0x1407e3843 48c785f802000000000000 mov qword ptr [rbp + 0x2f8], 0
0x1407e384e e980000000 jmp 0x1407e38d3
0x1407e3853 4531c9 xor r9d, r9d
0x1407e3856 f685d802000001 test byte ptr [rbp + 0x2d8], 1
0x1407e385d 75b3 jne 0x1407e3812
0x1407e385f 4531ff xor r15d, r15d
0x1407e3862 eb1c jmp 0x1407e3880
0x1407e3864 4889d1 mov rcx, rdx
0x1407e3867 4889f2 mov rdx, rsi
0x1407e386a 4c01fa add rdx, r15
0x1407e386d 4881c270ffffff add rdx, -0x90
0x1407e3874 48898d38010000 mov qword ptr [rbp + 0x138], rcx
0x1407e387b 4889f9 mov rcx, rdi
0x1407e387e eb53 jmp 0x1407e38d3
0x1407e3880 4c8b85f8020000 mov r8, qword ptr [rbp + 0x2f8]
0x1407e3887 488b9560020000 mov rdx, qword ptr [rbp + 0x260]
0x1407e388e 4885d2 test rdx, rdx
0x1407e3891 0f95c0 setne al
0x1407e3894 84851f030000 test byte ptr [rbp + 0x31f], al
0x1407e389a 0f84e60b0000 je 0x1407e4486
0x1407e38a0 4c898d00030000 mov qword ptr [rbp + 0x300], r9
0x1407e38a7 b802000000 mov eax, 2
0x1407e38ac 488985d8020000 mov qword ptr [rbp + 0x2d8], rax
0x1407e38b3 4489d0 mov eax, r10d
0x1407e38b6 898518030000 mov dword ptr [rbp + 0x318], eax
0x1407e38bc 4c8985f8020000 mov qword ptr [rbp + 0x2f8], r8
0x1407e38c3 48c7856002000000000000 mov qword ptr [rbp + 0x260], 0
0x1407e38ce b802000000 mov eax, 2
0x1407e38d3 488985a0010000 mov qword ptr [rbp + 0x1a0], rax
0x1407e38da 488b8538010000 mov rax, qword ptr [rbp + 0x138]
0x1407e38e1 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
0x1407e38e8 e883772300 call 0x140a1b070
0x1407e38ed 488bb5c0010000 mov rsi, qword ptr [rbp + 0x1c0]
0x1407e38f4 4c8ba5c8010000 mov r12, qword ptr [rbp + 0x1c8]
0x1407e38fb 440fb7add8010000 movzx r13d, word ptr [rbp + 0x1d8]
0x1407e3903 488b85e0010000 mov rax, qword ptr [rbp + 0x1e0]
0x1407e390a 48898508030000 mov qword ptr [rbp + 0x308], rax
0x1407e3911 488b85e8010000 mov rax, qword ptr [rbp + 0x1e8]
0x1407e3918 488985f0020000 mov qword ptr [rbp + 0x2f0], rax
0x1407e391f 4883c050 add rax, 0x50
0x1407e3923 48898528010000 mov qword ptr [rbp + 0x128], rax
0x1407e392a 4489ea mov edx, r13d
0x1407e392d 4883bd0803000000 cmp qword ptr [rbp + 0x308], 0
0x1407e3935 0f84e50a0000 je 0x1407e4420
0x1407e393b 6685d2 test dx, dx
0x1407e393e 7420 je 0x1407e3960
0x1407e3940 448d6aff lea r13d, [rdx - 1]
0x1407e3944 4121d5 and r13d, edx
0x1407e3947 4885f6 test rsi, rsi
0x1407e394a 753d jne 0x1407e3989
0x1407e394c e9cf0a0000 jmp 0x1407e4420
0x1407e3951 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1407e3960 66410f6f0424 movdqa xmm0, xmmword ptr [r12]
0x1407e3966 660fd7d0 pmovmskb edx, xmm0
0x1407e396a 4883c6c0 add rsi, -0x40
0x1407e396e 4983c410 add r12, 0x10
0x1407e3972 81faffff0000 cmp edx, 0xffff
0x1407e3978 74e6 je 0x1407e3960
0x1407e397a b8feffffff mov eax, 0xfffffffe
0x1407e397f 29d0 sub eax, edx
0x1407e3981 f7d2 not edx
0x1407e3983 4189d5 mov r13d, edx
0x1407e3986 4121c5 and r13d, eax
0x1407e3989 f30fbcc2 tzcnt eax, edx
0x1407e398d c1e002 shl eax, 2
0x1407e3990 4889f1 mov rcx, rsi
0x1407e3993 4829c1 sub rcx, rax
0x1407e3996 8b79fc mov edi, dword ptr [rcx - 4]
0x1407e3999 89bdc0010000 mov dword ptr [rbp + 0x1c0], edi
0x1407e399f 488b85f0020000 mov rax, qword ptr [rbp + 0x2f0]
0x1407e39a6 4883784800 cmp qword ptr [rax + 0x48], 0
0x1407e39ab 0f84a30a0000 je 0x1407e4454
0x1407e39b1 488b8d28010000 mov rcx, qword ptr [rbp + 0x128]
0x1407e39b8 488d95c0010000 lea rdx, [rbp + 0x1c0]
0x1407e39bf e80cd5dcff call 0x1405b0ed0
0x1407e39c4 48ff8d08030000 dec qword ptr [rbp + 0x308]
0x1407e39cb 488b95f0020000 mov rdx, qword ptr [rbp + 0x2f0]
0x1407e39d2 488b4a30 mov rcx, qword ptr [rdx + 0x30]
0x1407e39d6 488b5238 mov rdx, qword ptr [rdx + 0x38]
0x1407e39da 4989d0 mov r8, rdx
0x1407e39dd 4921c0 and r8, rax
0x1407e39e0 48c1e839 shr rax, 0x39
0x1407e39e4 660f6ec0 movd xmm0, eax
0x1407e39e8 660f60c0 punpcklbw xmm0, xmm0
0x1407e39ec f20f70c000 pshuflw xmm0, xmm0, 0
0x1407e39f1 660f70c000 pshufd xmm0, xmm0, 0
0x1407e39f6 488d41b0 lea rax, [rcx - 0x50]
0x1407e39fa 4531c9 xor r9d, r9d
0x1407e39fd f3420f6f0c01 movdqu xmm1, xmmword ptr [rcx + r8]
0x1407e3a03 660f6fd1 movdqa xmm2, xmm1
0x1407e3a07 660f74d0 pcmpeqb xmm2, xmm0
0x1407e3a0b 66440fd7d2 pmovmskb r10d, xmm2
0x1407e3a10 4585d2 test r10d, r10d
0x1407e3a13 742b je 0x1407e3a40
0x1407e3a15 f3450fbcda tzcnt r11d, r10d
0x1407e3a1a 4d01c3 add r11, r8
0x1407e3a1d 4921d3 and r11, rdx
0x1407e3a20 49f7db neg r11
0x1407e3a23 4f8d1c9b lea r11, [r11 + r11*4]
0x1407e3a27 49c1e304 shl r11, 4
0x1407e3a2b 423b3c18 cmp edi, dword ptr [rax + r11]
0x1407e3a2f 743f je 0x1407e3a70
0x1407e3a31 458d5aff lea r11d, [r10 - 1]
0x1407e3a35 664521d3 and r11w, r10w
0x1407e3a39 4589da mov r10d, r11d
0x1407e3a3c 75d7 jne 0x1407e3a15
0x1407e3a3e 6690 nop 
0x1407e3a40 660f74ce pcmpeqb xmm1, xmm6
0x1407e3a44 66440fd7d1 pmovmskb r10d, xmm1
0x1407e3a49 4585d2 test r10d, r10d
0x1407e3a4c 0f85020a0000 jne 0x1407e4454
0x1407e3a52 4d01c8 add r8, r9
0x1407e3a55 4983c010 add r8, 0x10
0x1407e3a59 4983c110 add r9, 0x10
0x1407e3a5d 4921d0 and r8, rdx
0x1407e3a60 eb9b jmp 0x1407e39fd
0x1407e3a62 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1407e3a70 420fb65c19fe movzx ebx, byte ptr [rcx + r11 - 2]
0x1407e3a76 8d43c8 lea eax, [rbx - 0x38]
0x1407e3a79 3c08 cmp al, 8
0x1407e3a7b 410f92c0 setb r8b
0x1407e3a7f 3c06 cmp al, 6
0x1407e3a81 0f95c0 setne al
0x1407e3a84 4489ea mov edx, r13d
0x1407e3a87 4184c0 test r8b, al
0x1407e3a8a 0f859dfeffff jne 0x1407e392d
0x1407e3a90 4c01d9 add rcx, r11
0x1407e3a93 0fb641da movzx eax, byte ptr [rcx - 0x26]
0x1407e3a97 8d48fe lea ecx, [rax - 2]
0x1407e3a9a 48ffc8 dec rax
0x1407e3a9d 80f903 cmp cl, 3
0x1407e3aa0 b900000000 mov ecx, 0
0x1407e3aa5 480f43c1 cmovae rax, rcx
0x1407e3aa9 4489ea mov edx, r13d
0x1407e3aac 4885c0 test rax, rax
0x1407e3aaf 0f8478feffff je 0x1407e392d
0x1407e3ab5 4883f801 cmp rax, 1
0x1407e3ab9 0f85d5000000 jne 0x1407e3b94
0x1407e3abf 488d8da0010000 lea rcx, [rbp + 0x1a0]
0x1407e3ac6 e875872300 call 0x140a1c240
0x1407e3acb 488b8d30010000 mov rcx, qword ptr [rbp + 0x130]
0x1407e3ad2 4889c2 mov rdx, rax
0x1407e3ad5 e8d6e72a00 call 0x140a922b0
0x1407e3ada 4889c1 mov rcx, rax
0x1407e3add 48c1e920 shr rcx, 0x20
0x1407e3ae1 0f849b000000 je 0x1407e3b82
0x1407e3ae7 89c0 mov eax, eax
0x1407e3ae9 488b9568020000 mov rdx, qword ptr [rbp + 0x268]
0x1407e3af0 483b4210 cmp rax, qword ptr [rdx + 0x10]
0x1407e3af4 0f8388000000 jae 0x1407e3b82
0x1407e3afa 488b5208 mov rdx, qword ptr [rdx + 8]
0x1407e3afe 488d0480 lea rax, [rax + rax*4]
0x1407e3b02 390c82 cmp dword ptr [rdx + rax*4], ecx
0x1407e3b05 757b jne 0x1407e3b82
0x1407e3b07 488d0482 lea rax, [rdx + rax*4]
0x1407e3b0b 8b4804 mov ecx, dword ptr [rax + 4]
0x1407e3b0e baffffffff mov edx, 0xffffffff
0x1407e3b13 4839d1 cmp rcx, rdx
0x1407e3b16 746a je 0x1407e3b82
0x1407e3b18 4c8b8d60010000 mov r9, qword ptr [rbp + 0x160]
0x1407e3b1f 49394938 cmp qword ptr [r9 + 0x38], rcx
0x1407e3b23 765d jbe 0x1407e3b82
0x1407e3b25 498b5128 mov rdx, qword ptr [r9 + 0x28]
0x1407e3b29 4189c8 mov r8d, ecx
0x1407e3b2c 41c1e806 shr r8d, 6
0x1407e3b30 4a8b14c2 mov rdx, qword ptr [rdx + r8*8]
0x1407e3b34 480fa3ca bt rdx, rcx
0x1407e3b38 7348 jae 0x1407e3b82
0x1407e3b3a 8b480c mov ecx, dword ptr [rax + 0xc]
0x1407e3b3d 8b4010 mov eax, dword ptr [rax + 0x10]
0x1407e3b40 488b9568020000 mov rdx, qword ptr [rbp + 0x268]
0x1407e3b47 488b92a8010000 mov rdx, qword ptr [rdx + 0x1a8]
0x1407e3b4e 488d0cc9 lea rcx, [rcx + rcx*8]
0x1407e3b52 4d8b8110010000 mov r8, qword ptr [r9 + 0x110]
0x1407e3b59 4c8b4cca18 mov r9, qword ptr [rdx + rcx*8 + 0x18]
0x1407e3b5e 488b4cca38 mov rcx, qword ptr [rdx + rcx*8 + 0x38]
0x1407e3b63 4a8b0cc1 mov rcx, qword ptr [rcx + r8*8]
0x1407e3b67 48f7d1 not rcx
0x1407e3b6a 488d1449 lea rdx, [rcx + rcx*2]
0x1407e3b6e 48c1e204 shl rdx, 4
0x1407e3b72 486bc858 imul rcx, rax, 0x58
0x1407e3b76 49034c1110 add rcx, qword ptr [r9 + rdx + 0x10]
0x1407e3b7b e800f80f00 call 0x1408e3380
0x1407e3b80 eb02 jmp 0x1407e3b84
0x1407e3b82 31c0 xor eax, eax
0x1407e3b84 80fb09 cmp bl, 9
0x1407e3b87 740b je 0x1407e3b94
0x1407e3b89 4489ea mov edx, r13d
0x1407e3b8c 84c0 test al, al
0x1407e3b8e 0f8499fdffff je 0x1407e392d
0x1407e3b94 8b858c020000 mov eax, dword ptr [rbp + 0x28c]
0x1407e3b9a 488b8d48010000 mov rcx, qword ptr [rbp + 0x148]
0x1407e3ba1 8901 mov dword ptr [rcx], eax
0x1407e3ba3 4c8b85a0010000 mov r8, qword ptr [rbp + 0x1a0]
0x1407e3baa 4c8b8da8010000 mov r9, qword ptr [rbp + 0x1a8]
0x1407e3bb1 c644242000 mov byte ptr [rsp + 0x20], 0
0x1407e3bb6 488b8d40010000 mov rcx, qword ptr [rbp + 0x140]
0x1407e3bbd 89fa mov edx, edi
0x1407e3bbf e8ec1c0400 call 0x1408258b0
0x1407e3bc4 48898590020000 mov qword ptr [rbp + 0x290], rax
0x1407e3bcb 488d8de0000000 lea rcx, [rbp + 0xe0]
0x1407e3bd2 488b9550010000 mov rdx, qword ptr [rbp + 0x150]
0x1407e3bd9 4189f8 mov r8d, edi
0x1407e3bdc 4c8d0d45542a02 lea r9, [rip + 0x22a5445]
0x1407e3be3 e8386e2300 call 0x140a1aa20
0x1407e3be8 488d8dc0010000 lea rcx, [rbp + 0x1c0]
0x1407e3bef 89da mov edx, ebx
0x1407e3bf1 4c8b8558010000 mov r8, qword ptr [rbp + 0x158]
0x1407e3bf8 e893520d00 call 0x1408b8e90
0x1407e3bfd 0fb6db movzx ebx, bl
0x1407e3c00 488bbdc0010000 mov rdi, qword ptr [rbp + 0x1c0]
0x1407e3c07 488b85c8010000 mov rax, qword ptr [rbp + 0x1c8]
0x1407e3c0e 48898570010000 mov qword ptr [rbp + 0x170], rax
0x1407e3c15 488b85d0010000 mov rax, qword ptr [rbp + 0x1d0]
0x1407e3c1c 48898578010000 mov qword ptr [rbp + 0x178], rax
0x1407e3c23 f30f6f07 movdqu xmm0, xmmword ptr [rdi]
0x1407e3c27 f30f6f4f10 movdqu xmm1, xmmword ptr [rdi + 0x10]
0x1407e3c2c 660f7f8dc0020000 movdqa xmmword ptr [rbp + 0x2c0], xmm1
0x1407e3c34 660f7f85b0020000 movdqa xmmword ptr [rbp + 0x2b0], xmm0
0x1407e3c3c 83fb2b cmp ebx, 0x2b
0x1407e3c3f 0f87c8000000 ja 0x1407e3d0d
0x1407e3c45 89d8 mov eax, ebx
0x1407e3c47 488d0d0a542a02 lea rcx, [rip + 0x22a540a]
0x1407e3c4e 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x1407e3c52 4801c8 add rax, rcx
0x1407e3c55 660f6e052f8c2902 movd xmm0, dword ptr [rip + 0x2298c2f]
0x1407e3c5d f30f7e0d4b502a02 movq xmm1, qword ptr [rip + 0x22a504b]
0x1407e3c65 ffe0 jmp rax
0x1407e3c67 660f6e051d8c2902 movd xmm0, dword ptr [rip + 0x2298c1d]
0x1407e3c6f f30f7e0d09512a02 movq xmm1, qword ptr [rip + 0x22a5109]
0x1407e3c77 eb71 jmp 0x1407e3cea
0x1407e3c79 66410f6fc5 movdqa xmm0, xmm13
0x1407e3c7e 66410f6fcf movdqa xmm1, xmm15
0x1407e3c83 eb65 jmp 0x1407e3cea
0x1407e3c85 660f6fc7 movdqa xmm0, xmm7
0x1407e3c89 66410f6fcb movdqa xmm1, xmm11
0x1407e3c8e eb5a jmp 0x1407e3cea
0x1407e3c90 66410f6fc1 movdqa xmm0, xmm9
0x1407e3c95 66410f6fcc movdqa xmm1, xmm12
0x1407e3c9a eb4e jmp 0x1407e3cea
0x1407e3c9c 660f6e05fc502a02 movd xmm0, dword ptr [rip + 0x22a50fc]
0x1407e3ca4 f30f7e0de4502a02 movq xmm1, qword ptr [rip + 0x22a50e4]
0x1407e3cac eb3c jmp 0x1407e3cea
0x1407e3cae 66410f6fc0 movdqa xmm0, xmm8
0x1407e3cb3 66410f6fca movdqa xmm1, xmm10
0x1407e3cb8 eb30 jmp 0x1407e3cea
0x1407e3cba 660f6fc7 movdqa xmm0, xmm7
0x1407e3cbe f30f7e0d8a502a02 movq xmm1, qword ptr [rip + 0x22a508a]
0x1407e3cc6 eb22 jmp 0x1407e3cea
0x1407e3cc8 660f6e05a0502a02 movd xmm0, dword ptr [rip + 0x22a50a0]
0x1407e3cd0 f30f7e0d88502a02 movq xmm1, qword ptr [rip + 0x22a5088]
0x1407e3cd8 eb10 jmp 0x1407e3cea
0x1407e3cda 660f6e052e502a02 movd xmm0, dword ptr [rip + 0x22a502e]
0x1407e3ce2 f30f7e0d16502a02 movq xmm1, qword ptr [rip + 0x22a5016]
0x1407e3cea 660fd68d18010000 movq qword ptr [rbp + 0x118], xmm1
0x1407e3cf2 660f7e8520010000 movd dword ptr [rbp + 0x120], xmm0
0x1407e3cfa 488d8db0020000 lea rcx, [rbp + 0x2b0]
0x1407e3d01 488d9518010000 lea rdx, [rbp + 0x118]
0x1407e3d08 e823484b00 call 0x140c98530
0x1407e3d0d f30f1085cc020000 movss xmm0, dword ptr [rbp + 0x2cc]
0x1407e3d15 f30f5c85c8020000 subss xmm0, dword ptr [rbp + 0x2c8]
0x1407e3d1d f30f108dc0020000 movss xmm1, dword ptr [rbp + 0x2c0]
0x1407e3d25 660f6e95c4020000 movd xmm2, dword ptr [rbp + 0x2c4]
0x1407e3d2d f30f118d70020000 movss dword ptr [rbp + 0x270], xmm1
0x1407e3d35 f30f118574020000 movss dword ptr [rbp + 0x274], xmm0
0x1407e3d3d 660f7e9578020000 movd dword ptr [rbp + 0x278], xmm2
0x1407e3d45 0f28c8 movaps xmm1, xmm0
0x1407e3d48 f30f590d40c91702 mulss xmm1, dword ptr [rip + 0x217c940]
0x1407e3d50 0f14c8 unpcklps xmm1, xmm0
0x1407e3d53 0f590d861b1402 mulps xmm1, xmmword ptr [rip + 0x2141b86]
0x1407e3d5a 0f138dc0010000 movlps qword ptr [rbp + 0x1c0], xmm1
0x1407e3d61 f30f118dc8010000 movss dword ptr [rbp + 0x1c8], xmm1
0x1407e3d69 488d4d50 lea rcx, [rbp + 0x50]
0x1407e3d6d 488d9570020000 lea rdx, [rbp + 0x270]
0x1407e3d74 4c8d85c0010000 lea r8, [rbp + 0x1c0]
0x1407e3d7b e870c20e00 call 0x1408cfff0
0x1407e3d80 f30f108dcc020000 movss xmm1, dword ptr [rbp + 0x2cc]
0x1407e3d88 f30f5c8dc8020000 subss xmm1, dword ptr [rbp + 0x2c8]
0x1407e3d90 0f2895c0020000 movaps xmm2, xmmword ptr [rbp + 0x2c0]
0x1407e3d97 f30f1085c4020000 movss xmm0, dword ptr [rbp + 0x2c4]
0x1407e3d9f f30f580541631702 addss xmm0, dword ptr [rip + 0x2176341]
0x1407e3da7 f30f5f05d51b1402 maxss xmm0, dword ptr [rip + 0x2141bd5]
0x1407e3daf 0f28d9 movaps xmm3, xmm1
0x1407e3db2 f30f591dd6c81702 mulss xmm3, dword ptr [rip + 0x217c8d6]
0x1407e3dba 0f14d9 unpcklps xmm3, xmm1
0x1407e3dbd 0f16ca movlhps xmm1, xmm2
0x1407e3dc0 0fc6cae2 shufps xmm1, xmm2, 0xe2
0x1407e3dc4 0f580d25342302 addps xmm1, xmmword ptr [rip + 0x2233425]
0x1407e3dcb 410f28d6 movaps xmm2, xmm14
0x1407e3dcf 0f5fd1 maxps xmm2, xmm1
0x1407e3dd2 0fc2c903 cmpunordps xmm1, xmm1
0x1407e3dd6 410f28e6 movaps xmm4, xmm14
0x1407e3dda 0f54e1 andps xmm4, xmm1
0x1407e3ddd 0f55ca andnps xmm1, xmm2
0x1407e3de0 0f56cc orps xmm1, xmm4
0x1407e3de3 0f138d70020000 movlps qword ptr [rbp + 0x270], xmm1
0x1407e3dea f30f118578020000 movss dword ptr [rbp + 0x278], xmm0
0x1407e3df2 0f591de71a1402 mulps xmm3, xmmword ptr [rip + 0x2141ae7]
0x1407e3df9 0f139dc0010000 movlps qword ptr [rbp + 0x1c0], xmm3
0x1407e3e00 f30f119dc8010000 movss dword ptr [rbp + 0x1c8], xmm3
0x1407e3e08 c6852703000001 mov byte ptr [rbp + 0x327], 1
0x1407e3e0f 488d4dc0 lea rcx, [rbp - 0x40]
0x1407e3e13 488d9570020000 lea rdx, [rbp + 0x270]
0x1407e3e1a 4c8d85c0010000 lea r8, [rbp + 0x1c0]
0x1407e3e21 e8cac10e00 call 0x1408cfff0
0x1407e3e26 48899d10030000 mov qword ptr [rbp + 0x310], rbx
0x1407e3e2d 4889bd68010000 mov qword ptr [rbp + 0x168], rdi
0x1407e3e34 48c785c001000001000000 mov qword ptr [rbp + 0x1c0], 1
0x1407e3e3f 48c785c801000001000000 mov qword ptr [rbp + 0x1c8], 1
0x1407e3e4a 0f284530 movaps xmm0, xmmword ptr [rbp + 0x30]
0x1407e3e4e 488dbdd0010000 lea rdi, [rbp + 0x1d0]
0x1407e3e55 0f294770 movaps xmmword ptr [rdi + 0x70], xmm0
0x1407e3e59 0f284520 movaps xmm0, xmmword ptr [rbp + 0x20]
0x1407e3e5d 0f294760 movaps xmmword ptr [rdi + 0x60], xmm0
0x1407e3e61 0f284510 movaps xmm0, xmmword ptr [rbp + 0x10]
0x1407e3e65 0f294750 movaps xmmword ptr [rdi + 0x50], xmm0
0x1407e3e69 0f284500 movaps xmm0, xmmword ptr [rbp]
0x1407e3e6d 0f294740 movaps xmmword ptr [rdi + 0x40], xmm0
0x1407e3e71 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
0x1407e3e75 0f284dd0 movaps xmm1, xmmword ptr [rbp - 0x30]
0x1407e3e79 0f2855e0 movaps xmm2, xmmword ptr [rbp - 0x20]
0x1407e3e7d 0f285df0 movaps xmm3, xmmword ptr [rbp - 0x10]
0x1407e3e81 0f295f30 movaps xmmword ptr [rdi + 0x30], xmm3
0x1407e3e85 0f295720 movaps xmmword ptr [rdi + 0x20], xmm2
0x1407e3e89 0f294f10 movaps xmmword ptr [rdi + 0x10], xmm1
0x1407e3e8d 0f2907 movaps xmmword ptr [rdi], xmm0
0x1407e3e90 0fb605720de502 movzx eax, byte ptr [rip + 0x2e50d72]
0x1407e3e97 b990000000 mov ecx, 0x90
0x1407e3e9c ba10000000 mov edx, 0x10
0x1407e3ea1 e86afde2ff call 0x140613c10
0x1407e3ea6 4885c0 test rax, rax
0x1407e3ea9 0f8412070000 je 0x1407e45c1
0x1407e3eaf 41b890000000 mov r8d, 0x90
0x1407e3eb5 4889c1 mov rcx, rax
0x1407e3eb8 488d95c0010000 lea rdx, [rbp + 0x1c0]
0x1407e3ebf 4889c3 mov rbx, rax
0x1407e3ec2 e839fd1302 call 0x142923c00
0x1407e3ec7 48899de0020000 mov qword ptr [rbp + 0x2e0], rbx
0x1407e3ece 48899d70020000 mov qword ptr [rbp + 0x270], rbx
0x1407e3ed5 0f2885c0000000 movaps xmm0, xmmword ptr [rbp + 0xc0]
0x1407e3edc 0f294770 movaps xmmword ptr [rdi + 0x70], xmm0
0x1407e3ee0 0f2885b0000000 movaps xmm0, xmmword ptr [rbp + 0xb0]
0x1407e3ee7 0f294760 movaps xmmword ptr [rdi + 0x60], xmm0
0x1407e3eeb 0f2885a0000000 movaps xmm0, xmmword ptr [rbp + 0xa0]
0x1407e3ef2 0f294750 movaps xmmword ptr [rdi + 0x50], xmm0
0x1407e3ef6 0f288590000000 movaps xmm0, xmmword ptr [rbp + 0x90]
0x1407e3efd 0f294740 movaps xmmword ptr [rdi + 0x40], xmm0
0x1407e3f01 0f284550 movaps xmm0, xmmword ptr [rbp + 0x50]
0x1407e3f05 0f284d60 movaps xmm1, xmmword ptr [rbp + 0x60]
0x1407e3f09 0f285570 movaps xmm2, xmmword ptr [rbp + 0x70]
0x1407e3f0d 0f289d80000000 movaps xmm3, xmmword ptr [rbp + 0x80]
0x1407e3f14 0f295f30 movaps xmmword ptr [rdi + 0x30], xmm3
0x1407e3f18 0f295720 movaps xmmword ptr [rdi + 0x20], xmm2
0x1407e3f1c 0f294f10 movaps xmmword ptr [rdi + 0x10], xmm1
0x1407e3f20 0f2907 movaps xmmword ptr [rdi], xmm0
0x1407e3f23 48c785c001000001000000 mov qword ptr [rbp + 0x1c0], 1
0x1407e3f2e 48c785c801000001000000 mov qword ptr [rbp + 0x1c8], 1
0x1407e3f39 0fb605c90ce502 movzx eax, byte ptr [rip + 0x2e50cc9]
0x1407e3f40 b990000000 mov ecx, 0x90
0x1407e3f45 ba10000000 mov edx, 0x10
0x1407e3f4a e8c1fce2ff call 0x140613c10
0x1407e3f4f 4885c0 test rax, rax
0x1407e3f52 48898598020000 mov qword ptr [rbp + 0x298], rax
0x1407e3f59 0f8451060000 je 0x1407e45b0
0x1407e3f5f 4889c7 mov rdi, rax
0x1407e3f62 41b890000000 mov r8d, 0x90
0x1407e3f68 4889c1 mov rcx, rax
0x1407e3f6b 488d9dc0010000 lea rbx, [rbp + 0x1c0]
0x1407e3f72 4889da mov rdx, rbx
0x1407e3f75 e886fc1302 call 0x142923c00
0x1407e3f7a 488d8df0000000 lea rcx, [rbp + 0xf0]
0x1407e3f81 8b4108 mov eax, dword ptr [rcx + 8]
0x1407e3f84 8985c8010000 mov dword ptr [rbp + 0x1c8], eax
0x1407e3f8a 488b01 mov rax, qword ptr [rcx]
0x1407e3f8d 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
0x1407e3f94 0f2885e0000000 movaps xmm0, xmmword ptr [rbp + 0xe0]
0x1407e3f9b 0f294540 movaps xmmword ptr [rbp + 0x40], xmm0
0x1407e3f9f 488d4540 lea rax, [rbp + 0x40]
0x1407e3fa3 4889442420 mov qword ptr [rsp + 0x20], rax
0x1407e3fa8 4989f8 mov r8, rdi
0x1407e3fab 488bbd90020000 mov rdi, qword ptr [rbp + 0x290]
0x1407e3fb2 4889f9 mov rcx, rdi
0x1407e3fb5 488b95e0020000 mov rdx, qword ptr [rbp + 0x2e0]
0x1407e3fbc 4989d9 mov r9, rbx
0x1407e3fbf e8ec120400 call 0x1408252b0
0x1407e3fc4 488b07 mov rax, qword ptr [rdi]
0x1407e3fc7 480b4708 or rax, qword ptr [rdi + 8]
0x1407e3fcb 488d85d0010000 lea rax, [rbp + 0x1d0]
0x1407e3fd2 488b9568010000 mov rdx, qword ptr [rbp + 0x168]
0x1407e3fd9 4c8b8510030000 mov r8, qword ptr [rbp + 0x310]
0x1407e3fe0 0f8587050000 jne 0x1407e456d
0x1407e3fe6 48c7470800000000 mov qword ptr [rdi + 8], 0
0x1407e3fee 48c70701000000 mov qword ptr [rdi], 1
0x1407e3ff5 0f2885e0000000 movaps xmm0, xmmword ptr [rbp + 0xe0]
0x1407e3ffc 0f288df0000000 movaps xmm1, xmmword ptr [rbp + 0xf0]
0x1407e4003 0f289500010000 movaps xmm2, xmmword ptr [rbp + 0x100]
0x1407e400a 0f294710 movaps xmmword ptr [rdi + 0x10], xmm0
0x1407e400e 0f294f20 movaps xmmword ptr [rdi + 0x20], xmm1
0x1407e4012 0f295730 movaps xmmword ptr [rdi + 0x30], xmm2
0x1407e4016 4c8b4a20 mov r9, qword ptr [rdx + 0x20]
0x1407e401a 4c8b5228 mov r10, qword ptr [rdx + 0x28]
0x1407e401e c740fb00000000 mov dword ptr [rax - 5], 0
0x1407e4025 48c740f300000000 mov qword ptr [rax - 0xd], 0
0x1407e402d 488b85c0010000 mov rax, qword ptr [rbp + 0x1c0]
0x1407e4034 488b8dc7010000 mov rcx, qword ptr [rbp + 0x1c7]
0x1407e403b 48898db7020000 mov qword ptr [rbp + 0x2b7], rcx
0x1407e4042 488985b0020000 mov qword ptr [rbp + 0x2b0], rax
0x1407e4049 4183c0fd add r8d, -3
0x1407e404d 4183f829 cmp r8d, 0x29
0x1407e4051 4c898de0020000 mov qword ptr [rbp + 0x2e0], r9
0x1407e4058 4c899598020000 mov qword ptr [rbp + 0x298], r10
0x1407e405f 0f87ac030000 ja 0x1407e4411
0x1407e4065 488d0d9c502a02 lea rcx, [rip + 0x22a509c]
0x1407e406c 4a630481 movsxd rax, dword ptr [rcx + r8*4]
0x1407e4070 4801c8 add rax, rcx
0x1407e4073 ffe0 jmp rax
0x1407e4075 c7851003000001000000 mov dword ptr [rbp + 0x310], 1
0x1407e407f f30f100589eb2902 movss xmm0, dword ptr [rip + 0x229eb89]
0x1407e4087 f30f118520030000 movss dword ptr [rbp + 0x320], xmm0
0x1407e408f f20f100529cc2902 movsd xmm0, qword ptr [rip + 0x229cc29]
0x1407e4097 e9cd000000 jmp 0x1407e4169
0x1407e409c c7851003000001000000 mov dword ptr [rbp + 0x310], 1
0x1407e40a6 0f57c0 xorps xmm0, xmm0
0x1407e40a9 f30f118520030000 movss dword ptr [rbp + 0x320], xmm0
0x1407e40b1 f20f1005074d2a02 movsd xmm0, qword ptr [rip + 0x22a4d07]
0x1407e40b9 e9ab000000 jmp 0x1407e4169
0x1407e40be c7851003000001000000 mov dword ptr [rbp + 0x310], 1
0x1407e40c8 0f57c0 xorps xmm0, xmm0
0x1407e40cb f30f118520030000 movss dword ptr [rbp + 0x320], xmm0
0x1407e40d3 f20f100585181402 movsd xmm0, qword ptr [rip + 0x2141885]
0x1407e40db e989000000 jmp 0x1407e4169
0x1407e40e0 c7851003000001000000 mov dword ptr [rbp + 0x310], 1
0x1407e40ea f30f1005fe4c2a02 movss xmm0, dword ptr [rip + 0x22a4cfe]
0x1407e40f2 f30f118520030000 movss dword ptr [rbp + 0x320], xmm0
0x1407e40fa f20f1005fe4c2a02 movsd xmm0, qword ptr [rip + 0x22a4cfe]
0x1407e4102 eb65 jmp 0x1407e4169
0x1407e4104 c7851003000001000000 mov dword ptr [rbp + 0x310], 1
0x1407e410e f30f1005da4c2a02 movss xmm0, dword ptr [rip + 0x22a4cda]
0x1407e4116 f30f118520030000 movss dword ptr [rbp + 0x320], xmm0
0x1407e411e f20f1005ba4c2a02 movsd xmm0, qword ptr [rip + 0x22a4cba]
0x1407e4126 eb41 jmp 0x1407e4169
0x1407e4128 c7851003000001000000 mov dword ptr [rbp + 0x310], 1
0x1407e4132 0f57c0 xorps xmm0, xmm0
0x1407e4135 f30f118520030000 movss dword ptr [rbp + 0x320], xmm0
0x1407e413d f20f10056b4c2a02 movsd xmm0, qword ptr [rip + 0x22a4c6b]
0x1407e4145 eb22 jmp 0x1407e4169
0x1407e4147 c7851003000001000000 mov dword ptr [rbp + 0x310], 1
0x1407e4151 f30f10054bbb2902 movss xmm0, dword ptr [rip + 0x229bb4b]
0x1407e4159 f30f118520030000 movss dword ptr [rbp + 0x320], xmm0
0x1407e4161 f20f1005674c2a02 movsd xmm0, qword ptr [rip + 0x22a4c67]
0x1407e4169 0f2985a0020000 movaps xmmword ptr [rbp + 0x2a0], xmm0
0x1407e4170 660f6f85e0000000 movdqa xmm0, xmmword ptr [rbp + 0xe0]
0x1407e4178 660f6f8df0000000 movdqa xmm1, xmmword ptr [rbp + 0xf0]
0x1407e4180 660f6f9500010000 movdqa xmm2, xmmword ptr [rbp + 0x100]
0x1407e4188 660f7f5570 movdqa xmmword ptr [rbp + 0x70], xmm2
0x1407e418d 660f7f4d60 movdqa xmmword ptr [rbp + 0x60], xmm1
0x1407e4192 660f7f4550 movdqa xmmword ptr [rbp + 0x50], xmm0
0x1407e4197 488b85b0020000 mov rax, qword ptr [rbp + 0x2b0]
0x1407e419e 488b8db7020000 mov rcx, qword ptr [rbp + 0x2b7]
0x1407e41a5 48898580010000 mov qword ptr [rbp + 0x180], rax
0x1407e41ac 48898d87010000 mov qword ptr [rbp + 0x187], rcx
0x1407e41b3 0fb65a45 movzx ebx, byte ptr [rdx + 0x45]
0x1407e41b7 4889f8 mov rax, rdi
0x1407e41ba 488bbfc0000000 mov rdi, qword ptr [rdi + 0xc0]
0x1407e41c1 483bb8b0000000 cmp rdi, qword ptr [rax + 0xb0]
0x1407e41c8 751a jne 0x1407e41e4
0x1407e41ca 488b8d90020000 mov rcx, qword ptr [rbp + 0x290]
0x1407e41d1 4881c1b0000000 add rcx, 0xb0
0x1407e41d8 488d15614e2a02 lea rdx, [rip + 0x22a4e61]
0x1407e41df e8cc710400 call 0x14082b3b0
0x1407e41e4 4c8b8d90020000 mov r9, qword ptr [rbp + 0x290]
0x1407e41eb 498b81b8000000 mov rax, qword ptr [r9 + 0xb8]
0x1407e41f2 4869cf60010000 imul rcx, rdi, 0x160
0x1407e41f9 48c7040800000000 mov qword ptr [rax + rcx], 0
0x1407e4201 48c744081000000000 mov qword ptr [rax + rcx + 0x10], 0
0x1407e420a 48c744082000000000 mov qword ptr [rax + rcx + 0x20], 0
0x1407e4213 c744083000000000 mov dword ptr [rax + rcx + 0x30], 0
0x1407e421b 488b95b0010000 mov rdx, qword ptr [rbp + 0x1b0]
0x1407e4222 4889540834 mov qword ptr [rax + rcx + 0x34], rdx
0x1407e4227 8b95b8010000 mov edx, dword ptr [rbp + 0x1b8]
0x1407e422d 8954083c mov dword ptr [rax + rcx + 0x3c], edx
0x1407e4231 c744084000000000 mov dword ptr [rax + rcx + 0x40], 0
0x1407e4239 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
0x1407e4240 0f108dd0010000 movups xmm1, xmmword ptr [rbp + 0x1d0]
0x1407e4247 0f1095e0010000 movups xmm2, xmmword ptr [rbp + 0x1e0]
0x1407e424e 0f109dec010000 movups xmm3, xmmword ptr [rbp + 0x1ec]
0x1407e4255 0f11440844 movups xmmword ptr [rax + rcx + 0x44], xmm0
0x1407e425a 0f114c0854 movups xmmword ptr [rax + rcx + 0x54], xmm1
0x1407e425f 0f11540864 movups xmmword ptr [rax + rcx + 0x64], xmm2
0x1407e4264 0f115c0870 movups xmmword ptr [rax + rcx + 0x70], xmm3
0x1407e4269 c784088000000000000000 mov dword ptr [rax + rcx + 0x80], 0
0x1407e4274 488b95d4000000 mov rdx, qword ptr [rbp + 0xd4]
0x1407e427b 4889940884000000 mov qword ptr [rax + rcx + 0x84], rdx
0x1407e4283 8b95dc000000 mov edx, dword ptr [rbp + 0xdc]
0x1407e4289 8994088c000000 mov dword ptr [rax + rcx + 0x8c], edx
0x1407e4290 8b9510030000 mov edx, dword ptr [rbp + 0x310]
0x1407e4296 89940890000000 mov dword ptr [rax + rcx + 0x90], edx
0x1407e429d 0f2885a0020000 movaps xmm0, xmmword ptr [rbp + 0x2a0]
0x1407e42a4 0f13840894000000 movlps qword ptr [rax + rcx + 0x94], xmm0
0x1407e42ac f30f108520030000 movss xmm0, dword ptr [rbp + 0x320]
0x1407e42b4 f30f1184089c000000 movss dword ptr [rax + rcx + 0x9c], xmm0
0x1407e42bd 488b95e0020000 mov rdx, qword ptr [rbp + 0x2e0]
0x1407e42c4 48899408a0000000 mov qword ptr [rax + rcx + 0xa0], rdx
0x1407e42cc 488b9598020000 mov rdx, qword ptr [rbp + 0x298]
0x1407e42d3 48899408a8000000 mov qword ptr [rax + rcx + 0xa8], rdx
0x1407e42db 488b9570010000 mov rdx, qword ptr [rbp + 0x170]
0x1407e42e2 48899408b0000000 mov qword ptr [rax + rcx + 0xb0], rdx
0x1407e42ea 488b9578010000 mov rdx, qword ptr [rbp + 0x178]
0x1407e42f1 48899408b8000000 mov qword ptr [rax + rcx + 0xb8], rdx
0x1407e42f9 0f284550 movaps xmm0, xmmword ptr [rbp + 0x50]
0x1407e42fd 0f284d60 movaps xmm1, xmmword ptr [rbp + 0x60]
0x1407e4301 660f6f5570 movdqa xmm2, xmmword ptr [rbp + 0x70]
0x1407e4306 660f7f9408e0000000 movdqa xmmword ptr [rax + rcx + 0xe0], xmm2
0x1407e430f 0f298c08d0000000 movaps xmmword ptr [rax + rcx + 0xd0], xmm1
0x1407e4317 0f298408c0000000 movaps xmmword ptr [rax + rcx + 0xc0], xmm0
0x1407e431f c78408f000000000000000 mov dword ptr [rax + rcx + 0xf0], 0
0x1407e432a c78408f800000000000000 mov dword ptr [rax + rcx + 0xf8], 0
0x1407e4335 0f108570020000 movups xmm0, xmmword ptr [rbp + 0x270]
0x1407e433c 0f118408fc000000 movups xmmword ptr [rax + rcx + 0xfc], xmm0
0x1407e4344 488b9580020000 mov rdx, qword ptr [rbp + 0x280]
0x1407e434b 488994080c010000 mov qword ptr [rax + rcx + 0x10c], rdx
0x1407e4353 c684081401000002 mov byte ptr [rax + rcx + 0x114], 2
0x1407e435b 0fb695ef020000 movzx edx, byte ptr [rbp + 0x2ef]
0x1407e4362 88940817010000 mov byte ptr [rax + rcx + 0x117], dl
0x1407e4369 0fb795ed020000 movzx edx, word ptr [rbp + 0x2ed]
0x1407e4370 6689940815010000 mov word ptr [rax + rcx + 0x115], dx
0x1407e4378 c784081801000000000000 mov dword ptr [rax + rcx + 0x118], 0
0x1407e4383 f30f6f85b0020000 movdqu xmm0, xmmword ptr [rbp + 0x2b0]
0x1407e438b f30f6f8dc0020000 movdqu xmm1, xmmword ptr [rbp + 0x2c0]
0x1407e4393 f30f7f84081c010000 movdqu xmmword ptr [rax + rcx + 0x11c], xmm0
0x1407e439c f30f7f8c082c010000 movdqu xmmword ptr [rax + rcx + 0x12c], xmm1
0x1407e43a5 c684083c01000002 mov byte ptr [rax + rcx + 0x13c], 2
0x1407e43ad 488b9580010000 mov rdx, qword ptr [rbp + 0x180]
0x1407e43b4 4c8b8587010000 mov r8, qword ptr [rbp + 0x187]
0x1407e43bb 4c89840844010000 mov qword ptr [rax + rcx + 0x144], r8
0x1407e43c3 488994083d010000 mov qword ptr [rax + rcx + 0x13d], rdx
0x1407e43cb c684084c01000002 mov byte ptr [rax + rcx + 0x14c], 2
0x1407e43d3 66c784084e0100000000 mov word ptr [rax + rcx + 0x14e], 0
0x1407e43dd 889c0850010000 mov byte ptr [rax + rcx + 0x150], bl
0x1407e43e4 488b9590010000 mov rdx, qword ptr [rbp + 0x190]
0x1407e43eb 4c8b8597010000 mov r8, qword ptr [rbp + 0x197]
0x1407e43f2 4c89840858010000 mov qword ptr [rax + rcx + 0x158], r8
0x1407e43fa 4889940851010000 mov qword ptr [rax + rcx + 0x151], rdx
0x1407e4402 48ffc7 inc rdi
0x1407e4405 4989b9c0000000 mov qword ptr [r9 + 0xc0], rdi
0x1407e440c e919f5ffff jmp 0x1407e392a
0x1407e4411 c7851003000000000000 mov dword ptr [rbp + 0x310], 0
0x1407e441b e950fdffff jmp 0x1407e4170
0x1407e4420 8b8518030000 mov eax, dword ptr [rbp + 0x318]
0x1407e4426 4189c2 mov r10d, eax
0x1407e4429 4c8b85d8020000 mov r8, qword ptr [rbp + 0x2d8]
0x1407e4430 4c8b8d00030000 mov r9, qword ptr [rbp + 0x300]
0x1407e4437 488d8dc0010000 lea rcx, [rbp + 0x1c0]
0x1407e443e 4c8985d8020000 mov qword ptr [rbp + 0x2d8], r8
0x1407e4445 4983f802 cmp r8, 2
0x1407e4449 0f850af3ffff jne 0x1407e3759
0x1407e444f e92cf4ffff jmp 0x1407e3880
0x1407e4454 488d0d8dce4302 lea rcx, [rip + 0x243ce8d]
0x1407e445b e8b04e0f02 call 0x1428d9310
0x1407e4460 e96b010000 jmp 0x1407e45d0
0x1407e4465 4885c0 test rax, rax
0x1407e4468 0f85e7000000 jne 0x1407e4555
0x1407e446e 31d2 xor edx, edx
0x1407e4470 e80be11002 call 0x1428f2580
0x1407e4475 4889c1 mov rcx, rax
0x1407e4478 4885c0 test rax, rax
0x1407e447b 0f854bf1ffff jne 0x1407e35cc
0x1407e4481 e9cf000000 jmp 0x1407e4555
0x1407e4486 4883bd5002000000 cmp qword ptr [rbp + 0x250], 0
0x1407e448e 744b je 0x1407e44db
0x1407e4490 e89b3c4300 call 0x140c18130
0x1407e4495 4889c1 mov rcx, rax
0x1407e4498 488b00 mov rax, qword ptr [rax]
0x1407e449b 4883f801 cmp rax, 1
0x1407e449f 0f8598000000 jne 0x1407e453d
0x1407e44a5 4883c108 add rcx, 8
0x1407e44a9 48833900 cmp qword ptr [rcx], 0
0x1407e44ad 0f85ae000000 jne 0x1407e4561
0x1407e44b3 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1407e44ba 48898d08030000 mov qword ptr [rbp + 0x308], rcx
0x1407e44c1 4883c108 add rcx, 8
0x1407e44c5 488b9558020000 mov rdx, qword ptr [rbp + 0x258]
0x1407e44cc e81f2c4300 call 0x140c170f0
0x1407e44d1 488b8508030000 mov rax, qword ptr [rbp + 0x308]
0x1407e44d8 48ff00 inc qword ptr [rax]
0x1407e44db 0f28b530030000 movaps xmm6, xmmword ptr [rbp + 0x330]
0x1407e44e2 0f28bd40030000 movaps xmm7, xmmword ptr [rbp + 0x340]
0x1407e44e9 440f288550030000 movaps xmm8, xmmword ptr [rbp + 0x350]
0x1407e44f1 440f288d60030000 movaps xmm9, xmmword ptr [rbp + 0x360]
0x1407e44f9 440f289570030000 movaps xmm10, xmmword ptr [rbp + 0x370]
0x1407e4501 440f289d80030000 movaps xmm11, xmmword ptr [rbp + 0x380]
0x1407e4509 440f28a590030000 movaps xmm12, xmmword ptr [rbp + 0x390]
0x1407e4511 440f28ada0030000 movaps xmm13, xmmword ptr [rbp + 0x3a0]
0x1407e4519 440f28b5b0030000 movaps xmm14, xmmword ptr [rbp + 0x3b0]
0x1407e4521 440f28bdc0030000 movaps xmm15, xmmword ptr [rbp + 0x3c0]
0x1407e4529 4881c458040000 add rsp, 0x458
0x1407e4530 5b pop rbx
0x1407e4531 5f pop rdi
0x1407e4532 5e pop rsi
0x1407e4533 415c pop r12
0x1407e4535 415d pop r13
0x1407e4537 415e pop r14
0x1407e4539 415f pop r15
0x1407e453b 5d pop rbp
0x1407e453c c3 ret 
0x1407e453d 4885c0 test rax, rax
0x1407e4540 7513 jne 0x1407e4555
0x1407e4542 31d2 xor edx, edx
0x1407e4544 e837e01002 call 0x1428f2580
0x1407e4549 4889c1 mov rcx, rax
0x1407e454c 4885c0 test rax, rax
0x1407e454f 0f8554ffffff jne 0x1407e44a9
0x1407e4555 488d0d6cce4302 lea rcx, [rip + 0x243ce6c]
0x1407e455c e86f3b0f02 call 0x1428d80d0
0x1407e4561 488d0dd0ce4302 lea rcx, [rip + 0x243ced0]
0x1407e4568 e8e34c0f02 call 0x1428d9250
0x1407e456d 488d05b4d12a02 lea rax, [rip + 0x22ad1b4]
0x1407e4574 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
0x1407e457b 48c785c801000001000000 mov qword ptr [rbp + 0x1c8], 1
0x1407e4586 48c785d001000008000000 mov qword ptr [rbp + 0x1d0], 8
0x1407e4591 0f57c0 xorps xmm0, xmm0
0x1407e4594 0f1185d8010000 movups xmmword ptr [rbp + 0x1d8], xmm0
0x1407e459b 488d1596d12a02 lea rdx, [rip + 0x22ad196]
0x1407e45a2 488d8dc0010000 lea rcx, [rbp + 0x1c0]
0x1407e45a9 e8e24d0f02 call 0x1428d9390
0x1407e45ae eb20 jmp 0x1407e45d0
0x1407e45b0 b910000000 mov ecx, 0x10
0x1407e45b5 ba90000000 mov edx, 0x90
0x1407e45ba e8414a0f02 call 0x1428d9000
0x1407e45bf eb0f jmp 0x1407e45d0
0x1407e45c1 b910000000 mov ecx, 0x10
0x1407e45c6 ba90000000 mov edx, 0x90
0x1407e45cb e8304a0f02 call 0x1428d9000
0x1407e45d0 0f0b ud2 
0x1407e45d2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1407e45e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1407e45e5 55 push rbp
0x1407e45e6 4157 push r15
0x1407e45e8 4156 push r14
0x1407e45ea 4155 push r13
0x1407e45ec 4154 push r12
0x1407e45ee 56 push rsi
0x1407e45ef 57 push rdi
0x1407e45f0 53 push rbx
0x1407e45f1 4881ecd8000000 sub rsp, 0xd8
0x1407e45f8 488daa80000000 lea rbp, [rdx + 0x80]
0x1407e45ff 66440f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm15
0x1407e4606 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x1407e460c 66440f7f6c2450 movdqa xmmword ptr [rsp + 0x50], xmm13
0x1407e4613 66440f7f642460 movdqa xmmword ptr [rsp + 0x60], xmm12
0x1407e461a 66440f7f5c2470 movdqa xmmword ptr [rsp + 0x70], xmm11
0x1407e4621 66440f7f942480000000 movdqa xmmword ptr [rsp + 0x80], xmm10
0x1407e462b 66440f7f8c2490000000 movdqa xmmword ptr [rsp + 0x90], xmm9
0x1407e4635 66440f7f8424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm8
0x1407e463f 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x1407e4648 660f7fb424c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm6
0x1407e4651 488b8508030000 mov rax, qword ptr [rbp + 0x308]
0x1407e4658 48ff00 inc qword ptr [rax]
0x1407e465b 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x1407e4663 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x1407e466b 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x1407e4674 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x1407e467d 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x1407e4686 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x1407e468c 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x1407e4692 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x1407e4698 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x1407e469e 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x1407e46a4 4881c4d8000000 add rsp, 0xd8
0x1407e46ab 5b pop rbx
0x1407e46ac 5f pop rdi
0x1407e46ad 5e pop rsi
0x1407e46ae 415c pop r12
0x1407e46b0 415d pop r13
0x1407e46b2 415e pop r14
0x1407e46b4 415f pop r15
0x1407e46b6 5d pop rbp
0x1407e46b7 c3 ret 
0x1407e46b8 0f1f840000000000 nop dword ptr [rax + rax]
0x1407e46c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1407e46c5 55 push rbp
0x1407e46c6 4157 push r15
0x1407e46c8 4156 push r14
0x1407e46ca 4155 push r13
0x1407e46cc 4154 push r12
0x1407e46ce 56 push rsi
0x1407e46cf 57 push rdi
0x1407e46d0 53 push rbx
0x1407e46d1 4881ecd8000000 sub rsp, 0xd8
0x1407e46d8 488daa80000000 lea rbp, [rdx + 0x80]
0x1407e46df 66440f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm15
0x1407e46e6 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x1407e46ec 66440f7f6c2450 movdqa xmmword ptr [rsp + 0x50], xmm13
0x1407e46f3 66440f7f642460 movdqa xmmword ptr [rsp + 0x60], xmm12
0x1407e46fa 66440f7f5c2470 movdqa xmmword ptr [rsp + 0x70], xmm11
0x1407e4701 66440f7f942480000000 movdqa xmmword ptr [rsp + 0x80], xmm10
0x1407e470b 66440f7f8c2490000000 movdqa xmmword ptr [rsp + 0x90], xmm9
0x1407e4715 66440f7f8424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm8
0x1407e471f 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x1407e4728 660f7fb424c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm6
0x1407e4731 c6852703000001 mov byte ptr [rbp + 0x327], 1
0x1407e4738 488d8dd0010000 lea rcx, [rbp + 0x1d0]
0x1407e473f e89cd4e1ff call 0x140601be0
0x1407e4744 c6852703000001 mov byte ptr [rbp + 0x327], 1
0x1407e474b 660f6fb424c0000000 movdqa xmm6, xmmword ptr [rsp + 0xc0]
0x1407e4754 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x1407e475d 66440f6f8424a0000000 movdqa xmm8, xmmword ptr [rsp + 0xa0]
0x1407e4767 66440f6f8c2490000000 movdqa xmm9, xmmword ptr [rsp + 0x90]
0x1407e4771 66440f6f942480000000 movdqa xmm10, xmmword ptr [rsp + 0x80]
0x1407e477b 66440f6f5c2470 movdqa xmm11, xmmword ptr [rsp + 0x70]
0x1407e4782 66440f6f642460 movdqa xmm12, xmmword ptr [rsp + 0x60]
0x1407e4789 66440f6f6c2450 movdqa xmm13, xmmword ptr [rsp + 0x50]
0x1407e4790 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x1407e4796 66440f6f7c2430 movdqa xmm15, xmmword ptr [rsp + 0x30]
0x1407e479d 4881c4d8000000 add rsp, 0xd8
0x1407e47a4 5b pop rbx
0x1407e47a5 5f pop rdi
0x1407e47a6 5e pop rsi
0x1407e47a7 415c pop r12
0x1407e47a9 415d pop r13
0x1407e47ab 415e pop r14
0x1407e47ad 415f pop r15
0x1407e47af 5d pop rbp
0x1407e47b0 c3 ret 
0x1407e47b1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1407e47c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1407e47c5 55 push rbp
0x1407e47c6 4157 push r15
0x1407e47c8 4156 push r14
0x1407e47ca 4155 push r13
0x1407e47cc 4154 push r12
0x1407e47ce 56 push rsi
0x1407e47cf 57 push rdi
0x1407e47d0 53 push rbx
0x1407e47d1 4881ecd8000000 sub rsp, 0xd8
0x1407e47d8 488daa80000000 lea rbp, [rdx + 0x80]
0x1407e47df 66440f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm15
0x1407e47e6 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x1407e47ec 66440f7f6c2450 movdqa xmmword ptr [rsp + 0x50], xmm13
0x1407e47f3 66440f7f642460 movdqa xmmword ptr [rsp + 0x60], xmm12
0x1407e47fa 66440f7f5c2470 movdqa xmmword ptr [rsp + 0x70], xmm11
0x1407e4801 66440f7f942480000000 movdqa xmmword ptr [rsp + 0x80], xmm10
0x1407e480b 66440f7f8c2490000000 movdqa xmmword ptr [rsp + 0x90], xmm9
0x1407e4815 66440f7f8424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm8
0x1407e481f 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x1407e4828 660f7fb424c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm6
0x1407e4831 488d8dd0010000 lea rcx, [rbp + 0x1d0]
0x1407e4838 e8a3d3e1ff call 0x140601be0
0x1407e483d 660f6fb424c0000000 movdqa xmm6, xmmword ptr [rsp + 0xc0]
0x1407e4846 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x1407e484f 66440f6f8424a0000000 movdqa xmm8, xmmword ptr [rsp + 0xa0]
0x1407e4859 66440f6f8c2490000000 movdqa xmm9, xmmword ptr [rsp + 0x90]
0x1407e4863 66440f6f942480000000 movdqa xmm10, xmmword ptr [rsp + 0x80]
0x1407e486d 66440f6f5c2470 movdqa xmm11, xmmword ptr [rsp + 0x70]
0x1407e4874 66440f6f642460 movdqa xmm12, xmmword ptr [rsp + 0x60]
0x1407e487b 66440f6f6c2450 movdqa xmm13, xmmword ptr [rsp + 0x50]
0x1407e4882 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x1407e4888 66440f6f7c2430 movdqa xmm15, xmmword ptr [rsp + 0x30]
0x1407e488f 4881c4d8000000 add rsp, 0xd8
0x1407e4896 5b pop rbx
0x1407e4897 5f pop rdi
0x1407e4898 5e pop rsi
0x1407e4899 415c pop r12
0x1407e489b 415d pop r13
0x1407e489d 415e pop r14
0x1407e489f 415f pop r15
0x1407e48a1 5d pop rbp
0x1407e48a2 c3 ret 
0x1407e48a3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1407e48b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1407e48b5 55 push rbp
0x1407e48b6 4157 push r15
0x1407e48b8 4156 push r14
0x1407e48ba 4155 push r13
0x1407e48bc 4154 push r12
0x1407e48be 56 push rsi
0x1407e48bf 57 push rdi
0x1407e48c0 53 push rbx
0x1407e48c1 4881ecd8000000 sub rsp, 0xd8
0x1407e48c8 488daa80000000 lea rbp, [rdx + 0x80]
0x1407e48cf 66440f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm15
0x1407e48d6 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x1407e48dc 66440f7f6c2450 movdqa xmmword ptr [rsp + 0x50], xmm13
0x1407e48e3 66440f7f642460 movdqa xmmword ptr [rsp + 0x60], xmm12
0x1407e48ea 66440f7f5c2470 movdqa xmmword ptr [rsp + 0x70], xmm11
0x1407e48f1 66440f7f942480000000 movdqa xmmword ptr [rsp + 0x80], xmm10
0x1407e48fb 66440f7f8c2490000000 movdqa xmmword ptr [rsp + 0x90], xmm9
0x1407e4905 66440f7f8424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm8
0x1407e490f 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x1407e4918 660f7fb424c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm6
0x1407e4921 488b85e0020000 mov rax, qword ptr [rbp + 0x2e0]
0x1407e4928 4883bd9802000000 cmp qword ptr [rbp + 0x298], 0
0x1407e4930 757f jne 0x1407e49b1
0x1407e4932 f048ff08 lock dec qword ptr [rax]
0x1407e4936 750c jne 0x1407e4944
0x1407e4938 488d8d70020000 lea rcx, [rbp + 0x270]
0x1407e493f e8dc7c3700 call 0x140b5c620
0x1407e4944 c6852703000000 mov byte ptr [rbp + 0x327], 0
0x1407e494b 660f6fb424c0000000 movdqa xmm6, xmmword ptr [rsp + 0xc0]
0x1407e4954 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x1407e495d 66440f6f8424a0000000 movdqa xmm8, xmmword ptr [rsp + 0xa0]
0x1407e4967 66440f6f8c2490000000 movdqa xmm9, xmmword ptr [rsp + 0x90]
0x1407e4971 66440f6f942480000000 movdqa xmm10, xmmword ptr [rsp + 0x80]
0x1407e497b 66440f6f5c2470 movdqa xmm11, xmmword ptr [rsp + 0x70]
0x1407e4982 66440f6f642460 movdqa xmm12, xmmword ptr [rsp + 0x60]
0x1407e4989 66440f6f6c2450 movdqa xmm13, xmmword ptr [rsp + 0x50]
0x1407e4990 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x1407e4996 66440f6f7c2430 movdqa xmm15, xmmword ptr [rsp + 0x30]
0x1407e499d 4881c4d8000000 add rsp, 0xd8
0x1407e49a4 5b pop rbx
0x1407e49a5 5f pop rdi
0x1407e49a6 5e pop rsi
0x1407e49a7 415c pop r12
0x1407e49a9 415d pop r13
0x1407e49ab 415e pop r14
0x1407e49ad 415f pop r15
0x1407e49af 5d pop rbp
0x1407e49b0 c3 ret 
0x1407e49b1 c6852703000000 mov byte ptr [rbp + 0x327], 0
0x1407e49b8 660f6fb424c0000000 movdqa xmm6, xmmword ptr [rsp + 0xc0]
0x1407e49c1 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x1407e49ca 66440f6f8424a0000000 movdqa xmm8, xmmword ptr [rsp + 0xa0]
0x1407e49d4 66440f6f8c2490000000 movdqa xmm9, xmmword ptr [rsp + 0x90]
0x1407e49de 66440f6f942480000000 movdqa xmm10, xmmword ptr [rsp + 0x80]
0x1407e49e8 66440f6f5c2470 movdqa xmm11, xmmword ptr [rsp + 0x70]
0x1407e49ef 66440f6f642460 movdqa xmm12, xmmword ptr [rsp + 0x60]
0x1407e49f6 66440f6f6c2450 movdqa xmm13, xmmword ptr [rsp + 0x50]
0x1407e49fd 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x1407e4a03 66440f6f7c2430 movdqa xmm15, xmmword ptr [rsp + 0x30]
0x1407e4a0a 4881c4d8000000 add rsp, 0xd8
0x1407e4a11 5b pop rbx
0x1407e4a12 5f pop rdi
0x1407e4a13 5e pop rsi
0x1407e4a14 415c pop r12
0x1407e4a16 415d pop r13
0x1407e4a18 415e pop r14
0x1407e4a1a 415f pop r15
0x1407e4a1c 5d pop rbp
0x1407e4a1d c3 ret 
0x1407e4a1e 6690 nop 
0x1407e4a20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1407e4a25 55 push rbp
0x1407e4a26 4157 push r15
0x1407e4a28 4156 push r14
0x1407e4a2a 4155 push r13
0x1407e4a2c 4154 push r12
0x1407e4a2e 56 push rsi
0x1407e4a2f 57 push rdi
0x1407e4a30 53 push rbx
0x1407e4a31 4881ecd8000000 sub rsp, 0xd8
0x1407e4a38 488daa80000000 lea rbp, [rdx + 0x80]
0x1407e4a3f 66440f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm15
0x1407e4a46 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x1407e4a4c 66440f7f6c2450 movdqa xmmword ptr [rsp + 0x50], xmm13
0x1407e4a53 66440f7f642460 movdqa xmmword ptr [rsp + 0x60], xmm12
0x1407e4a5a 66440f7f5c2470 movdqa xmmword ptr [rsp + 0x70], xmm11
0x1407e4a61 66440f7f942480000000 movdqa xmmword ptr [rsp + 0x80], xmm10
0x1407e4a6b 66440f7f8c2490000000 movdqa xmmword ptr [rsp + 0x90], xmm9
0x1407e4a75 66440f7f8424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm8
0x1407e4a7f 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x1407e4a88 660f7fb424c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm6
0x1407e4a91 80bd2703000000 cmp byte ptr [rbp + 0x327], 0
0x1407e4a98 746f je 0x1407e4b09
0x1407e4a9a 488d4d50 lea rcx, [rbp + 0x50]
0x1407e4a9e e83dd1e1ff call 0x140601be0
0x1407e4aa3 660f6fb424c0000000 movdqa xmm6, xmmword ptr [rsp + 0xc0]
0x1407e4aac 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x1407e4ab5 66440f6f8424a0000000 movdqa xmm8, xmmword ptr [rsp + 0xa0]
0x1407e4abf 66440f6f8c2490000000 movdqa xmm9, xmmword ptr [rsp + 0x90]
0x1407e4ac9 66440f6f942480000000 movdqa xmm10, xmmword ptr [rsp + 0x80]
0x1407e4ad3 66440f6f5c2470 movdqa xmm11, xmmword ptr [rsp + 0x70]
0x1407e4ada 66440f6f642460 movdqa xmm12, xmmword ptr [rsp + 0x60]
0x1407e4ae1 66440f6f6c2450 movdqa xmm13, xmmword ptr [rsp + 0x50]
0x1407e4ae8 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x1407e4aee 66440f6f7c2430 movdqa xmm15, xmmword ptr [rsp + 0x30]
0x1407e4af5 4881c4d8000000 add rsp, 0xd8
0x1407e4afc 5b pop rbx
0x1407e4afd 5f pop rdi
0x1407e4afe 5e pop rsi
0x1407e4aff 415c pop r12
0x1407e4b01 415d pop r13
0x1407e4b03 415e pop r14
0x1407e4b05 415f pop r15
0x1407e4b07 5d pop rbp
0x1407e4b08 c3 ret 
0x1407e4b09 660f6fb424c0000000 movdqa xmm6, xmmword ptr [rsp + 0xc0]
0x1407e4b12 660f6fbc24b0000000 movdqa xmm7, xmmword ptr [rsp + 0xb0]
0x1407e4b1b 66440f6f8424a0000000 movdqa xmm8, xmmword ptr [rsp + 0xa0]
0x1407e4b25 66440f6f8c2490000000 movdqa xmm9, xmmword ptr [rsp + 0x90]
0x1407e4b2f 66440f6f942480000000 movdqa xmm10, xmmword ptr [rsp + 0x80]
0x1407e4b39 66440f6f5c2470 movdqa xmm11, xmmword ptr [rsp + 0x70]
0x1407e4b40 66440f6f642460 movdqa xmm12, xmmword ptr [rsp + 0x60]
0x1407e4b47 66440f6f6c2450 movdqa xmm13, xmmword ptr [rsp + 0x50]
0x1407e4b4e 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x1407e4b54 66440f6f7c2430 movdqa xmm15, xmmword ptr [rsp + 0x30]
0x1407e4b5b 4881c4d8000000 add rsp, 0xd8
0x1407e4b62 5b pop rbx
0x1407e4b63 5f pop rdi
0x1407e4b64 5e pop rsi
0x1407e4b65 415c pop r12
0x1407e4b67 415d pop r13
0x1407e4b69 415e pop r14
0x1407e4b6b 415f pop r15
0x1407e4b6d 5d pop rbp
0x1407e4b6e c3 ret 
0x1407e4b6f 90 nop 
0x1407e4b70 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1407e4b75 55 push rbp
0x1407e4b76 4157 push r15
0x1407e4b78 4156 push r14
0x1407e4b7a 4155 push r13
0x1407e4b7c 4154 push r12
0x1407e4b7e 56 push rsi
0x1407e4b7f 57 push rdi
0x1407e4b80 53 push rbx
0x1407e4b81 4881ecd8000000 sub rsp, 0xd8
0x1407e4b88 488daa80000000 lea rbp, [rdx + 0x80]
0x1407e4b8f 66440f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm15
0x1407e4b96 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x1407e4b9c 66440f7f6c2450 movdqa xmmword ptr [rsp + 0x50], xmm13
0x1407e4ba3 66440f7f642460 movdqa xmmword ptr [rsp + 0x60], xmm12
0x1407e4baa 66440f7f5c2470 movdqa xmmword ptr [rsp + 0x70], xmm11
0x1407e4bb1 66440f7f942480000000 movdqa xmmword ptr [rsp + 0x80], xmm10
0x1407e4bbb 66440f7f8c2490000000 movdqa xmmword ptr [rsp + 0x90], xmm9
0x1407e4bc5 66440f7f8424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm8
0x1407e4bcf 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x1407e4bd8 660f7fb424c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm6
0x1407e4be1 488b85f0020000 mov rax, qword ptr [rbp + 0x2f0]
0x1407e4be8 48ff00 inc qword ptr [rax]
0x1407e4beb 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x1407e4bf3 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x1407e4bfb 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x1407e4c04 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x1407e4c0d 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x1407e4c16 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x1407e4c1c 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x1407e4c22 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x1407e4c28 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x1407e4c2e 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x1407e4c34 4881c4d8000000 add rsp, 0xd8
0x1407e4c3b 5b pop rbx
0x1407e4c3c 5f pop rdi
0x1407e4c3d 5e pop rsi
0x1407e4c3e 415c pop r12
0x1407e4c40 415d pop r13
0x1407e4c42 415e pop r14
0x1407e4c44 415f pop r15
0x1407e4c46 5d pop rbp
0x1407e4c47 c3 ret 
0x1407e4c48 0f1f840000000000 nop dword ptr [rax + rax]
0x1407e4c50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1407e4c55 55 push rbp
0x1407e4c56 4157 push r15
0x1407e4c58 4156 push r14
0x1407e4c5a 4155 push r13
0x1407e4c5c 4154 push r12
0x1407e4c5e 56 push rsi
0x1407e4c5f 57 push rdi
0x1407e4c60 53 push rbx
0x1407e4c61 4881ecd8000000 sub rsp, 0xd8
0x1407e4c68 488daa80000000 lea rbp, [rdx + 0x80]
0x1407e4c6f 66440f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm15
0x1407e4c76 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x1407e4c7c 66440f7f6c2450 movdqa xmmword ptr [rsp + 0x50], xmm13
0x1407e4c83 66440f7f642460 movdqa xmmword ptr [rsp + 0x60], xmm12
0x1407e4c8a 66440f7f5c2470 movdqa xmmword ptr [rsp + 0x70], xmm11
0x1407e4c91 66440f7f942480000000 movdqa xmmword ptr [rsp + 0x80], xmm10
0x1407e4c9b 66440f7f8c2490000000 movdqa xmmword ptr [rsp + 0x90], xmm9
0x1407e4ca5 66440f7f8424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm8
0x1407e4caf 660f7fbc24b0000000 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x1407e4cb8 660f7fb424c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm6
0x1407e4cc1 4883bd5002000000 cmp qword ptr [rbp + 0x250], 0
0x1407e4cc9 742f je 0x1407e4cfa
0x1407e4ccb 488d8558020000 lea rax, [rbp + 0x258]
0x1407e4cd2 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
0x1407e4cd9 488d85b0010000 lea rax, [rbp + 0x1b0]
0x1407e4ce0 48898590010000 mov qword ptr [rbp + 0x190], rax
0x1407e4ce7 488d0d2a422a02 lea rcx, [rip + 0x22a422a]
0x1407e4cee 488d9590010000 lea rdx, [rbp + 0x190]
0x1407e4cf5 e8c62ea700 call 0x141257bc0
0x1407e4cfa 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x1407e4d02 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x1407e4d0a 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x1407e4d13 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x1407e4d1c 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x1407e4d25 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x1407e4d2b 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x1407e4d31 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x1407e4d37 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x1407e4d3d 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x1407e4d43 4881c4d8000000 add rsp, 0xd8
0x1407e4d4a 5b pop rbx
0x1407e4d4b 5f pop rdi
0x1407e4d4c 5e pop rsi
0x1407e4d4d 415c pop r12
0x1407e4d4f 415d pop r13
0x1407e4d51 415e pop r14
0x1407e4d53 415f pop r15
0x1407e4d55 5d pop rbp
0x1407e4d56 c3 ret 
