0x1406868d0 push rbp
0x1406868d1 push r15
0x1406868d3 push r14
0x1406868d5 push r13
0x1406868d7 push r12
0x1406868d9 push rsi
0x1406868da push rdi
0x1406868db push rbx
0x1406868dc sub rsp, 0x578
0x1406868e3 lea rbp, [rsp + 0x80]
0x1406868eb movaps xmmword ptr [rbp + 0x4e0], xmm15
0x1406868f3 movaps xmmword ptr [rbp + 0x4d0], xmm14
0x1406868fb movaps xmmword ptr [rbp + 0x4c0], xmm13
0x140686903 movaps xmmword ptr [rbp + 0x4b0], xmm12
0x14068690b movaps xmmword ptr [rbp + 0x4a0], xmm11
0x140686913 movaps xmmword ptr [rbp + 0x490], xmm10
0x14068691b movaps xmmword ptr [rbp + 0x480], xmm9
0x140686923 movaps xmmword ptr [rbp + 0x470], xmm8
0x14068692b movaps xmmword ptr [rbp + 0x460], xmm7
0x140686932 movaps xmmword ptr [rbp + 0x450], xmm6
0x140686939 mov qword ptr [rbp + 0x448], 0xfffffffffffffffe
0x140686944 mov rsi, rdx
0x140686947 call 0x140d4d740
0x14068694c mov rdi, rax
0x14068694f test byte ptr [rax], 1
0x140686952 je 0x140686965
0x140686954 mov rcx, rdi
0x140686957 add rcx, 8
0x14068695b mov rax, qword ptr [rdi + 8]
0x14068695f mov rdx, qword ptr [rdi + 0x10]
0x140686963 jmp 0x140686980
0x140686965 call 0x140d64e00
0x14068696a mov rcx, rdi
0x14068696d add rcx, 8
0x140686971 mov qword ptr [rdi], 1
0x140686978 mov qword ptr [rdi + 8], rax
0x14068697c mov qword ptr [rdi + 0x10], rdx
0x140686980 lea r8, [rax + 1]
0x140686984 mov qword ptr [rcx], r8
0x140686987 movups xmm6, xmmword ptr [rip + 0x2391132]
0x14068698e movaps xmmword ptr [rbp - 0x20], xmm6
0x140686992 movups xmm7, xmmword ptr [rip + 0x2391117]
0x140686999 movaps xmmword ptr [rbp - 0x30], xmm7
0x14068699d mov qword ptr [rbp - 0x10], rax
0x1406869a1 mov qword ptr [rbp - 8], rdx
0x1406869a5 call 0x140d4d740
0x1406869aa mov rdi, rax
0x1406869ad test byte ptr [rax], 1
0x1406869b0 je 0x1406869c3
0x1406869b2 mov rcx, rdi
0x1406869b5 add rcx, 8
0x1406869b9 mov rax, qword ptr [rdi + 8]
0x1406869bd mov rdx, qword ptr [rdi + 0x10]
0x1406869c1 jmp 0x1406869de
0x1406869c3 call 0x140d64e00
0x1406869c8 mov rcx, rdi
0x1406869cb add rcx, 8
0x1406869cf mov qword ptr [rdi], 1
0x1406869d6 mov qword ptr [rdi + 8], rax
0x1406869da mov qword ptr [rdi + 0x10], rdx
0x1406869de lea r8, [rax + 1]
0x1406869e2 mov qword ptr [rcx], r8
0x1406869e5 movaps xmmword ptr [rbp - 0x50], xmm6
0x1406869e9 movaps xmmword ptr [rbp - 0x60], xmm7
0x1406869ed mov qword ptr [rbp - 0x40], rax
0x1406869f1 mov qword ptr [rbp - 0x38], rdx
0x1406869f5 call 0x140d4d740
0x1406869fa mov rdi, rax
0x1406869fd test byte ptr [rax], 1
0x140686a00 je 0x140686a13
0x140686a02 mov rcx, rdi
0x140686a05 add rcx, 8
0x140686a09 mov rax, qword ptr [rdi + 8]
0x140686a0d mov rdx, qword ptr [rdi + 0x10]
0x140686a11 jmp 0x140686a2e
0x140686a13 call 0x140d64e00
0x140686a18 mov rcx, rdi
0x140686a1b add rcx, 8
0x140686a1f mov qword ptr [rdi], 1
0x140686a26 mov qword ptr [rdi + 8], rax
0x140686a2a mov qword ptr [rdi + 0x10], rdx
0x140686a2e lea r8, [rax + 1]
0x140686a32 mov qword ptr [rcx], r8
0x140686a35 movaps xmmword ptr [rbp + 0x2b0], xmm6
0x140686a3c movaps xmmword ptr [rbp + 0x2a0], xmm7
0x140686a43 mov qword ptr [rbp + 0x2c0], rax
0x140686a4a mov qword ptr [rbp + 0x2c8], rdx
0x140686a51 call 0x140d4d740
0x140686a56 mov rdi, rax
0x140686a59 test byte ptr [rax], 1
0x140686a5c je 0x140686a6f
0x140686a5e mov rcx, rdi
0x140686a61 add rcx, 8
0x140686a65 mov rax, qword ptr [rdi + 8]
0x140686a69 mov rdx, qword ptr [rdi + 0x10]
0x140686a6d jmp 0x140686a8a
0x140686a6f call 0x140d64e00
0x140686a74 mov rcx, rdi
0x140686a77 add rcx, 8
0x140686a7b mov qword ptr [rdi], 1
0x140686a82 mov qword ptr [rdi + 8], rax
0x140686a86 mov qword ptr [rdi + 0x10], rdx
0x140686a8a lea r8, [rax + 1]
0x140686a8e mov qword ptr [rcx], r8
0x140686a91 movaps xmmword ptr [rbp + 0x2e0], xmm6
0x140686a98 movaps xmmword ptr [rbp + 0x2d0], xmm7
0x140686a9f mov qword ptr [rbp + 0x2f0], rax
0x140686aa6 mov qword ptr [rbp + 0x2f8], rdx
0x140686aad mov qword ptr [rbp + 0x288], 0
0x140686ab8 mov qword ptr [rbp + 0x290], 8
0x140686ac3 mov qword ptr [rbp + 0x298], 0
0x140686ace movzx eax, byte ptr [rip + 0x2fae134]
0x140686ad5 mov ecx, 0x40
0x140686ada mov edx, 8
0x140686adf call 0x140613c10
0x140686ae4 mov qword ptr [rbp + 0x440], rax
0x140686aeb test rax, rax
0x140686aee je 0x14068760b
0x140686af4 xor ecx, ecx
0x140686af6 call 0x1400d9b20
0x140686afb test rax, rax
0x140686afe je 0x14068761c
0x140686b04 movups xmm0, xmmword ptr [rax]
0x140686b07 inc qword ptr [rax]
0x140686b0a mov rax, qword ptr [rbp + 0x440]
0x140686b11 mov qword ptr [rax], 1
0x140686b18 mov qword ptr [rax + 8], 1
0x140686b20 movups xmm1, xmmword ptr [rip + 0x23916d9]
0x140686b27 movups xmmword ptr [rax + 0x10], xmm1
0x140686b2b movups xmm1, xmmword ptr [rip + 0x23916de]
0x140686b32 movups xmmword ptr [rax + 0x20], xmm1
0x140686b36 movups xmmword ptr [rax + 0x30], xmm0
0x140686b3a mov qword ptr [rbp + 0x280], rax
0x140686b41 call 0x14093fa60
0x140686b46 mov dword ptr [rbp + 0x348], eax
0x140686b4c movss dword ptr [rbp + 0x34c], xmm0
0x140686b54 call 0x14093fa60
0x140686b59 mov dword ptr [rbp + 0x350], eax
0x140686b5f movss dword ptr [rbp + 0x354], xmm0
0x140686b67 call 0x14093fa60
0x140686b6c mov dword ptr [rbp + 0x358], eax
0x140686b72 movss dword ptr [rbp + 0x35c], xmm0
0x140686b7a call 0x14093fa60
0x140686b7f mov dword ptr [rbp + 0x360], eax
0x140686b85 movss dword ptr [rbp + 0x364], xmm0
0x140686b8d call 0x14093fa60
0x140686b92 mov dword ptr [rbp + 0x368], eax
0x140686b98 movss dword ptr [rbp + 0x36c], xmm0
0x140686ba0 call 0x14093fa60
0x140686ba5 mov dword ptr [rbp + 0x370], eax
0x140686bab movss dword ptr [rbp + 0x374], xmm0
0x140686bb3 call 0x14093fa60
0x140686bb8 mov dword ptr [rbp + 0x378], eax
0x140686bbe movss dword ptr [rbp + 0x37c], xmm0
0x140686bc6 call 0x14093fa60
0x140686bcb mov dword ptr [rbp + 0x380], eax
0x140686bd1 movss dword ptr [rbp + 0x384], xmm0
0x140686bd9 call 0x14093fa60
0x140686bde mov dword ptr [rbp + 0x388], eax
0x140686be4 movss dword ptr [rbp + 0x38c], xmm0
0x140686bec call 0x14093fa60
0x140686bf1 mov dword ptr [rbp + 0x390], eax
0x140686bf7 movss dword ptr [rbp + 0x394], xmm0
0x140686bff call 0x14093fa60
0x140686c04 mov dword ptr [rbp + 0x398], eax
0x140686c0a movss dword ptr [rbp + 0x39c], xmm0
0x140686c12 call 0x14093fa60
0x140686c17 mov dword ptr [rbp + 0x3a0], eax
0x140686c1d movss dword ptr [rbp + 0x3a4], xmm0
0x140686c25 call 0x14093fa60
0x140686c2a mov dword ptr [rbp + 0x3a8], eax
0x140686c30 movss dword ptr [rbp + 0x3ac], xmm0
0x140686c38 call 0x14093fa60
0x140686c3d mov dword ptr [rbp + 0x3b0], eax
0x140686c43 movss dword ptr [rbp + 0x3b4], xmm0
0x140686c4b call 0x14093fa60
0x140686c50 mov dword ptr [rbp + 0x3b8], eax
0x140686c56 movss dword ptr [rbp + 0x3bc], xmm0
0x140686c5e call 0x14093fa60
0x140686c63 mov dword ptr [rbp + 0x3c0], eax
0x140686c69 movss dword ptr [rbp + 0x3c4], xmm0
0x140686c71 call 0x14093fa60
0x140686c76 mov dword ptr [rbp + 0x3c8], eax
0x140686c7c movss dword ptr [rbp + 0x3cc], xmm0
0x140686c84 call 0x14093fa60
0x140686c89 mov dword ptr [rbp + 0x3d0], eax
0x140686c8f movss dword ptr [rbp + 0x3d4], xmm0
0x140686c97 call 0x14093fa60
0x140686c9c mov dword ptr [rbp + 0x3d8], eax
0x140686ca2 movss dword ptr [rbp + 0x3dc], xmm0
0x140686caa call 0x14093fa60
0x140686caf mov dword ptr [rbp + 0x3e0], eax
0x140686cb5 movss dword ptr [rbp + 0x3e4], xmm0
0x140686cbd call 0x14093fa60
0x140686cc2 mov dword ptr [rbp + 0x3e8], eax
0x140686cc8 movss dword ptr [rbp + 0x3ec], xmm0
0x140686cd0 call 0x14093fa60
0x140686cd5 mov dword ptr [rbp + 0x3f0], eax
0x140686cdb movss dword ptr [rbp + 0x3f4], xmm0
0x140686ce3 call 0x14093fa60
0x140686ce8 mov dword ptr [rbp + 0x3f8], eax
0x140686cee movss dword ptr [rbp + 0x3fc], xmm0
0x140686cf6 call 0x14093fa60
0x140686cfb mov dword ptr [rbp + 0x400], eax
0x140686d01 movss dword ptr [rbp + 0x404], xmm0
0x140686d09 call 0x14093fa60
0x140686d0e mov dword ptr [rbp + 0x408], eax
0x140686d14 movss dword ptr [rbp + 0x40c], xmm0
0x140686d1c call 0x14093fa60
0x140686d21 mov dword ptr [rbp + 0x410], eax
0x140686d27 movss dword ptr [rbp + 0x414], xmm0
0x140686d2f call 0x14093fa60
0x140686d34 mov dword ptr [rbp + 0x418], eax
0x140686d3a movss dword ptr [rbp + 0x41c], xmm0
0x140686d42 call 0x14093fa60
0x140686d47 mov dword ptr [rbp + 0x420], eax
0x140686d4d movss dword ptr [rbp + 0x424], xmm0
0x140686d55 call 0x14093fa60
0x140686d5a mov dword ptr [rbp + 0x428], eax
0x140686d60 movss dword ptr [rbp + 0x42c], xmm0
0x140686d68 call 0x14093fa60
0x140686d6d mov dword ptr [rbp + 0x430], eax
0x140686d73 movaps xmm15, xmm0
0x140686d77 call 0x14093fa60
0x140686d7c mov dword ptr [rbp + 0x434], eax
0x140686d82 movaps xmm6, xmm0
0x140686d85 call 0x14093fa60
0x140686d8a mov dword ptr [rbp + 0x438], eax
0x140686d90 movaps xmm7, xmm0
0x140686d93 call 0x14093fa60
0x140686d98 mov dword ptr [rbp + 0x43c], eax
0x140686d9e movaps xmm8, xmm0
0x140686da2 call 0x14093fa60
0x140686da7 mov r15d, eax
0x140686daa movaps xmm9, xmm0
0x140686dae call 0x14093fa60
0x140686db3 mov r12d, eax
0x140686db6 movaps xmm10, xmm0
0x140686dba call 0x14093fa60
0x140686dbf mov r13d, eax
0x140686dc2 movaps xmm11, xmm0
0x140686dc6 call 0x14093fa60
0x140686dcb mov edi, eax
0x140686dcd movaps xmm12, xmm0
0x140686dd1 call 0x14093fa60
0x140686dd6 mov ebx, eax
0x140686dd8 movaps xmm13, xmm0
0x140686ddc call 0x14093fa60
0x140686de1 mov r14d, eax
0x140686de4 movaps xmm14, xmm0
0x140686de8 call 0x14093fa60
0x140686ded xorps xmm1, xmm1
0x140686df0 movups xmmword ptr [rbp + 0x190], xmm1
0x140686df7 movups xmmword ptr [rbp + 0x180], xmm1
0x140686dfe movups xmmword ptr [rbp + 0x170], xmm1
0x140686e05 movaps xmm1, xmmword ptr [rbp - 0x30]
0x140686e09 movups xmmword ptr [rbp + 0x1a0], xmm1
0x140686e10 movaps xmm1, xmmword ptr [rbp - 0x20]
0x140686e14 movups xmmword ptr [rbp + 0x1b0], xmm1
0x140686e1b movaps xmm1, xmmword ptr [rbp - 0x10]
0x140686e1f movups xmmword ptr [rbp + 0x1c0], xmm1
0x140686e26 movaps xmm1, xmmword ptr [rbp - 0x60]
0x140686e2a movups xmmword ptr [rbp + 0x1d0], xmm1
0x140686e31 movaps xmm1, xmmword ptr [rbp - 0x50]
0x140686e35 movups xmmword ptr [rbp + 0x1e0], xmm1
0x140686e3c movaps xmm1, xmmword ptr [rbp - 0x40]
0x140686e40 movups xmmword ptr [rbp + 0x1f0], xmm1
0x140686e47 movaps xmm1, xmmword ptr [rbp + 0x2a0]
0x140686e4e movups xmmword ptr [rbp + 0x200], xmm1
0x140686e55 movaps xmm1, xmmword ptr [rbp + 0x2b0]
0x140686e5c movups xmmword ptr [rbp + 0x210], xmm1
0x140686e63 movaps xmm1, xmmword ptr [rbp + 0x2c0]
0x140686e6a movups xmmword ptr [rbp + 0x220], xmm1
0x140686e71 mov qword ptr [rbp], 0
0x140686e79 mov qword ptr [rbp + 8], 8
0x140686e81 mov qword ptr [rbp + 0x10], 0
0x140686e89 movaps xmm1, xmmword ptr [rbp + 0x2d0]
0x140686e90 movups xmmword ptr [rbp + 0x230], xmm1
0x140686e97 movaps xmm1, xmmword ptr [rbp + 0x2e0]
0x140686e9e movups xmmword ptr [rbp + 0x240], xmm1
0x140686ea5 movaps xmm1, xmmword ptr [rbp + 0x2f0]
0x140686eac movups xmmword ptr [rbp + 0x250], xmm1
0x140686eb3 mov qword ptr [rbp + 0x268], 0
0x140686ebe mov dword ptr [rbp + 0x270], 0
0x140686ec8 movups xmm1, xmmword ptr [rbp + 0x288]
0x140686ecf movups xmmword ptr [rbp + 0x18], xmm1
0x140686ed3 mov rcx, qword ptr [rbp + 0x298]
0x140686eda mov qword ptr [rbp + 0x28], rcx
0x140686ede mov rcx, qword ptr [rbp + 0x440]
0x140686ee5 mov qword ptr [rbp + 0x260], rcx
0x140686eec mov ecx, dword ptr [rbp + 0x348]
0x140686ef2 mov dword ptr [rbp + 0x30], ecx
0x140686ef5 movss xmm1, dword ptr [rbp + 0x34c]
0x140686efd movss dword ptr [rbp + 0x34], xmm1
0x140686f02 mov ecx, dword ptr [rbp + 0x350]
0x140686f08 mov dword ptr [rbp + 0x38], ecx
0x140686f0b movss xmm1, dword ptr [rbp + 0x354]
0x140686f13 movss dword ptr [rbp + 0x3c], xmm1
0x140686f18 mov ecx, dword ptr [rbp + 0x358]
0x140686f1e mov dword ptr [rbp + 0x40], ecx
0x140686f21 movss xmm1, dword ptr [rbp + 0x35c]
0x140686f29 movss dword ptr [rbp + 0x44], xmm1
0x140686f2e mov ecx, dword ptr [rbp + 0x360]
0x140686f34 mov dword ptr [rbp + 0x48], ecx
0x140686f37 movss xmm1, dword ptr [rbp + 0x364]
0x140686f3f movss dword ptr [rbp + 0x4c], xmm1
0x140686f44 mov ecx, dword ptr [rbp + 0x368]
0x140686f4a mov dword ptr [rbp + 0x50], ecx
0x140686f4d movss xmm1, dword ptr [rbp + 0x36c]
0x140686f55 movss dword ptr [rbp + 0x54], xmm1
0x140686f5a mov ecx, dword ptr [rbp + 0x370]
0x140686f60 mov dword ptr [rbp + 0x58], ecx
0x140686f63 movss xmm1, dword ptr [rbp + 0x374]
0x140686f6b movss dword ptr [rbp + 0x5c], xmm1
0x140686f70 mov ecx, dword ptr [rbp + 0x378]
0x140686f76 mov dword ptr [rbp + 0x60], ecx
0x140686f79 movss xmm1, dword ptr [rbp + 0x37c]
0x140686f81 movss dword ptr [rbp + 0x64], xmm1
0x140686f86 mov ecx, dword ptr [rbp + 0x380]
0x140686f8c mov dword ptr [rbp + 0x68], ecx
0x140686f8f movss xmm1, dword ptr [rbp + 0x384]
0x140686f97 movss dword ptr [rbp + 0x6c], xmm1
0x140686f9c mov ecx, dword ptr [rbp + 0x388]
0x140686fa2 mov dword ptr [rbp + 0x70], ecx
0x140686fa5 movss xmm1, dword ptr [rbp + 0x38c]
0x140686fad movss dword ptr [rbp + 0x74], xmm1
0x140686fb2 mov ecx, dword ptr [rbp + 0x390]
0x140686fb8 mov dword ptr [rbp + 0x78], ecx
0x140686fbb movss xmm1, dword ptr [rbp + 0x394]
0x140686fc3 movss dword ptr [rbp + 0x7c], xmm1
0x140686fc8 mov ecx, dword ptr [rbp + 0x398]
0x140686fce mov dword ptr [rbp + 0x80], ecx
0x140686fd4 movss xmm1, dword ptr [rbp + 0x39c]
0x140686fdc movss dword ptr [rbp + 0x84], xmm1
0x140686fe4 mov ecx, dword ptr [rbp + 0x3a0]
0x140686fea mov dword ptr [rbp + 0x88], ecx
0x140686ff0 movss xmm1, dword ptr [rbp + 0x3a4]
0x140686ff8 movss dword ptr [rbp + 0x8c], xmm1
0x140687000 mov ecx, dword ptr [rbp + 0x3a8]
0x140687006 mov dword ptr [rbp + 0x90], ecx
0x14068700c movss xmm1, dword ptr [rbp + 0x3ac]
0x140687014 movss dword ptr [rbp + 0x94], xmm1
0x14068701c mov ecx, dword ptr [rbp + 0x3b0]
0x140687022 mov dword ptr [rbp + 0x98], ecx
0x140687028 movss xmm1, dword ptr [rbp + 0x3b4]
0x140687030 movss dword ptr [rbp + 0x9c], xmm1
0x140687038 mov ecx, dword ptr [rbp + 0x3b8]
0x14068703e mov dword ptr [rbp + 0xa0], ecx
0x140687044 movss xmm1, dword ptr [rbp + 0x3bc]
0x14068704c movss dword ptr [rbp + 0xa4], xmm1
0x140687054 mov ecx, dword ptr [rbp + 0x3c0]
0x14068705a mov dword ptr [rbp + 0xa8], ecx
0x140687060 movss xmm1, dword ptr [rbp + 0x3c4]
0x140687068 movss dword ptr [rbp + 0xac], xmm1
0x140687070 mov ecx, dword ptr [rbp + 0x3c8]
0x140687076 mov dword ptr [rbp + 0xb0], ecx
0x14068707c movss xmm1, dword ptr [rbp + 0x3cc]
0x140687084 movss dword ptr [rbp + 0xb4], xmm1
0x14068708c mov ecx, dword ptr [rbp + 0x3d0]
0x140687092 mov dword ptr [rbp + 0xb8], ecx
0x140687098 movss xmm1, dword ptr [rbp + 0x3d4]
0x1406870a0 movss dword ptr [rbp + 0xbc], xmm1
0x1406870a8 mov ecx, dword ptr [rbp + 0x3d8]
0x1406870ae mov dword ptr [rbp + 0xc0], ecx
0x1406870b4 movss xmm1, dword ptr [rbp + 0x3dc]
0x1406870bc movss dword ptr [rbp + 0xc4], xmm1
0x1406870c4 mov ecx, dword ptr [rbp + 0x3e0]
0x1406870ca mov dword ptr [rbp + 0xc8], ecx
0x1406870d0 movss xmm1, dword ptr [rbp + 0x3e4]
0x1406870d8 movss dword ptr [rbp + 0xcc], xmm1
0x1406870e0 mov ecx, dword ptr [rbp + 0x3e8]
0x1406870e6 mov dword ptr [rbp + 0xd0], ecx
0x1406870ec movss xmm1, dword ptr [rbp + 0x3ec]
0x1406870f4 movss dword ptr [rbp + 0xd4], xmm1
0x1406870fc mov ecx, dword ptr [rbp + 0x3f0]
0x140687102 mov dword ptr [rbp + 0xd8], ecx
0x140687108 movss xmm1, dword ptr [rbp + 0x3f4]
0x140687110 movss dword ptr [rbp + 0xdc], xmm1
0x140687118 mov ecx, dword ptr [rbp + 0x3f8]
0x14068711e mov dword ptr [rbp + 0xe0], ecx
0x140687124 movss xmm1, dword ptr [rbp + 0x3fc]
0x14068712c movss dword ptr [rbp + 0xe4], xmm1
0x140687134 mov ecx, dword ptr [rbp + 0x400]
0x14068713a mov dword ptr [rbp + 0xe8], ecx
0x140687140 movss xmm1, dword ptr [rbp + 0x404]
0x140687148 movss dword ptr [rbp + 0xec], xmm1
0x140687150 mov ecx, dword ptr [rbp + 0x408]
0x140687156 mov dword ptr [rbp + 0xf0], ecx
0x14068715c movss xmm1, dword ptr [rbp + 0x40c]
0x140687164 movss dword ptr [rbp + 0xf4], xmm1
0x14068716c mov ecx, dword ptr [rbp + 0x410]
0x140687172 mov dword ptr [rbp + 0xf8], ecx
0x140687178 movss xmm1, dword ptr [rbp + 0x414]
0x140687180 movss dword ptr [rbp + 0xfc], xmm1
0x140687188 mov ecx, dword ptr [rbp + 0x418]
0x14068718e mov dword ptr [rbp + 0x100], ecx
0x140687194 movss xmm1, dword ptr [rbp + 0x41c]
0x14068719c movss dword ptr [rbp + 0x104], xmm1
0x1406871a4 mov ecx, dword ptr [rbp + 0x420]
0x1406871aa mov dword ptr [rbp + 0x108], ecx
0x1406871b0 movss xmm1, dword ptr [rbp + 0x424]
0x1406871b8 movss dword ptr [rbp + 0x10c], xmm1
0x1406871c0 mov ecx, dword ptr [rbp + 0x428]
0x1406871c6 mov dword ptr [rbp + 0x110], ecx
0x1406871cc movss xmm1, dword ptr [rbp + 0x42c]
0x1406871d4 movss dword ptr [rbp + 0x114], xmm1
0x1406871dc mov ecx, dword ptr [rbp + 0x430]
0x1406871e2 mov dword ptr [rbp + 0x118], ecx
0x1406871e8 movss dword ptr [rbp + 0x11c], xmm15
0x1406871f1 mov ecx, dword ptr [rbp + 0x434]
0x1406871f7 mov dword ptr [rbp + 0x120], ecx
0x1406871fd movss dword ptr [rbp + 0x124], xmm6
0x140687205 mov ecx, dword ptr [rbp + 0x438]
0x14068720b mov dword ptr [rbp + 0x128], ecx
0x140687211 movss dword ptr [rbp + 0x12c], xmm7
0x140687219 mov ecx, dword ptr [rbp + 0x43c]
0x14068721f mov dword ptr [rbp + 0x130], ecx
0x140687225 movss dword ptr [rbp + 0x134], xmm8
0x14068722e mov dword ptr [rbp + 0x138], r15d
0x140687235 movss dword ptr [rbp + 0x13c], xmm9
0x14068723e mov dword ptr [rbp + 0x140], r12d
0x140687245 movss dword ptr [rbp + 0x144], xmm10
0x14068724e mov dword ptr [rbp + 0x148], r13d
0x140687255 movss dword ptr [rbp + 0x14c], xmm11
0x14068725e mov dword ptr [rbp + 0x150], edi
0x140687264 movss dword ptr [rbp + 0x154], xmm12
0x14068726d mov dword ptr [rbp + 0x158], ebx
0x140687273 movss dword ptr [rbp + 0x15c], xmm13
0x14068727c mov dword ptr [rbp + 0x160], r14d
0x140687283 movss dword ptr [rbp + 0x164], xmm14
0x14068728c mov dword ptr [rbp + 0x168], eax
0x140687292 movss dword ptr [rbp + 0x16c], xmm0
0x14068729a mov rdx, rbp
0x14068729d mov rcx, rsi
0x1406872a0 call 0x140fd19d0
0x1406872a5 lea rcx, [rsi + 0x38]
0x1406872a9 movabs rdx, 0x8e1bd251962ba784
0x1406872b3 movabs r8, 0x6a782374c0e6529b
0x1406872bd call 0x1400dd6f0
0x1406872c2 test rax, rax
0x1406872c5 je 0x140687599
0x1406872cb cmp qword ptr [rsi + 0x218], rdx
0x1406872d2 jbe 0x140687599
0x1406872d8 mov rax, qword ptr [rsi + 0x210]
0x1406872df mov rcx, qword ptr [rax + rdx*8]
0x1406872e3 test rcx, rcx
0x1406872e6 je 0x140687599
0x1406872ec mov rax, qword ptr [rsi + 0x1e0]
0x1406872f3 not rcx
0x1406872f6 lea rcx, [rcx + rcx*2]
0x1406872fa shl rcx, 5
0x1406872fe cmp qword ptr [rax + rcx + 0x20], 0
0x140687304 je 0x140687599
0x14068730a add rax, rcx
0x14068730d mov rcx, qword ptr [rax + 0x10]
0x140687311 mov edx, dword ptr [rsi + 0x5b0]
0x140687317 mov dword ptr [rax + 0x5c], edx
0x14068731a call 0x140fd50c0
0x14068731f mov rcx, rsi
0x140687322 call 0x140fd3030
0x140687327 mov rcx, rbp
0x14068732a call 0x140f98b20
0x14068732f mov rcx, qword ptr [rip + 0x2fa503a]
0x140687336 lea r8, [rip + 0x2530033]
0x14068733d lea rdx, [rbp + 0x2a0]
0x140687344 call 0x140fdc2a0
0x140687349 mov r8, rdx
0x14068734c mov rcx, rbp
0x14068734f mov rdx, rax
0x140687352 call 0x140fdde70
0x140687357 movups xmm0, xmmword ptr [rbp + 0x60]
0x14068735b movaps xmmword ptr [rbp + 0x330], xmm0
0x140687362 movups xmm0, xmmword ptr [rbp + 0x50]
0x140687366 movaps xmmword ptr [rbp + 0x320], xmm0
0x14068736d movups xmm0, xmmword ptr [rbp + 0x40]
0x140687371 movaps xmmword ptr [rbp + 0x310], xmm0
0x140687378 movups xmm0, xmmword ptr [rbp]
0x14068737c movups xmm1, xmmword ptr [rbp + 0x10]
0x140687380 movups xmm2, xmmword ptr [rbp + 0x20]
0x140687384 movups xmm3, xmmword ptr [rbp + 0x30]
0x140687388 movaps xmmword ptr [rbp + 0x300], xmm3
0x14068738f movaps xmmword ptr [rbp + 0x2f0], xmm2
0x140687396 movaps xmmword ptr [rbp + 0x2e0], xmm1
0x14068739d movaps xmmword ptr [rbp + 0x2d0], xmm0
0x1406873a4 mov byte ptr [rbp + 0x2a0], 6
0x1406873ab mov rcx, qword ptr [rip + 0x2fa4fbe]
0x1406873b2 lea r8, [rip + 0x253a7bf]
0x1406873b9 lea rdx, [rbp + 0x2a0]
0x1406873c0 call 0x140fdc2a0
0x1406873c5 mov r8, rdx
0x1406873c8 lea rcx, [rbp + 0x2d0]
0x1406873cf mov rdx, rax
0x1406873d2 call 0x140fddf80
0x1406873d7 movaps xmm0, xmmword ptr [rbp + 0x330]
0x1406873de movaps xmmword ptr [rbp + 0x60], xmm0
0x1406873e2 movaps xmm0, xmmword ptr [rbp + 0x320]
0x1406873e9 movaps xmmword ptr [rbp + 0x50], xmm0
0x1406873ed movaps xmm0, xmmword ptr [rbp + 0x310]
0x1406873f4 movaps xmmword ptr [rbp + 0x40], xmm0
0x1406873f8 movaps xmm0, xmmword ptr [rbp + 0x2d0]
0x1406873ff movaps xmm1, xmmword ptr [rbp + 0x2e0]
0x140687406 movaps xmm2, xmmword ptr [rbp + 0x2f0]
0x14068740d movaps xmm3, xmmword ptr [rbp + 0x300]
0x140687414 movaps xmmword ptr [rbp + 0x30], xmm3
0x140687418 movaps xmmword ptr [rbp + 0x20], xmm2
0x14068741c movaps xmmword ptr [rbp + 0x10], xmm1
0x140687420 movaps xmmword ptr [rbp], xmm0
0x140687424 mov rdx, rbp
0x140687427 mov rcx, rsi
0x14068742a call 0x140fd2dd0
0x14068742f mov rcx, rbp
0x140687432 call 0x140f98e70
0x140687437 mov byte ptr [rbp + 0x2a0], 0x2c
0x14068743e mov rcx, qword ptr [rip + 0x2fa4f2b]
0x140687445 lea r8, [rip + 0x253a72c]
0x14068744c lea rdx, [rbp + 0x2a0]
0x140687453 call 0x140fdc2a0
0x140687458 mov r8, rdx
0x14068745b mov rcx, rbp
0x14068745e mov rdx, rax
0x140687461 call 0x140fddf80
0x140687466 movups xmm0, xmmword ptr [rbp + 0x60]
0x14068746a movaps xmmword ptr [rbp + 0x330], xmm0
0x140687471 movups xmm0, xmmword ptr [rbp + 0x50]
0x140687475 movaps xmmword ptr [rbp + 0x320], xmm0
0x14068747c movups xmm0, xmmword ptr [rbp + 0x40]
0x140687480 movaps xmmword ptr [rbp + 0x310], xmm0
0x140687487 movups xmm0, xmmword ptr [rbp]
0x14068748b movups xmm1, xmmword ptr [rbp + 0x10]
0x14068748f movups xmm2, xmmword ptr [rbp + 0x20]
0x140687493 movups xmm3, xmmword ptr [rbp + 0x30]
0x140687497 movaps xmmword ptr [rbp + 0x300], xmm3
0x14068749e movaps xmmword ptr [rbp + 0x2f0], xmm2
0x1406874a5 movaps xmmword ptr [rbp + 0x2e0], xmm1
0x1406874ac movaps xmmword ptr [rbp + 0x2d0], xmm0
0x1406874b3 mov rcx, qword ptr [rip + 0x2fa4eb6]
0x1406874ba lea r8, [rip + 0x252f12f]
0x1406874c1 lea rdx, [rbp + 0x2a0]
0x1406874c8 call 0x140fdc2a0
0x1406874cd mov r8, rdx
0x1406874d0 lea rcx, [rbp + 0x2d0]
0x1406874d7 mov rdx, rax
0x1406874da call 0x140fdde70
0x1406874df movaps xmm0, xmmword ptr [rbp + 0x330]
0x1406874e6 movaps xmmword ptr [rbp + 0x60], xmm0
0x1406874ea movaps xmm0, xmmword ptr [rbp + 0x320]
0x1406874f1 movaps xmmword ptr [rbp + 0x50], xmm0
0x1406874f5 movaps xmm0, xmmword ptr [rbp + 0x310]
0x1406874fc movaps xmmword ptr [rbp + 0x40], xmm0
0x140687500 movaps xmm0, xmmword ptr [rbp + 0x2d0]
0x140687507 movaps xmm1, xmmword ptr [rbp + 0x2e0]
0x14068750e movaps xmm2, xmmword ptr [rbp + 0x2f0]
0x140687515 movaps xmm3, xmmword ptr [rbp + 0x300]
0x14068751c movaps xmmword ptr [rbp + 0x30], xmm3
0x140687520 movaps xmmword ptr [rbp + 0x20], xmm2
0x140687524 movaps xmmword ptr [rbp + 0x10], xmm1
0x140687528 movaps xmmword ptr [rbp], xmm0
0x14068752c mov rdx, rbp
0x14068752f mov rcx, rsi
0x140687532 call 0x140fd2dd0
0x140687537 movaps xmm6, xmmword ptr [rbp + 0x450]
0x14068753e movaps xmm7, xmmword ptr [rbp + 0x460]
0x140687545 movaps xmm8, xmmword ptr [rbp + 0x470]
0x14068754d movaps xmm9, xmmword ptr [rbp + 0x480]
0x140687555 movaps xmm10, xmmword ptr [rbp + 0x490]
0x14068755d movaps xmm11, xmmword ptr [rbp + 0x4a0]
0x140687565 movaps xmm12, xmmword ptr [rbp + 0x4b0]
0x14068756d movaps xmm13, xmmword ptr [rbp + 0x4c0]
0x140687575 movaps xmm14, xmmword ptr [rbp + 0x4d0]
0x14068757d movaps xmm15, xmmword ptr [rbp + 0x4e0]
0x140687585 add rsp, 0x578
0x14068758c pop rbx
0x14068758d pop rdi
0x14068758e pop rsi
0x14068758f pop r12
0x140687591 pop r13
0x140687593 pop r14
0x140687595 pop r15
0x140687597 pop rbp
0x140687598 ret 
0x140687599 lea rax, [rip + 0x252deb0]
0x1406875a0 mov qword ptr [rbp + 0x2d0], rax
0x1406875a7 mov qword ptr [rbp + 0x2d8], 0x27
0x1406875b2 lea rax, [rbp + 0x2d0]
0x1406875b9 mov qword ptr [rbp + 0x2a0], rax
0x1406875c0 lea rax, [rip - 0x57d737]
0x1406875c7 mov qword ptr [rbp + 0x2a8], rax
0x1406875ce lea rax, [rip + 0x252dfab]
0x1406875d5 mov qword ptr [rbp], rax
0x1406875d9 mov qword ptr [rbp + 8], 2
0x1406875e1 mov qword ptr [rbp + 0x20], 0
0x1406875e9 lea rax, [rbp + 0x2a0]
0x1406875f0 mov qword ptr [rbp + 0x10], rax
0x1406875f4 mov qword ptr [rbp + 0x18], 1
0x1406875fc lea rdx, [rip + 0x252de35]
0x140687603 mov rcx, rbp
0x140687606 call 0x1428d9390
0x14068760b mov ecx, 8
0x140687610 mov edx, 0x40
0x140687615 call 0x1428d9000
0x14068761a jmp 0x140687628
0x14068761c lea rcx, [rip + 0x252dcb5]
0x140687623 call 0x1428d80d0
0x140687628 ud2 
0x14068762a nop word ptr [rax + rax]
0x140687630 mov qword ptr [rsp + 0x10], rdx
0x140687635 push rbp
0x140687636 push r15
0x140687638 push r14
0x14068763a push r13
0x14068763c push r12
0x14068763e push rsi
0x14068763f push rdi
0x140687640 push rbx
0x140687641 sub rsp, 0xc8
0x140687648 lea rbp, [rdx + 0x80]
0x14068764f movaps xmmword ptr [rsp + 0x20], xmm15
0x140687655 movaps xmmword ptr [rsp + 0x30], xmm14
0x14068765b movaps xmmword ptr [rsp + 0x40], xmm13
0x140687661 movaps xmmword ptr [rsp + 0x50], xmm12
0x140687667 movaps xmmword ptr [rsp + 0x60], xmm11
0x14068766d movaps xmmword ptr [rsp + 0x70], xmm10
0x140687673 movaps xmmword ptr [rsp + 0x80], xmm9
0x14068767c movaps xmmword ptr [rsp + 0x90], xmm8
0x140687685 movaps xmmword ptr [rsp + 0xa0], xmm7
0x14068768d movaps xmmword ptr [rsp + 0xb0], xmm6
0x140687695 lea rcx, [rbp - 0x30]
0x140687699 mov r8d, 0xc0
0x14068769f mov r9d, 0x10
0x1406876a5 lea rdx, [rbp - 0x10]
0x1406876a9 call 0x140052d80
0x1406876ae movaps xmm6, xmmword ptr [rsp + 0xb0]
0x1406876b6 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x1406876be movaps xmm8, xmmword ptr [rsp + 0x90]
0x1406876c7 movaps xmm9, xmmword ptr [rsp + 0x80]
0x1406876d0 movaps xmm10, xmmword ptr [rsp + 0x70]
0x1406876d6 movaps xmm11, xmmword ptr [rsp + 0x60]
0x1406876dc movaps xmm12, xmmword ptr [rsp + 0x50]
0x1406876e2 movaps xmm13, xmmword ptr [rsp + 0x40]
0x1406876e8 movaps xmm14, xmmword ptr [rsp + 0x30]
0x1406876ee movaps xmm15, xmmword ptr [rsp + 0x20]
0x1406876f4 add rsp, 0xc8
0x1406876fb pop rbx
0x1406876fc pop rdi
0x1406876fd pop rsi
0x1406876fe pop r12
0x140687700 pop r13
0x140687702 pop r14
0x140687704 pop r15
0x140687706 pop rbp
0x140687707 ret 
0x140687708 nop dword ptr [rax + rax]
0x140687710 mov qword ptr [rsp + 0x10], rdx
0x140687715 push rbp
0x140687716 push r15
0x140687718 push r14
0x14068771a push r13
0x14068771c push r12
0x14068771e push rsi
0x14068771f push rdi
0x140687720 push rbx
0x140687721 sub rsp, 0xc8
0x140687728 lea rbp, [rdx + 0x80]
0x14068772f movaps xmmword ptr [rsp + 0x20], xmm15
0x140687735 movaps xmmword ptr [rsp + 0x30], xmm14
0x14068773b movaps xmmword ptr [rsp + 0x40], xmm13
0x140687741 movaps xmmword ptr [rsp + 0x50], xmm12
0x140687747 movaps xmmword ptr [rsp + 0x60], xmm11
0x14068774d movaps xmmword ptr [rsp + 0x70], xmm10
0x140687753 movaps xmmword ptr [rsp + 0x80], xmm9
0x14068775c movaps xmmword ptr [rsp + 0x90], xmm8
0x140687765 movaps xmmword ptr [rsp + 0xa0], xmm7
0x14068776d movaps xmmword ptr [rsp + 0xb0], xmm6
0x140687775 lea rcx, [rbp - 0x60]
0x140687779 call 0x140149c70
0x14068777e movaps xmm6, xmmword ptr [rsp + 0xb0]
0x140687786 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x14068778e movaps xmm8, xmmword ptr [rsp + 0x90]
0x140687797 movaps xmm9, xmmword ptr [rsp + 0x80]
0x1406877a0 movaps xmm10, xmmword ptr [rsp + 0x70]
0x1406877a6 movaps xmm11, xmmword ptr [rsp + 0x60]
0x1406877ac movaps xmm12, xmmword ptr [rsp + 0x50]
0x1406877b2 movaps xmm13, xmmword ptr [rsp + 0x40]
0x1406877b8 movaps xmm14, xmmword ptr [rsp + 0x30]
0x1406877be movaps xmm15, xmmword ptr [rsp + 0x20]
0x1406877c4 add rsp, 0xc8
0x1406877cb pop rbx
0x1406877cc pop rdi
0x1406877cd pop rsi
0x1406877ce pop r12
0x1406877d0 pop r13
0x1406877d2 pop r14
0x1406877d4 pop r15
0x1406877d6 pop rbp
0x1406877d7 ret 
0x1406877d8 nop dword ptr [rax + rax]
0x1406877e0 mov qword ptr [rsp + 0x10], rdx
0x1406877e5 push rbp
0x1406877e6 push r15
0x1406877e8 push r14
0x1406877ea push r13
0x1406877ec push r12
0x1406877ee push rsi
0x1406877ef push rdi
0x1406877f0 push rbx
0x1406877f1 sub rsp, 0xc8
0x1406877f8 lea rbp, [rdx + 0x80]
0x1406877ff movaps xmmword ptr [rsp + 0x20], xmm15
0x140687805 movaps xmmword ptr [rsp + 0x30], xmm14
0x14068780b movaps xmmword ptr [rsp + 0x40], xmm13
0x140687811 movaps xmmword ptr [rsp + 0x50], xmm12
0x140687817 movaps xmmword ptr [rsp + 0x60], xmm11
0x14068781d movaps xmmword ptr [rsp + 0x70], xmm10
0x140687823 movaps xmmword ptr [rsp + 0x80], xmm9
0x14068782c movaps xmmword ptr [rsp + 0x90], xmm8
0x140687835 movaps xmmword ptr [rsp + 0xa0], xmm7
0x14068783d movaps xmmword ptr [rsp + 0xb0], xmm6
0x140687845 lea rcx, [rbp + 0x2a0]
0x14068784c mov r8d, 0xc0
0x140687852 mov r9d, 0x10
0x140687858 lea rdx, [rbp + 0x2c0]
0x14068785f call 0x140052d80
0x140687864 movaps xmm6, xmmword ptr [rsp + 0xb0]
0x14068786c movaps xmm7, xmmword ptr [rsp + 0xa0]
0x140687874 movaps xmm8, xmmword ptr [rsp + 0x90]
0x14068787d movaps xmm9, xmmword ptr [rsp + 0x80]
0x140687886 movaps xmm10, xmmword ptr [rsp + 0x70]
0x14068788c movaps xmm11, xmmword ptr [rsp + 0x60]
0x140687892 movaps xmm12, xmmword ptr [rsp + 0x50]
0x140687898 movaps xmm13, xmmword ptr [rsp + 0x40]
0x14068789e movaps xmm14, xmmword ptr [rsp + 0x30]
0x1406878a4 movaps xmm15, xmmword ptr [rsp + 0x20]
0x1406878aa add rsp, 0xc8
0x1406878b1 pop rbx
0x1406878b2 pop rdi
0x1406878b3 pop rsi
0x1406878b4 pop r12
0x1406878b6 pop r13
0x1406878b8 pop r14
0x1406878ba pop r15
0x1406878bc pop rbp
0x1406878bd ret 
0x1406878be nop 
0x1406878c0 mov qword ptr [rsp + 0x10], rdx
0x1406878c5 push rbp
0x1406878c6 push r15
0x1406878c8 push r14
0x1406878ca push r13
0x1406878cc push r12
0x1406878ce push rsi
0x1406878cf push rdi
0x1406878d0 push rbx
0x1406878d1 sub rsp, 0xc8
0x1406878d8 lea rbp, [rdx + 0x80]
0x1406878df movaps xmmword ptr [rsp + 0x20], xmm15
0x1406878e5 movaps xmmword ptr [rsp + 0x30], xmm14
0x1406878eb movaps xmmword ptr [rsp + 0x40], xmm13
0x1406878f1 movaps xmmword ptr [rsp + 0x50], xmm12
0x1406878f7 movaps xmmword ptr [rsp + 0x60], xmm11
0x1406878fd movaps xmmword ptr [rsp + 0x70], xmm10
0x140687903 movaps xmmword ptr [rsp + 0x80], xmm9
0x14068790c movaps xmmword ptr [rsp + 0x90], xmm8
0x140687915 movaps xmmword ptr [rsp + 0xa0], xmm7
0x14068791d movaps xmmword ptr [rsp + 0xb0], xmm6
0x140687925 mov edx, 0x40
0x14068792a mov r8d, 8
0x140687930 mov rcx, qword ptr [rbp + 0x440]
0x140687937 call 0x140613c20
0x14068793c movaps xmm6, xmmword ptr [rsp + 0xb0]
0x140687944 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x14068794c movaps xmm8, xmmword ptr [rsp + 0x90]
0x140687955 movaps xmm9, xmmword ptr [rsp + 0x80]
0x14068795e movaps xmm10, xmmword ptr [rsp + 0x70]
0x140687964 movaps xmm11, xmmword ptr [rsp + 0x60]
0x14068796a movaps xmm12, xmmword ptr [rsp + 0x50]
0x140687970 movaps xmm13, xmmword ptr [rsp + 0x40]
0x140687976 movaps xmm14, xmmword ptr [rsp + 0x30]
0x14068797c movaps xmm15, xmmword ptr [rsp + 0x20]
0x140687982 add rsp, 0xc8
0x140687989 pop rbx
0x14068798a pop rdi
0x14068798b pop rsi
0x14068798c pop r12
0x14068798e pop r13
0x140687990 pop r14
0x140687992 pop r15
0x140687994 pop rbp
0x140687995 ret 
0x140687996 nop word ptr cs:[rax + rax]
0x1406879a0 mov qword ptr [rsp + 0x10], rdx
0x1406879a5 push rbp
0x1406879a6 push r15
0x1406879a8 push r14
0x1406879aa push r13
0x1406879ac push r12
0x1406879ae push rsi
0x1406879af push rdi
0x1406879b0 push rbx
0x1406879b1 sub rsp, 0xc8
0x1406879b8 lea rbp, [rdx + 0x80]
0x1406879bf movaps xmmword ptr [rsp + 0x20], xmm15
0x1406879c5 movaps xmmword ptr [rsp + 0x30], xmm14
0x1406879cb movaps xmmword ptr [rsp + 0x40], xmm13
0x1406879d1 movaps xmmword ptr [rsp + 0x50], xmm12
0x1406879d7 movaps xmmword ptr [rsp + 0x60], xmm11
0x1406879dd movaps xmmword ptr [rsp + 0x70], xmm10
0x1406879e3 movaps xmmword ptr [rsp + 0x80], xmm9
0x1406879ec movaps xmmword ptr [rsp + 0x90], xmm8
0x1406879f5 movaps xmmword ptr [rsp + 0xa0], xmm7
0x1406879fd movaps xmmword ptr [rsp + 0xb0], xmm6
0x140687a05 lea rcx, [rbp + 0x288]
0x140687a0c call 0x140faa130
0x140687a11 lea rcx, [rbp + 0x2d0]
0x140687a18 mov r8d, 0x20
0x140687a1e mov r9d, 0x10
0x140687a24 lea rdx, [rbp + 0x2f0]
0x140687a2b call 0x14004f440
0x140687a30 movaps xmm6, xmmword ptr [rsp + 0xb0]
0x140687a38 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x140687a40 movaps xmm8, xmmword ptr [rsp + 0x90]
0x140687a49 movaps xmm9, xmmword ptr [rsp + 0x80]
0x140687a52 movaps xmm10, xmmword ptr [rsp + 0x70]
0x140687a58 movaps xmm11, xmmword ptr [rsp + 0x60]
0x140687a5e movaps xmm12, xmmword ptr [rsp + 0x50]
0x140687a64 movaps xmm13, xmmword ptr [rsp + 0x40]
0x140687a6a movaps xmm14, xmmword ptr [rsp + 0x30]
0x140687a70 movaps xmm15, xmmword ptr [rsp + 0x20]
0x140687a76 add rsp, 0xc8
0x140687a7d pop rbx
0x140687a7e pop rdi
0x140687a7f pop rsi
0x140687a80 pop r12
0x140687a82 pop r13
0x140687a84 pop r14
0x140687a86 pop r15
0x140687a88 pop rbp
0x140687a89 ret 
0x140687a8a nop word ptr [rax + rax]
0x140687a90 mov qword ptr [rsp + 0x10], rdx
0x140687a95 push rbp
0x140687a96 push r15
0x140687a98 push r14
0x140687a9a push r13
0x140687a9c push r12
0x140687a9e push rsi
0x140687a9f push rdi
0x140687aa0 push rbx
0x140687aa1 sub rsp, 0xc8
0x140687aa8 lea rbp, [rdx + 0x80]
0x140687aaf movaps xmmword ptr [rsp + 0x20], xmm15
0x140687ab5 movaps xmmword ptr [rsp + 0x30], xmm14
0x140687abb movaps xmmword ptr [rsp + 0x40], xmm13
0x140687ac1 movaps xmmword ptr [rsp + 0x50], xmm12
0x140687ac7 movaps xmmword ptr [rsp + 0x60], xmm11
0x140687acd movaps xmmword ptr [rsp + 0x70], xmm10
0x140687ad3 movaps xmmword ptr [rsp + 0x80], xmm9
0x140687adc movaps xmmword ptr [rsp + 0x90], xmm8
0x140687ae5 movaps xmmword ptr [rsp + 0xa0], xmm7
0x140687aed movaps xmmword ptr [rsp + 0xb0], xmm6
0x140687af5 mov rax, qword ptr [rbp + 0x440]
0x140687afc lock dec qword ptr [rax]
0x140687b00 jne 0x140687b0e
0x140687b02 lea rcx, [rbp + 0x280]
0x140687b09 call 0x140b8e120
0x140687b0e movaps xmm6, xmmword ptr [rsp + 0xb0]
0x140687b16 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x140687b1e movaps xmm8, xmmword ptr [rsp + 0x90]
0x140687b27 movaps xmm9, xmmword ptr [rsp + 0x80]
0x140687b30 movaps xmm10, xmmword ptr [rsp + 0x70]
0x140687b36 movaps xmm11, xmmword ptr [rsp + 0x60]
0x140687b3c movaps xmm12, xmmword ptr [rsp + 0x50]
0x140687b42 movaps xmm13, xmmword ptr [rsp + 0x40]
0x140687b48 movaps xmm14, xmmword ptr [rsp + 0x30]
0x140687b4e movaps xmm15, xmmword ptr [rsp + 0x20]
0x140687b54 add rsp, 0xc8
0x140687b5b pop rbx
0x140687b5c pop rdi
0x140687b5d pop rsi
0x140687b5e pop r12
0x140687b60 pop r13
0x140687b62 pop r14
0x140687b64 pop r15
0x140687b66 pop rbp
0x140687b67 ret 
0x140687b68 nop dword ptr [rax + rax]
0x140687b70 mov qword ptr [rsp + 0x10], rdx
0x140687b75 push rbp
0x140687b76 push r15
0x140687b78 push r14
0x140687b7a push r13
0x140687b7c push r12
0x140687b7e push rsi
0x140687b7f push rdi
0x140687b80 push rbx
0x140687b81 sub rsp, 0xc8
0x140687b88 lea rbp, [rdx + 0x80]
0x140687b8f movaps xmmword ptr [rsp + 0x20], xmm15
0x140687b95 movaps xmmword ptr [rsp + 0x30], xmm14
0x140687b9b movaps xmmword ptr [rsp + 0x40], xmm13
0x140687ba1 movaps xmmword ptr [rsp + 0x50], xmm12
0x140687ba7 movaps xmmword ptr [rsp + 0x60], xmm11
0x140687bad movaps xmmword ptr [rsp + 0x70], xmm10
0x140687bb3 movaps xmmword ptr [rsp + 0x80], xmm9
0x140687bbc movaps xmmword ptr [rsp + 0x90], xmm8
0x140687bc5 movaps xmmword ptr [rsp + 0xa0], xmm7
0x140687bcd movaps xmmword ptr [rsp + 0xb0], xmm6
0x140687bd5 mov rcx, rbp
0x140687bd8 call 0x140029880
0x140687bdd movaps xmm6, xmmword ptr [rsp + 0xb0]
0x140687be5 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x140687bed movaps xmm8, xmmword ptr [rsp + 0x90]
0x140687bf6 movaps xmm9, xmmword ptr [rsp + 0x80]
0x140687bff movaps xmm10, xmmword ptr [rsp + 0x70]
0x140687c05 movaps xmm11, xmmword ptr [rsp + 0x60]
0x140687c0b movaps xmm12, xmmword ptr [rsp + 0x50]
0x140687c11 movaps xmm13, xmmword ptr [rsp + 0x40]
0x140687c17 movaps xmm14, xmmword ptr [rsp + 0x30]
0x140687c1d movaps xmm15, xmmword ptr [rsp + 0x20]
0x140687c23 add rsp, 0xc8
0x140687c2a pop rbx
0x140687c2b pop rdi
0x140687c2c pop rsi
0x140687c2d pop r12
0x140687c2f pop r13
0x140687c31 pop r14
0x140687c33 pop r15
0x140687c35 pop rbp
0x140687c36 ret 
0x140687c37 nop word ptr [rax + rax]
0x140687c40 mov qword ptr [rsp + 0x10], rdx
0x140687c45 push rbp
0x140687c46 push r15
0x140687c48 push r14
0x140687c4a push r13
0x140687c4c push r12
0x140687c4e push rsi
0x140687c4f push rdi
0x140687c50 push rbx
0x140687c51 sub rsp, 0xc8
0x140687c58 lea rbp, [rdx + 0x80]
0x140687c5f movaps xmmword ptr [rsp + 0x20], xmm15
0x140687c65 movaps xmmword ptr [rsp + 0x30], xmm14
0x140687c6b movaps xmmword ptr [rsp + 0x40], xmm13
0x140687c71 movaps xmmword ptr [rsp + 0x50], xmm12
0x140687c77 movaps xmmword ptr [rsp + 0x60], xmm11
0x140687c7d movaps xmmword ptr [rsp + 0x70], xmm10
0x140687c83 movaps xmmword ptr [rsp + 0x80], xmm9
0x140687c8c movaps xmmword ptr [rsp + 0x90], xmm8
0x140687c95 movaps xmmword ptr [rsp + 0xa0], xmm7
0x140687c9d movaps xmmword ptr [rsp + 0xb0], xmm6
0x140687ca5 lea rcx, [rbp + 0x2d0]
0x140687cac call 0x140029880
0x140687cb1 movaps xmm6, xmmword ptr [rsp + 0xb0]
0x140687cb9 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x140687cc1 movaps xmm8, xmmword ptr [rsp + 0x90]
0x140687cca movaps xmm9, xmmword ptr [rsp + 0x80]
0x140687cd3 movaps xmm10, xmmword ptr [rsp + 0x70]
0x140687cd9 movaps xmm11, xmmword ptr [rsp + 0x60]
0x140687cdf movaps xmm12, xmmword ptr [rsp + 0x50]
0x140687ce5 movaps xmm13, xmmword ptr [rsp + 0x40]
0x140687ceb movaps xmm14, xmmword ptr [rsp + 0x30]
0x140687cf1 movaps xmm15, xmmword ptr [rsp + 0x20]
0x140687cf7 add rsp, 0xc8
0x140687cfe pop rbx
0x140687cff pop rdi
0x140687d00 pop rsi
0x140687d01 pop r12
0x140687d03 pop r13
0x140687d05 pop r14
0x140687d07 pop r15
0x140687d09 pop rbp
0x140687d0a ret 
0x140687d0b nop dword ptr [rax + rax]
0x140687d10 mov qword ptr [rsp + 0x10], rdx
0x140687d15 push rbp
0x140687d16 push r15
0x140687d18 push r14
0x140687d1a push r13
0x140687d1c push r12
0x140687d1e push rsi
0x140687d1f push rdi
0x140687d20 push rbx
0x140687d21 sub rsp, 0xc8
0x140687d28 lea rbp, [rdx + 0x80]
0x140687d2f movaps xmmword ptr [rsp + 0x20], xmm15
0x140687d35 movaps xmmword ptr [rsp + 0x30], xmm14
0x140687d3b movaps xmmword ptr [rsp + 0x40], xmm13
0x140687d41 movaps xmmword ptr [rsp + 0x50], xmm12
0x140687d47 movaps xmmword ptr [rsp + 0x60], xmm11
0x140687d4d movaps xmmword ptr [rsp + 0x70], xmm10
0x140687d53 movaps xmmword ptr [rsp + 0x80], xmm9
0x140687d5c movaps xmmword ptr [rsp + 0x90], xmm8
0x140687d65 movaps xmmword ptr [rsp + 0xa0], xmm7
0x140687d6d movaps xmmword ptr [rsp + 0xb0], xmm6
0x140687d75 mov rcx, rbp
0x140687d78 call 0x140029880
0x140687d7d movaps xmm6, xmmword ptr [rsp + 0xb0]
0x140687d85 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x140687d8d movaps xmm8, xmmword ptr [rsp + 0x90]
0x140687d96 movaps xmm9, xmmword ptr [rsp + 0x80]
0x140687d9f movaps xmm10, xmmword ptr [rsp + 0x70]
0x140687da5 movaps xmm11, xmmword ptr [rsp + 0x60]
0x140687dab movaps xmm12, xmmword ptr [rsp + 0x50]
0x140687db1 movaps xmm13, xmmword ptr [rsp + 0x40]
0x140687db7 movaps xmm14, xmmword ptr [rsp + 0x30]
0x140687dbd movaps xmm15, xmmword ptr [rsp + 0x20]
0x140687dc3 add rsp, 0xc8
0x140687dca pop rbx
0x140687dcb pop rdi
0x140687dcc pop rsi
0x140687dcd pop r12
0x140687dcf pop r13
0x140687dd1 pop r14
0x140687dd3 pop r15
0x140687dd5 pop rbp
0x140687dd6 ret 
0x140687dd7 nop word ptr [rax + rax]
0x140687de0 mov qword ptr [rsp + 0x10], rdx
0x140687de5 push rbp
0x140687de6 push r15
0x140687de8 push r14
0x140687dea push r13
0x140687dec push r12
0x140687dee push rsi
0x140687def push rdi
0x140687df0 push rbx
0x140687df1 sub rsp, 0xc8
0x140687df8 lea rbp, [rdx + 0x80]
0x140687dff movaps xmmword ptr [rsp + 0x20], xmm15
0x140687e05 movaps xmmword ptr [rsp + 0x30], xmm14
0x140687e0b movaps xmmword ptr [rsp + 0x40], xmm13
0x140687e11 movaps xmmword ptr [rsp + 0x50], xmm12
0x140687e17 movaps xmmword ptr [rsp + 0x60], xmm11
0x140687e1d movaps xmmword ptr [rsp + 0x70], xmm10
0x140687e23 movaps xmmword ptr [rsp + 0x80], xmm9
0x140687e2c movaps xmmword ptr [rsp + 0x90], xmm8
0x140687e35 movaps xmmword ptr [rsp + 0xa0], xmm7
0x140687e3d movaps xmmword ptr [rsp + 0xb0], xmm6
0x140687e45 lea rcx, [rbp + 0x2d0]
0x140687e4c call 0x140029880
0x140687e51 movaps xmm6, xmmword ptr [rsp + 0xb0]
0x140687e59 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x140687e61 movaps xmm8, xmmword ptr [rsp + 0x90]
0x140687e6a movaps xmm9, xmmword ptr [rsp + 0x80]
0x140687e73 movaps xmm10, xmmword ptr [rsp + 0x70]
0x140687e79 movaps xmm11, xmmword ptr [rsp + 0x60]
0x140687e7f movaps xmm12, xmmword ptr [rsp + 0x50]
0x140687e85 movaps xmm13, xmmword ptr [rsp + 0x40]
0x140687e8b movaps xmm14, xmmword ptr [rsp + 0x30]
0x140687e91 movaps xmm15, xmmword ptr [rsp + 0x20]
0x140687e97 add rsp, 0xc8
0x140687e9e pop rbx
0x140687e9f pop rdi
0x140687ea0 pop rsi
0x140687ea1 pop r12
0x140687ea3 pop r13
0x140687ea5 pop r14
0x140687ea7 pop r15
0x140687ea9 pop rbp
0x140687eaa ret 
