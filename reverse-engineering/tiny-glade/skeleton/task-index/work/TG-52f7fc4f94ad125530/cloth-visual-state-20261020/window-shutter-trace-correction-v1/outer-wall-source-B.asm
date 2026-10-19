1407ec3a1: b9feffffff mov ecx, 0xfffffffe
1407ec3a6: 29c1 sub ecx, eax
1407ec3a8: f7d0 not eax
1407ec3aa: 89c2 mov edx, eax
1407ec3ac: 21ca and edx, ecx
1407ec3ae: 89958c010000 mov dword ptr [rbp + 0x18c], edx
1407ec3b4: f30fbcc0 tzcnt eax, eax
1407ec3b8: 48f7d8 neg rax
1407ec3bb: 4869c098000000 imul rax, rax, 0x98
1407ec3c2: 4a8d1428 lea rdx, [rax + r13]
1407ec3c6: 4881c270ffffff add rdx, -0x90
1407ec3cd: 498b840568ffffff mov rax, qword ptr [r13 + rax - 0x98]
1407ec3d5: 488985f0010000 mov qword ptr [rbp + 0x1f0], rax
1407ec3dc: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ec3e3: e888ec2200 call 0x140a1b070 ; country_core::resources::walls::decorator_storage::Decorators::iter_modified
