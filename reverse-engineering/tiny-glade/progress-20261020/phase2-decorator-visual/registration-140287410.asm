140287410 push rbp
140287411 push rsi
140287412 push rdi
140287413 push rbx
140287414 sub rsp, 0x3e8
14028741b lea rbp, [rsp + 0x80]
140287423 mov qword ptr [rbp + 0x360], 0xfffffffffffffffe
14028742e mov rsi, rcx
140287431 movzx eax, byte ptr [rip + 0x33ad7d1]
140287438 mov ecx, 0x2a0
14028743d mov edx, 8
140287442 call 0x140613c10
140287447 mov qword ptr [rbp + 0x358], rax
14028744e test rax, rax
140287451 je 0x1402878d3
140287457 lea rcx, [rbp + 0x160]
14028745e call 0x140285ee0
140287463 lea rcx, [rbp + 0xf0]
14028746a call 0x140105110
14028746f xorps xmm0, xmm0
140287472 movups xmmword ptr [rbp + 0x238], xmm0
140287479 movups xmmword ptr [rbp + 0x250], xmm0
140287480 movups xmmword ptr [rbp + 0x268], xmm0
140287487 movups xmmword ptr [rbp + 0x280], xmm0
14028748e movups xmmword ptr [rbp + 0x298], xmm0
140287495 mov dword ptr [rbp + 0x2a8], 0
14028749f movups xmmword ptr [rbp + 0x2d0], xmm0
1402874a6 movups xmmword ptr [rbp + 0x2e8], xmm0
1402874ad movups xmmword ptr [rbp + 0x300], xmm0
1402874b4 movups xmmword ptr [rbp + 0x318], xmm0
1402874bb movups xmmword ptr [rbp + 0x330], xmm0
1402874c2 mov dword ptr [rbp + 0x340], 0
1402874cc mov qword ptr [rbp + 0x1d0], 0
1402874d7 mov qword ptr [rbp + 0x218], 0
1402874e2 mov qword ptr [rbp + 0x220], 8
1402874ed mov qword ptr [rbp + 0x228], 0
1402874f8 mov qword ptr [rbp + 0x230], 0x10
140287503 mov qword ptr [rbp + 0x248], 0x10
14028750e mov qword ptr [rbp + 0x260], 0x10
140287519 mov qword ptr [rbp + 0x278], 0x10
140287524 mov qword ptr [rbp + 0x290], 0x10
14028752f movabs rbx, 0x8000000000000000
140287539 mov qword ptr [rbp + 0x2b0], rbx
140287540 lea rax, [rip + 0x2722d9d]
140287547 mov qword ptr [rbp + 0x2b8], rax
14028754e mov qword ptr [rbp + 0x2c0], 0x55
140287559 mov qword ptr [rbp + 0x2c8], 0x10
140287564 mov qword ptr [rbp + 0x2e0], 0x10
14028756f mov qword ptr [rbp + 0x2f8], 0x10
14028757a mov qword ptr [rbp + 0x310], 0x10
140287585 mov qword ptr [rbp + 0x328], 0x10
140287590 mov dword ptr [rbp + 0x348], 0
14028759a mov word ptr [rbp + 0x34c], 1
1402875a3 mov dword ptr [rbp + 0x350], 0
1402875ad movzx eax, byte ptr [rip + 0x33ad655]
1402875b4 mov ecx, 0x188
1402875b9 mov edx, 8
1402875be call 0x140613c10
1402875c3 test rax, rax
1402875c6 je 0x1402878e2
1402875cc mov rdi, rax
1402875cf lea rdx, [rbp + 0x1d0]
1402875d6 mov r8d, 0x188
1402875dc mov rcx, rax
1402875df call 0x142923c00
1402875e4 lea r8, [rip + 0x27227b5]
1402875eb lea rcx, [rbp + 0x80]
1402875f2 mov rdx, rdi
1402875f5 call 0x140ce4b10
1402875fa lea rcx, [rbp + 0x10]
1402875fe call 0x1400290b0
140287603 lea rcx, [rbp - 0x60]
140287607 call 0x140104d90
14028760c lea rcx, [rbp + 0x1d0]
140287613 call 0x1400c6200
140287618 movups xmm0, xmmword ptr [rbp + 0x1c0]
14028761f mov rax, qword ptr [rbp + 0x358]
140287626 movups xmmword ptr [rax + 0x60], xmm0
14028762a movups xmm0, xmmword ptr [rbp + 0x1b0]
140287631 movups xmmword ptr [rax + 0x50], xmm0
140287635 movups xmm0, xmmword ptr [rbp + 0x1a0]
14028763c movups xmmword ptr [rax + 0x40], xmm0
140287640 movups xmm0, xmmword ptr [rbp + 0x160]
140287647 movups xmm1, xmmword ptr [rbp + 0x170]
14028764e movups xmm2, xmmword ptr [rbp + 0x180]
140287655 movups xmm3, xmmword ptr [rbp + 0x190]
14028765c movups xmmword ptr [rax + 0x30], xmm3
140287660 movups xmmword ptr [rax + 0x20], xmm2
140287664 movups xmmword ptr [rax + 0x10], xmm1
140287668 movups xmmword ptr [rax], xmm0
14028766b movups xmm0, xmmword ptr [rbp + 0xf0]
140287672 movups xmm1, xmmword ptr [rbp + 0x100]
140287679 movups xmm2, xmmword ptr [rbp + 0x110]
140287680 movups xmm3, xmmword ptr [rbp + 0x120]
140287687 movups xmmword ptr [rax + 0x70], xmm0
14028768b movups xmmword ptr [rax + 0x80], xmm1
140287692 movups xmmword ptr [rax + 0x90], xmm2
140287699 movups xmmword ptr [rax + 0xa0], xmm3
1402876a0 movups xmm0, xmmword ptr [rbp + 0x130]
1402876a7 movups xmmword ptr [rax + 0xb0], xmm0
1402876ae movups xmm0, xmmword ptr [rbp + 0x140]
1402876b5 movups xmmword ptr [rax + 0xc0], xmm0
1402876bc movups xmm0, xmmword ptr [rbp + 0x150]
1402876c3 movups xmmword ptr [rax + 0xd0], xmm0
1402876ca movups xmm0, xmmword ptr [rbp + 0xe0]
1402876d1 movups xmmword ptr [rax + 0x140], xmm0
1402876d8 movups xmm0, xmmword ptr [rbp + 0xd0]
1402876df movups xmmword ptr [rax + 0x130], xmm0
1402876e6 movups xmm0, xmmword ptr [rbp + 0xc0]
1402876ed movups xmmword ptr [rax + 0x120], xmm0
1402876f4 movups xmm0, xmmword ptr [rbp + 0x80]
1402876fb movups xmm1, xmmword ptr [rbp + 0x90]
140287702 movups xmm2, xmmword ptr [rbp + 0xa0]
140287709 movups xmm3, xmmword ptr [rbp + 0xb0]
140287710 movups xmmword ptr [rax + 0x110], xmm3
140287717 movups xmmword ptr [rax + 0x100], xmm2
14028771e movups xmmword ptr [rax + 0xf0], xmm1
140287725 movups xmmword ptr [rax + 0xe0], xmm0
14028772c movups xmm0, xmmword ptr [rbp + 0x10]
140287730 movups xmm1, xmmword ptr [rbp + 0x20]
140287734 movups xmm2, xmmword ptr [rbp + 0x30]
140287738 movups xmm3, xmmword ptr [rbp + 0x40]
14028773c movups xmmword ptr [rax + 0x150], xmm0
140287743 movups xmmword ptr [rax + 0x160], xmm1
14028774a movups xmmword ptr [rax + 0x170], xmm2
140287751 movups xmmword ptr [rax + 0x180], xmm3
140287758 movups xmm0, xmmword ptr [rbp + 0x50]
14028775c movups xmmword ptr [rax + 0x190], xmm0
140287763 movups xmm0, xmmword ptr [rbp + 0x60]
140287767 movups xmmword ptr [rax + 0x1a0], xmm0
14028776e movups xmm0, xmmword ptr [rbp + 0x70]
140287772 movups xmmword ptr [rax + 0x1b0], xmm0
140287779 movups xmm0, xmmword ptr [rbp - 0x60]
14028777d movups xmm1, xmmword ptr [rbp - 0x50]
140287781 movups xmm2, xmmword ptr [rbp - 0x40]
140287785 movups xmm3, xmmword ptr [rbp - 0x30]
140287789 movups xmmword ptr [rax + 0x1c0], xmm0
140287790 movups xmmword ptr [rax + 0x1d0], xmm1
140287797 movups xmmword ptr [rax + 0x1e0], xmm2
14028779e movups xmmword ptr [rax + 0x1f0], xmm3
1402877a5 movups xmm0, xmmword ptr [rbp - 0x20]
1402877a9 movups xmmword ptr [rax + 0x200], xmm0
1402877b0 movups xmm0, xmmword ptr [rbp]
1402877b4 movups xmmword ptr [rax + 0x220], xmm0
1402877bb movups xmm0, xmmword ptr [rbp - 0x10]
1402877bf movups xmmword ptr [rax + 0x210], xmm0
1402877c6 movups xmm0, xmmword ptr [rbp + 0x230]
1402877cd movups xmmword ptr [rax + 0x290], xmm0
1402877d4 movups xmm0, xmmword ptr [rbp + 0x220]
1402877db movups xmmword ptr [rax + 0x280], xmm0
1402877e2 movups xmm0, xmmword ptr [rbp + 0x210]
1402877e9 movups xmmword ptr [rax + 0x270], xmm0
1402877f0 movups xmm0, xmmword ptr [rbp + 0x1d0]
1402877f7 movups xmm1, xmmword ptr [rbp + 0x1e0]
1402877fe movups xmm2, xmmword ptr [rbp + 0x1f0]
140287805 movups xmm3, xmmword ptr [rbp + 0x200]
14028780c movups xmmword ptr [rax + 0x260], xmm3
140287813 movups xmmword ptr [rax + 0x250], xmm2
14028781a movups xmmword ptr [rax + 0x230], xmm0
140287821 movups xmmword ptr [rax + 0x240], xmm1
140287828 mov qword ptr [rbp + 0x1d8], 6
140287833 mov qword ptr [rbp + 0x1e0], rax
14028783a mov qword ptr [rbp + 0x1e8], 6
140287845 mov qword ptr [rbp + 0x1f0], 0
140287850 mov qword ptr [rbp + 0x1f8], 8
14028785b xorps xmm0, xmm0
14028785e movups xmmword ptr [rbp + 0x200], xmm0
140287865 mov qword ptr [rbp + 0x1d0], rbx
14028786c lea rcx, [rbp + 0x208]
140287873 call 0x140d2fa20
140287878 movups xmm0, xmmword ptr [rbp + 0x230]
14028787f movups xmmword ptr [rsi + 0x60], xmm0
140287883 movups xmm0, xmmword ptr [rbp + 0x220]
14028788a movups xmmword ptr [rsi + 0x50], xmm0
14028788e movups xmm0, xmmword ptr [rbp + 0x210]
140287895 movups xmmword ptr [rsi + 0x40], xmm0
140287899 movups xmm0, xmmword ptr [rbp + 0x1d0]
1402878a0 movups xmm1, xmmword ptr [rbp + 0x1e0]
1402878a7 movups xmm2, xmmword ptr [rbp + 0x1f0]
1402878ae movups xmm3, xmmword ptr [rbp + 0x200]
1402878b5 movups xmmword ptr [rsi + 0x30], xmm3
1402878b9 movups xmmword ptr [rsi + 0x20], xmm2
1402878bd movups xmmword ptr [rsi + 0x10], xmm1
1402878c1 movups xmmword ptr [rsi], xmm0
1402878c4 mov rax, rsi
1402878c7 add rsp, 0x3e8
1402878ce pop rbx
1402878cf pop rdi
1402878d0 pop rsi
1402878d1 pop rbp
1402878d2 ret 
1402878d3 mov ecx, 8
1402878d8 mov edx, 0x2a0
1402878dd call 0x1428d9000
1402878e2 mov ecx, 8
1402878e7 mov edx, 0x188
1402878ec call 0x1428d9000
1402878f1 ud2 
1402878f3 nop word ptr cs:[rax + rax]
140287900 mov qword ptr [rsp + 0x10], rdx
140287905 push rbp
140287906 push rsi
140287907 push rdi
140287908 push rbx
140287909 sub rsp, 0x28
14028790d lea rbp, [rdx + 0x80]
140287914 mov edx, 0x2a0
140287919 mov r8d, 8
14028791f mov rcx, qword ptr [rbp + 0x358]
140287926 call 0x140613c20
14028792b nop 
14028792c add rsp, 0x28
140287930 pop rbx
140287931 pop rdi
140287932 pop rsi
140287933 pop rbp
140287934 ret 
140287935 nop word ptr cs:[rax + rax]
140287940 mov qword ptr [rsp + 0x10], rdx
140287945 push rbp
140287946 push rsi
140287947 push rdi
140287948 push rbx
140287949 sub rsp, 0x28
14028794d lea rbp, [rdx + 0x80]
140287954 lea rcx, [rbp + 0x160]
14028795b call 0x140029880
140287960 nop 
140287961 add rsp, 0x28
140287965 pop rbx
140287966 pop rdi
140287967 pop rsi
140287968 pop rbp
140287969 ret 
14028796a nop word ptr [rax + rax]
140287970 mov qword ptr [rsp + 0x10], rdx
140287975 push rbp
140287976 push rsi
140287977 push rdi
140287978 push rbx
140287979 sub rsp, 0x28
14028797d lea rbp, [rdx + 0x80]
140287984 lea rcx, [rbp + 0x1d0]
14028798b call 0x1405d0e50
140287990 nop 
140287991 add rsp, 0x28
140287995 pop rbx
140287996 pop rdi
140287997 pop rsi
140287998 pop rbp
140287999 ret 
14028799a nop word ptr [rax + rax]
1402879a0 mov qword ptr [rsp + 0x10], rdx
1402879a5 push rbp
1402879a6 push rsi
1402879a7 push rdi
1402879a8 push rbx
1402879a9 sub rsp, 0x28
1402879ad lea rbp, [rdx + 0x80]
1402879b4 lea rcx, [rbp + 0xf0]
1402879bb call 0x140029880
1402879c0 nop 
1402879c1 add rsp, 0x28
1402879c5 pop rbx
1402879c6 pop rdi
1402879c7 pop rsi
1402879c8 pop rbp
1402879c9 ret 
1402879ca nop word ptr [rax + rax]
1402879d0 mov qword ptr [rsp + 0x10], rdx
1402879d5 push rbp
1402879d6 push rsi
1402879d7 push rdi
1402879d8 push rbx
1402879d9 sub rsp, 0x28
1402879dd lea rbp, [rdx + 0x80]
1402879e4 lea rcx, [rbp + 0x80]
1402879eb call 0x140029880
1402879f0 nop 
1402879f1 add rsp, 0x28
1402879f5 pop rbx
1402879f6 pop rdi
1402879f7 pop rsi
1402879f8 pop rbp
1402879f9 ret 
1402879fa nop word ptr [rax + rax]
140287a00 mov qword ptr [rsp + 0x10], rdx
140287a05 push rbp
140287a06 push rsi
140287a07 push rdi
140287a08 push rbx
140287a09 sub rsp, 0x28
140287a0d lea rbp, [rdx + 0x80]
140287a14 lea rcx, [rbp + 0x10]
140287a18 call 0x140029880
140287a1d nop 
140287a1e add rsp, 0x28
140287a22 pop rbx
140287a23 pop rdi
140287a24 pop rsi
140287a25 pop rbp
140287a26 ret 
140287a27 nop word ptr [rax + rax]
140287a30 mov qword ptr [rsp + 0x10], rdx
140287a35 push rbp
140287a36 push rsi
140287a37 push rdi
140287a38 push rbx
140287a39 sub rsp, 0x28
140287a3d lea rbp, [rdx + 0x80]
140287a44 lea rcx, [rbp - 0x60]
140287a48 call 0x140029880
140287a4d nop 
140287a4e add rsp, 0x28
140287a52 pop rbx
140287a53 pop rdi
140287a54 pop rsi
140287a55 pop rbp
140287a56 ret 
140287a57 nop word ptr [rax + rax]
140287a60 mov qword ptr [rsp + 0x10], rdx
140287a65 push rbp
140287a66 push rsi
140287a67 push rdi
140287a68 push rbx
140287a69 sub rsp, 0x28
140287a6d lea rbp, [rdx + 0x80]
140287a74 lea rcx, [rbp + 0x1d0]
140287a7b call 0x140029880
140287a80 nop 
140287a81 add rsp, 0x28
140287a85 pop rbx
140287a86 pop rdi
140287a87 pop rsi
140287a88 pop rbp
140287a89 ret 