0x140fed8a0 push rbp
0x140fed8a1 push rsi
0x140fed8a2 push rdi
0x140fed8a3 push rbx
0x140fed8a4 sub rsp, 0x548
0x140fed8ab lea rbp, [rsp + 0x80]
0x140fed8b3 mov qword ptr [rbp + 0x4c0], 0xfffffffffffffffe
0x140fed8be mov rsi, rcx
0x140fed8c1 movzx eax, byte ptr [rip + 0x2647341]
0x140fed8c8 mov ecx, 0x2a0
0x140fed8cd mov edx, 8
0x140fed8d2 call 0x140613c10
0x140fed8d7 test rax, rax
0x140fed8da je 0x140fedeec
0x140fed8e0 mov qword ptr [rbp + 0x4b8], rax
0x140fed8e7 xorps xmm0, xmm0
0x140fed8ea movups xmmword ptr [rbp + 0x268], xmm0
0x140fed8f1 movups xmmword ptr [rbp + 0x280], xmm0
0x140fed8f8 movups xmmword ptr [rbp + 0x298], xmm0
0x140fed8ff movups xmmword ptr [rbp + 0x2b0], xmm0
0x140fed906 movups xmmword ptr [rbp + 0x2c8], xmm0
0x140fed90d mov dword ptr [rbp + 0x2d8], 0
0x140fed917 movups xmmword ptr [rbp + 0x300], xmm0
0x140fed91e movups xmmword ptr [rbp + 0x318], xmm0
0x140fed925 movups xmmword ptr [rbp + 0x330], xmm0
0x140fed92c movups xmmword ptr [rbp + 0x348], xmm0
0x140fed933 movups xmmword ptr [rbp + 0x360], xmm0
0x140fed93a mov dword ptr [rbp + 0x370], 0
0x140fed944 mov qword ptr [rbp + 0x248], 0
0x140fed94f mov qword ptr [rbp + 0x250], 8
0x140fed95a mov qword ptr [rbp + 0x258], 0
0x140fed965 mov qword ptr [rbp + 0x260], 0x10
0x140fed970 mov qword ptr [rbp + 0x278], 0x10
0x140fed97b mov qword ptr [rbp + 0x290], 0x10
0x140fed986 mov qword ptr [rbp + 0x2a8], 0x10
0x140fed991 mov qword ptr [rbp + 0x2c0], 0x10
0x140fed99c movabs rbx, 0x8000000000000000
0x140fed9a6 mov qword ptr [rbp + 0x2e0], rbx
0x140fed9ad lea rax, [rip + 0x1bd59db]
0x140fed9b4 mov qword ptr [rbp + 0x2e8], rax
0x140fed9bb mov qword ptr [rbp + 0x2f0], 0x25
0x140fed9c6 mov qword ptr [rbp + 0x2f8], 0x10
0x140fed9d1 mov qword ptr [rbp + 0x310], 0x10
0x140fed9dc mov qword ptr [rbp + 0x328], 0x10
0x140fed9e7 mov qword ptr [rbp + 0x340], 0x10
0x140fed9f2 mov qword ptr [rbp + 0x358], 0x10
0x140fed9fd mov dword ptr [rbp + 0x378], 0
0x140feda07 mov word ptr [rbp + 0x37c], 1
0x140feda10 mov qword ptr [rbp + 0x480], rbx
0x140feda17 mov dword ptr [rbp + 0x4a8], 0
0x140feda21 movzx eax, byte ptr [rip + 0x26471e1]
0x140feda28 mov ecx, 0x268
0x140feda2d mov edx, 8
0x140feda32 call 0x140613c10
0x140feda37 test rax, rax
0x140feda3a je 0x140fedefb
0x140feda40 mov rdi, rax
0x140feda43 lea rdx, [rbp + 0x248]
0x140feda4a mov r8d, 0x268
0x140feda50 mov rcx, rax
0x140feda53 call 0x142923c00
0x140feda58 lea r8, [rip + 0x1bd5809]
0x140feda5f lea rcx, [rbp + 0x168]
0x140feda66 mov rdx, rdi
0x140feda69 call 0x140ce4b10
0x140feda6e lea rcx, [rbp + 0xf8]
0x140feda75 call 0x141e9f890
0x140feda7a lea rcx, [rbp + 0x88]
0x140feda81 call 0x141e9fa40
0x140feda86 lea rcx, [rbp + 0x18]
0x140feda8a call 0x141e9f6e0
0x140feda8f lea rcx, [rbp - 0x58]
0x140feda93 call 0x141ea83d0
0x140feda98 xorps xmm0, xmm0
0x140feda9b movups xmmword ptr [rbp + 0x268], xmm0
0x140fedaa2 movups xmmword ptr [rbp + 0x280], xmm0
0x140fedaa9 movups xmmword ptr [rbp + 0x298], xmm0
0x140fedab0 movups xmmword ptr [rbp + 0x2b0], xmm0
0x140fedab7 movups xmmword ptr [rbp + 0x2c8], xmm0
0x140fedabe mov dword ptr [rbp + 0x2d8], 0
0x140fedac8 movups xmmword ptr [rbp + 0x300], xmm0
0x140fedacf movups xmmword ptr [rbp + 0x318], xmm0
0x140fedad6 movups xmmword ptr [rbp + 0x330], xmm0
0x140fedadd movups xmmword ptr [rbp + 0x348], xmm0
0x140fedae4 movups xmmword ptr [rbp + 0x360], xmm0
0x140fedaeb mov dword ptr [rbp + 0x370], 0
0x140fedaf5 mov qword ptr [rbp + 0x248], 0
0x140fedb00 mov qword ptr [rbp + 0x250], 8
0x140fedb0b mov qword ptr [rbp + 0x258], 0
0x140fedb16 mov qword ptr [rbp + 0x260], 0x10
0x140fedb21 mov qword ptr [rbp + 0x278], 0x10
0x140fedb2c mov qword ptr [rbp + 0x290], 0x10
0x140fedb37 mov qword ptr [rbp + 0x2a8], 0x10
0x140fedb42 mov qword ptr [rbp + 0x2c0], 0x10
0x140fedb4d mov qword ptr [rbp + 0x2e0], rbx
0x140fedb54 lea rax, [rip + 0x1bd57e7]
0x140fedb5b mov qword ptr [rbp + 0x2e8], rax
0x140fedb62 mov qword ptr [rbp + 0x2f0], 0x24
0x140fedb6d mov qword ptr [rbp + 0x2f8], 0x10
0x140fedb78 mov qword ptr [rbp + 0x310], 0x10
0x140fedb83 mov qword ptr [rbp + 0x328], 0x10
0x140fedb8e mov qword ptr [rbp + 0x340], 0x10
0x140fedb99 mov qword ptr [rbp + 0x358], 0x10
0x140fedba4 mov dword ptr [rbp + 0x378], 0
0x140fedbae mov word ptr [rbp + 0x37c], 1
0x140fedbb7 mov qword ptr [rbp + 0x480], rbx
0x140fedbbe mov dword ptr [rbp + 0x4b0], 0
0x140fedbc8 movzx eax, byte ptr [rip + 0x264703a]
0x140fedbcf mov ecx, 0x270
0x140fedbd4 mov edx, 8
0x140fedbd9 call 0x140613c10
0x140fedbde test rax, rax
0x140fedbe1 je 0x140fedf0c
0x140fedbe7 mov rdi, rax
0x140fedbea lea rdx, [rbp + 0x248]
0x140fedbf1 mov r8d, 0x270
0x140fedbf7 mov rcx, rax
0x140fedbfa call 0x142923c00
0x140fedbff lea r8, [rip + 0x1bd55aa]
0x140fedc06 lea rcx, [rbp + 0x1d8]
0x140fedc0d mov rdx, rdi
0x140fedc10 call 0x140ce4b10
0x140fedc15 movups xmm0, xmmword ptr [rbp + 0x1c8]
0x140fedc1c mov rax, qword ptr [rbp + 0x4b8]
0x140fedc23 movups xmmword ptr [rax + 0x60], xmm0
0x140fedc27 movups xmm0, xmmword ptr [rbp + 0x1b8]
0x140fedc2e movups xmmword ptr [rax + 0x50], xmm0
0x140fedc32 movups xmm0, xmmword ptr [rbp + 0x1a8]
0x140fedc39 movups xmmword ptr [rax + 0x40], xmm0
0x140fedc3d movups xmm0, xmmword ptr [rbp + 0x168]
0x140fedc44 movups xmm1, xmmword ptr [rbp + 0x178]
0x140fedc4b movups xmm2, xmmword ptr [rbp + 0x188]
0x140fedc52 movups xmm3, xmmword ptr [rbp + 0x198]
0x140fedc59 movups xmmword ptr [rax + 0x30], xmm3
0x140fedc5d movups xmmword ptr [rax + 0x20], xmm2
0x140fedc61 movups xmmword ptr [rax + 0x10], xmm1
0x140fedc65 movups xmmword ptr [rax], xmm0
0x140fedc68 movups xmm0, xmmword ptr [rbp + 0xf8]
0x140fedc6f movups xmm1, xmmword ptr [rbp + 0x108]
0x140fedc76 movups xmm2, xmmword ptr [rbp + 0x118]
0x140fedc7d movups xmm3, xmmword ptr [rbp + 0x128]
0x140fedc84 movups xmmword ptr [rax + 0x70], xmm0
0x140fedc88 movups xmmword ptr [rax + 0x80], xmm1
0x140fedc8f movups xmmword ptr [rax + 0x90], xmm2
0x140fedc96 movups xmmword ptr [rax + 0xa0], xmm3
0x140fedc9d movups xmm0, xmmword ptr [rbp + 0x138]
0x140fedca4 movups xmmword ptr [rax + 0xb0], xmm0
0x140fedcab movups xmm0, xmmword ptr [rbp + 0x148]
0x140fedcb2 movups xmmword ptr [rax + 0xc0], xmm0
0x140fedcb9 movups xmm0, xmmword ptr [rbp + 0x158]
0x140fedcc0 movups xmmword ptr [rax + 0xd0], xmm0
0x140fedcc7 movups xmm0, xmmword ptr [rbp + 0xe8]
0x140fedcce movups xmmword ptr [rax + 0x140], xmm0
0x140fedcd5 movups xmm0, xmmword ptr [rbp + 0xd8]
0x140fedcdc movups xmmword ptr [rax + 0x130], xmm0
0x140fedce3 movups xmm0, xmmword ptr [rbp + 0xc8]
0x140fedcea movups xmmword ptr [rax + 0x120], xmm0
0x140fedcf1 movups xmm0, xmmword ptr [rbp + 0x88]
0x140fedcf8 movups xmm1, xmmword ptr [rbp + 0x98]
0x140fedcff movups xmm2, xmmword ptr [rbp + 0xa8]
0x140fedd06 movups xmm3, xmmword ptr [rbp + 0xb8]
0x140fedd0d movups xmmword ptr [rax + 0x110], xmm3
0x140fedd14 movups xmmword ptr [rax + 0x100], xmm2
0x140fedd1b movups xmmword ptr [rax + 0xf0], xmm1
0x140fedd22 movups xmmword ptr [rax + 0xe0], xmm0
0x140fedd29 movups xmm0, xmmword ptr [rbp + 0x18]
0x140fedd2d movups xmm1, xmmword ptr [rbp + 0x28]
0x140fedd31 movups xmm2, xmmword ptr [rbp + 0x38]
0x140fedd35 movups xmm3, xmmword ptr [rbp + 0x48]
0x140fedd39 movups xmmword ptr [rax + 0x150], xmm0
0x140fedd40 movups xmmword ptr [rax + 0x160], xmm1
0x140fedd47 movups xmmword ptr [rax + 0x170], xmm2
0x140fedd4e movups xmmword ptr [rax + 0x180], xmm3
0x140fedd55 movups xmm0, xmmword ptr [rbp + 0x58]
0x140fedd59 movups xmmword ptr [rax + 0x190], xmm0
0x140fedd60 movups xmm0, xmmword ptr [rbp + 0x68]
0x140fedd64 movups xmmword ptr [rax + 0x1a0], xmm0
0x140fedd6b movups xmm0, xmmword ptr [rbp + 0x78]
0x140fedd6f movups xmmword ptr [rax + 0x1b0], xmm0
0x140fedd76 movups xmm0, xmmword ptr [rbp - 0x58]
0x140fedd7a movups xmm1, xmmword ptr [rbp - 0x48]
0x140fedd7e movups xmm2, xmmword ptr [rbp - 0x38]
0x140fedd82 movups xmm3, xmmword ptr [rbp - 0x28]
0x140fedd86 movups xmmword ptr [rax + 0x1c0], xmm0
0x140fedd8d movups xmmword ptr [rax + 0x1d0], xmm1
0x140fedd94 movups xmmword ptr [rax + 0x1e0], xmm2
0x140fedd9b movups xmmword ptr [rax + 0x1f0], xmm3
0x140fedda2 movups xmm0, xmmword ptr [rbp - 0x18]
0x140fedda6 movups xmmword ptr [rax + 0x200], xmm0
0x140feddad movups xmm0, xmmword ptr [rbp + 8]
0x140feddb1 movups xmmword ptr [rax + 0x220], xmm0
0x140feddb8 movups xmm0, xmmword ptr [rbp - 8]
0x140feddbc movups xmmword ptr [rax + 0x210], xmm0
0x140feddc3 movups xmm0, xmmword ptr [rbp + 0x238]
0x140feddca movups xmmword ptr [rax + 0x290], xmm0
0x140feddd1 movups xmm0, xmmword ptr [rbp + 0x228]
0x140feddd8 movups xmmword ptr [rax + 0x280], xmm0
0x140fedddf movups xmm0, xmmword ptr [rbp + 0x218]
0x140fedde6 movups xmmword ptr [rax + 0x270], xmm0
0x140fedded movups xmm0, xmmword ptr [rbp + 0x1d8]
0x140feddf4 movups xmm1, xmmword ptr [rbp + 0x1e8]
0x140feddfb movups xmm2, xmmword ptr [rbp + 0x1f8]
0x140fede02 movups xmm3, xmmword ptr [rbp + 0x208]
0x140fede09 movups xmmword ptr [rax + 0x260], xmm3
0x140fede10 movups xmmword ptr [rax + 0x250], xmm2
0x140fede17 movups xmmword ptr [rax + 0x230], xmm0
0x140fede1e movups xmmword ptr [rax + 0x240], xmm1
0x140fede25 mov qword ptr [rbp + 0x250], 6
0x140fede30 mov qword ptr [rbp + 0x258], rax
0x140fede37 mov qword ptr [rbp + 0x260], 6
0x140fede42 mov qword ptr [rbp + 0x268], 0
0x140fede4d mov qword ptr [rbp + 0x270], 8
0x140fede58 xorps xmm0, xmm0
0x140fede5b movups xmmword ptr [rbp + 0x278], xmm0
0x140fede62 mov rax, qword ptr [rbp + 0x1e8]
0x140fede69 mov qword ptr [rbp + 0x298], rax
0x140fede70 movups xmm0, xmmword ptr [rbp + 0x1d8]
0x140fede77 movups xmmword ptr [rbp + 0x288], xmm0
0x140fede7e mov qword ptr [rbp + 0x248], rbx
0x140fede85 lea rcx, [rbp + 0x280]
0x140fede8c call 0x140fd4720
0x140fede91 movups xmm0, xmmword ptr [rbp + 0x2a8]
0x140fede98 movups xmmword ptr [rsi + 0x60], xmm0
0x140fede9c movups xmm0, xmmword ptr [rbp + 0x298]
0x140fedea3 movups xmmword ptr [rsi + 0x50], xmm0
0x140fedea7 movups xmm0, xmmword ptr [rbp + 0x288]
0x140fedeae movups xmmword ptr [rsi + 0x40], xmm0
0x140fedeb2 movups xmm0, xmmword ptr [rbp + 0x248]
0x140fedeb9 movups xmm1, xmmword ptr [rbp + 0x258]
0x140fedec0 movups xmm2, xmmword ptr [rbp + 0x268]
0x140fedec7 movups xmm3, xmmword ptr [rbp + 0x278]
0x140fedece movups xmmword ptr [rsi + 0x30], xmm3
0x140feded2 movups xmmword ptr [rsi + 0x20], xmm2
0x140feded6 movups xmmword ptr [rsi + 0x10], xmm1
0x140fededa movups xmmword ptr [rsi], xmm0
0x140fededd mov rax, rsi
0x140fedee0 add rsp, 0x548
0x140fedee7 pop rbx
0x140fedee8 pop rdi
0x140fedee9 pop rsi
0x140fedeea pop rbp
0x140fedeeb ret 
0x140fedeec mov ecx, 8
0x140fedef1 mov edx, 0x2a0
0x140fedef6 call 0x1428d9000
0x140fedefb mov ecx, 8
0x140fedf00 mov edx, 0x268
0x140fedf05 call 0x1428d9000
0x140fedf0a jmp 0x140fedf1b
0x140fedf0c mov ecx, 8
0x140fedf11 mov edx, 0x270
