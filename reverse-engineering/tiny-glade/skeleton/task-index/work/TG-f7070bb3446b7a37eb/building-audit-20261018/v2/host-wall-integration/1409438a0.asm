0x1409438a0 55 push rbp
0x1409438a1 4156 push r14
0x1409438a3 56 push rsi
0x1409438a4 57 push rdi
0x1409438a5 53 push rbx
0x1409438a6 4881ec20040000 sub rsp, 0x420
0x1409438ad 488dac2480000000 lea rbp, [rsp + 0x80]
0x1409438b5 48c78598030000feffffff mov qword ptr [rbp + 0x398], 0xfffffffffffffffe
0x1409438c0 4889ce mov rsi, rcx
0x1409438c3 4c8d0576f01602 lea r8, [rip + 0x216f076]
0x1409438ca 488d4d10 lea rcx, [rbp + 0x10]
0x1409438ce b20e mov dl, 0xe
0x1409438d0 e85b4ddf00 call 0x141738630
0x1409438d5 488d8df0000000 lea rcx, [rbp + 0xf0]
0x1409438dc e8cf46df00 call 0x141737fb0
0x1409438e1 c685b002000023 mov byte ptr [rbp + 0x2b0], 0x23
0x1409438e8 488b0d818ace02 mov rcx, qword ptr [rip + 0x2ce8a81]
0x1409438ef 4c8d056ac43102 lea r8, [rip + 0x231c46a]
0x1409438f6 488d95b0020000 lea rdx, [rbp + 0x2b0]
0x1409438fd e89edf1d00 call 0x140b218a0
0x140943902 4989d0 mov r8, rdx
0x140943905 488d8df0000000 lea rcx, [rbp + 0xf0]
0x14094390c 4889c2 mov rdx, rax
0x14094390f e8ecf6b000 call 0x141453000
0x140943914 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
0x14094391b 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
0x140943922 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
0x140943929 0f298570030000 movaps xmmword ptr [rbp + 0x370], xmm0
0x140943930 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
0x140943937 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
0x14094393e 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
0x140943945 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
0x14094394c 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
0x140943953 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
0x14094395a 0f299d50030000 movaps xmmword ptr [rbp + 0x350], xmm3
0x140943961 0f299540030000 movaps xmmword ptr [rbp + 0x340], xmm2
0x140943968 0f298d30030000 movaps xmmword ptr [rbp + 0x330], xmm1
0x14094396f 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
0x140943976 4c8d05dbef1602 lea r8, [rip + 0x216efdb]
0x14094397d 488d4da0 lea rcx, [rbp - 0x60]
0x140943981 b228 mov dl, 0x28
0x140943983 e8b8fd1900 call 0x140ae3740
0x140943988 488d8df0000000 lea rcx, [rbp + 0xf0]
0x14094398f e87cb40400 call 0x14098ee10
0x140943994 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
0x14094399b 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
0x1409439a2 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
0x1409439a9 0f298500030000 movaps xmmword ptr [rbp + 0x300], xmm0
0x1409439b0 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
0x1409439b7 0f2985f0020000 movaps xmmword ptr [rbp + 0x2f0], xmm0
0x1409439be 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
0x1409439c5 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
0x1409439cc 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
0x1409439d3 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
0x1409439da 0f299de0020000 movaps xmmword ptr [rbp + 0x2e0], xmm3
0x1409439e1 0f2995d0020000 movaps xmmword ptr [rbp + 0x2d0], xmm2
0x1409439e8 0f298dc0020000 movaps xmmword ptr [rbp + 0x2c0], xmm1
0x1409439ef 0f2985b0020000 movaps xmmword ptr [rbp + 0x2b0], xmm0
0x1409439f6 488d8d80000000 lea rcx, [rbp + 0x80]
0x1409439fd 488d95b0020000 lea rdx, [rbp + 0x2b0]
0x140943a04 e8d740df00 call 0x141737ae0
0x140943a09 c685b002000029 mov byte ptr [rbp + 0x2b0], 0x29
0x140943a10 488b0d5989ce02 mov rcx, qword ptr [rip + 0x2ce8959]
0x140943a17 4c8d0542c33102 lea r8, [rip + 0x231c342]
0x140943a1e 488dbdb0020000 lea rdi, [rbp + 0x2b0]
0x140943a25 4889fa mov rdx, rdi
0x140943a28 e873de1d00 call 0x140b218a0
0x140943a2d 4989d0 mov r8, rdx
0x140943a30 488d9d80000000 lea rbx, [rbp + 0x80]
0x140943a37 4889d9 mov rcx, rbx
0x140943a3a 4889c2 mov rdx, rax
0x140943a3d e8bef5b000 call 0x141453000
0x140943a42 0f1085e0000000 movups xmm0, xmmword ptr [rbp + 0xe0]
0x140943a49 0f2985a0020000 movaps xmmword ptr [rbp + 0x2a0], xmm0
0x140943a50 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
0x140943a57 0f298590020000 movaps xmmword ptr [rbp + 0x290], xmm0
0x140943a5e 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x140943a65 0f298580020000 movaps xmmword ptr [rbp + 0x280], xmm0
0x140943a6c 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
0x140943a73 0f108d90000000 movups xmm1, xmmword ptr [rbp + 0x90]
0x140943a7a 0f1095a0000000 movups xmm2, xmmword ptr [rbp + 0xa0]
0x140943a81 0f109db0000000 movups xmm3, xmmword ptr [rbp + 0xb0]
0x140943a88 0f299d70020000 movaps xmmword ptr [rbp + 0x270], xmm3
0x140943a8f 0f299560020000 movaps xmmword ptr [rbp + 0x260], xmm2
0x140943a96 0f298d50020000 movaps xmmword ptr [rbp + 0x250], xmm1
0x140943a9d 0f298540020000 movaps xmmword ptr [rbp + 0x240], xmm0
0x140943aa4 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x140943aa8 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x140943aac 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
0x140943ab0 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
0x140943ab4 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
0x140943abb 0f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm1
0x140943ac2 0f299510010000 movaps xmmword ptr [rbp + 0x110], xmm2
0x140943ac9 0f299d20010000 movaps xmmword ptr [rbp + 0x120], xmm3
0x140943ad0 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
0x140943ad4 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
0x140943adb 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x140943adf 0f298540010000 movaps xmmword ptr [rbp + 0x140], xmm0
0x140943ae6 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
0x140943aea 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
0x140943af1 0f288580030000 movaps xmm0, xmmword ptr [rbp + 0x380]
0x140943af8 0f2985c0010000 movaps xmmword ptr [rbp + 0x1c0], xmm0
0x140943aff 0f288570030000 movaps xmm0, xmmword ptr [rbp + 0x370]
0x140943b06 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
0x140943b0d 0f288560030000 movaps xmm0, xmmword ptr [rbp + 0x360]
0x140943b14 0f2985a0010000 movaps xmmword ptr [rbp + 0x1a0], xmm0
0x140943b1b 0f288520030000 movaps xmm0, xmmword ptr [rbp + 0x320]
0x140943b22 0f288d30030000 movaps xmm1, xmmword ptr [rbp + 0x330]
0x140943b29 0f289540030000 movaps xmm2, xmmword ptr [rbp + 0x340]
0x140943b30 0f289d50030000 movaps xmm3, xmmword ptr [rbp + 0x350]
0x140943b37 0f299d90010000 movaps xmmword ptr [rbp + 0x190], xmm3
0x140943b3e 0f299580010000 movaps xmmword ptr [rbp + 0x180], xmm2
0x140943b45 0f298d70010000 movaps xmmword ptr [rbp + 0x170], xmm1
0x140943b4c 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
0x140943b53 0f104500 movups xmm0, xmmword ptr [rbp]
0x140943b57 0f298530020000 movaps xmmword ptr [rbp + 0x230], xmm0
0x140943b5e 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
0x140943b62 0f298520020000 movaps xmmword ptr [rbp + 0x220], xmm0
0x140943b69 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
0x140943b6d 0f298510020000 movaps xmmword ptr [rbp + 0x210], xmm0
0x140943b74 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
0x140943b78 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
0x140943b7c 0f1055c0 movups xmm2, xmmword ptr [rbp - 0x40]
0x140943b80 0f105dd0 movups xmm3, xmmword ptr [rbp - 0x30]
0x140943b84 0f299d00020000 movaps xmmword ptr [rbp + 0x200], xmm3
0x140943b8b 0f2995f0010000 movaps xmmword ptr [rbp + 0x1f0], xmm2
0x140943b92 0f298de0010000 movaps xmmword ptr [rbp + 0x1e0], xmm1
0x140943b99 0f2985d0010000 movaps xmmword ptr [rbp + 0x1d0], xmm0
0x140943ba0 488d95f0000000 lea rdx, [rbp + 0xf0]
0x140943ba7 4889f1 mov rcx, rsi
0x140943baa e8112e2200 call 0x140b669c0
0x140943baf 4c8d7638 lea r14, [rsi + 0x38]
0x140943bb3 48ba84a72b9651d21b8e movabs rdx, 0x8e1bd251962ba784
0x140943bbd 49b89b52e6c07423786a movabs r8, 0x6a782374c0e6529b
0x140943bc7 4c89f1 mov rcx, r14
0x140943bca e8219b79ff call 0x1400dd6f0
0x140943bcf 4885c0 test rax, rax
0x140943bd2 0f84c1000000 je 0x140943c99
0x140943bd8 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x140943bdf 0f86b4000000 jbe 0x140943c99
0x140943be5 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x140943bec 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140943bf0 4885c9 test rcx, rcx
0x140943bf3 0f84a0000000 je 0x140943c99
0x140943bf9 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x140943c00 48f7d1 not rcx
0x140943c03 488d0c49 lea rcx, [rcx + rcx*2]
0x140943c07 48c1e105 shl rcx, 5
0x140943c0b 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140943c11 0f8482000000 je 0x140943c99
0x140943c17 4801c8 add rax, rcx
0x140943c1a 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140943c1e 8b96b0050000 mov edx, dword ptr [rsi + 0x5b0]
0x140943c24 89505c mov dword ptr [rax + 0x5c], edx
0x140943c27 e844cfd500 call 0x1416a0b70
0x140943c2c 48bafe6a98a7e8ff5725 movabs rdx, 0x2557ffe8a7986afe
0x140943c36 49b837070e9b3c233249 movabs r8, 0x4932233c9b0e0737
0x140943c40 4c89f1 mov rcx, r14
0x140943c43 e8a89a79ff call 0x1400dd6f0
0x140943c48 4885c0 test rax, rax
0x140943c4b 7433 je 0x140943c80
0x140943c4d 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x140943c54 762a jbe 0x140943c80
0x140943c56 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x140943c5d 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x140943c61 4885c0 test rax, rax
0x140943c64 741a je 0x140943c80
0x140943c66 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
0x140943c6d 48f7d0 not rax
0x140943c70 488d0440 lea rax, [rax + rax*2]
0x140943c74 48c1e005 shl rax, 5
0x140943c78 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
0x140943c7e 7508 jne 0x140943c88
0x140943c80 4889f1 mov rcx, rsi
0x140943c83 e8f8f6bc00 call 0x141513380
0x140943c88 4889f0 mov rax, rsi
0x140943c8b 4881c420040000 add rsp, 0x420
0x140943c92 5b pop rbx
0x140943c93 5f pop rdi
0x140943c94 5e pop rsi
0x140943c95 415e pop r14
0x140943c97 5d pop rbp
0x140943c98 c3 ret 
0x140943c99 488d0520f01a02 lea rax, [rip + 0x21af020]
0x140943ca0 488985b0020000 mov qword ptr [rbp + 0x2b0], rax
0x140943ca7 48c785b802000027000000 mov qword ptr [rbp + 0x2b8], 0x27
0x140943cb2 4889bd80000000 mov qword ptr [rbp + 0x80], rdi
0x140943cb9 488d05d0617cff lea rax, [rip - 0x839e30]
0x140943cc0 48898588000000 mov qword ptr [rbp + 0x88], rax
0x140943cc7 488d0522f11a02 lea rax, [rip + 0x21af122]
0x140943cce 488985f0000000 mov qword ptr [rbp + 0xf0], rax
0x140943cd5 48c785f800000002000000 mov qword ptr [rbp + 0xf8], 2
0x140943ce0 48c7851001000000000000 mov qword ptr [rbp + 0x110], 0
0x140943ceb 48899d00010000 mov qword ptr [rbp + 0x100], rbx
0x140943cf2 48c7850801000001000000 mov qword ptr [rbp + 0x108], 1
0x140943cfd 488d15a4ef1a02 lea rdx, [rip + 0x21aefa4]
0x140943d04 488d8df0000000 lea rcx, [rbp + 0xf0]
0x140943d0b e88056f901 call 0x1428d9390
0x140943d10 cc int3 
0x140943d11 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140943d20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140943d25 55 push rbp
0x140943d26 4156 push r14
0x140943d28 56 push rsi
0x140943d29 57 push rdi
0x140943d2a 53 push rbx
0x140943d2b 4883ec20 sub rsp, 0x20
0x140943d2f 488daa80000000 lea rbp, [rdx + 0x80]
0x140943d36 488d4d10 lea rcx, [rbp + 0x10]
0x140943d3a e8415b6eff call 0x140029880
0x140943d3f 90 nop 
0x140943d40 4883c420 add rsp, 0x20
0x140943d44 5b pop rbx
0x140943d45 5f pop rdi
0x140943d46 5e pop rsi
0x140943d47 415e pop r14
0x140943d49 5d pop rbp
0x140943d4a c3 ret 
0x140943d4b 0f1f440000 nop dword ptr [rax + rax]
0x140943d50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140943d55 55 push rbp
0x140943d56 4156 push r14
0x140943d58 56 push rsi
0x140943d59 57 push rdi
0x140943d5a 53 push rbx
0x140943d5b 4883ec20 sub rsp, 0x20
0x140943d5f 488daa80000000 lea rbp, [rdx + 0x80]
0x140943d66 488d8df0000000 lea rcx, [rbp + 0xf0]
0x140943d6d e80e5b6eff call 0x140029880
0x140943d72 90 nop 
0x140943d73 4883c420 add rsp, 0x20
0x140943d77 5b pop rbx
0x140943d78 5f pop rdi
0x140943d79 5e pop rsi
0x140943d7a 415e pop r14
0x140943d7c 5d pop rbp
0x140943d7d c3 ret 
0x140943d7e 6690 nop 
0x140943d80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140943d85 55 push rbp
0x140943d86 4156 push r14
0x140943d88 56 push rsi
0x140943d89 57 push rdi
0x140943d8a 53 push rbx
0x140943d8b 4883ec20 sub rsp, 0x20
0x140943d8f 488daa80000000 lea rbp, [rdx + 0x80]
0x140943d96 488d8d20030000 lea rcx, [rbp + 0x320]
0x140943d9d e8de5a6eff call 0x140029880
0x140943da2 90 nop 
0x140943da3 4883c420 add rsp, 0x20
0x140943da7 5b pop rbx
0x140943da8 5f pop rdi
0x140943da9 5e pop rsi
0x140943daa 415e pop r14
0x140943dac 5d pop rbp
0x140943dad c3 ret 
0x140943dae 6690 nop 
0x140943db0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140943db5 55 push rbp
0x140943db6 4156 push r14
0x140943db8 56 push rsi
0x140943db9 57 push rdi
0x140943dba 53 push rbx
0x140943dbb 4883ec20 sub rsp, 0x20
0x140943dbf 488daa80000000 lea rbp, [rdx + 0x80]
0x140943dc6 488d4da0 lea rcx, [rbp - 0x60]
0x140943dca e8b15a6eff call 0x140029880
0x140943dcf 90 nop 
0x140943dd0 4883c420 add rsp, 0x20
0x140943dd4 5b pop rbx
0x140943dd5 5f pop rdi
0x140943dd6 5e pop rsi
0x140943dd7 415e pop r14
0x140943dd9 5d pop rbp
0x140943dda c3 ret 
0x140943ddb 0f1f440000 nop dword ptr [rax + rax]
0x140943de0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140943de5 55 push rbp
0x140943de6 4156 push r14
0x140943de8 56 push rsi
0x140943de9 57 push rdi
0x140943dea 53 push rbx
0x140943deb 4883ec20 sub rsp, 0x20
0x140943def 488daa80000000 lea rbp, [rdx + 0x80]
0x140943df6 488d8d80000000 lea rcx, [rbp + 0x80]
0x140943dfd e87e5a6eff call 0x140029880
0x140943e02 90 nop 
0x140943e03 4883c420 add rsp, 0x20
0x140943e07 5b pop rbx
0x140943e08 5f pop rdi
0x140943e09 5e pop rsi
0x140943e0a 415e pop r14
0x140943e0c 5d pop rbp
0x140943e0d c3 ret 
