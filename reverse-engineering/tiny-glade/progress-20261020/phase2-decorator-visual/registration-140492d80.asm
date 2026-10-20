140492d80 push rbp
140492d81 push rsi
140492d82 push rdi
140492d83 push rbx
140492d84 sub rsp, 0x378
140492d8b lea rbp, [rsp + 0x80]
140492d93 mov qword ptr [rbp + 0x2f0], 0xfffffffffffffffe
140492d9e mov rsi, rcx
140492da1 movzx eax, byte ptr [rip + 0x31a1e61]
140492da8 mov ecx, 0xe0
140492dad mov edx, 8
140492db2 call 0x140613c10
140492db7 test rax, rax
140492dba je 0x140493030
140492dc0 mov qword ptr [rbp + 0x2e8], rax
140492dc7 xorps xmm0, xmm0
140492dca movups xmmword ptr [rbp - 0x40], xmm0
140492dce movups xmmword ptr [rbp - 0x28], xmm0
140492dd2 movups xmmword ptr [rbp - 0x10], xmm0
140492dd6 movups xmmword ptr [rbp + 8], xmm0
140492dda movups xmmword ptr [rbp + 0x20], xmm0
140492dde mov dword ptr [rbp + 0x30], 0
140492de5 movups xmmword ptr [rbp + 0x58], xmm0
140492de9 movups xmmword ptr [rbp + 0x70], xmm0
140492ded movups xmmword ptr [rbp + 0x88], xmm0
140492df4 movups xmmword ptr [rbp + 0xa0], xmm0
140492dfb movups xmmword ptr [rbp + 0xb8], xmm0
140492e02 mov dword ptr [rbp + 0xc8], 0
140492e0c mov qword ptr [rbp - 0x60], 0
140492e14 mov qword ptr [rbp - 0x58], 8
140492e1c mov qword ptr [rbp - 0x50], 0
140492e24 mov qword ptr [rbp - 0x48], 0x10
140492e2c mov qword ptr [rbp - 0x30], 0x10
140492e34 mov qword ptr [rbp - 0x18], 0x10
140492e3c mov qword ptr [rbp], 0x10
140492e44 mov qword ptr [rbp + 0x18], 0x10
140492e4c movabs rbx, 0x8000000000000000
140492e56 mov qword ptr [rbp + 0x38], rbx
140492e5a lea rax, [rip + 0x254c15a]
140492e61 mov qword ptr [rbp + 0x40], rax
140492e65 mov qword ptr [rbp + 0x48], 0x66
140492e6d mov qword ptr [rbp + 0x50], 0x10
140492e75 mov qword ptr [rbp + 0x68], 0x10
140492e7d mov qword ptr [rbp + 0x80], 0x10
140492e88 mov qword ptr [rbp + 0x98], 0x10
140492e93 mov qword ptr [rbp + 0xb0], 0x10
140492e9e mov dword ptr [rbp + 0xd0], 0
140492ea8 mov word ptr [rbp + 0xd4], 1
140492eb1 mov qword ptr [rbp + 0x1d0], rbx
140492eb8 mov dword ptr [rbp + 0x270], 0
140492ec2 movzx eax, byte ptr [rip + 0x31a1d40]
140492ec9 mov ecx, 0x2d8
140492ece mov edx, 8
140492ed3 call 0x140613c10
140492ed8 test rax, rax
140492edb je 0x14049303f
140492ee1 mov rdi, rax
140492ee4 lea rdx, [rbp - 0x60]
140492ee8 mov r8d, 0x2d8
140492eee mov rcx, rax
140492ef1 call 0x142923c00
140492ef6 lea r8, [rip + 0x254bfa3]
140492efd lea rcx, [rbp + 0x278]
140492f04 mov rdx, rdi
140492f07 call 0x140ce4b10
140492f0c lea rcx, [rbp - 0x60]
140492f10 call 0x14015ce60
140492f15 movups xmm0, xmmword ptr [rbp + 0x2d8]
140492f1c mov rax, qword ptr [rbp + 0x2e8]
140492f23 movups xmmword ptr [rax + 0x60], xmm0
140492f27 movups xmm0, xmmword ptr [rbp + 0x2c8]
140492f2e movups xmmword ptr [rax + 0x50], xmm0
140492f32 movups xmm0, xmmword ptr [rbp + 0x2b8]
140492f39 movups xmmword ptr [rax + 0x40], xmm0
140492f3d movups xmm0, xmmword ptr [rbp + 0x278]
140492f44 movups xmm1, xmmword ptr [rbp + 0x288]
140492f4b movups xmm2, xmmword ptr [rbp + 0x298]
140492f52 movups xmm3, xmmword ptr [rbp + 0x2a8]
140492f59 movups xmmword ptr [rax + 0x30], xmm3
140492f5d movups xmmword ptr [rax + 0x20], xmm2
140492f61 movups xmmword ptr [rax + 0x10], xmm1
140492f65 movups xmmword ptr [rax], xmm0
140492f68 movups xmm0, xmmword ptr [rbp - 0x60]
140492f6c movups xmm1, xmmword ptr [rbp - 0x50]
140492f70 movups xmm2, xmmword ptr [rbp - 0x40]
140492f74 movups xmm3, xmmword ptr [rbp - 0x30]
140492f78 movups xmmword ptr [rax + 0x70], xmm0
140492f7c movups xmmword ptr [rax + 0x80], xmm1
140492f83 movups xmmword ptr [rax + 0x90], xmm2
140492f8a movups xmmword ptr [rax + 0xa0], xmm3
140492f91 movups xmm0, xmmword ptr [rbp - 0x20]
140492f95 movups xmmword ptr [rax + 0xb0], xmm0
140492f9c movups xmm0, xmmword ptr [rbp - 0x10]
140492fa0 movups xmmword ptr [rax + 0xc0], xmm0
140492fa7 movups xmm0, xmmword ptr [rbp]
140492fab movups xmmword ptr [rax + 0xd0], xmm0
140492fb2 mov qword ptr [rbp - 0x58], 2
140492fba mov qword ptr [rbp - 0x50], rax
140492fbe mov qword ptr [rbp - 0x48], 2
140492fc6 mov qword ptr [rbp - 0x40], 0
140492fce mov qword ptr [rbp - 0x38], 8
140492fd6 xorps xmm0, xmm0
140492fd9 movups xmmword ptr [rbp - 0x30], xmm0
140492fdd mov qword ptr [rbp - 0x60], rbx
140492fe1 lea rcx, [rbp - 0x28]
140492fe5 call 0x1401705d0
140492fea movups xmm0, xmmword ptr [rbp]
140492fee movups xmmword ptr [rsi + 0x60], xmm0
140492ff2 movups xmm0, xmmword ptr [rbp - 0x10]
140492ff6 movups xmmword ptr [rsi + 0x50], xmm0
140492ffa movups xmm0, xmmword ptr [rbp - 0x20]
140492ffe movups xmmword ptr [rsi + 0x40], xmm0
140493002 movups xmm0, xmmword ptr [rbp - 0x60]
140493006 movups xmm1, xmmword ptr [rbp - 0x50]
14049300a movups xmm2, xmmword ptr [rbp - 0x40]
14049300e movups xmm3, xmmword ptr [rbp - 0x30]
140493012 movups xmmword ptr [rsi + 0x30], xmm3
140493016 movups xmmword ptr [rsi + 0x20], xmm2
14049301a movups xmmword ptr [rsi + 0x10], xmm1
14049301e movups xmmword ptr [rsi], xmm0
140493021 mov rax, rsi
140493024 add rsp, 0x378
14049302b pop rbx
14049302c pop rdi
14049302d pop rsi
14049302e pop rbp
14049302f ret 
140493030 mov ecx, 8
140493035 mov edx, 0xe0
14049303a call 0x1428d9000
14049303f mov ecx, 8
140493044 mov edx, 0x2d8
140493049 call 0x1428d9000
14049304e ud2 
140493050 mov qword ptr [rsp + 0x10], rdx
140493055 push rbp
140493056 push rsi
140493057 push rdi
140493058 push rbx
140493059 sub rsp, 0x28
14049305d lea rbp, [rdx + 0x80]
140493064 lea rcx, [rbp - 0x60]
140493068 call 0x1405d96d0
14049306d nop 
14049306e add rsp, 0x28
140493072 pop rbx
140493073 pop rdi
140493074 pop rsi
140493075 pop rbp
140493076 ret 
140493077 nop word ptr [rax + rax]
140493080 mov qword ptr [rsp + 0x10], rdx
140493085 push rbp
140493086 push rsi
140493087 push rdi
140493088 push rbx
140493089 sub rsp, 0x28
14049308d lea rbp, [rdx + 0x80]
140493094 mov edx, 0xe0
140493099 mov r8d, 8
14049309f mov rcx, qword ptr [rbp + 0x2e8]
1404930a6 call 0x140613c20
1404930ab nop 
1404930ac add rsp, 0x28
1404930b0 pop rbx
1404930b1 pop rdi
1404930b2 pop rsi
1404930b3 pop rbp
1404930b4 ret 
1404930b5 nop word ptr cs:[rax + rax]
1404930c0 mov qword ptr [rsp + 0x10], rdx
1404930c5 push rbp
1404930c6 push rsi
1404930c7 push rdi
1404930c8 push rbx
1404930c9 sub rsp, 0x28
1404930cd lea rbp, [rdx + 0x80]
1404930d4 lea rcx, [rbp + 0x278]
1404930db call 0x140029880
1404930e0 nop 
1404930e1 add rsp, 0x28
1404930e5 pop rbx
1404930e6 pop rdi
1404930e7 pop rsi
1404930e8 pop rbp
1404930e9 ret 
1404930ea nop word ptr [rax + rax]
1404930f0 mov qword ptr [rsp + 0x10], rdx
1404930f5 push rbp
1404930f6 push rsi
1404930f7 push rdi
1404930f8 push rbx
1404930f9 sub rsp, 0x28
1404930fd lea rbp, [rdx + 0x80]
140493104 lea rcx, [rbp - 0x60]
140493108 call 0x140029880
14049310d nop 
14049310e add rsp, 0x28
140493112 pop rbx
140493113 pop rdi
140493114 pop rsi
140493115 pop rbp
140493116 ret 