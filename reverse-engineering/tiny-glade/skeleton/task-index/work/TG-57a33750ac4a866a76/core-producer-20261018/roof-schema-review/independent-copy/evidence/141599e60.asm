0x141599e60 55 push rbp
0x141599e61 4157 push r15
0x141599e63 4156 push r14
0x141599e65 4154 push r12
0x141599e67 56 push rsi
0x141599e68 57 push rdi
0x141599e69 53 push rbx
0x141599e6a 4881ec90000000 sub rsp, 0x90
0x141599e71 488dac2480000000 lea rbp, [rsp + 0x80]
0x141599e79 48c74508feffffff mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x141599e81 4d89c4 mov r12, r8
0x141599e84 4d85c0 test r8, r8
0x141599e87 0f889f010000 js 0x14159a02c
0x141599e8d 4c89cf mov rdi, r9
0x141599e90 4889ce mov rsi, rcx
0x141599e93 7444 je 0x141599ed9
0x141599e95 4989d6 mov r14, rdx
0x141599e98 0fb6056aad0902 movzx eax, byte ptr [rip + 0x209ad6a]
0x141599e9f 41bf01000000 mov r15d, 1
0x141599ea5 ba01000000 mov edx, 1
0x141599eaa 4c89e1 mov rcx, r12
0x141599ead e85e9d07ff call 0x140613c10
0x141599eb2 4885c0 test rax, rax
0x141599eb5 0f8474010000 je 0x14159a02f
0x141599ebb 4889c3 mov rbx, rax
0x141599ebe 4889c1 mov rcx, rax
0x141599ec1 4c89f2 mov rdx, r14
0x141599ec4 4d89e0 mov r8, r12
0x141599ec7 e8349d3801 call 0x142923c00
0x141599ecc 488b16 mov rdx, qword ptr [rsi]
0x141599ecf 4889d0 mov rax, rdx
0x141599ed2 48f7d8 neg rax
0x141599ed5 7112 jno 0x141599ee9
0x141599ed7 eb21 jmp 0x141599efa
0x141599ed9 bb01000000 mov ebx, 1
0x141599ede 488b16 mov rdx, qword ptr [rsi]
0x141599ee1 4889d0 mov rax, rdx
0x141599ee4 48f7d8 neg rax
0x141599ee7 7011 jo 0x141599efa
0x141599ee9 730f jae 0x141599efa
0x141599eeb 488b4e08 mov rcx, qword ptr [rsi + 8]
0x141599eef 41b801000000 mov r8d, 1
0x141599ef5 e8269d07ff call 0x140613c20
0x141599efa 48895e08 mov qword ptr [rsi + 8], rbx
0x141599efe 4c896610 mov qword ptr [rsi + 0x10], r12
0x141599f02 48b80000000000000080 movabs rax, 0x8000000000000000
0x141599f0c 488906 mov qword ptr [rsi], rax
0x141599f0f 488b4608 mov rax, qword ptr [rsi + 8]
0x141599f13 488945d0 mov qword ptr [rbp - 0x30], rax
0x141599f17 488b4610 mov rax, qword ptr [rsi + 0x10]
0x141599f1b 488945d8 mov qword ptr [rbp - 0x28], rax
0x141599f1f 4c8965c8 mov qword ptr [rbp - 0x38], r12
0x141599f23 803f00 cmp byte ptr [rdi], 0
0x141599f26 0fb605dcac0902 movzx eax, byte ptr [rip + 0x209acdc]
0x141599f2d 742c je 0x141599f5b
0x141599f2f bf08000000 mov edi, 8
0x141599f34 b908000000 mov ecx, 8
0x141599f39 ba01000000 mov edx, 1
0x141599f3e e8cd9c07ff call 0x140613c10
0x141599f43 4885c0 test rax, rax
0x141599f46 0f84f5000000 je 0x14159a041
0x141599f4c 48b94f76657268616e67 movabs rcx, 0x676e61687265764f
0x141599f56 488908 mov qword ptr [rax], rcx
0x141599f59 eb2e jmp 0x141599f89
0x141599f5b bf07000000 mov edi, 7
0x141599f60 b907000000 mov ecx, 7
0x141599f65 ba01000000 mov edx, 1
0x141599f6a e8a19c07ff call 0x140613c10
0x141599f6f 4885c0 test rax, rax
0x141599f72 0f84e5000000 je 0x14159a05d
0x141599f78 8b0d0d586f01 mov ecx, dword ptr [rip + 0x16f580d]
0x141599f7e 894803 mov dword ptr [rax + 3], ecx
0x141599f81 8b0d01586f01 mov ecx, dword ptr [rip + 0x16f5801]
0x141599f87 8908 mov dword ptr [rax], ecx
0x141599f89 c645a803 mov byte ptr [rbp - 0x58], 3
0x141599f8d 48897db0 mov qword ptr [rbp - 0x50], rdi
0x141599f91 488945b8 mov qword ptr [rbp - 0x48], rax
0x141599f95 48897dc0 mov qword ptr [rbp - 0x40], rdi
0x141599f99 4883c618 add rsi, 0x18
0x141599f9d 488d4de0 lea rcx, [rbp - 0x20]
0x141599fa1 4c8d45c8 lea r8, [rbp - 0x38]
0x141599fa5 4c8d4da8 lea r9, [rbp - 0x58]
0x141599fa9 4889f2 mov rdx, rsi
0x141599fac e87f1aeaff call 0x14143ba30
0x141599fb1 0fb645e0 movzx eax, byte ptr [rbp - 0x20]
0x141599fb5 4883f806 cmp rax, 6
0x141599fb9 7754 ja 0x14159a00f
0x141599fbb 488d0d7aea6d01 lea rcx, [rip + 0x16dea7a]
0x141599fc2 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x141599fc6 4801c8 add rax, rcx
0x141599fc9 ffe0 jmp rax
0x141599fcb 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x141599fcf 4885d2 test rdx, rdx
0x141599fd2 7444 je 0x14159a018
0x141599fd4 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x141599fd8 41b801000000 mov r8d, 1
0x141599fde eb28 jmp 0x14159a008
0x141599fe0 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x141599fe4 488b55f8 mov rdx, qword ptr [rbp - 8]
0x141599fe8 48894d00 mov qword ptr [rbp], rcx
0x141599fec e87fed2eff call 0x140888d70
0x141599ff1 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x141599ff5 4885d2 test rdx, rdx
0x141599ff8 741e je 0x14159a018
0x141599ffa 48c1e205 shl rdx, 5
0x141599ffe 41b808000000 mov r8d, 8
0x14159a004 488b4d00 mov rcx, qword ptr [rbp]
0x14159a008 e8139c07ff call 0x140613c20
0x14159a00d eb09 jmp 0x14159a018
0x14159a00f 488d4de8 lea rcx, [rbp - 0x18]
0x14159a013 e828ef2fff call 0x140898f40
0x14159a018 31c0 xor eax, eax
0x14159a01a 4881c490000000 add rsp, 0x90
0x14159a021 5b pop rbx
0x14159a022 5f pop rdi
0x14159a023 5e pop rsi
0x14159a024 415c pop r12
0x14159a026 415e pop r14
0x14159a028 415f pop r15
0x14159a02a 5d pop rbp
0x14159a02b c3 ret 
0x14159a02c 4531ff xor r15d, r15d
0x14159a02f 4c8d05a2e16d01 lea r8, [rip + 0x16de1a2]
0x14159a036 4c89f9 mov rcx, r15
0x14159a039 4c89e2 mov rdx, r12
0x14159a03c e8a2ef3301 call 0x1428d8fe3
0x14159a041 4c896500 mov qword ptr [rbp], r12
0x14159a045 4c8d05bc526f01 lea r8, [rip + 0x16f52bc]
0x14159a04c b901000000 mov ecx, 1
0x14159a051 ba08000000 mov edx, 8
0x14159a056 e888ef3301 call 0x1428d8fe3
0x14159a05b eb1a jmp 0x14159a077
0x14159a05d 4c896500 mov qword ptr [rbp], r12
0x14159a061 4c8d05a0526f01 lea r8, [rip + 0x16f52a0]
0x14159a068 b901000000 mov ecx, 1
0x14159a06d ba07000000 mov edx, 7
0x14159a072 e86cef3301 call 0x1428d8fe3
0x14159a077 0f0b ud2 
0x14159a079 0f1f8000000000 nop dword ptr [rax]
0x14159a080 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14159a085 55 push rbp
0x14159a086 4157 push r15
0x14159a088 4156 push r14
0x14159a08a 4154 push r12
0x14159a08c 56 push rsi
0x14159a08d 57 push rdi
0x14159a08e 53 push rbx
0x14159a08f 4883ec20 sub rsp, 0x20
0x14159a093 488daa80000000 lea rbp, [rdx + 0x80]
0x14159a09a 48837d0000 cmp qword ptr [rbp], 0
0x14159a09f 7413 je 0x14159a0b4
0x14159a0a1 488b4dd0 mov rcx, qword ptr [rbp - 0x30]
0x14159a0a5 41b801000000 mov r8d, 1
0x14159a0ab 488b5500 mov rdx, qword ptr [rbp]
0x14159a0af e86c9b07ff call 0x140613c20
0x14159a0b4 90 nop 
0x14159a0b5 4883c420 add rsp, 0x20
0x14159a0b9 5b pop rbx
0x14159a0ba 5f pop rdi
0x14159a0bb 5e pop rsi
0x14159a0bc 415c pop r12
0x14159a0be 415e pop r14
0x14159a0c0 415f pop r15
0x14159a0c2 5d pop rbp
0x14159a0c3 c3 ret 
0x14159a0c4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14159a0d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14159a0d5 55 push rbp
0x14159a0d6 4157 push r15
0x14159a0d8 4156 push r14
0x14159a0da 4154 push r12
0x14159a0dc 56 push rsi
0x14159a0dd 57 push rdi
0x14159a0de 53 push rbx
0x14159a0df 4883ec20 sub rsp, 0x20
0x14159a0e3 488daa80000000 lea rbp, [rdx + 0x80]
0x14159a0ea 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x14159a0ee 4885d2 test rdx, rdx
0x14159a0f1 7413 je 0x14159a106
0x14159a0f3 48c1e205 shl rdx, 5
0x14159a0f7 41b808000000 mov r8d, 8
0x14159a0fd 488b4d00 mov rcx, qword ptr [rbp]
0x14159a101 e81a9b07ff call 0x140613c20
0x14159a106 90 nop 
0x14159a107 4883c420 add rsp, 0x20
0x14159a10b 5b pop rbx
0x14159a10c 5f pop rdi
0x14159a10d 5e pop rsi
0x14159a10e 415c pop r12
0x14159a110 415e pop r14
0x14159a112 415f pop r15
0x14159a114 5d pop rbp
0x14159a115 c3 ret 
