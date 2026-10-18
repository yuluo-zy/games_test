0x1405bdf20 push rsi
0x1405bdf21 push rdi
0x1405bdf22 sub rsp, 0x78
0x1405bdf26 movabs rax, 0x736f6d6570736575
0x1405bdf30 mov qword ptr [rsp + 0x30], rax
0x1405bdf35 movabs rax, 0x6c7967656e657261
0x1405bdf3f mov qword ptr [rsp + 0x38], rax
0x1405bdf44 movabs rax, 0x646f72616e646f6d
0x1405bdf4e mov qword ptr [rsp + 0x40], rax
0x1405bdf53 movabs rax, 0x7465646279746573
0x1405bdf5d mov qword ptr [rsp + 0x48], rax
0x1405bdf62 xorps xmm0, xmm0
0x1405bdf65 movups xmmword ptr [rsp + 0x50], xmm0
0x1405bdf6a movups xmmword ptr [rsp + 0x60], xmm0
0x1405bdf6f mov qword ptr [rsp + 0x70], 0
0x1405bdf78 mov rax, qword ptr [rcx]
0x1405bdf7b mov rdi, qword ptr [rcx + 0x10]
0x1405bdf7f mov qword ptr [rsp + 0x28], rax
0x1405bdf84 lea rsi, [rsp + 0x30]
0x1405bdf89 lea rdx, [rsp + 0x28]
0x1405bdf8e mov r8d, 8
0x1405bdf94 mov rcx, rsi
0x1405bdf97 call 0x140106160
0x1405bdf9c mov qword ptr [rsp + 0x28], rdi
0x1405bdfa1 lea rdx, [rsp + 0x28]
0x1405bdfa6 mov r8d, 8
0x1405bdfac mov rcx, rsi
0x1405bdfaf call 0x140106160
0x1405bdfb4 mov r9, qword ptr [rsp + 0x60]
0x1405bdfb9 shl r9, 0x38
0x1405bdfbd or r9, qword ptr [rsp + 0x68]
0x1405bdfc2 mov rdx, qword ptr [rsp + 0x40]
0x1405bdfc7 mov rcx, qword ptr [rsp + 0x48]
0x1405bdfcc xor rcx, r9
0x1405bdfcf mov rax, qword ptr [rsp + 0x30]
0x1405bdfd4 add rax, rdx
0x1405bdfd7 mov r8, qword ptr [rsp + 0x38]
0x1405bdfdc rol rdx, 0xd
0x1405bdfe0 add r8, rcx
0x1405bdfe3 xor rdx, rax
0x1405bdfe6 rol rcx, 0x10
0x1405bdfea xor rcx, r8
0x1405bdfed rol rax, 0x20
0x1405bdff1 add r8, rdx
0x1405bdff4 add rax, rcx
0x1405bdff7 rol rdx, 0x11
0x1405bdffb xor rdx, r8
0x1405bdffe rol rcx, 0x15
0x1405be002 rol r8, 0x20
0x1405be006 xor rcx, rax
0x1405be009 xor rax, r9