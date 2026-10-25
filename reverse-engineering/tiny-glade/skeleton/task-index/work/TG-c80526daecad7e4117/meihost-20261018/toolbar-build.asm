0x14068d610 push rbp
0x14068d611 push rsi
0x14068d612 push rdi
0x14068d613 sub rsp, 0x1f0
0x14068d61a lea rbp, [rsp + 0x80]
0x14068d622 mov qword ptr [rbp + 0x168], 0xfffffffffffffffe
0x14068d62d mov rsi, rdx
0x14068d630 lea rdi, [rbp + 0x10]
0x14068d634 mov rcx, rdi
0x14068d637 call 0x140fe7410
0x14068d63c mov qword ptr [rbp + 0xa8], 0
0x14068d647 mov qword ptr [rbp + 0xb0], 8
0x14068d652 mov qword ptr [rbp + 0xb8], 0
0x14068d65d mov rcx, rsi
0x14068d660 mov rdx, rdi
0x14068d663 call 0x140fd02c0
0x14068d668 xorps xmm1, xmm1
0x14068d66b mov rcx, rax
0x14068d66e call 0x140fd25b0
0x14068d673 lea rdi, [rsi + 0x38]
0x14068d677 movabs rdx, 0xb0e748f9e3aacb57
0x14068d681 movabs r8, 0x757117fa8299e96e
0x14068d68b mov rcx, rdi
0x14068d68e call 0x1400dd6f0
0x14068d693 test rax, rax
0x14068d696 je 0x14068d6cb
0x14068d698 cmp qword ptr [rsi + 0x218], rdx
0x14068d69f jbe 0x14068d6cb
0x14068d6a1 mov rax, qword ptr [rsi + 0x210]
0x14068d6a8 mov rax, qword ptr [rax + rdx*8]
0x14068d6ac test rax, rax
0x14068d6af je 0x14068d6cb
0x14068d6b1 mov rcx, qword ptr [rsi + 0x1e0]
0x14068d6b8 not rax
0x14068d6bb lea rax, [rax + rax*2]
0x14068d6bf shl rax, 5
0x14068d6c3 cmp qword ptr [rcx + rax + 0x20], 0
0x14068d6c9 jne 0x14068d6d3
0x14068d6cb mov rcx, rsi
0x14068d6ce call 0x140fea370
0x14068d6d3 movabs rdx, 0x4d2642e55f9027d1
0x14068d6dd movabs r8, 0x4d297e300d4880fc
0x14068d6e7 mov rcx, rdi
0x14068d6ea call 0x1400dd6f0
0x14068d6ef test rax, rax
0x14068d6f2 je 0x14068d727
0x14068d6f4 cmp qword ptr [rsi + 0x218], rdx
0x14068d6fb jbe 0x14068d727
0x14068d6fd mov rax, qword ptr [rsi + 0x210]
0x14068d704 mov rax, qword ptr [rax + rdx*8]
0x14068d708 test rax, rax
0x14068d70b je 0x14068d727
0x14068d70d mov rcx, qword ptr [rsi + 0x1e0]
0x14068d714 not rax
0x14068d717 lea rax, [rax + rax*2]
0x14068d71b shl rax, 5
0x14068d71f cmp qword ptr [rcx + rax + 0x20], 0
0x14068d725 jne 0x14068d72f
0x14068d727 mov rcx, rsi
0x14068d72a call 0x140feb600
0x14068d72f movabs rdx, 0x189e38b5bedf98fb
0x14068d739 movabs r8, 0xd33f6912825c2993
0x14068d743 mov rcx, rdi
0x14068d746 call 0x1400dd6f0
0x14068d74b test rax, rax
0x14068d74e je 0x14068d783
0x14068d750 cmp qword ptr [rsi + 0x218], rdx
0x14068d757 jbe 0x14068d783
0x14068d759 mov rax, qword ptr [rsi + 0x210]
0x14068d760 mov rax, qword ptr [rax + rdx*8]
0x14068d764 test rax, rax
0x14068d767 je 0x14068d783
0x14068d769 mov rcx, qword ptr [rsi + 0x1e0]
0x14068d770 not rax
0x14068d773 lea rax, [rax + rax*2]
0x14068d777 shl rax, 5
0x14068d77b cmp qword ptr [rcx + rax + 0x20], 0
0x14068d781 jne 0x14068d78b
0x14068d783 mov rcx, rsi
0x14068d786 call 0x140fe8490
0x14068d78b movabs rdx, 0x1ce85a7892ad8401
0x14068d795 movabs r8, 0xd56a4e01466a73a2
0x14068d79f mov rcx, rdi
0x14068d7a2 call 0x1400dd6f0
0x14068d7a7 test rax, rax
0x14068d7aa je 0x14068d7df
0x14068d7ac cmp qword ptr [rsi + 0x218], rdx
0x14068d7b3 jbe 0x14068d7df
0x14068d7b5 mov rax, qword ptr [rsi + 0x210]
0x14068d7bc mov rax, qword ptr [rax + rdx*8]
0x14068d7c0 test rax, rax
0x14068d7c3 je 0x14068d7df
0x14068d7c5 mov rcx, qword ptr [rsi + 0x1e0]
0x14068d7cc not rax
0x14068d7cf lea rax, [rax + rax*2]
0x14068d7d3 shl rax, 5
0x14068d7d7 cmp qword ptr [rcx + rax + 0x20], 0
0x14068d7dd jne 0x14068d7e7
0x14068d7df mov rcx, rsi
0x14068d7e2 call 0x140fea9a0
0x14068d7e7 movabs rdx, 0x62bb6cad4efc598
0x14068d7f1 movabs r8, 0x22acd43c16b1856e
0x14068d7fb mov rcx, rdi
0x14068d7fe call 0x1400dd6f0
0x14068d803 test rax, rax
0x14068d806 je 0x14068d83b
0x14068d808 cmp qword ptr [rsi + 0x218], rdx
0x14068d80f jbe 0x14068d83b
0x14068d811 mov rax, qword ptr [rsi + 0x210]
0x14068d818 mov rax, qword ptr [rax + rdx*8]
0x14068d81c test rax, rax
0x14068d81f je 0x14068d83b
0x14068d821 mov rcx, qword ptr [rsi + 0x1e0]
0x14068d828 not rax
0x14068d82b lea rax, [rax + rax*2]
0x14068d82f shl rax, 5
0x14068d833 cmp qword ptr [rcx + rax + 0x20], 0
0x14068d839 jne 0x14068d843
0x14068d83b mov rcx, rsi
0x14068d83e call 0x140fe8ac0
0x14068d843 movabs rdx, 0xd150ed151a658bf
0x14068d84d movabs r8, 0x1dd614aa5ea5002
0x14068d857 mov rcx, rdi
0x14068d85a call 0x1400dd6f0
0x14068d85f test rax, rax
0x14068d862 je 0x14068d897
0x14068d864 cmp qword ptr [rsi + 0x218], rdx
0x14068d86b jbe 0x14068d897
0x14068d86d mov rax, qword ptr [rsi + 0x210]
0x14068d874 mov rax, qword ptr [rax + rdx*8]
0x14068d878 test rax, rax
0x14068d87b je 0x14068d897
0x14068d87d mov rcx, qword ptr [rsi + 0x1e0]
0x14068d884 not rax
0x14068d887 lea rax, [rax + rax*2]
0x14068d88b shl rax, 5
0x14068d88f cmp qword ptr [rcx + rax + 0x20], 0
0x14068d895 jne 0x14068d89f
0x14068d897 mov rcx, rsi
0x14068d89a call 0x140fe90f0
0x14068d89f movabs rdx, 0x5eaaca67868e0174
0x14068d8a9 movabs r8, 0x67675b60a5763276
0x14068d8b3 mov rcx, rdi
0x14068d8b6 call 0x1400dd6f0
0x14068d8bb test rax, rax
0x14068d8be je 0x14068d8f3
0x14068d8c0 cmp qword ptr [rsi + 0x218], rdx
0x14068d8c7 jbe 0x14068d8f3
0x14068d8c9 mov rax, qword ptr [rsi + 0x210]
0x14068d8d0 mov rax, qword ptr [rax + rdx*8]
0x14068d8d4 test rax, rax
0x14068d8d7 je 0x14068d8f3
0x14068d8d9 mov rcx, qword ptr [rsi + 0x1e0]
0x14068d8e0 not rax
0x14068d8e3 lea rax, [rax + rax*2]
0x14068d8e7 shl rax, 5
0x14068d8eb cmp qword ptr [rcx + rax + 0x20], 0
0x14068d8f1 jne 0x14068d8fb
0x14068d8f3 mov rcx, rsi
0x14068d8f6 call 0x140feafd0
0x14068d8fb movabs rdx, 0x7e239f0526778219
0x14068d905 movabs r8, 0x52d214c46f437022
0x14068d90f mov rcx, rdi
0x14068d912 call 0x1400dd6f0
0x14068d917 test rax, rax
0x14068d91a je 0x14068d94f
0x14068d91c cmp qword ptr [rsi + 0x218], rdx
0x14068d923 jbe 0x14068d94f
0x14068d925 mov rax, qword ptr [rsi + 0x210]
0x14068d92c mov rax, qword ptr [rax + rdx*8]
0x14068d930 test rax, rax
0x14068d933 je 0x14068d94f
0x14068d935 mov rcx, qword ptr [rsi + 0x1e0]
0x14068d93c not rax
0x14068d93f lea rax, [rax + rax*2]
0x14068d943 shl rax, 5
0x14068d947 cmp qword ptr [rcx + rax + 0x20], 0
0x14068d94d jne 0x14068d957
0x14068d94f mov rcx, rsi
0x14068d952 call 0x140fe9710
0x14068d957 movabs rdx, 0x73f303a4e7b78adc
0x14068d961 movabs r8, 0xed4f372bb4ea136e
0x14068d96b mov rcx, rdi
0x14068d96e call 0x1400dd6f0
0x14068d973 test rax, rax
0x14068d976 je 0x14068d9ab
0x14068d978 cmp qword ptr [rsi + 0x218], rdx
0x14068d97f jbe 0x14068d9ab
0x14068d981 mov rax, qword ptr [rsi + 0x210]
0x14068d988 mov rax, qword ptr [rax + rdx*8]
0x14068d98c test rax, rax
0x14068d98f je 0x14068d9ab
0x14068d991 mov rcx, qword ptr [rsi + 0x1e0]
0x14068d998 not rax
0x14068d99b lea rax, [rax + rax*2]
0x14068d99f shl rax, 5
0x14068d9a3 cmp qword ptr [rcx + rax + 0x20], 0
0x14068d9a9 jne 0x14068d9b3
0x14068d9ab mov rcx, rsi
0x14068d9ae call 0x140fe7e60
0x14068d9b3 mov rcx, rsi
0x14068d9b6 call 0x140fd2b00
0x14068d9bb lea rcx, [rbp + 0x10]
0x14068d9bf call 0x140fee0e0
0x14068d9c4 mov byte ptr [rbp + 0xf0], 7
0x14068d9cb mov rcx, qword ptr [rip + 0x2f9e99e]
0x14068d9d2 lea r8, [rip + 0x253419f]
0x14068d9d9 lea rdx, [rbp + 0xf0]
0x14068d9e0 call 0x140fdc2a0
0x14068d9e5 mov r8, rdx
0x14068d9e8 lea rcx, [rbp + 0x10]
0x14068d9ec mov rdx, rax
0x14068d9ef call 0x140fddf80
0x14068d9f4 movups xmm0, xmmword ptr [rbp + 0x70]
0x14068d9f8 movaps xmmword ptr [rbp], xmm0
0x14068d9fc movups xmm0, xmmword ptr [rbp + 0x60]
0x14068da00 movaps xmmword ptr [rbp - 0x10], xmm0
0x14068da04 movups xmm0, xmmword ptr [rbp + 0x50]
0x14068da08 movaps xmmword ptr [rbp - 0x20], xmm0
0x14068da0c movups xmm0, xmmword ptr [rbp + 0x10]
0x14068da10 movups xmm1, xmmword ptr [rbp + 0x20]
0x14068da14 movups xmm2, xmmword ptr [rbp + 0x30]
0x14068da18 movups xmm3, xmmword ptr [rbp + 0x40]
0x14068da1c movaps xmmword ptr [rbp - 0x30], xmm3
0x14068da20 movaps xmmword ptr [rbp - 0x40], xmm2
0x14068da24 movaps xmmword ptr [rbp - 0x50], xmm1
0x14068da28 movaps xmmword ptr [rbp - 0x60], xmm0
0x14068da2c lea rcx, [rbp + 0xf0]
0x14068da33 call 0x140fef7c0
0x14068da38 mov byte ptr [rbp + 0x167], 0x24
0x14068da3f mov rcx, qword ptr [rip + 0x2f9e92a]
0x14068da46 lea r8, [rip + 0x253412b]
0x14068da4d lea rdx, [rbp + 0x167]
0x14068da54 call 0x140fdc2a0
0x14068da59 mov r8, rdx
0x14068da5c lea rcx, [rbp + 0xf0]
0x14068da63 mov rdx, rax
0x14068da66 call 0x140fddf80
0x14068da6b movups xmm0, xmmword ptr [rbp + 0x150]
0x14068da72 movaps xmmword ptr [rbp + 0xe0], xmm0
0x14068da79 movups xmm0, xmmword ptr [rbp + 0x140]
0x14068da80 movaps xmmword ptr [rbp + 0xd0], xmm0
0x14068da87 movups xmm0, xmmword ptr [rbp + 0x130]
0x14068da8e movaps xmmword ptr [rbp + 0xc0], xmm0
0x14068da95 movups xmm0, xmmword ptr [rbp + 0xf0]
0x14068da9c movups xmm1, xmmword ptr [rbp + 0x100]
0x14068daa3 movups xmm2, xmmword ptr [rbp + 0x110]
0x14068daaa movups xmm3, xmmword ptr [rbp + 0x120]
0x14068dab1 movaps xmmword ptr [rbp + 0xb0], xmm3
0x14068dab8 movaps xmmword ptr [rbp + 0xa0], xmm2
0x14068dabf movaps xmmword ptr [rbp + 0x90], xmm1
0x14068dac6 movaps xmmword ptr [rbp + 0x80], xmm0
0x14068dacd movaps xmm0, xmmword ptr [rbp - 0x60]
0x14068dad1 movaps xmm1, xmmword ptr [rbp - 0x50]
0x14068dad5 movaps xmm2, xmmword ptr [rbp - 0x40]
0x14068dad9 movaps xmm3, xmmword ptr [rbp - 0x30]
0x14068dadd movaps xmmword ptr [rbp + 0x10], xmm0
0x14068dae1 movaps xmmword ptr [rbp + 0x20], xmm1
0x14068dae5 movaps xmmword ptr [rbp + 0x30], xmm2
0x14068dae9 movaps xmmword ptr [rbp + 0x40], xmm3
0x14068daed movaps xmm0, xmmword ptr [rbp - 0x20]
0x14068daf1 movaps xmmword ptr [rbp + 0x50], xmm0
0x14068daf5 movaps xmm0, xmmword ptr [rbp - 0x10]
0x14068daf9 movaps xmmword ptr [rbp + 0x60], xmm0
0x14068dafd movaps xmm0, xmmword ptr [rbp]
0x14068db01 movaps xmmword ptr [rbp + 0x70], xmm0
0x14068db05 lea rdx, [rbp + 0x10]
0x14068db09 mov rcx, rsi
0x14068db0c call 0x140fd2c40
0x14068db11 lea rcx, [rbp + 0x10]
0x14068db15 call 0x140fed8a0
0x14068db1a mov byte ptr [rbp - 0x60], 0x2c
0x14068db1e mov rcx, qword ptr [rip + 0x2f9e84b]
0x14068db25 lea r8, [rip + 0x253404c]
0x14068db2c lea rdx, [rbp - 0x60]
0x14068db30 call 0x140fdc2a0
0x14068db35 mov r8, rdx
0x14068db38 lea rcx, [rbp + 0x10]
0x14068db3c mov rdx, rax
0x14068db3f call 0x140fddf80
0x14068db44 movups xmm0, xmmword ptr [rbp + 0x70]
0x14068db48 movaps xmmword ptr [rbp + 0x150], xmm0
0x14068db4f movups xmm0, xmmword ptr [rbp + 0x60]
0x14068db53 movaps xmmword ptr [rbp + 0x140], xmm0
0x14068db5a movups xmm0, xmmword ptr [rbp + 0x50]
0x14068db5e movaps xmmword ptr [rbp + 0x130], xmm0
0x14068db65 movups xmm0, xmmword ptr [rbp + 0x10]
0x14068db69 movups xmm1, xmmword ptr [rbp + 0x20]
0x14068db6d movups xmm2, xmmword ptr [rbp + 0x30]
0x14068db71 movups xmm3, xmmword ptr [rbp + 0x40]
0x14068db75 movaps xmmword ptr [rbp + 0x120], xmm3
0x14068db7c movaps xmmword ptr [rbp + 0x110], xmm2
0x14068db83 movaps xmmword ptr [rbp + 0x100], xmm1
0x14068db8a movaps xmmword ptr [rbp + 0xf0], xmm0
0x14068db91 lea rdx, [rbp + 0xf0]
0x14068db98 mov rcx, rsi
0x14068db9b call 0x140fd2dd0
0x14068dba0 nop 
0x14068dba1 add rsp, 0x1f0
0x14068dba8 pop rdi
0x14068dba9 pop rsi
0x14068dbaa pop rbp
0x14068dbab ret 
0x14068dbac nop dword ptr [rax]
0x14068dbb0 mov qword ptr [rsp + 0x10], rdx
0x14068dbb5 push rbp
0x14068dbb6 push rsi
0x14068dbb7 push rdi
0x14068dbb8 sub rsp, 0x20
0x14068dbbc lea rbp, [rdx + 0x80]
0x14068dbc3 lea rcx, [rbp + 0x10]
0x14068dbc7 call 0x140029880
0x14068dbcc nop 
0x14068dbcd add rsp, 0x20
0x14068dbd1 pop rdi
0x14068dbd2 pop rsi
0x14068dbd3 pop rbp
0x14068dbd4 ret 
0x14068dbd5 nop word ptr cs:[rax + rax]
0x14068dbe0 mov qword ptr [rsp + 0x10], rdx
0x14068dbe5 push rbp
0x14068dbe6 push rsi
0x14068dbe7 push rdi
0x14068dbe8 sub rsp, 0x20
0x14068dbec lea rbp, [rdx + 0x80]
0x14068dbf3 lea rcx, [rbp - 0x60]
0x14068dbf7 call 0x140029880
0x14068dbfc nop 
0x14068dbfd add rsp, 0x20
0x14068dc01 pop rdi
0x14068dc02 pop rsi
0x14068dc03 pop rbp
0x14068dc04 ret 
0x14068dc05 nop word ptr cs:[rax + rax]
0x14068dc10 mov qword ptr [rsp + 0x10], rdx
0x14068dc15 push rbp
0x14068dc16 push rsi
0x14068dc17 push rdi
0x14068dc18 sub rsp, 0x20
0x14068dc1c lea rbp, [rdx + 0x80]
0x14068dc23 lea rcx, [rbp + 0xf0]
0x14068dc2a call 0x140029880
0x14068dc2f nop 
0x14068dc30 add rsp, 0x20
0x14068dc34 pop rdi
0x14068dc35 pop rsi
0x14068dc36 pop rbp
0x14068dc37 ret 
0x14068dc38 nop dword ptr [rax + rax]
0x14068dc40 mov qword ptr [rsp + 0x10], rdx
0x14068dc45 push rbp
0x14068dc46 push rsi
0x14068dc47 push rdi
0x14068dc48 sub rsp, 0x20
0x14068dc4c lea rbp, [rdx + 0x80]
0x14068dc53 lea rcx, [rbp + 0x10]
0x14068dc57 call 0x140029880
0x14068dc5c nop 
0x14068dc5d add rsp, 0x20
0x14068dc61 pop rdi
0x14068dc62 pop rsi
0x14068dc63 pop rbp
0x14068dc64 ret 
