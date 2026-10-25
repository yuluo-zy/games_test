0x1402ad500 55 push rbp
0x1402ad501 56 push rsi
0x1402ad502 57 push rdi
0x1402ad503 4881ec90010000 sub rsp, 0x190
0x1402ad50a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1402ad512 48c78508010000feffffff mov qword ptr [rbp + 0x108], 0xfffffffffffffffe
0x1402ad51d 4889ce mov rsi, rcx
0x1402ad520 0f57c0 xorps xmm0, xmm0
0x1402ad523 0f1145e8 movups xmmword ptr [rbp - 0x18], xmm0
0x1402ad527 0f114500 movups xmmword ptr [rbp], xmm0
0x1402ad52b 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
0x1402ad52f 0f114530 movups xmmword ptr [rbp + 0x30], xmm0
0x1402ad533 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1402ad537 c7455800000000 mov dword ptr [rbp + 0x58], 0
0x1402ad53e 0f118580000000 movups xmmword ptr [rbp + 0x80], xmm0
0x1402ad545 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
0x1402ad54c 0f1185b0000000 movups xmmword ptr [rbp + 0xb0], xmm0
0x1402ad553 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
0x1402ad55a 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x1402ad561 c785f000000000000000 mov dword ptr [rbp + 0xf0], 0
0x1402ad56b 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x1402ad573 48c745c800000000 mov qword ptr [rbp - 0x38], 0
0x1402ad57b 48c745d008000000 mov qword ptr [rbp - 0x30], 8
0x1402ad583 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x1402ad58b 48c745e010000000 mov qword ptr [rbp - 0x20], 0x10
0x1402ad593 48c745f810000000 mov qword ptr [rbp - 8], 0x10
0x1402ad59b 48c7451010000000 mov qword ptr [rbp + 0x10], 0x10
0x1402ad5a3 48c7452810000000 mov qword ptr [rbp + 0x28], 0x10
0x1402ad5ab 48c7454010000000 mov qword ptr [rbp + 0x40], 0x10
0x1402ad5b3 48b80000000000000080 movabs rax, 0x8000000000000000
0x1402ad5bd 48894560 mov qword ptr [rbp + 0x60], rax
0x1402ad5c1 488d05c4fa6f02 lea rax, [rip + 0x26ffac4]
0x1402ad5c8 48894568 mov qword ptr [rbp + 0x68], rax
0x1402ad5cc 48c745705d000000 mov qword ptr [rbp + 0x70], 0x5d
0x1402ad5d4 48c7457810000000 mov qword ptr [rbp + 0x78], 0x10
0x1402ad5dc 48c7859000000010000000 mov qword ptr [rbp + 0x90], 0x10
0x1402ad5e7 48c785a800000010000000 mov qword ptr [rbp + 0xa8], 0x10
0x1402ad5f2 48c785c000000010000000 mov qword ptr [rbp + 0xc0], 0x10
0x1402ad5fd 48c785d800000010000000 mov qword ptr [rbp + 0xd8], 0x10
0x1402ad608 c785f800000000000000 mov dword ptr [rbp + 0xf8], 0
0x1402ad612 66c785fc0000000100 mov word ptr [rbp + 0xfc], 1
0x1402ad61b c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x1402ad625 0fb605dd753803 movzx eax, byte ptr [rip + 0x33875dd]
0x1402ad62c b968010000 mov ecx, 0x168
0x1402ad631 ba08000000 mov edx, 8
0x1402ad636 e8d5653600 call 0x140613c10
0x1402ad63b 4885c0 test rax, rax
0x1402ad63e 7435 je 0x1402ad675
0x1402ad640 4889c7 mov rdi, rax
0x1402ad643 488d55a0 lea rdx, [rbp - 0x60]
0x1402ad647 41b868010000 mov r8d, 0x168
0x1402ad64d 4889c1 mov rcx, rax
0x1402ad650 e8ab656702 call 0x142923c00
0x1402ad655 4c8d05bcf86f02 lea r8, [rip + 0x26ff8bc]
0x1402ad65c 4889f1 mov rcx, rsi
0x1402ad65f 4889fa mov rdx, rdi
0x1402ad662 e8a974a300 call 0x140ce4b10
0x1402ad667 4889f0 mov rax, rsi
0x1402ad66a 4881c490010000 add rsp, 0x190
0x1402ad671 5f pop rdi
0x1402ad672 5e pop rsi
0x1402ad673 5d pop rbp
0x1402ad674 c3 ret 
0x1402ad675 b908000000 mov ecx, 8
0x1402ad67a ba68010000 mov edx, 0x168
0x1402ad67f e87cb96202 call 0x1428d9000
0x1402ad684 0f0b ud2 
0x1402ad686 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1402ad690 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1402ad695 55 push rbp
0x1402ad696 56 push rsi
0x1402ad697 57 push rdi
0x1402ad698 4883ec20 sub rsp, 0x20
0x1402ad69c 488daa80000000 lea rbp, [rdx + 0x80]
0x1402ad6a3 488d4da0 lea rcx, [rbp - 0x60]
0x1402ad6a7 e864983300 call 0x1405e6f10
0x1402ad6ac 90 nop 
0x1402ad6ad 4883c420 add rsp, 0x20
0x1402ad6b1 5f pop rdi
0x1402ad6b2 5e pop rsi
0x1402ad6b3 5d pop rbp
0x1402ad6b4 c3 ret 
