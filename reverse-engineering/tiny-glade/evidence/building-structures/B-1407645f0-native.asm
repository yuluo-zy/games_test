1407645f0 push rbp
1407645f1 push r15
1407645f3 push r14
1407645f5 push r13
1407645f7 push r12
1407645f9 push rsi
1407645fa push rdi
1407645fb push rbx
1407645fc sub rsp, 0x1c8
140764603 lea rbp, [rsp + 0x80]
14076460b movaps xmmword ptr [rbp + 0x130], xmm10
140764613 movaps xmmword ptr [rbp + 0x120], xmm9
14076461b movaps xmmword ptr [rbp + 0x110], xmm8
140764623 movaps xmmword ptr [rbp + 0x100], xmm7
14076462a movaps xmmword ptr [rbp + 0xf0], xmm6
140764631 mov qword ptr [rbp + 0xe8], 0xfffffffffffffffe
14076463c mov rsi, rcx
14076463f mov rax, qword ptr [rbp + 0x1b0]
140764646 mov rcx, qword ptr [rbp + 0x1b8]
14076464d mov r15d, dword ptr [rcx + 0x258]
140764654 mov r12, qword ptr [rcx + 0xe0]
14076465b mov r13, qword ptr [rcx + 0xe8]
140764662 movss dword ptr [rbp + 0xc], xmm1
140764667 test byte ptr [rax], 1
14076466a je 0x14076467b
14076466c movss xmm1, dword ptr [rax + 4]
140764671 movss xmm2, dword ptr [rax + 8]
140764676 jmp 0x140764706
14076467b mov rax, qword ptr [rsi + 0x10]
14076467f test rax, rax
140764682 je 0x140765016
140764688 mov rcx, qword ptr [rsi + 0x28]
14076468c test rcx, rcx
14076468f je 0x140765026
140764695 lea rdx, [rax - 1]
140764699 movabs r9, 0x3fffffffffffffff
1407646a3 and r9, rdx
1407646a6 cmp rcx, r9
1407646a9 jbe 0x140765036
1407646af mov rcx, qword ptr [rsi + 8]
1407646b3 mov rdx, qword ptr [rsi + 0x20]
1407646b7 movss xmm3, dword ptr [rdx + 4]
1407646bc add rdx, 4
1407646c0 movss xmm0, dword ptr [rcx]
1407646c4 addss xmm0, xmm3
1407646c8 shl rax, 2
1407646cc xor r9d, r9d
1407646cf nop 
1407646d0 movss xmm4, dword ptr [rdx]
1407646d4 movss xmm1, dword ptr [rcx + r9]
1407646da addss xmm1, xmm4
1407646de movaps xmm2, xmm0
1407646e1 cmpltss xmm2, xmm1
1407646e6 andps xmm4, xmm2
1407646e9 andnps xmm2, xmm3
1407646ec orps xmm2, xmm4
1407646ef maxss xmm1, xmm0
1407646f3 add rdx, 0xc
1407646f7 add r9, 4
1407646fb movaps xmm0, xmm1
1407646fe movaps xmm3, xmm2
140764701 cmp rax, r9
140764704 jne 0x1407646d0
140764706 movss dword ptr [rbp + 8], xmm1
14076470b movss dword ptr [rbp + 4], xmm2
140764710 mov byte ptr [rbp + 0xd8], 0
140764717 movss xmm6, dword ptr [rip + 0x21c11e5]
14076471f xorps xmm7, xmm7
140764722 cmp r15d, 2
140764726 jne 0x140764d2f
14076472c mov rbx, r8
14076472f lea rdi, [rsi + 0x58]
140764733 mov rcx, rdi
140764736 call 0x141200370
14076473b mov dword ptr [rbp + 0x28], eax
14076473e mov dword ptr [rbp + 0x2c], edx
140764741 lea rcx, [rbp + 0x28]
140764745 call 0x140bece40
14076474a mov dword ptr [rbp + 0x98], eax
140764750 test eax, eax
140764752 setne r14b
140764756 mov rcx, rdi
140764759 mov rdi, rbx
14076475c call 0x141200380
140764761 mov dword ptr [rbp + 0x28], eax
140764764 mov dword ptr [rbp + 0x2c], edx
140764767 lea rcx, [rbp + 0x28]
14076476b call 0x140bece40
140764770 cmp eax, edi
140764772 setne cl
140764775 test r14b, cl
140764778 jne 0x140764d2f
14076477e mov dword ptr [rbp + 0x9c], eax
140764784 mov dword ptr [rbp + 0xe0], r15d
14076478b mov rax, qword ptr [rsi + 0x20]
14076478f mov rcx, qword ptr [rsi + 0x28]
140764793 lea rcx, [rcx + rcx*2]
140764797 lea rcx, [rax + rcx*4]
14076479b mov qword ptr [rbp + 0x28], rax
14076479f mov qword ptr [rbp + 0x30], rcx
1407647a3 mov qword ptr [rbp + 0x38], 0
1407647ab lea rcx, [rbp + 0x28]
1407647af call 0x141237ea0
1407647b4 movss xmm8, dword ptr [rip + 0x21cfedb]
1407647bd test rax, rax
1407647c0 je 0x140764821
1407647c2 lea r15, [rbp + 0x28]
1407647c6 nop word ptr cs:[rax + rax]
1407647d0 movss xmm0, dword ptr [rax + 4]
1407647d5 subss xmm0, dword ptr [rdx + 4]
1407647da mulss xmm0, xmm0
1407647de movss xmm1, dword ptr [rax + 8]
1407647e3 movss xmm2, dword ptr [rax]
1407647e7 unpcklps xmm2, xmm1
1407647ea movss xmm1, dword ptr [rdx]
1407647ee movss xmm3, dword ptr [rdx + 8]
1407647f3 unpcklps xmm1, xmm3
1407647f6 subps xmm2, xmm1
1407647f9 mulps xmm2, xmm2
1407647fc addss xmm0, xmm2
140764800 movshdup xmm1, xmm2
140764804 addss xmm1, xmm0
140764808 xorps xmm0, xmm0
14076480b sqrtss xmm0, xmm1
14076480f addss xmm8, xmm0
140764814 mov rcx, r15
140764817 call 0x141237ea0
14076481c test rax, rax
14076481f jne 0x1407647d0
140764821 mov byte ptr [rbp + 0xd8], 1
140764828 cmp dword ptr [rbp + 0x98], 0
14076482f mov r15d, dword ptr [rbp + 0xe0]
140764836 je 0x140764855
140764838 mov byte ptr [rbp + 0xd9], 1
14076483f movss dword ptr [rbp + 0xdc], xmm8
140764848 cmp dword ptr [rbp + 0x9c], edi
14076484e je 0x140764865
140764850 jmp 0x140764d2f
140764855 mov byte ptr [rbp + 0xd9], 0
14076485c movss dword ptr [rbp + 0xdc], xmm8
140764865 mov qword ptr [rbp - 0x50], r13
140764869 mov qword ptr [rbp - 0x48], r12
14076486d mov r15, qword ptr [rsi + 0x28]
140764871 xor edi, edi
140764873 mov ecx, 0xc
140764878 mov rax, r15
14076487b mul rcx
14076487e mov r12, rax
140764881 jo 0x140765045
140764887 movabs rax, 0x7ffffffffffffffc
140764891 cmp r12, rax
140764894 ja 0x140765045
14076489a mov r13, qword ptr [rsi + 0x20]
14076489e test r12, r12
1407648a1 je 0x1407648cd
1407648a3 movzx eax, byte ptr [rip + 0x2ed035f]
1407648aa mov edi, 4
1407648af mov edx, 4
1407648b4 mov rcx, r12
1407648b7 call 0x140613c10
1407648bc test rax, rax
1407648bf je 0x140765045
1407648c5 mov r14, rax
1407648c8 mov rdi, r15
1407648cb jmp 0x1407648d5
1407648cd mov r14d, 4
1407648d3 xor edi, edi
1407648d5 mov rcx, r14
1407648d8 mov rdx, r13
1407648db mov r8, r12
1407648de call 0x142923c00
1407648e3 mov qword ptr [rbp + 0xa0], rdi
1407648ea mov qword ptr [rbp + 0xa8], r14
1407648f1 mov qword ptr [rbp + 0xb0], r15
1407648f8 lea rcx, [rbp + 0x28]
1407648fc lea r15, [rbp + 0xa0]
140764903 xor edi, edi
140764905 mov rdx, r15
140764908 xor r8d, r8d
14076490b call 0x1411f9be0
140764910 cmp rdi, qword ptr [rbp + 0x28]
140764914 jo 0x140765057
14076491a mov rax, qword ptr [rbp + 0x58]
14076491e mov qword ptr [rbp + 0xd0], rax
140764925 movups xmm0, xmmword ptr [rbp + 0x28]
140764929 movups xmm1, xmmword ptr [rbp + 0x38]
14076492d movups xmm2, xmmword ptr [rbp + 0x48]
140764931 movaps xmmword ptr [rbp + 0xc0], xmm2
140764938 movaps xmmword ptr [rbp + 0xb0], xmm1
14076493f movaps xmmword ptr [rbp + 0xa0], xmm0
140764946 mov r12, qword ptr [rbp + 0xa8]
14076494d mov rax, qword ptr [rbp + 0xb0]
140764954 lea rax, [rax + rax*2]
140764958 lea r8, [r12 + rax*4]
14076495c lea r9, [rip + 0x24b422d]
140764963 lea rcx, [rbp - 0x40]
140764967 mov rdx, r12
14076496a call 0x1402ba9e0
14076496f mov r13, qword ptr [rbp - 0x50]
140764973 lea rcx, [rbp + 0x28]
140764977 lea rdx, [rbp - 0x40]
14076497b xor r8d, r8d
14076497e call 0x1411f9920
140764983 xor eax, eax
140764985 cmp rax, qword ptr [rbp + 0x28]
140764989 jo 0x14076508d
14076498f mov rax, qword ptr [rbp + 0x58]
140764993 mov qword ptr [rbp - 0x10], rax
140764997 movups xmm0, xmmword ptr [rbp + 0x28]
14076499b movups xmm1, xmmword ptr [rbp + 0x38]
14076499f movups xmm2, xmmword ptr [rbp + 0x48]
1407649a3 movaps xmmword ptr [rbp - 0x20], xmm2
1407649a7 movaps xmmword ptr [rbp - 0x30], xmm1
1407649ab movaps xmmword ptr [rbp - 0x40], xmm0
1407649af mov rax, qword ptr [rbp + 0xa0]
1407649b6 test rax, rax
1407649b9 je 0x1407649d1
1407649bb shl rax, 2
1407649bf lea rdx, [rax + rax*2]
1407649c3 mov r8d, 4
1407649c9 mov rcx, r12
1407649cc call 0x140613c20
1407649d1 mov rdx, qword ptr [rbp + 0xb8]
1407649d8 test rdx, rdx
1407649db je 0x1407649f3
1407649dd mov rcx, qword ptr [rbp + 0xc0]
1407649e4 shl rdx, 2
1407649e8 mov r8d, 4
1407649ee call 0x140613c20
1407649f3 movss xmm0, dword ptr [rbp - 0x10]
1407649f8 ucomiss xmm0, dword ptr [rip + 0x23177d1]
1407649ff mov r12, qword ptr [rbp - 0x48]
140764a03 jbe 0x140764af6
140764a09 movss xmm1, dword ptr [rip + 0x23177bf]
140764a11 divss xmm1, xmm0
140764a15 movss xmm7, dword ptr [rip + 0x2318eb3]
140764a1d minss xmm7, xmm1
140764a21 cmp dword ptr [rbp + 0x98], 0
140764a28 je 0x140764b0d
140764a2e movss xmm0, dword ptr [rip + 0x21c0ece]
140764a36 movaps xmm6, xmm0
140764a39 subss xmm6, xmm7
140764a3d movss dword ptr [rbp + 0x10], xmm6
140764a42 xorps xmm1, xmm1
140764a45 ucomiss xmm6, xmm1
140764a48 jb 0x140764f8a
140764a4e ucomiss xmm0, xmm6
140764a51 jb 0x140764f8a
140764a57 mov qword ptr [rbp + 0x10], 0
140764a5f mov qword ptr [rbp + 0x18], 4
140764a67 mov qword ptr [rbp + 0x20], 0
140764a6f mov byte ptr [rbp + 0xe7], 1
140764a76 lea rcx, [rbp - 0x40]
140764a7a lea r9, [rbp + 0x10]
140764a7e movaps xmm2, xmm6
140764a81 call 0x1411f8130
140764a86 mov rax, qword ptr [rbp + 0x20]
140764a8a mov qword ptr [rbp + 0xb0], rax
140764a91 movups xmm0, xmmword ptr [rbp + 0x10]
140764a95 movaps xmmword ptr [rbp + 0xa0], xmm0
140764a9c mov byte ptr [rbp + 0xe7], 0
140764aa3 lea rcx, [rbp + 0x28]
140764aa7 lea rdx, [rbp + 0xa0]
140764aae xor r8d, r8d
140764ab1 call 0x1411f9920
140764ab6 xor eax, eax
140764ab8 cmp rax, qword ptr [rbp + 0x28]
140764abc jo 0x1407650bf
140764ac2 mov rax, qword ptr [rbp + 0x58]
140764ac6 mov qword ptr [rbp + 0xd0], rax
140764acd movups xmm0, xmmword ptr [rbp + 0x28]
140764ad1 movups xmm1, xmmword ptr [rbp + 0x38]
140764ad5 movups xmm2, xmmword ptr [rbp + 0x48]
140764ad9 movaps xmmword ptr [rbp + 0xc0], xmm2
140764ae0 movaps xmmword ptr [rbp + 0xb0], xmm1
140764ae7 movaps xmmword ptr [rbp + 0xa0], xmm0
140764aee xorps xmm7, xmm7
140764af1 jmp 0x140764bb1
140764af6 movss xmm6, dword ptr [rip + 0x21c0e06]
140764afe xorps xmm7, xmm7
140764b01 mov r15d, dword ptr [rbp + 0xe0]
140764b08 jmp 0x140764cf7
140764b0d cmp dword ptr [rbp + 0x9c], ebx
140764b13 jne 0x140764b78
140764b15 movss xmm8, dword ptr [rip + 0x2318db6]
140764b1e minss xmm8, xmm7
140764b23 lea rdx, [rip + 0x2318f1e]
140764b2a movaps xmm0, xmm8
140764b2e call 0x141208a00
140764b33 movss xmm6, dword ptr [rip + 0x21c0dc9]
140764b3b subss xmm6, xmm8
140764b40 lea rdx, [rip + 0x2318f19]
140764b47 movaps xmm0, xmm6
140764b4a call 0x141208a00
140764b4f lea rax, [rip + 0x2318f22]
140764b56 mov qword ptr [rsp + 0x20], rax
140764b5b lea rcx, [rbp + 0xa0]
140764b62 lea rdx, [rbp - 0x40]
140764b66 movaps xmm2, xmm8
140764b6a movaps xmm3, xmm6
140764b6d call 0x1411f8640
140764b72 movaps xmm7, xmm8
140764b76 jmp 0x140764bb1
140764b78 lea rdx, [rip + 0x2318e99]
140764b7f movaps xmm0, xmm7
140764b82 call 0x141208a00
140764b87 movss xmm6, dword ptr [rip + 0x21c0d75]
140764b8f lea rax, [rip + 0x2318e9a]
140764b96 mov qword ptr [rsp + 0x20], rax
140764b9b lea rcx, [rbp + 0xa0]
140764ba2 lea rdx, [rbp - 0x40]
140764ba6 movaps xmm2, xmm7
140764ba9 movaps xmm3, xmm6
140764bac call 0x1411f8640
140764bb1 mov rax, qword ptr [rsi + 0x28]
140764bb5 test rax, rax
140764bb8 je 0x140764cac
140764bbe mov rbx, qword ptr [rsi + 0x20]
140764bc2 xorps xmm8, xmm8
140764bc6 cvtsi2ss xmm8, rax
140764bcb lea rax, [rax + rax*2]
140764bcf lea r14, [rbx + rax*4]
140764bd3 addss xmm8, dword ptr [rip + 0x21f5508]
140764bdc xor r15d, r15d
140764bdf xorps xmm9, xmm9
140764be3 movss xmm10, dword ptr [rip + 0x21c0d18]
140764bec lea rdi, [rbp + 0xa0]
140764bf3 nop word ptr cs:[rax + rax]
140764c00 xorps xmm1, xmm1
140764c03 cvtsi2ss xmm1, r15
140764c08 divss xmm1, xmm8
140764c0d movss dword ptr [rbp], xmm1
140764c12 ucomiss xmm1, xmm9
140764c16 jb 0x140764f1b
140764c1c ucomiss xmm10, xmm1
140764c20 jb 0x140764f1b
140764c26 ucomiss xmm1, xmm1
140764c29 jp 0x140764f6d
140764c2f mov rcx, rdi
140764c32 call 0x1411f8c10
140764c37 mov rdx, qword ptr [rbp + 0xb0]
140764c3e mov ecx, eax
140764c40 cmp rdx, rcx
140764c43 jbe 0x140765005
140764c49 lea rax, [rcx + 1]
140764c4d cmp rax, rdx
140764c50 jae 0x140764ff1
140764c56 mov rax, qword ptr [rbp + 0xa8]
140764c5d movaps xmm1, xmm10
140764c61 subss xmm1, xmm0
140764c65 movss xmm2, dword ptr [rax + rcx*8]
140764c6a mulss xmm2, xmm1
140764c6e mulss xmm1, dword ptr [rax + rcx*8 + 4]
140764c74 movss xmm3, dword ptr [rax + rcx*8 + 8]
140764c7a mulss xmm3, xmm0
140764c7e addss xmm3, xmm2
140764c82 movss xmm2, dword ptr [rax + rcx*8 + 0xc]
140764c88 mulss xmm2, xmm0
140764c8c addss xmm2, xmm1
140764c90 movss dword ptr [rbx], xmm3
140764c94 movss dword ptr [rbx + 8], xmm2
140764c99 lea rax, [rbx + 0xc]
140764c9d inc r15
140764ca0 mov rbx, rax
140764ca3 cmp rax, r14
140764ca6 jne 0x140764c00
140764cac mov rdx, qword ptr [rbp + 0xa0]
140764cb3 test rdx, rdx
140764cb6 je 0x140764cce
140764cb8 mov rcx, qword ptr [rbp + 0xa8]
140764cbf shl rdx, 3
140764cc3 mov r8d, 4
140764cc9 call 0x140613c20
140764cce mov rdx, qword ptr [rbp + 0xb8]
140764cd5 test rdx, rdx
140764cd8 mov r15d, dword ptr [rbp + 0xe0]
140764cdf je 0x140764cf7
140764ce1 mov rcx, qword ptr [rbp + 0xc0]
140764ce8 shl rdx, 2
140764cec mov r8d, 4
140764cf2 call 0x140613c20
140764cf7 mov rdx, qword ptr [rbp - 0x40]
140764cfb test rdx, rdx
140764cfe je 0x140764d13
140764d00 mov rcx, qword ptr [rbp - 0x38]
140764d04 shl rdx, 3
140764d08 mov r8d, 4
140764d0e call 0x140613c20
140764d13 mov rdx, qword ptr [rbp - 0x28]
140764d17 test rdx, rdx
140764d1a je 0x140764d2f
140764d1c mov rcx, qword ptr [rbp - 0x20]
140764d20 shl rdx, 2
140764d24 mov r8d, 4
140764d2a call 0x140613c20
140764d2f lea rax, [rsi + 0x18]
140764d33 mov rcx, qword ptr [rsi + 0x20]
140764d37 mov rdx, qword ptr [rsi + 0x28]
140764d3b lea rdx, [rdx + rdx*2]
140764d3f lea rdx, [rcx + rdx*4]
140764d43 mov qword ptr [rbp + 0x28], rcx
140764d47 mov qword ptr [rbp + 0x30], rdx
140764d4b mov qword ptr [rbp + 0x38], 0
140764d53 lea rcx, [rbp + 8]
140764d57 mov qword ptr [rbp + 0x40], rcx
140764d5b lea rcx, [rbp + 4]
140764d5f mov qword ptr [rbp + 0x48], rcx
140764d63 mov qword ptr [rbp + 0x50], rsi
140764d67 mov qword ptr [rbp + 0x58], rax
140764d6b lea rax, [rbp + 0xc]
140764d6f mov qword ptr [rbp + 0x60], rax
140764d73 lea rax, [rbp + 0xd8]
140764d7a mov qword ptr [rbp + 0x68], rax
140764d7e lea r8, [rip + 0x2318bab]
140764d85 lea rcx, [rbp + 0xa0]
140764d8c lea rdx, [rbp + 0x28]
140764d90 call 0x141202d50
140764d95 mov rdx, qword ptr [rsi]
140764d98 test rdx, rdx
140764d9b je 0x140764db0
140764d9d mov rcx, qword ptr [rsi + 8]
140764da1 shl rdx, 2
140764da5 mov r8d, 4
140764dab call 0x140613c20
140764db0 mov rax, qword ptr [rbp + 0xb0]
140764db7 mov qword ptr [rsi + 0x10], rax
140764dbb movups xmm0, xmmword ptr [rbp + 0xa0]
140764dc2 movups xmmword ptr [rsi], xmm0
140764dc5 cmp r15d, 1
140764dc9 ja 0x140764edb
140764dcf mov eax, dword ptr [rsi + 0x60]
140764dd2 mov dword ptr [rbp + 0x50], eax
140764dd5 mov rax, qword ptr [rsi + 0x58]
140764dd9 mov qword ptr [rbp + 0x48], rax
140764ddd mov r14, qword ptr [rsi + 8]
140764de1 mov rax, qword ptr [rsi + 0x10]
140764de5 lea rcx, [r14 + rax*4]
140764de9 mov rdx, qword ptr [rsi + 0x20]
140764ded mov r8, qword ptr [rsi + 0x28]
140764df1 lea r8, [r8 + r8*2]
140764df5 lea r8, [rdx + r8*4]
140764df9 xorps xmm0, xmm0
140764dfc movups xmmword ptr [rbp + 0x80], xmm0
140764e03 mov qword ptr [rbp + 0x90], 0
140764e0e mov qword ptr [rbp + 0x30], rcx
140764e12 mov qword ptr [rbp + 0x38], r12
140764e16 mov qword ptr [rbp + 0x40], r13
140764e1a movups xmmword ptr [rbp + 0x58], xmm0
140764e1e mov qword ptr [rbp + 0x68], 0
140764e26 mov qword ptr [rbp + 0x70], rdx
140764e2a mov qword ptr [rbp + 0x78], r8
140764e2e test rax, rax
140764e31 je 0x140764edb
140764e37 lea rsi, [rbp + 0x48]
140764e3b lea rdi, [rbp + 0xa0]
140764e42 nop word ptr cs:[rax + rax]
140764e50 lea rax, [r14 + 4]
140764e54 mov qword ptr [rbp + 0x28], rax
140764e58 mov rcx, rdi
140764e5b mov rdx, rsi
140764e5e call 0x1412002f0
140764e63 test byte ptr [rbp + 0xa0], 1
140764e6a je 0x140764edb
140764e6c mov ecx, dword ptr [rbp + 0xa4]
140764e72 mov edx, dword ptr [rbp + 0xa8]
140764e78 mov r15, qword ptr [rbp + 0x38]
140764e7c mov rbx, qword ptr [rbp + 0x40]
140764e80 call 0x141200280
140764e85 cmp rax, rbx
140764e88 jae 0x140764fdf
140764e8e mov rcx, qword ptr [rbp + 0x70]
140764e92 cmp rcx, qword ptr [rbp + 0x78]
140764e96 je 0x140764edb
140764e98 lea rdx, [rcx + 0xc]
140764e9c mov qword ptr [rbp + 0x70], rdx
140764ea0 movss xmm0, dword ptr [r15 + rax*4]
140764ea6 subss xmm0, dword ptr [rcx + 4]
140764eab movss xmm1, dword ptr [r14]
140764eb0 movaps xmm2, xmm1
140764eb3 cmpunordss xmm2, xmm1
140764eb8 movaps xmm3, xmm2
140764ebb andps xmm3, xmm0
140764ebe minss xmm0, xmm1
140764ec2 andnps xmm2, xmm0
140764ec5 orps xmm2, xmm3
140764ec8 movss dword ptr [r14], xmm2
140764ecd mov r14, qword ptr [rbp + 0x28]
140764ed1 cmp r14, qword ptr [rbp + 0x30]
140764ed5 jne 0x140764e50
140764edb movaps xmm0, xmm7
140764ede movaps xmm1, xmm6
140764ee1 movaps xmm6, xmmword ptr [rbp + 0xf0]
140764ee8 movaps xmm7, xmmword ptr [rbp + 0x100]
140764eef movaps xmm8, xmmword ptr [rbp + 0x110]
140764ef7 movaps xmm9, xmmword ptr [rbp + 0x120]
140764eff movaps xmm10, xmmword ptr [rbp + 0x130]
140764f07 add rsp, 0x1c8
140764f0e pop rbx
140764f0f pop rdi
140764f10 pop rsi
140764f11 pop r12
140764f13 pop r13
140764f15 pop r14
140764f17 pop r15
140764f19 pop rbp
140764f1a ret 
140764f1b mov rax, rbp
140764f1e mov qword ptr [rbp + 0x10], rax
140764f22 lea rax, [rip + 0x6279e7]
140764f29 mov qword ptr [rbp + 0x18], rax
140764f2d lea rax, [rip + 0x24b56a4]
140764f34 mov qword ptr [rbp + 0x28], rax
140764f38 mov qword ptr [rbp + 0x30], 1
140764f40 mov qword ptr [rbp + 0x48], 0
140764f48 lea rax, [rbp + 0x10]
140764f4c mov qword ptr [rbp + 0x38], rax
140764f50 mov qword ptr [rbp + 0x40], 1
140764f58 lea rdx, [rip + 0x2318b31]
140764f5f lea rcx, [rbp + 0x28]
140764f63 call 0x1428d9390
140764f68 jmp 0x1407650fc
140764f6d lea rcx, [rip + 0x24b5674]
140764f74 lea r8, [rip + 0x2318b15]
140764f7b mov edx, 0x1d
140764f80 call 0x1428d9430
140764f85 jmp 0x1407650fc
140764f8a lea rax, [rbp + 0x10]
140764f8e mov qword ptr [rbp + 0xa0], rax
140764f95 lea rax, [rip + 0x627974]
140764f9c mov qword ptr [rbp + 0xa8], rax
140764fa3 lea rax, [rip + 0x24b562e]
140764faa mov qword ptr [rbp + 0x28], rax
140764fae mov qword ptr [rbp + 0x30], 1
140764fb6 mov qword ptr [rbp + 0x48], 0
140764fbe mov qword ptr [rbp + 0x38], r15
140764fc2 mov qword ptr [rbp + 0x40], 1
140764fca lea rdx, [rip + 0x2318a17]
140764fd1 lea rcx, [rbp + 0x28]
140764fd5 call 0x1428d9390
140764fda jmp 0x1407650fc
140764fdf lea r8, [rip + 0x24b9ad2]
140764fe6 mov rcx, rax
140764fe9 mov rdx, rbx
140764fec call 0x1428d9518
140764ff1 lea r8, [rip + 0x24b3e68]
140764ff8 mov rcx, rax
140764ffb call 0x1428d9518
140765000 jmp 0x1407650fc
140765005 lea r8, [rip + 0x24b3e3c]
14076500c call 0x1428d9518
140765011 jmp 0x1407650fc
140765016 lea r8, [rip + 0x231896b]
14076501d xor ecx, ecx
14076501f xor edx, edx
140765021 call 0x1428d9518
140765026 lea r8, [rip + 0x2318973]
14076502d xor ecx, ecx
14076502f xor edx, edx
140765031 call 0x1428d9518
140765036 lea r8, [rip + 0x231897b]
14076503d mov rdx, rcx
140765040 call 0x1428d9518
140765045 lea r8, [rip + 0x24b4f24]
14076504c mov rcx, rdi
14076504f mov rdx, r12
140765052 call 0x1428d8fe3
140765057 movups xmm0, xmmword ptr [rbp + 0x30]
14076505b movaps xmmword ptr [rbp + 0xa0], xmm0
140765062 lea rax, [rip + 0x2318967]
140765069 mov qword ptr [rsp + 0x20], rax
14076506e lea rcx, [rip + 0x24b3b53]
140765075 lea r9, [rip + 0x24b3b2c]
14076507c lea r8, [rbp + 0xa0]
140765083 mov edx, 0x2b
140765088 call 0x1428d9760
14076508d movups xmm0, xmmword ptr [rbp + 0x30]
140765091 movaps xmmword ptr [rbp - 0x40], xmm0
140765095 lea rax, [rip + 0x24b42dc]
14076509c mov qword ptr [rsp + 0x20], rax
1407650a1 lea rcx, [rip + 0x24b3b20]
1407650a8 lea r9, [rip + 0x24b3af9]
1407650af lea r8, [rbp - 0x40]
1407650b3 mov edx, 0x2b
1407650b8 call 0x1428d9760
1407650bd jmp 0x1407650fc
1407650bf movups xmm0, xmmword ptr [rbp + 0x30]
1407650c3 movaps xmmword ptr [rbp + 0xa0], xmm0
1407650ca mov byte ptr [rbp + 0xe7], 0
1407650d1 lea rax, [rip + 0x2318928]
1407650d8 mov qword ptr [rsp + 0x20], rax
1407650dd lea rcx, [rip + 0x24b3ae4]
1407650e4 lea r9, [rip + 0x24b3abd]
1407650eb lea r8, [rbp + 0xa0]
1407650f2 mov edx, 0x2b
1407650f7 call 0x1428d9760
1407650fc ud2 
