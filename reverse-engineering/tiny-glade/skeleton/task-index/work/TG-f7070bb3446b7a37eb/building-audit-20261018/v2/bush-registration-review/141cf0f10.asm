0x141cf0f10 55 push rbp
0x141cf0f11 56 push rsi
0x141cf0f12 57 push rdi
0x141cf0f13 4881ec10040000 sub rsp, 0x410
0x141cf0f1a 488dac2480000000 lea rbp, [rsp + 0x80]
0x141cf0f22 48c78588030000feffffff mov qword ptr [rbp + 0x388], 0xfffffffffffffffe
0x141cf0f2d 4889ce mov rsi, rcx
0x141cf0f30 0f57c0 xorps xmm0, xmm0
0x141cf0f33 0f1145c0 movups xmmword ptr [rbp - 0x40], xmm0
0x141cf0f37 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
0x141cf0f3b 0f1145f0 movups xmmword ptr [rbp - 0x10], xmm0
0x141cf0f3f 0f114508 movups xmmword ptr [rbp + 8], xmm0
0x141cf0f43 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x141cf0f47 c7453000000000 mov dword ptr [rbp + 0x30], 0
0x141cf0f4e 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x141cf0f52 0f114570 movups xmmword ptr [rbp + 0x70], xmm0
0x141cf0f56 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x141cf0f5d 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
0x141cf0f64 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x141cf0f6b c785c800000000000000 mov dword ptr [rbp + 0xc8], 0
0x141cf0f75 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x141cf0f7d 48c745a808000000 mov qword ptr [rbp - 0x58], 8
0x141cf0f85 48c745b000000000 mov qword ptr [rbp - 0x50], 0
0x141cf0f8d 48c745b810000000 mov qword ptr [rbp - 0x48], 0x10
0x141cf0f95 48c745d010000000 mov qword ptr [rbp - 0x30], 0x10
0x141cf0f9d 48c745e810000000 mov qword ptr [rbp - 0x18], 0x10
0x141cf0fa5 48c7450010000000 mov qword ptr [rbp], 0x10
0x141cf0fad 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x141cf0fb5 48b80000000000000080 movabs rax, 0x8000000000000000
0x141cf0fbf 48894538 mov qword ptr [rbp + 0x38], rax
0x141cf0fc3 488d0d5e792401 lea rcx, [rip + 0x124795e]
0x141cf0fca 48894d40 mov qword ptr [rbp + 0x40], rcx
0x141cf0fce 48c7454855000000 mov qword ptr [rbp + 0x48], 0x55
0x141cf0fd6 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x141cf0fde 48c7456810000000 mov qword ptr [rbp + 0x68], 0x10
0x141cf0fe6 48c7858000000010000000 mov qword ptr [rbp + 0x80], 0x10
0x141cf0ff1 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
0x141cf0ffc 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x141cf1007 c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x141cf1011 66c785d40000000100 mov word ptr [rbp + 0xd4], 1
0x141cf101a 488985d8000000 mov qword ptr [rbp + 0xd8], rax
0x141cf1021 c7858003000000000000 mov dword ptr [rbp + 0x380], 0
0x141cf102b 0fb605d73b9401 movzx eax, byte ptr [rip + 0x1943bd7]
0x141cf1032 b9e8030000 mov ecx, 0x3e8
0x141cf1037 ba08000000 mov edx, 8
0x141cf103c e8cf2b92fe call 0x140613c10
0x141cf1041 4885c0 test rax, rax
0x141cf1044 7435 je 0x141cf107b
0x141cf1046 4889c7 mov rdi, rax
0x141cf1049 488d55a0 lea rdx, [rbp - 0x60]
0x141cf104d 41b8e8030000 mov r8d, 0x3e8
0x141cf1053 4889c1 mov rcx, rax
0x141cf1056 e8a52bc300 call 0x142923c00
0x141cf105b 4c8d050e782401 lea r8, [rip + 0x124780e]
0x141cf1062 4889f1 mov rcx, rsi
0x141cf1065 4889fa mov rdx, rdi
0x141cf1068 e8a33afffe call 0x140ce4b10
0x141cf106d 4889f0 mov rax, rsi
0x141cf1070 4881c410040000 add rsp, 0x410
0x141cf1077 5f pop rdi
0x141cf1078 5e pop rsi
0x141cf1079 5d pop rbp
0x141cf107a c3 ret 
0x141cf107b b908000000 mov ecx, 8
0x141cf1080 bae8030000 mov edx, 0x3e8
0x141cf1085 e8767fbe00 call 0x1428d9000
0x141cf108a 0f0b ud2 
0x141cf108c 0f1f4000 nop dword ptr [rax]
0x141cf1090 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141cf1095 55 push rbp
0x141cf1096 56 push rsi
0x141cf1097 57 push rdi
0x141cf1098 4883ec20 sub rsp, 0x20
0x141cf109c 488daa80000000 lea rbp, [rdx + 0x80]
0x141cf10a3 488d4da0 lea rcx, [rbp - 0x60]
0x141cf10a7 e8a42a1eff call 0x140ed3b50
0x141cf10ac 90 nop 
0x141cf10ad 4883c420 add rsp, 0x20
0x141cf10b1 5f pop rdi
0x141cf10b2 5e pop rsi
0x141cf10b3 5d pop rbp
0x141cf10b4 c3 ret 
