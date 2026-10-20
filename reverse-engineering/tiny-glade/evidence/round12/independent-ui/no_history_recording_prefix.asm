1409a8f10: 56                       push rsi
1409a8f11: 4883ec20                 sub rsp, 0x20
1409a8f15: 4889ce                   mov rsi, rcx
1409a8f18: 80a2a9000000fe           and byte ptr [rdx + 0xa9], 0xfe
1409a8f1f: 41b8b0000000             mov r8d, 0xb0
1409a8f25: e8d6acf701               call 0x142923c00
1409a8f2a: 4889f0                   mov rax, rsi
1409a8f2d: 4883c420                 add rsp, 0x20
1409a8f31: 5e                       pop rsi
1409a8f32: c3                       ret 
1409a8f33: cc                       int3 
1409a8f34: cc                       int3 
1409a8f35: cc                       int3 
1409a8f36: cc                       int3 
1409a8f37: cc                       int3 
1409a8f38: cc                       int3 
1409a8f39: cc                       int3 
1409a8f3a: cc                       int3 
1409a8f3b: cc                       int3 
1409a8f3c: cc                       int3 
1409a8f3d: cc                       int3 
1409a8f3e: cc                       int3 
1409a8f3f: cc                       int3 
1409a8f40: 56                       push rsi
1409a8f41: 4883ec20                 sub rsp, 0x20
1409a8f45: 4889ce                   mov rsi, rcx
1409a8f48: 0fb682a9000000           movzx eax, byte ptr [rdx + 0xa9]
