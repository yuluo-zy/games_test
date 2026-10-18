0x1413fb780 55 push rbp
0x1413fb781 56 push rsi
0x1413fb782 57 push rdi
0x1413fb783 4881ecc0010000 sub rsp, 0x1c0
0x1413fb78a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1413fb792 48c78538010000feffffff mov qword ptr [rbp + 0x138], 0xfffffffffffffffe
0x1413fb79d 4889ce mov rsi, rcx
0x1413fb7a0 0f57c0 xorps xmm0, xmm0
0x1413fb7a3 0f1145c0 movups xmmword ptr [rbp - 0x40], xmm0
0x1413fb7a7 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
0x1413fb7ab 0f1145f0 movups xmmword ptr [rbp - 0x10], xmm0
0x1413fb7af 0f114508 movups xmmword ptr [rbp + 8], xmm0
0x1413fb7b3 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x1413fb7b7 c7453000000000 mov dword ptr [rbp + 0x30], 0
0x1413fb7be 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x1413fb7c2 0f114570 movups xmmword ptr [rbp + 0x70], xmm0
0x1413fb7c6 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x1413fb7cd 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
0x1413fb7d4 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x1413fb7db c785c800000000000000 mov dword ptr [rbp + 0xc8], 0
0x1413fb7e5 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x1413fb7ed 48c745a808000000 mov qword ptr [rbp - 0x58], 8
0x1413fb7f5 48c745b000000000 mov qword ptr [rbp - 0x50], 0
0x1413fb7fd 48c745b810000000 mov qword ptr [rbp - 0x48], 0x10
0x1413fb805 48c745d010000000 mov qword ptr [rbp - 0x30], 0x10
0x1413fb80d 48c745e810000000 mov qword ptr [rbp - 0x18], 0x10
0x1413fb815 48c7450010000000 mov qword ptr [rbp], 0x10
0x1413fb81d 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x1413fb825 48b80000000000000080 movabs rax, 0x8000000000000000
0x1413fb82f 48894538 mov qword ptr [rbp + 0x38], rax
0x1413fb833 488d0d9bf48501 lea rcx, [rip + 0x185f49b]
0x1413fb83a 48894d40 mov qword ptr [rbp + 0x40], rcx
0x1413fb83e 48c745483d000000 mov qword ptr [rbp + 0x48], 0x3d
0x1413fb846 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x1413fb84e 48c7456810000000 mov qword ptr [rbp + 0x68], 0x10
0x1413fb856 48c7858000000010000000 mov qword ptr [rbp + 0x80], 0x10
0x1413fb861 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
0x1413fb86c 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x1413fb877 c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x1413fb881 66c785d40000000100 mov word ptr [rbp + 0xd4], 1
0x1413fb88a 488985f0000000 mov qword ptr [rbp + 0xf0], rax
0x1413fb891 c7853001000000000000 mov dword ptr [rbp + 0x130], 0
0x1413fb89b 0fb60567932302 movzx eax, byte ptr [rip + 0x2239367]
0x1413fb8a2 b998010000 mov ecx, 0x198
0x1413fb8a7 ba08000000 mov edx, 8
0x1413fb8ac e85f8321ff call 0x140613c10
0x1413fb8b1 4885c0 test rax, rax
0x1413fb8b4 7435 je 0x1413fb8eb
0x1413fb8b6 4889c7 mov rdi, rax
0x1413fb8b9 488d55a0 lea rdx, [rbp - 0x60]
0x1413fb8bd 41b898010000 mov r8d, 0x198
0x1413fb8c3 4889c1 mov rcx, rax
0x1413fb8c6 e835835201 call 0x142923c00
0x1413fb8cb 4c8d0506f18501 lea r8, [rip + 0x185f106]
0x1413fb8d2 4889f1 mov rcx, rsi
0x1413fb8d5 4889fa mov rdx, rdi
0x1413fb8d8 e833928eff call 0x140ce4b10
0x1413fb8dd 4889f0 mov rax, rsi
0x1413fb8e0 4881c4c0010000 add rsp, 0x1c0
0x1413fb8e7 5f pop rdi
0x1413fb8e8 5e pop rsi
0x1413fb8e9 5d pop rbp
0x1413fb8ea c3 ret 
0x1413fb8eb b908000000 mov ecx, 8
0x1413fb8f0 ba98010000 mov edx, 0x198
0x1413fb8f5 e806d74d01 call 0x1428d9000
0x1413fb8fa 0f0b ud2 
0x1413fb8fc 0f1f4000 nop dword ptr [rax]
0x1413fb900 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1413fb905 55 push rbp
0x1413fb906 56 push rsi
0x1413fb907 57 push rdi
0x1413fb908 4883ec20 sub rsp, 0x20
0x1413fb90c 488daa80000000 lea rbp, [rdx + 0x80]
0x1413fb913 488d4da0 lea rcx, [rbp - 0x60]
0x1413fb917 e8241b20ff call 0x1405fd440
0x1413fb91c 90 nop 
0x1413fb91d 4883c420 add rsp, 0x20
0x1413fb921 5f pop rdi
0x1413fb922 5e pop rsi
0x1413fb923 5d pop rbp
0x1413fb924 c3 ret 
