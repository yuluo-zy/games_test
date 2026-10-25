0x1408a82f0 55 push rbp
0x1408a82f1 4157 push r15
0x1408a82f3 4156 push r14
0x1408a82f5 4155 push r13
0x1408a82f7 4154 push r12
0x1408a82f9 56 push rsi
0x1408a82fa 57 push rdi
0x1408a82fb 53 push rbx
0x1408a82fc 4881ec18040000 sub rsp, 0x418
0x1408a8303 488dac2480000000 lea rbp, [rsp + 0x80]
0x1408a830b 48c78590030000feffffff mov qword ptr [rbp + 0x390], 0xfffffffffffffffe
0x1408a8316 4589ce mov r14d, r9d
0x1408a8319 4c89c3 mov rbx, r8
0x1408a831c 4889d6 mov rsi, rdx
0x1408a831f 4989cf mov r15, rcx
0x1408a8322 488d7a38 lea rdi, [rdx + 0x38]
0x1408a8326 48ba13c85cff0a7c2ebe movabs rdx, 0xbe2e7c0aff5cc813
0x1408a8330 49b8761df0224e59468f movabs r8, 0x8f46594e22f01d76
0x1408a833a 4889f9 mov rcx, rdi
0x1408a833d e8ae5383ff call 0x1400dd6f0
0x1408a8342 4885c0 test rax, rax
0x1408a8345 0f842a080000 je 0x1408a8b75
0x1408a834b 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x1408a8352 0f861d080000 jbe 0x1408a8b75
0x1408a8358 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x1408a835f 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1408a8363 4885c9 test rcx, rcx
0x1408a8366 0f8409080000 je 0x1408a8b75
0x1408a836c 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x1408a8373 48f7d1 not rcx
0x1408a8376 488d0c49 lea rcx, [rcx + rcx*2]
0x1408a837a 48c1e105 shl rcx, 5
0x1408a837e 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1408a8384 0f84eb070000 je 0x1408a8b75
0x1408a838a 4801c8 add rax, rcx
0x1408a838d 4c8b6010 mov r12, qword ptr [rax + 0x10]
0x1408a8391 8b8eb0050000 mov ecx, dword ptr [rsi + 0x5b0]
0x1408a8397 89485c mov dword ptr [rax + 0x5c], ecx
0x1408a839a 48ba3febbf685e1584a2 movabs rdx, 0xa284155e68bfeb3f
0x1408a83a4 49b83a4ac61b44b3a284 movabs r8, 0x84a2b3441bc64a3a
0x1408a83ae 4889f9 mov rcx, rdi
0x1408a83b1 e83a5383ff call 0x1400dd6f0
0x1408a83b6 4885c0 test rax, rax
0x1408a83b9 0f84c2070000 je 0x1408a8b81
0x1408a83bf 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x1408a83c6 0f86b5070000 jbe 0x1408a8b81
0x1408a83cc 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x1408a83d3 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1408a83d7 4885c9 test rcx, rcx
0x1408a83da 0f84a1070000 je 0x1408a8b81
0x1408a83e0 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x1408a83e7 48f7d1 not rcx
0x1408a83ea 488d0c49 lea rcx, [rcx + rcx*2]
0x1408a83ee 48c1e105 shl rcx, 5
0x1408a83f2 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1408a83f8 0f8483070000 je 0x1408a8b81
0x1408a83fe 4801c8 add rax, rcx
0x1408a8401 488b5010 mov rdx, qword ptr [rax + 0x10]
0x1408a8405 8b8eb0050000 mov ecx, dword ptr [rsi + 0x5b0]
0x1408a840b 89485c mov dword ptr [rax + 0x5c], ecx
0x1408a840e 8b4248 mov eax, dword ptr [rdx + 0x48]
0x1408a8411 83f802 cmp eax, 2
0x1408a8414 0f8489040000 je 0x1408a88a3
0x1408a841a 4c89a568030000 mov qword ptr [rbp + 0x368], r12
0x1408a8421 48899578030000 mov qword ptr [rbp + 0x378], rdx
0x1408a8428 4488b58e030000 mov byte ptr [rbp + 0x38e], r14b
0x1408a842f 898574030000 mov dword ptr [rbp + 0x374], eax
0x1408a8435 48899d80030000 mov qword ptr [rbp + 0x380], rbx
0x1408a843c 488d4b04 lea rcx, [rbx + 4]
0x1408a8440 e84bd33200 call 0x140bd5790
0x1408a8445 4989c4 mov r12, rax
0x1408a8448 0fb61d4dc8d802 movzx ebx, byte ptr [rip + 0x2d8c84d]
0x1408a844f c60546c8d80201 mov byte ptr [rip + 0x2d8c846], 1
0x1408a8456 4d89fe mov r14, r15
0x1408a8459 4c8dbdd8010000 lea r15, [rbp + 0x1d8]
0x1408a8460 4c89f9 mov rcx, r15
0x1408a8463 4889c2 mov rdx, rax
0x1408a8466 41b825000000 mov r8d, 0x25
0x1408a846c e82fce3200 call 0x140bd52a0
0x1408a8471 4c8d6dd0 lea r13, [rbp - 0x30]
0x1408a8475 4c89e9 mov rcx, r13
0x1408a8478 4c89fa mov rdx, r15
0x1408a847b e810cf3200 call 0x140bd5390
0x1408a8480 4c8dbd60010000 lea r15, [rbp + 0x160]
0x1408a8487 4c89f9 mov rcx, r15
0x1408a848a 4c89ea mov rdx, r13
0x1408a848d e8dece3200 call 0x140bd5370
0x1408a8492 4c897c2420 mov qword ptr [rsp + 0x20], r15
0x1408a8497 4d89f7 mov r15, r14
0x1408a849a 4c8d0567361f02 lea r8, [rip + 0x21f3667]
0x1408a84a1 488d8dd8010000 lea rcx, [rbp + 0x1d8]
0x1408a84a8 41b90e000000 mov r9d, 0xe
0x1408a84ae 4c89f2 mov rdx, r14
0x1408a84b1 e8da430c00 call 0x14096c890
0x1408a84b6 0fb685fc010000 movzx eax, byte ptr [rbp + 0x1fc]
0x1408a84bd 88858f030000 mov byte ptr [rbp + 0x38f], al
0x1408a84c3 3c02 cmp al, 2
0x1408a84c5 0f84ce060000 je 0x1408a8b99
0x1408a84cb 8b85f8010000 mov eax, dword ptr [rbp + 0x1f8]
0x1408a84d1 8985d0010000 mov dword ptr [rbp + 0x1d0], eax
0x1408a84d7 0f1085d8010000 movups xmm0, xmmword ptr [rbp + 0x1d8]
0x1408a84de 0f108de8010000 movups xmm1, xmmword ptr [rbp + 0x1e8]
0x1408a84e5 0f298dc0010000 movaps xmmword ptr [rbp + 0x1c0], xmm1
0x1408a84ec 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
0x1408a84f3 488b05ae3ed802 mov rax, qword ptr [rip + 0x2d83eae]
0x1408a84fa 488b00 mov rax, qword ptr [rax]
0x1408a84fd 4883f804 cmp rax, 4
0x1408a8501 0f86b3000000 jbe 0x1408a85ba
0x1408a8507 488d0d4a361f02 lea rcx, [rip + 0x21f364a]
0x1408a850e e86d58ddff call 0x14067dd80
0x1408a8513 0f1000 movups xmm0, xmmword ptr [rax]
0x1408a8516 8b4010 mov eax, dword ptr [rax + 0x10]
0x1408a8519 48c7850802000005000000 mov qword ptr [rbp + 0x208], 5
0x1408a8524 488d0d45361f02 lea rcx, [rip + 0x21f3645]
0x1408a852b 48898d10020000 mov qword ptr [rbp + 0x210], rcx
0x1408a8532 48c785180200002b000000 mov qword ptr [rbp + 0x218], 0x2b
0x1408a853d 488d1504361f02 lea rdx, [rip + 0x21f3604]
0x1408a8544 48899528020000 mov qword ptr [rbp + 0x228], rdx
0x1408a854b 48c7853002000001000000 mov qword ptr [rbp + 0x230], 1
0x1408a8556 48c7853802000008000000 mov qword ptr [rbp + 0x238], 8
0x1408a8561 48c785d801000000000000 mov qword ptr [rbp + 0x1d8], 0
0x1408a856c 0f57c9 xorps xmm1, xmm1
0x1408a856f 0f118d40020000 movups xmmword ptr [rbp + 0x240], xmm1
0x1408a8576 48898de0010000 mov qword ptr [rbp + 0x1e0], rcx
0x1408a857d 48c785e80100002b000000 mov qword ptr [rbp + 0x1e8], 0x2b
0x1408a8588 48c785f001000000000000 mov qword ptr [rbp + 0x1f0], 0
0x1408a8593 0f1185f8010000 movups xmmword ptr [rbp + 0x1f8], xmm0
0x1408a859a c7852002000001000000 mov dword ptr [rbp + 0x220], 1
0x1408a85a4 898524020000 mov dword ptr [rbp + 0x224], eax
0x1408a85aa 488d4dd0 lea rcx, [rbp - 0x30]
0x1408a85ae 488d95d8010000 lea rdx, [rbp + 0x1d8]
0x1408a85b5 e8761d4a00 call 0x140d4a330
0x1408a85ba 4c897c2430 mov qword ptr [rsp + 0x30], r15
0x1408a85bf 4889742428 mov qword ptr [rsp + 0x28], rsi
0x1408a85c4 48c74424201f000000 mov qword ptr [rsp + 0x20], 0x1f
0x1408a85cd 488d15c7351f02 lea rdx, [rip + 0x21f35c7]
0x1408a85d4 4c8d0dd6351f02 lea r9, [rip + 0x21f35d6]
0x1408a85db 4c8dbdd8010000 lea r15, [rbp + 0x1d8]
0x1408a85e2 41b816000000 mov r8d, 0x16
0x1408a85e8 4c89f9 mov rcx, r15
0x1408a85eb e8b0acd500 call 0x1416032a0
0x1408a85f0 488b8580030000 mov rax, qword ptr [rbp + 0x380]
0x1408a85f7 48898560010000 mov qword ptr [rbp + 0x160], rax
0x1408a85fe c745d002000000 mov dword ptr [rbp - 0x30], 2
0x1408a8605 4c896c2420 mov qword ptr [rsp + 0x20], r13
0x1408a860a 488d8d60010000 lea rcx, [rbp + 0x160]
0x1408a8611 49b85ea9e91c548e97d9 movabs r8, 0xd9978e541ce9a95e
0x1408a861b 4c89fa mov rdx, r15
0x1408a861e 41b119 mov r9b, 0x19
0x1408a8621 e81a5c0400 call 0x1408ee240
0x1408a8626 4c8d7dd0 lea r15, [rbp - 0x30]
0x1408a862a 4c8dadd8010000 lea r13, [rbp + 0x1d8]
0x1408a8631 41b890010000 mov r8d, 0x190
0x1408a8637 4c89f9 mov rcx, r15
0x1408a863a 4c89ea mov rdx, r13
0x1408a863d e8beb50702 call 0x142923c00
0x1408a8642 488d85b0010000 lea rax, [rbp + 0x1b0]
0x1408a8649 48898560010000 mov qword ptr [rbp + 0x160], rax
0x1408a8650 c785d801000002000000 mov dword ptr [rbp + 0x1d8], 2
0x1408a865a 4c896c2420 mov qword ptr [rsp + 0x20], r13
0x1408a865f 488d8d60010000 lea rcx, [rbp + 0x160]
0x1408a8666 49b8ee66b23d64cd29ed movabs r8, 0xed29cd643db266ee
0x1408a8670 4c89fa mov rdx, r15
0x1408a8673 41b132 mov r9b, 0x32
0x1408a8676 e885560400 call 0x1408edd00
0x1408a867b 488d8dd8010000 lea rcx, [rbp + 0x1d8]
0x1408a8682 488d55d0 lea rdx, [rbp - 0x30]
0x1408a8686 41b890010000 mov r8d, 0x190
0x1408a868c e86fb50702 call 0x142923c00
0x1408a8691 4c89e0 mov rax, r12
0x1408a8694 48c1e820 shr rax, 0x20
0x1408a8698 0f108578020000 movups xmm0, xmmword ptr [rbp + 0x278]
0x1408a869f 0f2985a0010000 movaps xmmword ptr [rbp + 0x1a0], xmm0
0x1408a86a6 0f108568020000 movups xmm0, xmmword ptr [rbp + 0x268]
0x1408a86ad 0f298590010000 movaps xmmword ptr [rbp + 0x190], xmm0
0x1408a86b4 0f108558020000 movups xmm0, xmmword ptr [rbp + 0x258]
0x1408a86bb 0f298580010000 movaps xmmword ptr [rbp + 0x180], xmm0
0x1408a86c2 0f108548020000 movups xmm0, xmmword ptr [rbp + 0x248]
0x1408a86c9 0f298570010000 movaps xmmword ptr [rbp + 0x170], xmm0
0x1408a86d0 0f108538020000 movups xmm0, xmmword ptr [rbp + 0x238]
0x1408a86d7 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
0x1408a86de 488b8d10030000 mov rcx, qword ptr [rbp + 0x310]
0x1408a86e5 488b9558030000 mov rdx, qword ptr [rbp + 0x358]
0x1408a86ec 4c8b85e8010000 mov r8, qword ptr [rbp + 0x1e8]
0x1408a86f3 4c8945e0 mov qword ptr [rbp - 0x20], r8
0x1408a86f7 0f1085d8010000 movups xmm0, xmmword ptr [rbp + 0x1d8]
0x1408a86fe 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
0x1408a8702 0f1085f0010000 movups xmm0, xmmword ptr [rbp + 0x1f0]
0x1408a8709 0f294500 movaps xmmword ptr [rbp], xmm0
0x1408a870d 4c8b8500020000 mov r8, qword ptr [rbp + 0x200]
0x1408a8714 4c894510 mov qword ptr [rbp + 0x10], r8
0x1408a8718 0f108508020000 movups xmm0, xmmword ptr [rbp + 0x208]
0x1408a871f 0f1145e8 movups xmmword ptr [rbp - 0x18], xmm0
0x1408a8723 4c8b8518020000 mov r8, qword ptr [rbp + 0x218]
0x1408a872a 4c8945f8 mov qword ptr [rbp - 8], r8
0x1408a872e 0f108588020000 movups xmm0, xmmword ptr [rbp + 0x288]
0x1408a8735 0f108d98020000 movups xmm1, xmmword ptr [rbp + 0x298]
0x1408a873c 0f1095a8020000 movups xmm2, xmmword ptr [rbp + 0x2a8]
0x1408a8743 0f109db8020000 movups xmm3, xmmword ptr [rbp + 0x2b8]
0x1408a874a 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1408a874e 0f114d58 movups xmmword ptr [rbp + 0x58], xmm1
0x1408a8752 0f115568 movups xmmword ptr [rbp + 0x68], xmm2
0x1408a8756 0f115d78 movups xmmword ptr [rbp + 0x78], xmm3
0x1408a875a 0f1085c8020000 movups xmm0, xmmword ptr [rbp + 0x2c8]
0x1408a8761 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x1408a8768 0f1085d8020000 movups xmm0, xmmword ptr [rbp + 0x2d8]
0x1408a876f 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
0x1408a8776 4c8b8530020000 mov r8, qword ptr [rbp + 0x230]
0x1408a877d 4c894528 mov qword ptr [rbp + 0x28], r8
0x1408a8781 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
0x1408a8788 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
0x1408a878c 488995b0000000 mov qword ptr [rbp + 0xb0], rdx
0x1408a8793 48898da8000000 mov qword ptr [rbp + 0xa8], rcx
0x1408a879a c7453001000000 mov dword ptr [rbp + 0x30], 1
0x1408a87a1 44896534 mov dword ptr [rbp + 0x34], r12d
0x1408a87a5 894538 mov dword ptr [rbp + 0x38], eax
0x1408a87a8 c7453c01000000 mov dword ptr [rbp + 0x3c], 1
0x1408a87af 488d8d60010000 lea rcx, [rbp + 0x160]
0x1408a87b6 488d55d0 lea rdx, [rbp - 0x30]
0x1408a87ba e8114d1300 call 0x1409dd4d0
0x1408a87bf 4c8bad68030000 mov r13, qword ptr [rbp + 0x368]
0x1408a87c6 4d89f7 mov r15, r14
0x1408a87c9 80bd0e03000008 cmp byte ptr [rbp + 0x30e], 8
0x1408a87d0 440fb6b58e030000 movzx r14d, byte ptr [rbp + 0x38e]
0x1408a87d8 4c8ba578030000 mov r12, qword ptr [rbp + 0x378]
0x1408a87df 740c je 0x1408a87ed
0x1408a87e1 488d8df8020000 lea rcx, [rbp + 0x2f8]
0x1408a87e8 e8c39c3300 call 0x140be24b0
0x1408a87ed 488d8db4010000 lea rcx, [rbp + 0x1b4]
0x1408a87f4 e897cf3200 call 0x140bd5790
0x1408a87f9 41c744244801000000 mov dword ptr [r12 + 0x48], 1
0x1408a8802 498944244c mov qword ptr [r12 + 0x4c], rax
0x1408a8807 48bad1c2cc5c9da79c10 movabs rdx, 0x109ca79d5cccc2d1
0x1408a8811 49b80c84c93018bcea24 movabs r8, 0x24eabc1830c9840c
0x1408a881b 4889f9 mov rcx, rdi
0x1408a881e e8cd4e83ff call 0x1400dd6f0
0x1408a8823 4885c0 test rax, rax
0x1408a8826 0f8461030000 je 0x1408a8b8d
0x1408a882c 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x1408a8833 0f8654030000 jbe 0x1408a8b8d
0x1408a8839 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x1408a8840 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1408a8844 4885c0 test rax, rax
0x1408a8847 0f8440030000 je 0x1408a8b8d
0x1408a884d 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
0x1408a8854 48f7d0 not rax
0x1408a8857 488d1440 lea rdx, [rax + rax*2]
0x1408a885b 48c1e205 shl rdx, 5
0x1408a885f 48837c112000 cmp qword ptr [rcx + rdx + 0x20], 0
0x1408a8865 0f8422030000 je 0x1408a8b8d
0x1408a886b 0fb68500040000 movzx eax, byte ptr [rbp + 0x400]
0x1408a8872 4801d1 add rcx, rdx
0x1408a8875 488b4910 mov rcx, qword ptr [rcx + 0x10]
0x1408a8879 f30f104158 movss xmm0, dword ptr [rcx + 0x58]
0x1408a887e f30f5d05fa4c0b02 minss xmm0, dword ptr [rip + 0x20b4cfa]
0x1408a8886 41807d0500 cmp byte ptr [r13 + 5], 0
0x1408a888b 744f je 0x1408a88dc
0x1408a888d 4c89f9 mov rcx, r15
0x1408a8890 f3410f104d00 movss xmm1, dword ptr [r13]
0x1408a8896 f3410f114c2440 movss dword ptr [r12 + 0x40], xmm1
0x1408a889d 4d8b0424 mov r8, qword ptr [r12]
0x1408a88a1 eb73 jmp 0x1408a8916
0x1408a88a3 41807c240400 cmp byte ptr [r12 + 4], 0
0x1408a88a9 7425 je 0x1408a88d0
0x1408a88ab 48c70200000000 mov qword ptr [rdx], 0
0x1408a88b2 c6423e07 mov byte ptr [rdx + 0x3e], 7
0x1408a88b6 48c7424000000000 mov qword ptr [rdx + 0x40], 0
0x1408a88be c7424800000000 mov dword ptr [rdx + 0x48], 0
0x1408a88c5 66c742540000 mov word ptr [rdx + 0x54], 0
0x1408a88cb e98b020000 jmp 0x1408a8b5b
0x1408a88d0 c605c5c3d80200 mov byte ptr [rip + 0x2d8c3c5], 0
0x1408a88d7 e97f020000 jmp 0x1408a8b5b
0x1408a88dc 4d8b0424 mov r8, qword ptr [r12]
0x1408a88e0 4983f802 cmp r8, 2
0x1408a88e4 0f92c2 setb dl
0x1408a88e7 410a542454 or dl, byte ptr [r12 + 0x54]
0x1408a88ec 4c89f9 mov rcx, r15
0x1408a88ef 80fa01 cmp dl, 1
0x1408a88f2 7509 jne 0x1408a88fd
0x1408a88f4 f3410f104c2440 movss xmm1, dword ptr [r12 + 0x40]
0x1408a88fb eb19 jmp 0x1408a8916
0x1408a88fd 0f28c8 movaps xmm1, xmm0
0x1408a8900 f30f5e0d70471202 divss xmm1, dword ptr [rip + 0x2124770]
0x1408a8908 f3410f584c2440 addss xmm1, dword ptr [r12 + 0x40]
0x1408a890f f3410f114c2440 movss dword ptr [r12 + 0x40], xmm1
0x1408a8916 0fb6958f030000 movzx edx, byte ptr [rbp + 0x38f]
0x1408a891d 49ffc0 inc r8
0x1408a8920 4d890424 mov qword ptr [r12], r8
0x1408a8924 f3410f58442444 addss xmm0, dword ptr [r12 + 0x44]
0x1408a892b f3410f11442444 movss dword ptr [r12 + 0x44], xmm0
0x1408a8932 4d8d4c2408 lea r9, [r12 + 8]
0x1408a8937 4531c0 xor r8d, r8d
0x1408a893a 41807c243e07 cmp byte ptr [r12 + 0x3e], 7
0x1408a8940 4d89cf mov r15, r9
0x1408a8943 4d0f45c1 cmovne r8, r9
0x1408a8947 20d3 and bl, dl
0x1408a8949 80e301 and bl, 1
0x1408a894c 885c2448 mov byte ptr [rsp + 0x48], bl
0x1408a8950 88442440 mov byte ptr [rsp + 0x40], al
0x1408a8954 4488742438 mov byte ptr [rsp + 0x38], r14b
0x1408a8959 f30f114c2430 movss dword ptr [rsp + 0x30], xmm1
0x1408a895f f30f11442428 movss dword ptr [rsp + 0x28], xmm0
0x1408a8965 488b8580030000 mov rax, qword ptr [rbp + 0x380]
0x1408a896c 4889442420 mov qword ptr [rsp + 0x20], rax
0x1408a8971 4c8d8db0010000 lea r9, [rbp + 0x1b0]
0x1408a8978 4889f2 mov rdx, rsi
0x1408a897b e8f0030000 call 0x1408a8d70
0x1408a8980 f3410f10442440 movss xmm0, dword ptr [r12 + 0x40]
0x1408a8987 0f2e0576cf0702 ucomiss xmm0, dword ptr [rip + 0x207cf76]
0x1408a898e 0f82c1010000 jb 0x1408a8b55
0x1408a8994 41807d0501 cmp byte ptr [r13 + 5], 1
0x1408a8999 0f84b6010000 je 0x1408a8b55
0x1408a899f 41807c245400 cmp byte ptr [r12 + 0x54], 0
0x1408a89a5 0f85aa010000 jne 0x1408a8b55
0x1408a89ab 41837c244802 cmp dword ptr [r12 + 0x48], 2
0x1408a89b1 7428 je 0x1408a89db
0x1408a89b3 488b8578030000 mov rax, qword ptr [rbp + 0x378]
0x1408a89ba 80783e07 cmp byte ptr [rax + 0x3e], 7
0x1408a89be 741b je 0x1408a89db
0x1408a89c0 4c89bd80030000 mov qword ptr [rbp + 0x380], r15
0x1408a89c7 4c89f9 mov rcx, r15
0x1408a89ca e8b13179ff call 0x14003bb80
0x1408a89cf 488b8d80030000 mov rcx, qword ptr [rbp + 0x380]
0x1408a89d6 e8d59a3300 call 0x140be24b0
0x1408a89db 488b8578030000 mov rax, qword ptr [rbp + 0x378]
0x1408a89e2 c7404802000000 mov dword ptr [rax + 0x48], 2
0x1408a89e9 48ba1cd6a94e063df9d8 movabs rdx, 0xd8f93d064ea9d61c
0x1408a89f3 49b82f83d96670614764 movabs r8, 0x6447617066d9832f
0x1408a89fd 4889f9 mov rcx, rdi
0x1408a8a00 e8eb4c83ff call 0x1400dd6f0
0x1408a8a05 4885c0 test rax, rax
0x1408a8a08 0f84c0010000 je 0x1408a8bce
0x1408a8a0e 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x1408a8a15 0f86b3010000 jbe 0x1408a8bce
0x1408a8a1b 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x1408a8a22 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1408a8a26 4885c9 test rcx, rcx
0x1408a8a29 0f849f010000 je 0x1408a8bce
0x1408a8a2f 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x1408a8a36 48f7d1 not rcx
0x1408a8a39 488d0c49 lea rcx, [rcx + rcx*2]
0x1408a8a3d 48c1e105 shl rcx, 5
0x1408a8a41 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1408a8a47 0f8481010000 je 0x1408a8bce
0x1408a8a4d 4801c8 add rax, rcx
0x1408a8a50 488b7810 mov rdi, qword ptr [rax + 0x10]
0x1408a8a54 8b8eb0050000 mov ecx, dword ptr [rsi + 0x5b0]
0x1408a8a5a 89485c mov dword ptr [rax + 0x5c], ecx
0x1408a8a5d 488b7730 mov rsi, qword ptr [rdi + 0x30]
0x1408a8a61 488b5f40 mov rbx, qword ptr [rdi + 0x40]
0x1408a8a65 483b7720 cmp rsi, qword ptr [rdi + 0x20]
0x1408a8a69 7510 jne 0x1408a8a7b
0x1408a8a6b 488d4f20 lea rcx, [rdi + 0x20]
0x1408a8a6f 488d1502063e02 lea rdx, [rip + 0x23e0602]
0x1408a8a76 e8f59dcbff call 0x140562870
0x1408a8a7b 488b4728 mov rax, qword ptr [rdi + 0x28]
0x1408a8a7f 48891cf0 mov qword ptr [rax + rsi*8], rbx
0x1408a8a83 48ffc6 inc rsi
0x1408a8a86 48897730 mov qword ptr [rdi + 0x30], rsi
0x1408a8a8a 48ff4740 inc qword ptr [rdi + 0x40]
0x1408a8a8e 488b051339d802 mov rax, qword ptr [rip + 0x2d83913]
0x1408a8a95 488b00 mov rax, qword ptr [rax]
0x1408a8a98 4883f803 cmp rax, 3
0x1408a8a9c 0f82b3000000 jb 0x1408a8b55
0x1408a8aa2 488d0d7f311f02 lea rcx, [rip + 0x21f317f]
0x1408a8aa9 e8d252ddff call 0x14067dd80
0x1408a8aae 0f1000 movups xmm0, xmmword ptr [rax]
0x1408a8ab1 8b4010 mov eax, dword ptr [rax + 0x10]
0x1408a8ab4 48c7850802000003000000 mov qword ptr [rbp + 0x208], 3
0x1408a8abf 488d0daa301f02 lea rcx, [rip + 0x21f30aa]
0x1408a8ac6 48898d10020000 mov qword ptr [rbp + 0x210], rcx
0x1408a8acd 48c785180200002b000000 mov qword ptr [rbp + 0x218], 0x2b
0x1408a8ad8 488d1539311f02 lea rdx, [rip + 0x21f3139]
0x1408a8adf 48899528020000 mov qword ptr [rbp + 0x228], rdx
0x1408a8ae6 48c7853002000001000000 mov qword ptr [rbp + 0x230], 1
0x1408a8af1 48c7853802000008000000 mov qword ptr [rbp + 0x238], 8
0x1408a8afc 48c785d801000000000000 mov qword ptr [rbp + 0x1d8], 0
0x1408a8b07 0f57c9 xorps xmm1, xmm1
0x1408a8b0a 0f118d40020000 movups xmmword ptr [rbp + 0x240], xmm1
0x1408a8b11 48898de0010000 mov qword ptr [rbp + 0x1e0], rcx
0x1408a8b18 48c785e80100002b000000 mov qword ptr [rbp + 0x1e8], 0x2b
0x1408a8b23 48c785f001000000000000 mov qword ptr [rbp + 0x1f0], 0
0x1408a8b2e 0f1185f8010000 movups xmmword ptr [rbp + 0x1f8], xmm0
0x1408a8b35 c7852002000001000000 mov dword ptr [rbp + 0x220], 1
0x1408a8b3f 898524020000 mov dword ptr [rbp + 0x224], eax
0x1408a8b45 488d4dd0 lea rcx, [rbp - 0x30]
0x1408a8b49 488d95d8010000 lea rdx, [rbp + 0x1d8]
0x1408a8b50 e8db174a00 call 0x140d4a330
0x1408a8b55 8b8574030000 mov eax, dword ptr [rbp + 0x374]
0x1408a8b5b 83f802 cmp eax, 2
0x1408a8b5e 0f95c0 setne al
0x1408a8b61 4881c418040000 add rsp, 0x418
0x1408a8b68 5b pop rbx
0x1408a8b69 5f pop rdi
0x1408a8b6a 5e pop rsi
0x1408a8b6b 415c pop r12
0x1408a8b6d 415d pop r13
0x1408a8b6f 415e pop r14
0x1408a8b71 415f pop r15
0x1408a8b73 5d pop rbp
0x1408a8b74 c3 ret 
0x1408a8b75 488d0d5c2f1f02 lea rcx, [rip + 0x21f2f5c]
0x1408a8b7c e88f070302 call 0x1428d9310
0x1408a8b81 488d0d682f1f02 lea rcx, [rip + 0x21f2f68]
0x1408a8b88 e883070302 call 0x1428d9310
0x1408a8b8d 488d0d3c301f02 lea rcx, [rip + 0x21f303c]
0x1408a8b94 e877070302 call 0x1428d9310
0x1408a8b99 488b85d8010000 mov rax, qword ptr [rbp + 0x1d8]
0x1408a8ba0 488945d0 mov qword ptr [rbp - 0x30], rax
0x1408a8ba4 488d056d2f1f02 lea rax, [rip + 0x21f2f6d]
0x1408a8bab 4889442420 mov qword ptr [rsp + 0x20], rax
0x1408a8bb0 488d0d09251f02 lea rcx, [rip + 0x21f2509]
0x1408a8bb7 4c8d0de2241f02 lea r9, [rip + 0x21f24e2]
0x1408a8bbe 4c8d45d0 lea r8, [rbp - 0x30]
0x1408a8bc2 ba2b000000 mov edx, 0x2b
0x1408a8bc7 e8940b0302 call 0x1428d9760
0x1408a8bcc 0f0b ud2 
0x1408a8bce 488d0d13301f02 lea rcx, [rip + 0x21f3013]
0x1408a8bd5 e836070302 call 0x1428d9310
0x1408a8bda cc int3 
0x1408a8bdb 0f1f440000 nop dword ptr [rax + rax]
0x1408a8be0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408a8be5 55 push rbp
0x1408a8be6 4157 push r15
0x1408a8be8 4156 push r14
0x1408a8bea 4155 push r13
0x1408a8bec 4154 push r12
0x1408a8bee 56 push rsi
0x1408a8bef 57 push rdi
0x1408a8bf0 53 push rbx
0x1408a8bf1 4883ec58 sub rsp, 0x58
0x1408a8bf5 488daa80000000 lea rbp, [rdx + 0x80]
0x1408a8bfc 488b8d80030000 mov rcx, qword ptr [rbp + 0x380]
0x1408a8c03 e8a8983300 call 0x140be24b0
0x1408a8c08 90 nop 
0x1408a8c09 4883c458 add rsp, 0x58
0x1408a8c0d 5b pop rbx
0x1408a8c0e 5f pop rdi
0x1408a8c0f 5e pop rsi
0x1408a8c10 415c pop r12
0x1408a8c12 415d pop r13
0x1408a8c14 415e pop r14
0x1408a8c16 415f pop r15
0x1408a8c18 5d pop rbp
0x1408a8c19 c3 ret 
0x1408a8c1a 660f1f440000 nop word ptr [rax + rax]
0x1408a8c20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408a8c25 55 push rbp
0x1408a8c26 4157 push r15
0x1408a8c28 4156 push r14
0x1408a8c2a 4155 push r13
0x1408a8c2c 4154 push r12
0x1408a8c2e 56 push rsi
0x1408a8c2f 57 push rdi
0x1408a8c30 53 push rbx
0x1408a8c31 4883ec58 sub rsp, 0x58
0x1408a8c35 488daa80000000 lea rbp, [rdx + 0x80]
0x1408a8c3c 488b8578030000 mov rax, qword ptr [rbp + 0x378]
0x1408a8c43 c7404802000000 mov dword ptr [rax + 0x48], 2
0x1408a8c4a 4883c458 add rsp, 0x58
0x1408a8c4e 5b pop rbx
0x1408a8c4f 5f pop rdi
0x1408a8c50 5e pop rsi
0x1408a8c51 415c pop r12
0x1408a8c53 415d pop r13
0x1408a8c55 415e pop r14
0x1408a8c57 415f pop r15
0x1408a8c59 5d pop rbp
0x1408a8c5a c3 ret 
0x1408a8c5b 0f1f440000 nop dword ptr [rax + rax]
0x1408a8c60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408a8c65 55 push rbp
0x1408a8c66 4157 push r15
0x1408a8c68 4156 push r14
0x1408a8c6a 4155 push r13
0x1408a8c6c 4154 push r12
0x1408a8c6e 56 push rsi
0x1408a8c6f 57 push rdi
0x1408a8c70 53 push rbx
0x1408a8c71 4883ec58 sub rsp, 0x58
0x1408a8c75 488daa80000000 lea rbp, [rdx + 0x80]
0x1408a8c7c 80bd0e03000008 cmp byte ptr [rbp + 0x30e], 8
0x1408a8c83 740c je 0x1408a8c91
0x1408a8c85 488d8df8020000 lea rcx, [rbp + 0x2f8]
0x1408a8c8c e81f983300 call 0x140be24b0
0x1408a8c91 90 nop 
0x1408a8c92 4883c458 add rsp, 0x58
0x1408a8c96 5b pop rbx
0x1408a8c97 5f pop rdi
0x1408a8c98 5e pop rsi
0x1408a8c99 415c pop r12
0x1408a8c9b 415d pop r13
0x1408a8c9d 415e pop r14
0x1408a8c9f 415f pop r15
0x1408a8ca1 5d pop rbp
0x1408a8ca2 c3 ret 
0x1408a8ca3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1408a8cb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408a8cb5 55 push rbp
0x1408a8cb6 4157 push r15
0x1408a8cb8 4156 push r14
0x1408a8cba 4155 push r13
0x1408a8cbc 4154 push r12
0x1408a8cbe 56 push rsi
0x1408a8cbf 57 push rdi
0x1408a8cc0 53 push rbx
0x1408a8cc1 4883ec58 sub rsp, 0x58
0x1408a8cc5 488daa80000000 lea rbp, [rdx + 0x80]
0x1408a8ccc 488d4dd0 lea rcx, [rbp - 0x30]
0x1408a8cd0 e8dbe2feff call 0x140896fb0
0x1408a8cd5 90 nop 
0x1408a8cd6 4883c458 add rsp, 0x58
0x1408a8cda 5b pop rbx
0x1408a8cdb 5f pop rdi
0x1408a8cdc 5e pop rsi
0x1408a8cdd 415c pop r12
0x1408a8cdf 415d pop r13
0x1408a8ce1 415e pop r14
0x1408a8ce3 415f pop r15
0x1408a8ce5 5d pop rbp
0x1408a8ce6 c3 ret 
0x1408a8ce7 660f1f840000000000 nop word ptr [rax + rax]
0x1408a8cf0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408a8cf5 55 push rbp
0x1408a8cf6 4157 push r15
0x1408a8cf8 4156 push r14
0x1408a8cfa 4155 push r13
0x1408a8cfc 4154 push r12
0x1408a8cfe 56 push rsi
0x1408a8cff 57 push rdi
0x1408a8d00 53 push rbx
0x1408a8d01 4883ec58 sub rsp, 0x58
0x1408a8d05 488daa80000000 lea rbp, [rdx + 0x80]
0x1408a8d0c 488d8dd8010000 lea rcx, [rbp + 0x1d8]
0x1408a8d13 e898e2feff call 0x140896fb0
0x1408a8d18 90 nop 
0x1408a8d19 4883c458 add rsp, 0x58
0x1408a8d1d 5b pop rbx
0x1408a8d1e 5f pop rdi
0x1408a8d1f 5e pop rsi
0x1408a8d20 415c pop r12
0x1408a8d22 415d pop r13
0x1408a8d24 415e pop r14
0x1408a8d26 415f pop r15
0x1408a8d28 5d pop rbp
0x1408a8d29 c3 ret 
0x1408a8d2a 660f1f440000 nop word ptr [rax + rax]
0x1408a8d30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408a8d35 55 push rbp
0x1408a8d36 4157 push r15
0x1408a8d38 4156 push r14
0x1408a8d3a 4155 push r13
0x1408a8d3c 4154 push r12
0x1408a8d3e 56 push rsi
0x1408a8d3f 57 push rdi
0x1408a8d40 53 push rbx
0x1408a8d41 4883ec58 sub rsp, 0x58
0x1408a8d45 488daa80000000 lea rbp, [rdx + 0x80]
0x1408a8d4c 488d4dd0 lea rcx, [rbp - 0x30]
0x1408a8d50 e8db3e4a00 call 0x140d4cc30
0x1408a8d55 90 nop 
0x1408a8d56 4883c458 add rsp, 0x58
0x1408a8d5a 5b pop rbx
0x1408a8d5b 5f pop rdi
0x1408a8d5c 5e pop rsi
0x1408a8d5d 415c pop r12
0x1408a8d5f 415d pop r13
0x1408a8d61 415e pop r14
0x1408a8d63 415f pop r15
0x1408a8d65 5d pop rbp
0x1408a8d66 c3 ret 
