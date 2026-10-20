1400c9130 push rbp
1400c9131 push r14
1400c9133 push rsi
1400c9134 push rdi
1400c9135 push rbx
1400c9136 sub rsp, 0x580
1400c913d lea rbp, [rsp + 0x80]
1400c9145 mov qword ptr [rbp + 0x4f8], 0xfffffffffffffffe
1400c9150 mov ebx, edx
1400c9152 mov rsi, rcx
1400c9155 movzx eax, byte ptr [rip + 0x356baad]
1400c915c mov ecx, 0x380
1400c9161 mov edx, 8
1400c9166 call 0x140613c10
1400c916b test rax, rax
1400c916e je 0x1400c96f7
1400c9174 mov qword ptr [rbp + 0x4f0], rax
1400c917b xorps xmm0, xmm0
1400c917e movups xmmword ptr [rbp + 0x2d8], xmm0
1400c9185 movups xmmword ptr [rbp + 0x2f0], xmm0
1400c918c movups xmmword ptr [rbp + 0x308], xmm0
1400c9193 movups xmmword ptr [rbp + 0x320], xmm0
1400c919a movups xmmword ptr [rbp + 0x338], xmm0
1400c91a1 mov dword ptr [rbp + 0x348], 0
1400c91ab movups xmmword ptr [rbp + 0x370], xmm0
1400c91b2 movups xmmword ptr [rbp + 0x388], xmm0
1400c91b9 movups xmmword ptr [rbp + 0x3a0], xmm0
1400c91c0 movups xmmword ptr [rbp + 0x3b8], xmm0
1400c91c7 movups xmmword ptr [rbp + 0x3d0], xmm0
1400c91ce mov dword ptr [rbp + 0x3e0], 0
1400c91d8 mov qword ptr [rbp + 0x2b8], 0
1400c91e3 mov qword ptr [rbp + 0x2c0], 8
1400c91ee mov qword ptr [rbp + 0x2c8], 0
1400c91f9 mov qword ptr [rbp + 0x2d0], 0x10
1400c9204 mov qword ptr [rbp + 0x2e8], 0x10
1400c920f mov qword ptr [rbp + 0x300], 0x10
1400c921a mov qword ptr [rbp + 0x318], 0x10
1400c9225 mov qword ptr [rbp + 0x330], 0x10
1400c9230 movabs r14, 0x8000000000000000
1400c923a mov qword ptr [rbp + 0x350], r14
1400c9241 lea rax, [rip + 0x286da2a]
1400c9248 mov qword ptr [rbp + 0x358], rax
1400c924f mov qword ptr [rbp + 0x360], 0x52
1400c925a mov qword ptr [rbp + 0x368], 0x10
1400c9265 mov qword ptr [rbp + 0x380], 0x10
1400c9270 mov qword ptr [rbp + 0x398], 0x10
1400c927b mov qword ptr [rbp + 0x3b0], 0x10
1400c9286 mov qword ptr [rbp + 0x3c8], 0x10
1400c9291 mov dword ptr [rbp + 0x3e8], 0
1400c929b mov word ptr [rbp + 0x3ec], 1
1400c92a4 mov qword ptr [rbp + 0x3f0], r14
1400c92ab mov dword ptr [rbp + 0x4e8], 0
1400c92b5 movzx eax, byte ptr [rip + 0x356b94d]
1400c92bc mov ecx, 0x238
1400c92c1 mov edx, 8
1400c92c6 call 0x140613c10
1400c92cb test rax, rax
1400c92ce je 0x1400c9706
1400c92d4 mov rdi, rax
1400c92d7 lea rdx, [rbp + 0x2b8]
1400c92de mov r8d, 0x238
1400c92e4 mov rcx, rax
1400c92e7 call 0x142923c00
1400c92ec lea r8, [rip + 0x286d41d]
1400c92f3 lea rcx, [rbp + 0x1d8]
1400c92fa mov rdx, rdi
1400c92fd call 0x140ce4b10
1400c9302 lea rcx, [rbp + 0x168]
1400c9309 call 0x140374dc0
1400c930e lea rcx, [rbp + 0xf8]
1400c9315 call 0x1403e9d80
1400c931a lea rcx, [rbp + 0x88]
1400c9321 call 0x14031a6f0
1400c9326 lea rcx, [rbp + 0x18]
1400c932a call 0x140613490
1400c932f lea rcx, [rbp - 0x58]
1400c9333 call 0x140533150
1400c9338 lea rcx, [rbp + 0x248]
1400c933f call 0x14002d070
1400c9344 lea rcx, [rbp + 0x2b8]
1400c934b call 0x14002d220
1400c9350 movups xmm0, xmmword ptr [rbp + 0x238]
1400c9357 mov rax, qword ptr [rbp + 0x4f0]
1400c935e movups xmmword ptr [rax + 0x60], xmm0
1400c9362 movups xmm0, xmmword ptr [rbp + 0x228]
1400c9369 movups xmmword ptr [rax + 0x50], xmm0
1400c936d movups xmm0, xmmword ptr [rbp + 0x218]
1400c9374 movups xmmword ptr [rax + 0x40], xmm0
1400c9378 movups xmm0, xmmword ptr [rbp + 0x1d8]
1400c937f movups xmm1, xmmword ptr [rbp + 0x1e8]
1400c9386 movups xmm2, xmmword ptr [rbp + 0x1f8]
1400c938d movups xmm3, xmmword ptr [rbp + 0x208]
1400c9394 movups xmmword ptr [rax + 0x30], xmm3
1400c9398 movups xmmword ptr [rax + 0x20], xmm2
1400c939c movups xmmword ptr [rax + 0x10], xmm1
1400c93a0 movups xmmword ptr [rax], xmm0
1400c93a3 movups xmm0, xmmword ptr [rbp + 0x168]
1400c93aa movups xmm1, xmmword ptr [rbp + 0x178]
1400c93b1 movups xmm2, xmmword ptr [rbp + 0x188]
1400c93b8 movups xmm3, xmmword ptr [rbp + 0x198]
1400c93bf movups xmmword ptr [rax + 0x70], xmm0
1400c93c3 movups xmmword ptr [rax + 0x80], xmm1
1400c93ca movups xmmword ptr [rax + 0x90], xmm2
1400c93d1 movups xmmword ptr [rax + 0xa0], xmm3
1400c93d8 movups xmm0, xmmword ptr [rbp + 0x1a8]
1400c93df movups xmmword ptr [rax + 0xb0], xmm0
1400c93e6 movups xmm0, xmmword ptr [rbp + 0x1b8]
1400c93ed movups xmmword ptr [rax + 0xc0], xmm0
1400c93f4 movups xmm0, xmmword ptr [rbp + 0x1c8]
1400c93fb movups xmmword ptr [rax + 0xd0], xmm0
1400c9402 movups xmm0, xmmword ptr [rbp + 0x158]
1400c9409 movups xmmword ptr [rax + 0x140], xmm0
1400c9410 movups xmm0, xmmword ptr [rbp + 0x148]
1400c9417 movups xmmword ptr [rax + 0x130], xmm0
1400c941e movups xmm0, xmmword ptr [rbp + 0x138]
1400c9425 movups xmmword ptr [rax + 0x120], xmm0
1400c942c movups xmm0, xmmword ptr [rbp + 0xf8]
1400c9433 movups xmm1, xmmword ptr [rbp + 0x108]
1400c943a movups xmm2, xmmword ptr [rbp + 0x118]
1400c9441 movups xmm3, xmmword ptr [rbp + 0x128]
1400c9448 movups xmmword ptr [rax + 0x110], xmm3
1400c944f movups xmmword ptr [rax + 0x100], xmm2
1400c9456 movups xmmword ptr [rax + 0xf0], xmm1
1400c945d movups xmmword ptr [rax + 0xe0], xmm0
1400c9464 movups xmm0, xmmword ptr [rbp + 0x88]
1400c946b movups xmm1, xmmword ptr [rbp + 0x98]
1400c9472 movups xmm2, xmmword ptr [rbp + 0xa8]
1400c9479 movups xmm3, xmmword ptr [rbp + 0xb8]
1400c9480 movups xmmword ptr [rax + 0x150], xmm0
1400c9487 movups xmmword ptr [rax + 0x160], xmm1
1400c948e movups xmmword ptr [rax + 0x170], xmm2
1400c9495 movups xmmword ptr [rax + 0x180], xmm3
1400c949c movups xmm0, xmmword ptr [rbp + 0xc8]
1400c94a3 movups xmmword ptr [rax + 0x190], xmm0
1400c94aa movups xmm0, xmmword ptr [rbp + 0xd8]
1400c94b1 movups xmmword ptr [rax + 0x1a0], xmm0
1400c94b8 movups xmm0, xmmword ptr [rbp + 0xe8]
1400c94bf movups xmmword ptr [rax + 0x1b0], xmm0
1400c94c6 movups xmm0, xmmword ptr [rbp + 0x18]
1400c94ca movups xmm1, xmmword ptr [rbp + 0x28]
1400c94ce movups xmm2, xmmword ptr [rbp + 0x38]
1400c94d2 movups xmm3, xmmword ptr [rbp + 0x48]
1400c94d6 movups xmmword ptr [rax + 0x1c0], xmm0
1400c94dd movups xmmword ptr [rax + 0x1d0], xmm1
1400c94e4 movups xmmword ptr [rax + 0x1e0], xmm2
1400c94eb movups xmmword ptr [rax + 0x1f0], xmm3
1400c94f2 movups xmm0, xmmword ptr [rbp + 0x58]
1400c94f6 movups xmmword ptr [rax + 0x200], xmm0
1400c94fd movups xmm0, xmmword ptr [rbp + 0x78]
1400c9501 movups xmmword ptr [rax + 0x220], xmm0
1400c9508 movups xmm0, xmmword ptr [rbp + 0x68]
1400c950c movups xmmword ptr [rax + 0x210], xmm0
1400c9513 movups xmm0, xmmword ptr [rbp + 8]
1400c9517 movups xmmword ptr [rax + 0x290], xmm0
1400c951e movups xmm0, xmmword ptr [rbp - 8]
1400c9522 movups xmmword ptr [rax + 0x280], xmm0
1400c9529 movups xmm0, xmmword ptr [rbp - 0x18]
1400c952d movups xmmword ptr [rax + 0x270], xmm0
1400c9534 movups xmm0, xmmword ptr [rbp - 0x58]
1400c9538 movups xmm1, xmmword ptr [rbp - 0x48]
1400c953c movups xmm2, xmmword ptr [rbp - 0x38]
1400c9540 movups xmm3, xmmword ptr [rbp - 0x28]
1400c9544 movups xmmword ptr [rax + 0x260], xmm3
1400c954b movups xmmword ptr [rax + 0x250], xmm2
1400c9552 movups xmmword ptr [rax + 0x240], xmm1
1400c9559 movups xmmword ptr [rax + 0x230], xmm0
1400c9560 movups xmm0, xmmword ptr [rbp + 0x248]
1400c9567 movups xmm1, xmmword ptr [rbp + 0x258]
1400c956e movups xmm2, xmmword ptr [rbp + 0x268]
1400c9575 movups xmm3, xmmword ptr [rbp + 0x278]
1400c957c movups xmmword ptr [rax + 0x2a0], xmm0
1400c9583 movups xmmword ptr [rax + 0x2b0], xmm1
1400c958a movups xmmword ptr [rax + 0x2c0], xmm2
1400c9591 movups xmmword ptr [rax + 0x2d0], xmm3
1400c9598 movups xmm0, xmmword ptr [rbp + 0x288]
1400c959f movups xmmword ptr [rax + 0x2e0], xmm0
1400c95a6 movups xmm0, xmmword ptr [rbp + 0x298]
1400c95ad movups xmmword ptr [rax + 0x2f0], xmm0
1400c95b4 movups xmm0, xmmword ptr [rbp + 0x2a8]
1400c95bb movups xmmword ptr [rax + 0x300], xmm0
1400c95c2 movups xmm0, xmmword ptr [rbp + 0x2b8]
1400c95c9 movups xmm1, xmmword ptr [rbp + 0x2c8]
1400c95d0 movups xmm2, xmmword ptr [rbp + 0x2d8]
1400c95d7 movups xmm3, xmmword ptr [rbp + 0x2e8]
1400c95de movups xmmword ptr [rax + 0x310], xmm0
1400c95e5 movups xmmword ptr [rax + 0x320], xmm1
1400c95ec movups xmmword ptr [rax + 0x330], xmm2
1400c95f3 movups xmmword ptr [rax + 0x340], xmm3
1400c95fa movups xmm0, xmmword ptr [rbp + 0x2f8]
1400c9601 movups xmmword ptr [rax + 0x350], xmm0
1400c9608 movups xmm0, xmmword ptr [rbp + 0x318]
1400c960f movups xmmword ptr [rax + 0x370], xmm0
1400c9616 movups xmm0, xmmword ptr [rbp + 0x308]
1400c961d movups xmmword ptr [rax + 0x360], xmm0
1400c9624 mov qword ptr [rbp + 0x2c0], 8
1400c962f mov qword ptr [rbp + 0x2c8], rax
1400c9636 mov qword ptr [rbp + 0x2d0], 8
1400c9641 mov qword ptr [rbp + 0x2d8], 0
1400c964c mov qword ptr [rbp + 0x2e0], 8
1400c9657 xorps xmm0, xmm0
1400c965a movups xmmword ptr [rbp + 0x2e8], xmm0
1400c9661 mov qword ptr [rbp + 0x2b8], r14
1400c9668 mov byte ptr [rbp + 0x248], bl
1400c966e mov rcx, qword ptr [rip + 0x3562cfb]
1400c9675 lea r8, [rip + 0x286191c]
1400c967c lea rdx, [rbp + 0x248]
1400c9683 call 0x140364d20
1400c9688 mov r8, rdx
1400c968b lea rcx, [rbp + 0x2b8]
1400c9692 mov rdx, rax
1400c9695 call 0x1400d68c0
1400c969a movups xmm0, xmmword ptr [rbp + 0x318]
1400c96a1 movups xmmword ptr [rsi + 0x60], xmm0
1400c96a5 movups xmm0, xmmword ptr [rbp + 0x308]
1400c96ac movups xmmword ptr [rsi + 0x50], xmm0
1400c96b0 movups xmm0, xmmword ptr [rbp + 0x2f8]
1400c96b7 movups xmmword ptr [rsi + 0x40], xmm0
1400c96bb movups xmm0, xmmword ptr [rbp + 0x2b8]
1400c96c2 movups xmm1, xmmword ptr [rbp + 0x2c8]
1400c96c9 movups xmm2, xmmword ptr [rbp + 0x2d8]
1400c96d0 movups xmm3, xmmword ptr [rbp + 0x2e8]
1400c96d7 movups xmmword ptr [rsi + 0x30], xmm3
1400c96db movups xmmword ptr [rsi + 0x20], xmm2
1400c96df movups xmmword ptr [rsi + 0x10], xmm1
1400c96e3 movups xmmword ptr [rsi], xmm0
1400c96e6 mov rax, rsi
1400c96e9 add rsp, 0x580
1400c96f0 pop rbx
1400c96f1 pop rdi
1400c96f2 pop rsi
1400c96f3 pop r14
1400c96f5 pop rbp
1400c96f6 ret 
1400c96f7 mov ecx, 8
1400c96fc mov edx, 0x380
1400c9701 call 0x1428d9000
1400c9706 mov ecx, 8
1400c970b mov edx, 0x238
1400c9710 call 0x1428d9000
1400c9715 ud2 
1400c9717 nop word ptr [rax + rax]
1400c9720 mov qword ptr [rsp + 0x10], rdx
1400c9725 push rbp
1400c9726 push r14
1400c9728 push rsi
1400c9729 push rdi
1400c972a push rbx
1400c972b sub rsp, 0x20
1400c972f lea rbp, [rdx + 0x80]
1400c9736 lea rcx, [rbp + 0x2b8]
1400c973d call 0x1405d79a0
1400c9742 nop 
1400c9743 add rsp, 0x20
1400c9747 pop rbx
1400c9748 pop rdi
1400c9749 pop rsi
1400c974a pop r14
1400c974c pop rbp
1400c974d ret 
1400c974e nop 
1400c9750 mov qword ptr [rsp + 0x10], rdx
1400c9755 push rbp
1400c9756 push r14
1400c9758 push rsi
1400c9759 push rdi
1400c975a push rbx
1400c975b sub rsp, 0x20
1400c975f lea rbp, [rdx + 0x80]
1400c9766 mov edx, 0x380
1400c976b mov r8d, 8
1400c9771 mov rcx, qword ptr [rbp + 0x4f0]
1400c9778 call 0x140613c20
1400c977d nop 
1400c977e add rsp, 0x20
1400c9782 pop rbx
1400c9783 pop rdi
1400c9784 pop rsi
1400c9785 pop r14
1400c9787 pop rbp
1400c9788 ret 
1400c9789 nop dword ptr [rax]
1400c9790 mov qword ptr [rsp + 0x10], rdx
1400c9795 push rbp
1400c9796 push r14
1400c9798 push rsi
1400c9799 push rdi
1400c979a push rbx
1400c979b sub rsp, 0x20
1400c979f lea rbp, [rdx + 0x80]
1400c97a6 lea rcx, [rbp + 0x1d8]
1400c97ad call 0x140029880
1400c97b2 nop 
1400c97b3 add rsp, 0x20
1400c97b7 pop rbx
1400c97b8 pop rdi
1400c97b9 pop rsi
1400c97ba pop r14
1400c97bc pop rbp
1400c97bd ret 
1400c97be nop 
1400c97c0 mov qword ptr [rsp + 0x10], rdx
1400c97c5 push rbp
1400c97c6 push r14
1400c97c8 push rsi
1400c97c9 push rdi
1400c97ca push rbx
1400c97cb sub rsp, 0x20
1400c97cf lea rbp, [rdx + 0x80]
1400c97d6 lea rcx, [rbp + 0x168]
1400c97dd call 0x140029880
1400c97e2 nop 
1400c97e3 add rsp, 0x20
1400c97e7 pop rbx
1400c97e8 pop rdi
1400c97e9 pop rsi
1400c97ea pop r14
1400c97ec pop rbp
1400c97ed ret 
1400c97ee nop 
1400c97f0 mov qword ptr [rsp + 0x10], rdx
1400c97f5 push rbp
1400c97f6 push r14
1400c97f8 push rsi
1400c97f9 push rdi
1400c97fa push rbx
1400c97fb sub rsp, 0x20
1400c97ff lea rbp, [rdx + 0x80]
1400c9806 lea rcx, [rbp + 0xf8]
1400c980d call 0x140029880
1400c9812 nop 
1400c9813 add rsp, 0x20
1400c9817 pop rbx
1400c9818 pop rdi
1400c9819 pop rsi
1400c981a pop r14
1400c981c pop rbp
1400c981d ret 
1400c981e nop 
1400c9820 mov qword ptr [rsp + 0x10], rdx
1400c9825 push rbp
1400c9826 push r14
1400c9828 push rsi
1400c9829 push rdi
1400c982a push rbx
1400c982b sub rsp, 0x20
1400c982f lea rbp, [rdx + 0x80]
1400c9836 lea rcx, [rbp + 0x88]
1400c983d call 0x140029880
1400c9842 nop 
1400c9843 add rsp, 0x20
1400c9847 pop rbx
1400c9848 pop rdi
1400c9849 pop rsi
1400c984a pop r14
1400c984c pop rbp
1400c984d ret 
1400c984e nop 
1400c9850 mov qword ptr [rsp + 0x10], rdx
1400c9855 push rbp
1400c9856 push r14
1400c9858 push rsi
1400c9859 push rdi
1400c985a push rbx
1400c985b sub rsp, 0x20
1400c985f lea rbp, [rdx + 0x80]
1400c9866 lea rcx, [rbp + 0x18]
1400c986a call 0x140029880
1400c986f nop 
1400c9870 add rsp, 0x20
1400c9874 pop rbx
1400c9875 pop rdi
1400c9876 pop rsi
1400c9877 pop r14
1400c9879 pop rbp
1400c987a ret 
1400c987b nop dword ptr [rax + rax]
1400c9880 mov qword ptr [rsp + 0x10], rdx
1400c9885 push rbp
1400c9886 push r14
1400c9888 push rsi
1400c9889 push rdi
1400c988a push rbx
1400c988b sub rsp, 0x20
1400c988f lea rbp, [rdx + 0x80]
1400c9896 lea rcx, [rbp - 0x58]
1400c989a call 0x140029880
1400c989f nop 
1400c98a0 add rsp, 0x20
1400c98a4 pop rbx
1400c98a5 pop rdi
1400c98a6 pop rsi
1400c98a7 pop r14
1400c98a9 pop rbp
1400c98aa ret 
1400c98ab nop dword ptr [rax + rax]
1400c98b0 mov qword ptr [rsp + 0x10], rdx
1400c98b5 push rbp
1400c98b6 push r14
1400c98b8 push rsi
1400c98b9 push rdi
1400c98ba push rbx
1400c98bb sub rsp, 0x20
1400c98bf lea rbp, [rdx + 0x80]
1400c98c6 lea rcx, [rbp + 0x248]
1400c98cd call 0x140029880
1400c98d2 nop 
1400c98d3 add rsp, 0x20
1400c98d7 pop rbx
1400c98d8 pop rdi
1400c98d9 pop rsi
1400c98da pop r14
1400c98dc pop rbp
1400c98dd ret 
1400c98de nop 
1400c98e0 mov qword ptr [rsp + 0x10], rdx
1400c98e5 push rbp
1400c98e6 push r14
1400c98e8 push rsi
1400c98e9 push rdi
1400c98ea push rbx
1400c98eb sub rsp, 0x20
1400c98ef lea rbp, [rdx + 0x80]
1400c98f6 lea rcx, [rbp + 0x2b8]
1400c98fd call 0x140029880
1400c9902 nop 
1400c9903 add rsp, 0x20
1400c9907 pop rbx
1400c9908 pop rdi
1400c9909 pop rsi
1400c990a pop r14
1400c990c pop rbp
1400c990d ret 