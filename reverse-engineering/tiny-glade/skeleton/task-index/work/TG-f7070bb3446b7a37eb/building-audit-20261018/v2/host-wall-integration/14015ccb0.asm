0x14015ccb0 55 push rbp
0x14015ccb1 56 push rsi
0x14015ccb2 57 push rdi
0x14015ccb3 4881ecd0020000 sub rsp, 0x2d0
0x14015ccba 488dac2480000000 lea rbp, [rsp + 0x80]
0x14015ccc2 48c78548020000feffffff mov qword ptr [rbp + 0x248], 0xfffffffffffffffe
0x14015cccd 4889ce mov rsi, rcx
0x14015ccd0 0f57c0 xorps xmm0, xmm0
0x14015ccd3 0f1145c8 movups xmmword ptr [rbp - 0x38], xmm0
0x14015ccd7 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
0x14015ccdb 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x14015ccdf 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x14015cce3 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x14015cce7 c7453800000000 mov dword ptr [rbp + 0x38], 0
0x14015ccee 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x14015ccf2 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x14015ccf6 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x14015ccfd 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x14015cd04 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x14015cd0b c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x14015cd15 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x14015cd1d 48c745b008000000 mov qword ptr [rbp - 0x50], 8
0x14015cd25 48c745b800000000 mov qword ptr [rbp - 0x48], 0
0x14015cd2d 48c745c010000000 mov qword ptr [rbp - 0x40], 0x10
0x14015cd35 48c745d810000000 mov qword ptr [rbp - 0x28], 0x10
0x14015cd3d 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x14015cd45 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x14015cd4d 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x14015cd55 48b80000000000000080 movabs rax, 0x8000000000000000
0x14015cd5f 48894540 mov qword ptr [rbp + 0x40], rax
0x14015cd63 488d0d01f87f02 lea rcx, [rip + 0x27ff801]
0x14015cd6a 48894d48 mov qword ptr [rbp + 0x48], rcx
0x14015cd6e 48c7455061000000 mov qword ptr [rbp + 0x50], 0x61
0x14015cd76 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x14015cd7e 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x14015cd86 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x14015cd91 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x14015cd9c 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x14015cda7 c785d800000000000000 mov dword ptr [rbp + 0xd8], 0
0x14015cdb1 66c785dc0000000100 mov word ptr [rbp + 0xdc], 1
0x14015cdba 488985d8010000 mov qword ptr [rbp + 0x1d8], rax
0x14015cdc1 c7854002000000000000 mov dword ptr [rbp + 0x240], 0
0x14015cdcb 0fb605377e4d03 movzx eax, byte ptr [rip + 0x34d7e37]
0x14015cdd2 b9a0020000 mov ecx, 0x2a0
0x14015cdd7 ba08000000 mov edx, 8
0x14015cddc e82f6e4b00 call 0x140613c10
0x14015cde1 4885c0 test rax, rax
0x14015cde4 7435 je 0x14015ce1b
0x14015cde6 4889c7 mov rdi, rax
0x14015cde9 488d55a8 lea rdx, [rbp - 0x58]
0x14015cded 41b8a0020000 mov r8d, 0x2a0
0x14015cdf3 4889c1 mov rcx, rax
0x14015cdf6 e8056e7c02 call 0x142923c00
0x14015cdfb 4c8d056ef57f02 lea r8, [rip + 0x27ff56e]
0x14015ce02 4889f1 mov rcx, rsi
0x14015ce05 4889fa mov rdx, rdi
0x14015ce08 e8037db800 call 0x140ce4b10
0x14015ce0d 4889f0 mov rax, rsi
0x14015ce10 4881c4d0020000 add rsp, 0x2d0
0x14015ce17 5f pop rdi
0x14015ce18 5e pop rsi
0x14015ce19 5d pop rbp
0x14015ce1a c3 ret 
0x14015ce1b b908000000 mov ecx, 8
0x14015ce20 baa0020000 mov edx, 0x2a0
0x14015ce25 e8d6c17702 call 0x1428d9000
0x14015ce2a 0f0b ud2 
0x14015ce2c 0f1f4000 nop dword ptr [rax]
0x14015ce30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14015ce35 55 push rbp
0x14015ce36 56 push rsi
0x14015ce37 57 push rdi
0x14015ce38 4883ec20 sub rsp, 0x20
0x14015ce3c 488daa80000000 lea rbp, [rdx + 0x80]
0x14015ce43 488d4da8 lea rcx, [rbp - 0x58]
0x14015ce47 e864254700 call 0x1405cf3b0
0x14015ce4c 90 nop 
0x14015ce4d 4883c420 add rsp, 0x20
0x14015ce51 5f pop rdi
0x14015ce52 5e pop rsi
0x14015ce53 5d pop rbp
0x14015ce54 c3 ret 
