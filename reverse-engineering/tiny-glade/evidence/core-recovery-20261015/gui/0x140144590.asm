0x140144590 push     rbp
0x140144591 push     r15
0x140144593 push     r14
0x140144595 push     rsi
0x140144596 push     rdi
0x140144597 push     rbx
0x140144598 sub      rsp, 0x358
0x14014459f lea      rbp, [rsp + 0x80]
0x1401445a7 movaps   xmmword ptr [rbp + 0x2c0], xmm6
0x1401445ae mov      qword ptr [rbp + 0x2b8], 0xfffffffffffffffe
0x1401445b9 mov      rsi, rdx
0x1401445bc mov      rdi, rcx
0x1401445bf mov      qword ptr [rbp + 0x150], 4
0x1401445ca mov      byte ptr [rbp + 0x2b7], 0
0x1401445d1 lea      rdx, [rip + 0x280cdd0]
0x1401445d8 lea      rcx, [rbp - 0x58]
0x1401445dc mov      r8d, 3
0x1401445e2 call     0x1400568d0
0x1401445e7 movups   xmm0, xmmword ptr [rbp + 8]
0x1401445eb movaps   xmmword ptr [rbp + 0x2a0], xmm0
0x1401445f2 movups   xmm0, xmmword ptr [rbp - 8]
0x1401445f6 movaps   xmmword ptr [rbp + 0x290], xmm0
0x1401445fd movups   xmm0, xmmword ptr [rbp - 0x18]
0x140144601 movaps   xmmword ptr [rbp + 0x280], xmm0
0x140144608 movups   xmm0, xmmword ptr [rbp - 0x58]
0x14014460c movups   xmm1, xmmword ptr [rbp - 0x48]
0x140144610 movups   xmm2, xmmword ptr [rbp - 0x38]
0x140144614 movups   xmm3, xmmword ptr [rbp - 0x28]
0x140144618 movaps   xmmword ptr [rbp + 0x270], xmm3
0x14014461f movaps   xmmword ptr [rbp + 0x260], xmm2
0x140144626 movaps   xmmword ptr [rbp + 0x250], xmm1
0x14014462d movaps   xmmword ptr [rbp + 0x240], xmm0
0x140144634 mov      byte ptr [rbp + 0x2b7], 1
0x14014463b lea      rcx, [rbp - 0x58]
0x14014463f lea      rdx, [rbp + 0x150]
0x140144646 lea      r8, [rbp + 0x240]
0x14014464d call     0x140c46bd0
0x140144652 lea      rcx, [rbp + 0x1e0]
0x140144659 lea      rdx, [rbp - 0x58]
0x14014465d mov      r8, rsi
0x140144660 call     0x140c46d20
0x140144665 cmp      byte ptr [rbp + 0x234], 0
0x14014466c je       0x14014469f
0x14014466e mov      rax, qword ptr [rbp + 0x1f0]
0x140144675 lock dec qword ptr [rax]
0x140144679 jne      0x140144687
0x14014467b lea      rcx, [rbp + 0x1f0]
0x140144682 call     0x140c41170
0x140144687 mov      rax, qword ptr [rdi]
0x14014468a mov      rcx, qword ptr [rdi + 8]
0x14014468e movss    xmm0, dword ptr [rcx]
0x140144692 mov      dword ptr [rax], 0
0x140144698 movss    dword ptr [rax + 4], xmm0
0x14014469d jmp      0x1401446d9
0x14014469f mov      byte ptr [rbp - 0x58], 0
0x1401446a3 cmp      byte ptr [rbp + 0x233], 0
0x1401446aa je       0x1401446c0
0x1401446ac lea      rcx, [rbp + 0x1f0]
0x1401446b3 lea      rdx, [rbp - 0x58]
0x1401446b7 call     0x140042870
0x1401446bc test     al, al
0x1401446be jne      0x14014466e
0x1401446c0 mov      rax, qword ptr [rbp + 0x1f0]
0x1401446c7 lock dec qword ptr [rax]
0x1401446cb jne      0x1401446d9
0x1401446cd lea      rcx, [rbp + 0x1f0]
0x1401446d4 call     0x140c41170
0x1401446d9 mov      r14, qword ptr [rdi + 0x20]
0x1401446dd cmp      byte ptr [r14], 0
0x1401446e1 je       0x1401447b3
0x1401446e7 mov      qword ptr [rbp + 0x150], 4
0x1401446f2 mov      byte ptr [rbp + 0x2b5], 0
0x1401446f9 lea      rdx, [rip + 0x280ccab] ; @0x1429513ab '...Delete (2-click)Time ()'
0x140144700 lea      rcx, [rbp - 0x58]
0x140144704 mov      r8d, 3
0x14014470a call     0x1400568d0
0x14014470f movups   xmm0, xmmword ptr [rbp + 8]
0x140144713 movaps   xmmword ptr [rbp + 0x2a0], xmm0
0x14014471a movups   xmm0, xmmword ptr [rbp - 8]
0x14014471e movaps   xmmword ptr [rbp + 0x290], xmm0
0x140144725 movups   xmm0, xmmword ptr [rbp - 0x18]
0x140144729 movaps   xmmword ptr [rbp + 0x280], xmm0
0x140144730 movups   xmm0, xmmword ptr [rbp - 0x58]
0x140144734 movups   xmm1, xmmword ptr [rbp - 0x48]
0x140144738 movups   xmm2, xmmword ptr [rbp - 0x38]
0x14014473c movups   xmm3, xmmword ptr [rbp - 0x28]
0x140144740 movaps   xmmword ptr [rbp + 0x270], xmm3
0x140144747 movaps   xmmword ptr [rbp + 0x260], xmm2
0x14014474e movaps   xmmword ptr [rbp + 0x250], xmm1
0x140144755 movaps   xmmword ptr [rbp + 0x240], xmm0
0x14014475c mov      byte ptr [rbp + 0x2b5], 1
0x140144763 lea      rcx, [rbp - 0x58]
0x140144767 lea      rdx, [rbp + 0x150]
0x14014476e lea      r8, [rbp + 0x240]
0x140144775 call     0x140c46bd0
0x14014477a lea      rcx, [rbp + 0x1e0]
0x140144781 lea      rdx, [rbp - 0x58]
0x140144785 mov      r8, rsi
0x140144788 call     0x140c46d20
0x14014478d cmp      byte ptr [rbp + 0x234], 0
0x140144794 je       0x140144892
0x14014479a mov      rax, qword ptr [rbp + 0x1f0]
0x1401447a1 lock dec qword ptr [rax]
0x1401447a5 jne      0x1401447b3
0x1401447a7 lea      rcx, [rbp + 0x1f0]
0x1401447ae call     0x140c41170
0x1401447b3 mov      qword ptr [rbp + 0x150], 4
0x1401447be mov      byte ptr [rbp + 0x2b6], 0
0x1401447c5 lea      rdx, [rip + 0x280cbdf] ; @0x1429513ab '...Delete (2-click)Time ()'
0x1401447cc lea      rcx, [rbp - 0x58]
0x1401447d0 mov      r8d, 3
0x1401447d6 call     0x1400568d0
0x1401447db movups   xmm0, xmmword ptr [rbp + 8]
0x1401447df movaps   xmmword ptr [rbp + 0x2a0], xmm0
0x1401447e6 movups   xmm0, xmmword ptr [rbp - 8]
0x1401447ea movaps   xmmword ptr [rbp + 0x290], xmm0
0x1401447f1 movups   xmm0, xmmword ptr [rbp - 0x18]
0x1401447f5 movaps   xmmword ptr [rbp + 0x280], xmm0
0x1401447fc movups   xmm0, xmmword ptr [rbp - 0x58]
0x140144800 movups   xmm1, xmmword ptr [rbp - 0x48]
0x140144804 movups   xmm2, xmmword ptr [rbp - 0x38]
0x140144808 movups   xmm3, xmmword ptr [rbp - 0x28]
0x14014480c movaps   xmmword ptr [rbp + 0x270], xmm3
0x140144813 movaps   xmmword ptr [rbp + 0x260], xmm2
0x14014481a movaps   xmmword ptr [rbp + 0x250], xmm1
0x140144821 movaps   xmmword ptr [rbp + 0x240], xmm0
0x140144828 mov      byte ptr [rbp + 0x2b6], 1
0x14014482f lea      rcx, [rbp - 0x58]
0x140144833 lea      rdx, [rbp + 0x150]
0x14014483a lea      r8, [rbp + 0x240]
0x140144841 call     0x140c46bd0
0x140144846 lea      rcx, [rbp + 0x150]
0x14014484d lea      rdx, [rbp - 0x58]
0x140144851 mov      r8, rsi
0x140144854 call     0x140c46d20
0x140144859 mov      bl, 1
0x14014485b cmp      byte ptr [rbp + 0x1a4], 0
0x140144862 jne      0x140144883
0x140144864 mov      byte ptr [rbp - 0x58], 0
0x140144868 cmp      byte ptr [rbp + 0x1a3], 1
0x14014486f jne      0x1401448d4
0x140144871 lea      rcx, [rbp + 0x160]
0x140144878 lea      rdx, [rbp - 0x58]
0x14014487c call     0x140042870
0x140144881 mov      ebx, eax
0x140144883 mov      rax, qword ptr [rbp + 0x160]
0x14014488a lock dec qword ptr [rax]
0x14014488e je       0x1401448e3
0x140144890 jmp      0x1401448ef
0x140144892 mov      byte ptr [rbp - 0x58], 0
0x140144896 cmp      byte ptr [rbp + 0x233], 0
0x14014489d je       0x1401448b7
0x14014489f lea      rcx, [rbp + 0x1f0]
0x1401448a6 lea      rdx, [rbp - 0x58]
0x1401448aa call     0x140042870
0x1401448af test     al, al
0x1401448b1 jne      0x14014479a
0x1401448b7 mov      rax, qword ptr [rbp + 0x1f0]
0x1401448be lock dec qword ptr [rax]
0x1401448c2 jne      0x1401448d0
0x1401448c4 lea      rcx, [rbp + 0x1f0]
0x1401448cb call     0x140c41170
0x1401448d0 mov      bl, 1
0x1401448d2 jmp      0x1401448ef
0x1401448d4 xor      ebx, ebx
0x1401448d6 mov      rax, qword ptr [rbp + 0x160]
0x1401448dd lock dec qword ptr [rax]
0x1401448e1 jne      0x1401448ef
0x1401448e3 lea      rcx, [rbp + 0x160]
0x1401448ea call     0x140c41170
0x1401448ef mov      byte ptr [r14], bl
0x1401448f2 movups   xmm6, xmmword ptr [rdi + 8]
0x1401448f6 mov      rbx, qword ptr [rdi + 0x18]
0x1401448fa mov      r15, qword ptr [rdi + 0x28]
0x1401448fe mov      rdi, qword ptr [rdi]
0x140144901 movzx    eax, byte ptr [rip + 0x34f0301]
0x140144908 mov      ecx, 0x30
0x14014490d mov      edx, 8
0x140144912 call     0x140613c10
0x140144917 test     rax, rax
0x14014491a je       0x14014497e
0x14014491c movups   xmmword ptr [rax], xmm6
0x14014491f mov      qword ptr [rax + 0x10], rbx
0x140144923 mov      qword ptr [rax + 0x18], r15
0x140144927 mov      qword ptr [rax + 0x20], r14
0x14014492b mov      qword ptr [rax + 0x28], rdi
0x14014492f lea      rcx, [rip + 0x280c782]
0x140144936 mov      qword ptr [rsp + 0x20], rcx
0x14014493b lea      rcx, [rbp - 0x58]
0x14014493f movabs   r8, 0x20001000000
0x140144949 mov      rdx, rsi
0x14014494c mov      r9, rax
0x14014494f call     0x1400c2d20
0x140144954 mov      rax, qword ptr [rbp - 0x48]
0x140144958 lock dec qword ptr [rax]
0x14014495c jne      0x140144967
0x14014495e lea      rcx, [rbp - 0x48]
0x140144962 call     0x140c41170
0x140144967 movaps   xmm6, xmmword ptr [rbp + 0x2c0]
0x14014496e add      rsp, 0x358
0x140144975 pop      rbx
0x140144976 pop      rdi
0x140144977 pop      rsi
0x140144978 pop      r14
0x14014497a pop      r15
0x14014497c pop      rbp
0x14014497d ret      
0x14014497e mov      ecx, 8
0x140144983 mov      edx, 0x30
0x140144988 call     0x1428d9000