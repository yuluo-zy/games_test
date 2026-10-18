0x1402dfe10 55 push rbp
0x1402dfe11 56 push rsi
0x1402dfe12 57 push rdi
0x1402dfe13 4881ecb0010000 sub rsp, 0x1b0
0x1402dfe1a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1402dfe22 48c78528010000feffffff mov qword ptr [rbp + 0x128], 0xfffffffffffffffe
0x1402dfe2d 4889ce mov rsi, rcx
0x1402dfe30 0f57c0 xorps xmm0, xmm0
0x1402dfe33 0f114508 movups xmmword ptr [rbp + 8], xmm0
0x1402dfe37 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x1402dfe3b 0f114538 movups xmmword ptr [rbp + 0x38], xmm0
0x1402dfe3f 0f114550 movups xmmword ptr [rbp + 0x50], xmm0
0x1402dfe43 0f114568 movups xmmword ptr [rbp + 0x68], xmm0
0x1402dfe47 c7457800000000 mov dword ptr [rbp + 0x78], 0
0x1402dfe4e 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
0x1402dfe55 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x1402dfe5c 0f1185d0000000 movups xmmword ptr [rbp + 0xd0], xmm0
0x1402dfe63 0f1185e8000000 movups xmmword ptr [rbp + 0xe8], xmm0
0x1402dfe6a 0f118500010000 movups xmmword ptr [rbp + 0x100], xmm0
0x1402dfe71 c7851001000000000000 mov dword ptr [rbp + 0x110], 0
0x1402dfe7b 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x1402dfe83 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x1402dfe8b 48c745f008000000 mov qword ptr [rbp - 0x10], 8
0x1402dfe93 48c745f800000000 mov qword ptr [rbp - 8], 0
0x1402dfe9b 48c7450010000000 mov qword ptr [rbp], 0x10
0x1402dfea3 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x1402dfeab 48c7453010000000 mov qword ptr [rbp + 0x30], 0x10
0x1402dfeb3 48c7454810000000 mov qword ptr [rbp + 0x48], 0x10
0x1402dfebb 48c7456010000000 mov qword ptr [rbp + 0x60], 0x10
0x1402dfec3 48b80000000000000080 movabs rax, 0x8000000000000000
0x1402dfecd 48898580000000 mov qword ptr [rbp + 0x80], rax
0x1402dfed4 488d05213e6d02 lea rax, [rip + 0x26d3e21]
0x1402dfedb 48898588000000 mov qword ptr [rbp + 0x88], rax
0x1402dfee2 48c7859000000051000000 mov qword ptr [rbp + 0x90], 0x51
0x1402dfeed 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
0x1402dfef8 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x1402dff03 48c785c800000010000000 mov qword ptr [rbp + 0xc8], 0x10
0x1402dff0e 48c785e000000010000000 mov qword ptr [rbp + 0xe0], 0x10
0x1402dff19 48c785f800000010000000 mov qword ptr [rbp + 0xf8], 0x10
0x1402dff24 c7851801000000000000 mov dword ptr [rbp + 0x118], 0
0x1402dff2e 66c7851c0100000100 mov word ptr [rbp + 0x11c], 1
0x1402dff37 c7852001000000000000 mov dword ptr [rbp + 0x120], 0
0x1402dff41 0fb605c14c3503 movzx eax, byte ptr [rip + 0x3354cc1]
0x1402dff48 b980010000 mov ecx, 0x180
0x1402dff4d ba08000000 mov edx, 8
0x1402dff52 e8b93c3300 call 0x140613c10
0x1402dff57 4885c0 test rax, rax
0x1402dff5a 7435 je 0x1402dff91
0x1402dff5c 4889c7 mov rdi, rax
0x1402dff5f 488d55a8 lea rdx, [rbp - 0x58]
0x1402dff63 41b880010000 mov r8d, 0x180
0x1402dff69 4889c1 mov rcx, rax
0x1402dff6c e88f3c6402 call 0x142923c00
0x1402dff71 4c8d05103c6d02 lea r8, [rip + 0x26d3c10]
0x1402dff78 4889f1 mov rcx, rsi
0x1402dff7b 4889fa mov rdx, rdi
0x1402dff7e e88d4ba000 call 0x140ce4b10
0x1402dff83 4889f0 mov rax, rsi
0x1402dff86 4881c4b0010000 add rsp, 0x1b0
0x1402dff8d 5f pop rdi
0x1402dff8e 5e pop rsi
0x1402dff8f 5d pop rbp
0x1402dff90 c3 ret 
0x1402dff91 b908000000 mov ecx, 8
0x1402dff96 ba80010000 mov edx, 0x180
0x1402dff9b e860905f02 call 0x1428d9000
0x1402dffa0 0f0b ud2 
0x1402dffa2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1402dffb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1402dffb5 55 push rbp
0x1402dffb6 56 push rsi
0x1402dffb7 57 push rdi
0x1402dffb8 4883ec20 sub rsp, 0x20
0x1402dffbc 488daa80000000 lea rbp, [rdx + 0x80]
0x1402dffc3 488d4da8 lea rcx, [rbp - 0x58]
0x1402dffc7 e8a43e3100 call 0x1405f3e70
0x1402dffcc 90 nop 
0x1402dffcd 4883c420 add rsp, 0x20
0x1402dffd1 5f pop rdi
0x1402dffd2 5e pop rsi
0x1402dffd3 5d pop rbp
0x1402dffd4 c3 ret 
