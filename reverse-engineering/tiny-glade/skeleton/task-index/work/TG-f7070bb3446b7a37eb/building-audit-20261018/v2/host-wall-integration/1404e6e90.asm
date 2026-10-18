0x1404e6e90 55 push rbp
0x1404e6e91 56 push rsi
0x1404e6e92 57 push rdi
0x1404e6e93 4881ecc0010000 sub rsp, 0x1c0
0x1404e6e9a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1404e6ea2 48c78538010000feffffff mov qword ptr [rbp + 0x138], 0xfffffffffffffffe
0x1404e6ead 4889ce mov rsi, rcx
0x1404e6eb0 0f57c0 xorps xmm0, xmm0
0x1404e6eb3 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
0x1404e6eb7 0f114530 movups xmmword ptr [rbp + 0x30], xmm0
0x1404e6ebb 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1404e6ebf 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x1404e6ec3 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x1404e6ec7 c7858800000000000000 mov dword ptr [rbp + 0x88], 0
0x1404e6ed1 0f1185b0000000 movups xmmword ptr [rbp + 0xb0], xmm0
0x1404e6ed8 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
0x1404e6edf 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x1404e6ee6 0f1185f8000000 movups xmmword ptr [rbp + 0xf8], xmm0
0x1404e6eed 0f118510010000 movups xmmword ptr [rbp + 0x110], xmm0
0x1404e6ef4 c7852001000000000000 mov dword ptr [rbp + 0x120], 0
0x1404e6efe 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x1404e6f06 48c745f800000000 mov qword ptr [rbp - 8], 0
0x1404e6f0e 48c7450008000000 mov qword ptr [rbp], 8
0x1404e6f16 48c7450800000000 mov qword ptr [rbp + 8], 0
0x1404e6f1e 48c7451010000000 mov qword ptr [rbp + 0x10], 0x10
0x1404e6f26 48c7452810000000 mov qword ptr [rbp + 0x28], 0x10
0x1404e6f2e 48c7454010000000 mov qword ptr [rbp + 0x40], 0x10
0x1404e6f36 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x1404e6f3e 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x1404e6f46 48b80000000000000080 movabs rax, 0x8000000000000000
0x1404e6f50 48898590000000 mov qword ptr [rbp + 0x90], rax
0x1404e6f57 488d0533475002 lea rax, [rip + 0x2504733]
0x1404e6f5e 48898598000000 mov qword ptr [rbp + 0x98], rax
0x1404e6f65 48c785a000000065000000 mov qword ptr [rbp + 0xa0], 0x65
0x1404e6f70 48c785a800000010000000 mov qword ptr [rbp + 0xa8], 0x10
0x1404e6f7b 48c785c000000010000000 mov qword ptr [rbp + 0xc0], 0x10
0x1404e6f86 48c785d800000010000000 mov qword ptr [rbp + 0xd8], 0x10
0x1404e6f91 48c785f000000010000000 mov qword ptr [rbp + 0xf0], 0x10
0x1404e6f9c 48c7850801000010000000 mov qword ptr [rbp + 0x108], 0x10
0x1404e6fa7 c7852801000000000000 mov dword ptr [rbp + 0x128], 0
0x1404e6fb1 66c7852c0100000100 mov word ptr [rbp + 0x12c], 1
0x1404e6fba c7853001000000000000 mov dword ptr [rbp + 0x130], 0
0x1404e6fc4 0fb6053edc1403 movzx eax, byte ptr [rip + 0x314dc3e]
0x1404e6fcb b990010000 mov ecx, 0x190
0x1404e6fd0 ba08000000 mov edx, 8
0x1404e6fd5 e836cc1200 call 0x140613c10
0x1404e6fda 4885c0 test rax, rax
0x1404e6fdd 7435 je 0x1404e7014
0x1404e6fdf 4889c7 mov rdi, rax
0x1404e6fe2 488d55a8 lea rdx, [rbp - 0x58]
0x1404e6fe6 41b890010000 mov r8d, 0x190
0x1404e6fec 4889c1 mov rcx, rax
0x1404e6fef e80ccc4302 call 0x142923c00
0x1404e6ff4 4c8d0565435002 lea r8, [rip + 0x2504365]
0x1404e6ffb 4889f1 mov rcx, rsi
0x1404e6ffe 4889fa mov rdx, rdi
0x1404e7001 e80adb7f00 call 0x140ce4b10
0x1404e7006 4889f0 mov rax, rsi
0x1404e7009 4881c4c0010000 add rsp, 0x1c0
0x1404e7010 5f pop rdi
0x1404e7011 5e pop rsi
0x1404e7012 5d pop rbp
0x1404e7013 c3 ret 
0x1404e7014 b908000000 mov ecx, 8
0x1404e7019 ba90010000 mov edx, 0x190
0x1404e701e e8dd1f3f02 call 0x1428d9000
0x1404e7023 0f0b ud2 
0x1404e7025 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e7030 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e7035 55 push rbp
0x1404e7036 56 push rsi
0x1404e7037 57 push rdi
0x1404e7038 4883ec20 sub rsp, 0x20
0x1404e703c 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e7043 488d4da8 lea rcx, [rbp - 0x58]
0x1404e7047 e8f4b40e00 call 0x1405d2540
0x1404e704c 90 nop 
0x1404e704d 4883c420 add rsp, 0x20
0x1404e7051 5f pop rdi
0x1404e7052 5e pop rsi
0x1404e7053 5d pop rbp
0x1404e7054 c3 ret 
