0x140002c90 55 push rbp
0x140002c91 56 push rsi
0x140002c92 57 push rdi
0x140002c93 4881ec70040000 sub rsp, 0x470
0x140002c9a 488dac2480000000 lea rbp, [rsp + 0x80]
0x140002ca2 48c785e8030000feffffff mov qword ptr [rbp + 0x3e8], 0xfffffffffffffffe
0x140002cad 4889ce mov rsi, rcx
0x140002cb0 0f57c0 xorps xmm0, xmm0
0x140002cb3 0f1145c8 movups xmmword ptr [rbp - 0x38], xmm0
0x140002cb7 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
0x140002cbb 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x140002cbf 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x140002cc3 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x140002cc7 c7453800000000 mov dword ptr [rbp + 0x38], 0
0x140002cce 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x140002cd2 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x140002cd6 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x140002cdd 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x140002ce4 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x140002ceb c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x140002cf5 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x140002cfd 48c745b008000000 mov qword ptr [rbp - 0x50], 8
0x140002d05 48c745b800000000 mov qword ptr [rbp - 0x48], 0
0x140002d0d 48c745c010000000 mov qword ptr [rbp - 0x40], 0x10
0x140002d15 48c745d810000000 mov qword ptr [rbp - 0x28], 0x10
0x140002d1d 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x140002d25 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x140002d2d 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x140002d35 48b80000000000000080 movabs rax, 0x8000000000000000
0x140002d3f 48894540 mov qword ptr [rbp + 0x40], rax
0x140002d43 488d0deb2a9202 lea rcx, [rip + 0x2922aeb]
0x140002d4a 48894d48 mov qword ptr [rbp + 0x48], rcx
0x140002d4e 48c7455039000000 mov qword ptr [rbp + 0x50], 0x39
0x140002d56 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x140002d5e 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x140002d66 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x140002d71 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x140002d7c 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x140002d87 c785d800000000000000 mov dword ptr [rbp + 0xd8], 0
0x140002d91 66c785dc0000000100 mov word ptr [rbp + 0xdc], 1
0x140002d9a 488985e0000000 mov qword ptr [rbp + 0xe0], rax
0x140002da1 c785e003000000000000 mov dword ptr [rbp + 0x3e0], 0
0x140002dab 0fb605571e6303 movzx eax, byte ptr [rip + 0x3631e57]
0x140002db2 b940040000 mov ecx, 0x440
0x140002db7 ba08000000 mov edx, 8
0x140002dbc e84f0e6100 call 0x140613c10
0x140002dc1 4885c0 test rax, rax
0x140002dc4 7435 je 0x140002dfb
0x140002dc6 4889c7 mov rdi, rax
0x140002dc9 488d55a8 lea rdx, [rbp - 0x58]
0x140002dcd 41b840040000 mov r8d, 0x440
0x140002dd3 4889c1 mov rcx, rax
0x140002dd6 e8250e9202 call 0x142923c00
0x140002ddb 4c8d057e299202 lea r8, [rip + 0x292297e]
0x140002de2 4889f1 mov rcx, rsi
0x140002de5 4889fa mov rdx, rdi
0x140002de8 e8231dce00 call 0x140ce4b10
0x140002ded 4889f0 mov rax, rsi
0x140002df0 4881c470040000 add rsp, 0x470
0x140002df7 5f pop rdi
0x140002df8 5e pop rsi
0x140002df9 5d pop rbp
0x140002dfa c3 ret 
0x140002dfb b908000000 mov ecx, 8
0x140002e00 ba40040000 mov edx, 0x440
0x140002e05 e8f6618d02 call 0x1428d9000
0x140002e0a 0f0b ud2 
0x140002e0c 0f1f4000 nop dword ptr [rax]
0x140002e10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140002e15 55 push rbp
0x140002e16 56 push rsi
0x140002e17 57 push rdi
0x140002e18 4883ec20 sub rsp, 0x20
0x140002e1c 488daa80000000 lea rbp, [rdx + 0x80]
0x140002e23 488d4da8 lea rcx, [rbp - 0x58]
0x140002e27 e884fe5d00 call 0x1405e2cb0
0x140002e2c 90 nop 
0x140002e2d 4883c420 add rsp, 0x20
0x140002e31 5f pop rdi
0x140002e32 5e pop rsi
0x140002e33 5d pop rbp
0x140002e34 c3 ret 
