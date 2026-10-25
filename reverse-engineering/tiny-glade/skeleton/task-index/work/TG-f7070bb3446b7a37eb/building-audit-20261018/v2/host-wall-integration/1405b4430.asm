0x1405b4430 55 push rbp
0x1405b4431 56 push rsi
0x1405b4432 57 push rdi
0x1405b4433 4881eca0010000 sub rsp, 0x1a0
0x1405b443a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1405b4442 48c78518010000feffffff mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
0x1405b444d 4889ce mov rsi, rcx
0x1405b4450 0f57c0 xorps xmm0, xmm0
0x1405b4453 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x1405b4457 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x1405b445b 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x1405b445f 0f114540 movups xmmword ptr [rbp + 0x40], xmm0
0x1405b4463 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x1405b4467 c7456800000000 mov dword ptr [rbp + 0x68], 0
0x1405b446e 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x1405b4475 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x1405b447c 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x1405b4483 0f1185d8000000 movups xmmword ptr [rbp + 0xd8], xmm0
0x1405b448a 0f1185f0000000 movups xmmword ptr [rbp + 0xf0], xmm0
0x1405b4491 c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x1405b449b 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x1405b44a3 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x1405b44ab 48c745e008000000 mov qword ptr [rbp - 0x20], 8
0x1405b44b3 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x1405b44bb 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x1405b44c3 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x1405b44cb 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x1405b44d3 48c7453810000000 mov qword ptr [rbp + 0x38], 0x10
0x1405b44db 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x1405b44e3 48b80000000000000080 movabs rax, 0x8000000000000000
0x1405b44ed 48894570 mov qword ptr [rbp + 0x70], rax
0x1405b44f1 488d059cd24402 lea rax, [rip + 0x244d29c]
0x1405b44f8 48894578 mov qword ptr [rbp + 0x78], rax
0x1405b44fc 48c7858000000045000000 mov qword ptr [rbp + 0x80], 0x45
0x1405b4507 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x1405b4512 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x1405b451d 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x1405b4528 48c785d000000010000000 mov qword ptr [rbp + 0xd0], 0x10
0x1405b4533 48c785e800000010000000 mov qword ptr [rbp + 0xe8], 0x10
0x1405b453e c7850801000000000000 mov dword ptr [rbp + 0x108], 0
0x1405b4548 66c7850c0100000100 mov word ptr [rbp + 0x10c], 1
0x1405b4551 c7851001000000000000 mov dword ptr [rbp + 0x110], 0
0x1405b455b 0fb605a7060803 movzx eax, byte ptr [rip + 0x30806a7]
0x1405b4562 b978010000 mov ecx, 0x178
0x1405b4567 ba08000000 mov edx, 8
0x1405b456c e89ff60500 call 0x140613c10
0x1405b4571 4885c0 test rax, rax
0x1405b4574 7435 je 0x1405b45ab
0x1405b4576 4889c7 mov rdi, rax
0x1405b4579 488d55a0 lea rdx, [rbp - 0x60]
0x1405b457d 41b878010000 mov r8d, 0x178
0x1405b4583 4889c1 mov rcx, rax
0x1405b4586 e875f63602 call 0x142923c00
0x1405b458b 4c8d0536d04402 lea r8, [rip + 0x244d036]
0x1405b4592 4889f1 mov rcx, rsi
0x1405b4595 4889fa mov rdx, rdi
0x1405b4598 e873057300 call 0x140ce4b10
0x1405b459d 4889f0 mov rax, rsi
0x1405b45a0 4881c4a0010000 add rsp, 0x1a0
0x1405b45a7 5f pop rdi
0x1405b45a8 5e pop rsi
0x1405b45a9 5d pop rbp
0x1405b45aa c3 ret 
0x1405b45ab b908000000 mov ecx, 8
0x1405b45b0 ba78010000 mov edx, 0x178
0x1405b45b5 e8464a3202 call 0x1428d9000
0x1405b45ba 0f0b ud2 
0x1405b45bc 0f1f4000 nop dword ptr [rax]
0x1405b45c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1405b45c5 55 push rbp
0x1405b45c6 56 push rsi
0x1405b45c7 57 push rdi
0x1405b45c8 4883ec20 sub rsp, 0x20
0x1405b45cc 488daa80000000 lea rbp, [rdx + 0x80]
0x1405b45d3 488d4da0 lea rcx, [rbp - 0x60]
0x1405b45d7 e814550300 call 0x1405e9af0
0x1405b45dc 90 nop 
0x1405b45dd 4883c420 add rsp, 0x20
0x1405b45e1 5f pop rdi
0x1405b45e2 5e pop rsi
0x1405b45e3 5d pop rbp
0x1405b45e4 c3 ret 
