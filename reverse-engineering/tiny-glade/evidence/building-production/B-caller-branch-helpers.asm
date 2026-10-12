
0x1408e33b0
1408e33b0 movss xmm0, dword ptr [rcx + 0x30]
1408e33b5 movss xmm1, dword ptr [rcx + 0x34]
1408e33ba movss xmm2, dword ptr [rip + 0x2042542]
1408e33c2 cmpeqps xmm2, xmm1
1408e33c6 movss xmm1, dword ptr [rip + 0x20425b6]
1408e33ce cmpnleps xmm0, xmm1
1408e33d2 andps xmm0, xmm2
1408e33d5 movd eax, xmm0
1408e33d9 and al, 1
1408e33db ret 

0x1408e3250
1408e3250 movss xmm0, dword ptr [rip + 0x204272c]
1408e3258 ucomiss xmm0, dword ptr [rcx + 0x30]
1408e325c setb al
1408e325f ret 

0x1408e3380
1408e3380 movss xmm0, dword ptr [rip + 0x20425fc]
1408e3388 ucomiss xmm0, dword ptr [rcx + 0x30]
1408e338c setae al
1408e338f ret 

0x1408e33e0
1408e33e0 mov rax, rcx
1408e33e3 movss xmm0, dword ptr [rdx + 0x34]
1408e33e8 xor ecx, ecx
1408e33ea ucomiss xmm0, dword ptr [rip + 0x2042513]
1408e33f1 jne 0x1408e3418
1408e33f3 jp 0x1408e3418
1408e33f5 movss xmm0, dword ptr [rip + 0x2042587]
1408e33fd ucomiss xmm0, dword ptr [rdx + 0x30]
1408e3401 jae 0x1408e3418
1408e3403 movzx ecx, byte ptr [rdx + 0x3c]
1408e3407 mov qword ptr [rax + 8], rcx
1408e340b or rcx, 2
1408e340f mov qword ptr [rax + 0x10], rcx
1408e3413 mov ecx, 1
1408e3418 mov qword ptr [rax], rcx
1408e341b ret 

0x140833230
140833230 cmp byte ptr [rcx], 0
140833233 sete al
140833236 ret 

0x1408c5670
1408c5670 cmp byte ptr [rcx], 2
1408c5673 setae al
1408c5676 ret 

0x140b2a620
140b2a620 sub rsp, 0x58
140b2a624 test byte ptr [rcx + 0x108], 1
140b2a62b je 0x140b2a63a
140b2a62d movss xmm0, dword ptr [rcx + 0x10c]
140b2a635 add rsp, 0x58
140b2a639 ret 

0x140b2a5d0
140b2a5d0 sub rsp, 0x28
140b2a5d4 cmp dword ptr [rcx + 0x258], 2
140b2a5db jae 0x140b2a5ec

0x1411ff740
1411ff740 sub rsp, 0x58
1411ff744 test byte ptr [rcx], 1
1411ff747 je 0x1411ff755
1411ff749 add rcx, 4
1411ff74d mov rax, rcx

0x140c996d0
140c996d0 sub rsp, 0x68
140c996d4 cmp rdx, 5
140c996d8 jae 0x140c99770
140c996de xorps xmm1, xmm1
140c996e1 lea rax, [rip + 0x1e953b4]
140c996e8 movsxd rdx, dword ptr [rax + rdx*4]
140c996ec add rdx, rax
140c996ef xorps xmm0, xmm0
140c996f2 jmp rdx
140c996f4 movss xmm0, dword ptr [rcx + 0x14]
140c996f9 movss xmm2, dword ptr [rcx + 0x10]
140c996fe addss xmm2, xmm0
140c99702 addss xmm2, xmm2
140c99706 divss xmm0, xmm2
140c9970a jmp 0x140c9974e

0x140b2a5d0
140b2a5d0 sub rsp, 0x28
140b2a5d4 cmp dword ptr [rcx + 0x258], 2
140b2a5db jae 0x140b2a5ec
140b2a5dd add rcx, 0x258
140b2a5e4 mov rax, rcx
140b2a5e7 add rsp, 0x28
140b2a5eb ret 
140b2a5ec lea rcx, [rip + 0x1fc07fd]
140b2a5f3 call 0x1428d9310
140b2a5f8 int3 
140b2a5f9 int3 
140b2a5fa int3 
140b2a5fb int3 
140b2a5fc int3 
140b2a5fd int3 
140b2a5fe int3 
140b2a5ff int3 

0x1411ff740
1411ff740 sub rsp, 0x58
1411ff744 test byte ptr [rcx], 1
1411ff747 je 0x1411ff755
1411ff749 add rcx, 4
1411ff74d mov rax, rcx
1411ff750 add rsp, 0x58
1411ff754 ret 
1411ff755 lea rax, [rip + 0x1a1a1f4]
1411ff75c mov qword ptr [rsp + 0x28], rax
1411ff761 mov qword ptr [rsp + 0x30], 1
1411ff76a mov qword ptr [rsp + 0x38], 8
1411ff773 xorps xmm0, xmm0
1411ff776 movups xmmword ptr [rsp + 0x40], xmm0
1411ff77b lea rdx, [rip + 0x1a1a1de]
1411ff782 lea rcx, [rsp + 0x28]
1411ff787 call 0x1428d9390
1411ff78c int3 
1411ff78d int3 
1411ff78e int3 
1411ff78f int3 

0x140c996d0
140c996d0 sub rsp, 0x68
140c996d4 cmp rdx, 5
140c996d8 jae 0x140c99770
140c996de xorps xmm1, xmm1
140c996e1 lea rax, [rip + 0x1e953b4]
140c996e8 movsxd rdx, dword ptr [rax + rdx*4]
140c996ec add rdx, rax
140c996ef xorps xmm0, xmm0
140c996f2 jmp rdx
140c996f4 movss xmm0, dword ptr [rcx + 0x14]
140c996f9 movss xmm2, dword ptr [rcx + 0x10]
140c996fe addss xmm2, xmm0
140c99702 addss xmm2, xmm2
140c99706 divss xmm0, xmm2
140c9970a jmp 0x140c9974e
140c9970c movss xmm0, dword ptr [rip + 0x1c8c1f0]
140c99714 jmp 0x140c9974e
140c99716 movss xmm0, dword ptr [rcx + 0x14]
140c9971b addss xmm0, dword ptr [rcx + 0x10]
140c99720 movaps xmm2, xmm0
140c99723 addss xmm2, xmm0
140c99727 divss xmm0, xmm2
140c9972b jmp 0x140c9974e
140c9972d movss xmm2, dword ptr [rcx + 0x10]
140c99732 movss xmm3, dword ptr [rcx + 0x14]
140c99737 movaps xmm0, xmm3
140c9973a addss xmm0, xmm3
140c9973e addss xmm0, xmm2
140c99742 addss xmm3, xmm2
140c99746 addss xmm3, xmm3
140c9974a divss xmm0, xmm3
140c9974e movss dword ptr [rsp + 0x24], xmm0
140c99754 ucomiss xmm0, xmm1
140c99757 jb 0x140c99788
140c99759 movss xmm1, dword ptr [rip + 0x1c8c1a3]
140c99761 ucomiss xmm1, xmm0
140c99764 jb 0x140c99788
140c99766 ucomiss xmm0, xmm0
140c99769 jp 0x140c997e0
140c9976b add rsp, 0x68
140c9976f ret 
140c99770 lea rcx, [rip + 0x1e95049]
