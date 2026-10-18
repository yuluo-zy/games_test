0x1408c6e60 55 push rbp
0x1408c6e61 56 push rsi
0x1408c6e62 57 push rdi
0x1408c6e63 4881ecc0010000 sub rsp, 0x1c0
0x1408c6e6a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1408c6e72 48c78538010000feffffff mov qword ptr [rbp + 0x138], 0xfffffffffffffffe
0x1408c6e7d 4889ce mov rsi, rcx
0x1408c6e80 0f57c0 xorps xmm0, xmm0
0x1408c6e83 0f1145c8 movups xmmword ptr [rbp - 0x38], xmm0
0x1408c6e87 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
0x1408c6e8b 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x1408c6e8f 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x1408c6e93 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x1408c6e97 c7453800000000 mov dword ptr [rbp + 0x38], 0
0x1408c6e9e 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x1408c6ea2 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x1408c6ea6 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x1408c6ead 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x1408c6eb4 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x1408c6ebb c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x1408c6ec5 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x1408c6ecd 48c745b008000000 mov qword ptr [rbp - 0x50], 8
0x1408c6ed5 48c745b800000000 mov qword ptr [rbp - 0x48], 0
0x1408c6edd 48c745c010000000 mov qword ptr [rbp - 0x40], 0x10
0x1408c6ee5 48c745d810000000 mov qword ptr [rbp - 0x28], 0x10
0x1408c6eed 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x1408c6ef5 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x1408c6efd 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x1408c6f05 48b80000000000000080 movabs rax, 0x8000000000000000
0x1408c6f0f 48894540 mov qword ptr [rbp + 0x40], rax
0x1408c6f13 488d0dd3be1d02 lea rcx, [rip + 0x21dbed3]
0x1408c6f1a 48894d48 mov qword ptr [rbp + 0x48], rcx
0x1408c6f1e 48c7455038000000 mov qword ptr [rbp + 0x50], 0x38
0x1408c6f26 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x1408c6f2e 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x1408c6f36 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x1408c6f41 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x1408c6f4c 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x1408c6f57 c785d800000000000000 mov dword ptr [rbp + 0xd8], 0
0x1408c6f61 66c785dc0000000100 mov word ptr [rbp + 0xdc], 1
0x1408c6f6a 488985f0000000 mov qword ptr [rbp + 0xf0], rax
0x1408c6f71 c7853001000000000000 mov dword ptr [rbp + 0x130], 0
0x1408c6f7b 0fb60587dcd602 movzx eax, byte ptr [rip + 0x2d6dc87]
0x1408c6f82 b990010000 mov ecx, 0x190
0x1408c6f87 ba08000000 mov edx, 8
0x1408c6f8c e87fccd4ff call 0x140613c10
0x1408c6f91 4885c0 test rax, rax
0x1408c6f94 7435 je 0x1408c6fcb
0x1408c6f96 4889c7 mov rdi, rax
0x1408c6f99 488d55a8 lea rdx, [rbp - 0x58]
0x1408c6f9d 41b890010000 mov r8d, 0x190
0x1408c6fa3 4889c1 mov rcx, rax
0x1408c6fa6 e855cc0502 call 0x142923c00
0x1408c6fab 4c8d05cebb1d02 lea r8, [rip + 0x21dbbce]
0x1408c6fb2 4889f1 mov rcx, rsi
0x1408c6fb5 4889fa mov rdx, rdi
0x1408c6fb8 e853db4100 call 0x140ce4b10
0x1408c6fbd 4889f0 mov rax, rsi
0x1408c6fc0 4881c4c0010000 add rsp, 0x1c0
0x1408c6fc7 5f pop rdi
0x1408c6fc8 5e pop rsi
0x1408c6fc9 5d pop rbp
0x1408c6fca c3 ret 
0x1408c6fcb b908000000 mov ecx, 8
0x1408c6fd0 ba90010000 mov edx, 0x190
0x1408c6fd5 e826200102 call 0x1428d9000
0x1408c6fda 0f0b ud2 
0x1408c6fdc 0f1f4000 nop dword ptr [rax]
0x1408c6fe0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408c6fe5 55 push rbp
0x1408c6fe6 56 push rsi
0x1408c6fe7 57 push rdi
0x1408c6fe8 4883ec20 sub rsp, 0x20
0x1408c6fec 488daa80000000 lea rbp, [rdx + 0x80]
0x1408c6ff3 488d4da8 lea rcx, [rbp - 0x58]
0x1408c6ff7 e894f4d1ff call 0x1405e6490
0x1408c6ffc 90 nop 
0x1408c6ffd 4883c420 add rsp, 0x20
0x1408c7001 5f pop rdi
0x1408c7002 5e pop rsi
0x1408c7003 5d pop rbp
0x1408c7004 c3 ret 
