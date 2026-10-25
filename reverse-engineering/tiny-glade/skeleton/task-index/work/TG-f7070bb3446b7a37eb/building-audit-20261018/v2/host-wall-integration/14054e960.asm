0x14054e960 55 push rbp
0x14054e961 56 push rsi
0x14054e962 57 push rdi
0x14054e963 53 push rbx
0x14054e964 4881ec28050000 sub rsp, 0x528
0x14054e96b 488dac2480000000 lea rbp, [rsp + 0x80]
0x14054e973 48c785a0040000feffffff mov qword ptr [rbp + 0x4a0], 0xfffffffffffffffe
0x14054e97e 4889ce mov rsi, rcx
0x14054e981 0f57c0 xorps xmm0, xmm0
0x14054e984 0f114538 movups xmmword ptr [rbp + 0x38], xmm0
0x14054e988 0f114550 movups xmmword ptr [rbp + 0x50], xmm0
0x14054e98c 0f114568 movups xmmword ptr [rbp + 0x68], xmm0
0x14054e990 0f118580000000 movups xmmword ptr [rbp + 0x80], xmm0
0x14054e997 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
0x14054e99e c785a800000000000000 mov dword ptr [rbp + 0xa8], 0
0x14054e9a8 0f1185d0000000 movups xmmword ptr [rbp + 0xd0], xmm0
0x14054e9af 0f1185e8000000 movups xmmword ptr [rbp + 0xe8], xmm0
0x14054e9b6 0f118500010000 movups xmmword ptr [rbp + 0x100], xmm0
0x14054e9bd 0f118518010000 movups xmmword ptr [rbp + 0x118], xmm0
0x14054e9c4 0f118530010000 movups xmmword ptr [rbp + 0x130], xmm0
0x14054e9cb c7854001000000000000 mov dword ptr [rbp + 0x140], 0
0x14054e9d5 48c7451800000000 mov qword ptr [rbp + 0x18], 0
0x14054e9dd 48c7452008000000 mov qword ptr [rbp + 0x20], 8
0x14054e9e5 48c7452800000000 mov qword ptr [rbp + 0x28], 0
0x14054e9ed 48c7453010000000 mov qword ptr [rbp + 0x30], 0x10
0x14054e9f5 48c7454810000000 mov qword ptr [rbp + 0x48], 0x10
0x14054e9fd 48c7456010000000 mov qword ptr [rbp + 0x60], 0x10
0x14054ea05 48c7457810000000 mov qword ptr [rbp + 0x78], 0x10
0x14054ea0d 48c7859000000010000000 mov qword ptr [rbp + 0x90], 0x10
0x14054ea18 48b80000000000000080 movabs rax, 0x8000000000000000
0x14054ea22 488985b0000000 mov qword ptr [rbp + 0xb0], rax
0x14054ea29 488d0db6a94a02 lea rcx, [rip + 0x24aa9b6]
0x14054ea30 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
0x14054ea37 48c785c000000047000000 mov qword ptr [rbp + 0xc0], 0x47
0x14054ea42 48c785c800000010000000 mov qword ptr [rbp + 0xc8], 0x10
0x14054ea4d 48c785e000000010000000 mov qword ptr [rbp + 0xe0], 0x10
0x14054ea58 48c785f800000010000000 mov qword ptr [rbp + 0xf8], 0x10
0x14054ea63 48c7851001000010000000 mov qword ptr [rbp + 0x110], 0x10
0x14054ea6e 48c7852801000010000000 mov qword ptr [rbp + 0x128], 0x10
0x14054ea79 c7854801000000000000 mov dword ptr [rbp + 0x148], 0
0x14054ea83 66c7854c0100000100 mov word ptr [rbp + 0x14c], 1
0x14054ea8c 48898540020000 mov qword ptr [rbp + 0x240], rax
0x14054ea93 c7859804000000000000 mov dword ptr [rbp + 0x498], 0
0x14054ea9d 0fb60565610e03 movzx eax, byte ptr [rip + 0x30e6165]
0x14054eaa4 b988040000 mov ecx, 0x488
0x14054eaa9 ba08000000 mov edx, 8
0x14054eaae e85d510c00 call 0x140613c10
0x14054eab3 4885c0 test rax, rax
0x14054eab6 7445 je 0x14054eafd
0x14054eab8 4889c7 mov rdi, rax
0x14054eabb 488d5518 lea rdx, [rbp + 0x18]
0x14054eabf 41b888040000 mov r8d, 0x488
0x14054eac5 4889c1 mov rcx, rax
0x14054eac8 e833513d02 call 0x142923c00
0x14054eacd 4c8d05eca74a02 lea r8, [rip + 0x24aa7ec]
0x14054ead4 488d5da8 lea rbx, [rbp - 0x58]
0x14054ead8 4889d9 mov rcx, rbx
0x14054eadb 4889fa mov rdx, rdi
0x14054eade e82d607900 call 0x140ce4b10
0x14054eae3 4889f1 mov rcx, rsi
0x14054eae6 4889da mov rdx, rbx
0x14054eae9 e8823fb8ff call 0x1400d2a70
0x14054eaee 4889f0 mov rax, rsi
0x14054eaf1 4881c428050000 add rsp, 0x528
0x14054eaf8 5b pop rbx
0x14054eaf9 5f pop rdi
0x14054eafa 5e pop rsi
0x14054eafb 5d pop rbp
0x14054eafc c3 ret 
0x14054eafd b908000000 mov ecx, 8
0x14054eb02 ba88040000 mov edx, 0x488
0x14054eb07 e8f4a43802 call 0x1428d9000
0x14054eb0c 0f0b ud2 
0x14054eb0e 6690 nop 
0x14054eb10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14054eb15 55 push rbp
0x14054eb16 56 push rsi
0x14054eb17 57 push rdi
0x14054eb18 53 push rbx
0x14054eb19 4883ec28 sub rsp, 0x28
0x14054eb1d 488daa80000000 lea rbp, [rdx + 0x80]
0x14054eb24 488d4d18 lea rcx, [rbp + 0x18]
0x14054eb28 e8832b0800 call 0x1405d16b0
0x14054eb2d 90 nop 
0x14054eb2e 4883c428 add rsp, 0x28
0x14054eb32 5b pop rbx
0x14054eb33 5f pop rdi
0x14054eb34 5e pop rsi
0x14054eb35 5d pop rbp
0x14054eb36 c3 ret 
