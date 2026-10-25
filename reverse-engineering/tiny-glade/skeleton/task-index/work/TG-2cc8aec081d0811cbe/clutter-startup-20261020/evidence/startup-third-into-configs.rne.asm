1422c3460: 55 push rbp
1422c3461: 56 push rsi
1422c3462: 57 push rdi
1422c3463: 4881ecb0010000 sub rsp, 0x1b0
1422c346a: 488dac2480000000 lea rbp, [rsp + 0x80]
1422c3472: 48c78528010000feffffff mov qword ptr [rbp + 0x128], 0xfffffffffffffffe
1422c347d: 4889ce mov rsi, rcx
1422c3480: 0f57c0 xorps xmm0, xmm0
1422c3483: 0f1145c0 movups xmmword ptr [rbp - 0x40], xmm0
1422c3487: 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
1422c348b: 0f1145f0 movups xmmword ptr [rbp - 0x10], xmm0
1422c348f: 0f114508 movups xmmword ptr [rbp + 8], xmm0
1422c3493: 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
1422c3497: c7453000000000 mov dword ptr [rbp + 0x30], 0
1422c349e: 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
1422c34a2: 0f114570 movups xmmword ptr [rbp + 0x70], xmm0
1422c34a6: 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
1422c34ad: 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
1422c34b4: 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
1422c34bb: c785c800000000000000 mov dword ptr [rbp + 0xc8], 0
1422c34c5: 48c745a000000000 mov qword ptr [rbp - 0x60], 0
1422c34cd: 48c745a808000000 mov qword ptr [rbp - 0x58], 8
1422c34d5: 48c745b000000000 mov qword ptr [rbp - 0x50], 0
1422c34dd: 48c745b810000000 mov qword ptr [rbp - 0x48], 0x10
1422c34e5: 48c745d010000000 mov qword ptr [rbp - 0x30], 0x10
1422c34ed: 48c745e810000000 mov qword ptr [rbp - 0x18], 0x10
1422c34f5: 48c7450010000000 mov qword ptr [rbp], 0x10
1422c34fd: 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
1422c3505: 48b80000000000000080 movabs rax, 0x8000000000000000
1422c350f: 48894538 mov qword ptr [rbp + 0x38], rax
1422c3513: 488d0d2ed8d700 lea rcx, [rip + 0xd7d82e]
1422c351a: 48894d40 mov qword ptr [rbp + 0x40], rcx
1422c351e: 48c745484b000000 mov qword ptr [rbp + 0x48], 0x4b
1422c3526: 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
1422c352e: 48c7456810000000 mov qword ptr [rbp + 0x68], 0x10
1422c3536: 48c7858000000010000000 mov qword ptr [rbp + 0x80], 0x10
1422c3541: 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
1422c354c: 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
1422c3557: c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
1422c3561: 66c785d40000000100 mov word ptr [rbp + 0xd4], 1
1422c356a: 488985e0000000 mov qword ptr [rbp + 0xe0], rax
1422c3571: c7852001000000000000 mov dword ptr [rbp + 0x120], 0
1422c357b: 0fb60587163701 movzx eax, byte ptr [rip + 0x1371687]
1422c3582: b988010000 mov ecx, 0x188
1422c3587: ba08000000 mov edx, 8
1422c358c: e87f0635fe call 0x140613c10
1422c3591: 4885c0 test rax, rax
1422c3594: 7435 je 0x1422c35cb
1422c3596: 4889c7 mov rdi, rax
1422c3599: 488d55a0 lea rdx, [rbp - 0x60]
1422c359d: 41b888010000 mov r8d, 0x188
1422c35a3: 4889c1 mov rcx, rax
1422c35a6: e855066600 call 0x142923c00
1422c35ab: 4c8d05ded6d700 lea r8, [rip + 0xd7d6de]
1422c35b2: 4889f1 mov rcx, rsi
1422c35b5: 4889fa mov rdx, rdi
1422c35b8: e85315a2fe call 0x140ce4b10
1422c35bd: 4889f0 mov rax, rsi
1422c35c0: 4881c4b0010000 add rsp, 0x1b0
1422c35c7: 5f pop rdi
1422c35c8: 5e pop rsi
1422c35c9: 5d pop rbp
1422c35ca: c3 ret 
1422c35cb: b908000000 mov ecx, 8
1422c35d0: ba88010000 mov edx, 0x188
1422c35d5: e8265a6100 call 0x1428d9000
1422c35da: 0f0b ud2 
1422c35dc: 0f1f4000 nop dword ptr [rax]
1422c35e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1422c35e5: 55 push rbp
1422c35e6: 56 push rsi
1422c35e7: 57 push rdi
1422c35e8: 4883ec20 sub rsp, 0x20
1422c35ec: 488daa80000000 lea rbp, [rdx + 0x80]
1422c35f3: 488d4da0 lea rcx, [rbp - 0x60]
1422c35f7: e8443132fe call 0x1405e6740
1422c35fc: 90 nop 
1422c35fd: 4883c420 add rsp, 0x20
1422c3601: 5f pop rdi
1422c3602: 5e pop rsi
1422c3603: 5d pop rbp
1422c3604: c3 ret 
