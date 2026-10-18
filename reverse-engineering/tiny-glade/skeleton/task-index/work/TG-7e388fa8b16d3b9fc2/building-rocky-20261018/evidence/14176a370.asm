0x14176a370 55 push rbp
0x14176a371 56 push rsi
0x14176a372 57 push rdi
0x14176a373 4881ece0010000 sub rsp, 0x1e0
0x14176a37a 488dac2480000000 lea rbp, [rsp + 0x80]
0x14176a382 48c78558010000feffffff mov qword ptr [rbp + 0x158], 0xfffffffffffffffe
0x14176a38d 4889ce mov rsi, rcx
0x14176a390 0f57c0 xorps xmm0, xmm0
0x14176a393 0f1145c8 movups xmmword ptr [rbp - 0x38], xmm0
0x14176a397 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
0x14176a39b 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x14176a39f 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x14176a3a3 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x14176a3a7 c7453800000000 mov dword ptr [rbp + 0x38], 0
0x14176a3ae 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x14176a3b2 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x14176a3b6 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x14176a3bd 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x14176a3c4 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x14176a3cb c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x14176a3d5 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x14176a3dd 48c745b008000000 mov qword ptr [rbp - 0x50], 8
0x14176a3e5 48c745b800000000 mov qword ptr [rbp - 0x48], 0
0x14176a3ed 48c745c010000000 mov qword ptr [rbp - 0x40], 0x10
0x14176a3f5 48c745d810000000 mov qword ptr [rbp - 0x28], 0x10
0x14176a3fd 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x14176a405 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x14176a40d 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x14176a415 48b80000000000000080 movabs rax, 0x8000000000000000
0x14176a41f 48894540 mov qword ptr [rbp + 0x40], rax
0x14176a423 488d0d87d95601 lea rcx, [rip + 0x156d987]
0x14176a42a 48894d48 mov qword ptr [rbp + 0x48], rcx
0x14176a42e 48c7455037000000 mov qword ptr [rbp + 0x50], 0x37
0x14176a436 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x14176a43e 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x14176a446 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x14176a451 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x14176a45c 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x14176a467 c785d800000000000000 mov dword ptr [rbp + 0xd8], 0
0x14176a471 66c785dc0000000100 mov word ptr [rbp + 0xdc], 1
0x14176a47a 48898510010000 mov qword ptr [rbp + 0x110], rax
0x14176a481 c7855001000000000000 mov dword ptr [rbp + 0x150], 0
0x14176a48b 0fb60577a7ec01 movzx eax, byte ptr [rip + 0x1eca777]
0x14176a492 b9b0010000 mov ecx, 0x1b0
0x14176a497 ba08000000 mov edx, 8
0x14176a49c e86f97eafe call 0x140613c10
0x14176a4a1 4885c0 test rax, rax
0x14176a4a4 7435 je 0x14176a4db
0x14176a4a6 4889c7 mov rdi, rax
0x14176a4a9 488d55a8 lea rdx, [rbp - 0x58]
0x14176a4ad 41b8b0010000 mov r8d, 0x1b0
0x14176a4b3 4889c1 mov rcx, rax
0x14176a4b6 e845971b01 call 0x142923c00
0x14176a4bb 4c8d05ded75601 lea r8, [rip + 0x156d7de]
0x14176a4c2 4889f1 mov rcx, rsi
0x14176a4c5 4889fa mov rdx, rdi
0x14176a4c8 e843a657ff call 0x140ce4b10
0x14176a4cd 4889f0 mov rax, rsi
0x14176a4d0 4881c4e0010000 add rsp, 0x1e0
0x14176a4d7 5f pop rdi
0x14176a4d8 5e pop rsi
0x14176a4d9 5d pop rbp
0x14176a4da c3 ret 
0x14176a4db b908000000 mov ecx, 8
0x14176a4e0 bab0010000 mov edx, 0x1b0
0x14176a4e5 e816eb1601 call 0x1428d9000
0x14176a4ea 0f0b ud2 
0x14176a4ec 0f1f4000 nop dword ptr [rax]
0x14176a4f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14176a4f5 55 push rbp
0x14176a4f6 56 push rsi
0x14176a4f7 57 push rdi
0x14176a4f8 4883ec20 sub rsp, 0x20
0x14176a4fc 488daa80000000 lea rbp, [rdx + 0x80]
0x14176a503 488d4da8 lea rcx, [rbp - 0x58]
0x14176a507 e8849412ff call 0x140893990
0x14176a50c 90 nop 
0x14176a50d 4883c420 add rsp, 0x20
0x14176a511 5f pop rdi
0x14176a512 5e pop rsi
0x14176a513 5d pop rbp
0x14176a514 c3 ret 
