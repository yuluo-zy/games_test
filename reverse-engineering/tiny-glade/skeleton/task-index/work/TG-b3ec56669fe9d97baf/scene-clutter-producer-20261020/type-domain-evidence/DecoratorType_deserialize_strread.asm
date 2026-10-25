0x140a47da0 56 push rsi
0x140a47da1 57 push rdi
0x140a47da2 53 push rbx
0x140a47da3 4883ec50 sub rsp, 0x50
0x140a47da7 4889ce mov rsi, rcx
0x140a47daa 488b4a20 mov rcx, qword ptr [rdx + 0x20]
0x140a47dae 488b4228 mov rax, qword ptr [rdx + 0x28]
0x140a47db2 4839c8 cmp rax, rcx
0x140a47db5 733f jae 0x140a47df6
0x140a47db7 4c8b4218 mov r8, qword ptr [rdx + 0x18]
0x140a47dbb 48ffc0 inc rax
0x140a47dbe 48f7d9 neg rcx
0x140a47dc1 48bf0026000001000000 movabs rdi, 0x100002600
0x140a47dcb 0f1f440000 nop dword ptr [rax + rax]
0x140a47dd0 450fb64c00ff movzx r9d, byte ptr [r8 + rax - 1]
0x140a47dd6 4983f922 cmp r9, 0x22
0x140a47dda 7768 ja 0x140a47e44
0x140a47ddc 4c0fa3cf bt rdi, r9
0x140a47de0 732f jae 0x140a47e11
0x140a47de2 48894228 mov qword ptr [rdx + 0x28], rax
0x140a47de6 4c8d0c01 lea r9, [rcx + rax]
0x140a47dea 49ffc1 inc r9
0x140a47ded 48ffc0 inc rax
0x140a47df0 4983f901 cmp r9, 1
0x140a47df4 75da jne 0x140a47dd0
0x140a47df6 48c744242805000000 mov qword ptr [rsp + 0x28], 5
0x140a47dff 488d442428 lea rax, [rsp + 0x28]
0x140a47e04 4889d1 mov rcx, rdx
0x140a47e07 4889c2 mov rdx, rax
0x140a47e0a e8118be501 call 0x1428a0920
0x140a47e0f eb63 jmp 0x140a47e74
0x140a47e11 4983f922 cmp r9, 0x22
0x140a47e15 752d jne 0x140a47e44
0x140a47e17 488d4c2428 lea rcx, [rsp + 0x28]
0x140a47e1c e80f589c00 call 0x14140d630
0x140a47e21 807c242801 cmp byte ptr [rsp + 0x28], 1
0x140a47e26 7447 je 0x140a47e6f
0x140a47e28 0fb6442429 movzx eax, byte ptr [rsp + 0x29]
0x140a47e2d 488d0d34790802 lea rcx, [rip + 0x2087934]
0x140a47e34 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x140a47e38 4801c8 add rax, rcx
0x140a47e3b ffe0 jmp rax
0x140a47e3d 66c7060038 mov word ptr [rsi], 0x3800
0x140a47e42 eb37 jmp 0x140a47e7b
0x140a47e44 4183f97b cmp r9d, 0x7b
0x140a47e48 0f8507010000 jne 0x140a47f55
0x140a47e4e fe4a30 dec byte ptr [rdx + 0x30]
0x140a47e51 0f8444020000 je 0x140a4809b
0x140a47e57 48894228 mov qword ptr [rdx + 0x28], rax
0x140a47e5b 488d4c2428 lea rcx, [rsp + 0x28]
0x140a47e60 4889d3 mov rbx, rdx
0x140a47e63 e8c8579c00 call 0x14140d630
0x140a47e68 807c242800 cmp byte ptr [rsp + 0x28], 0
0x140a47e6d 7417 je 0x140a47e86
0x140a47e6f 488b442430 mov rax, qword ptr [rsp + 0x30]
0x140a47e74 48894608 mov qword ptr [rsi + 8], rax
0x140a47e78 c60601 mov byte ptr [rsi], 1
0x140a47e7b 4889f0 mov rax, rsi
0x140a47e7e 4883c450 add rsp, 0x50
0x140a47e82 5b pop rbx
0x140a47e83 5f pop rdi
0x140a47e84 5e pop rsi
0x140a47e85 c3 ret 
0x140a47e86 4889d9 mov rcx, rbx
0x140a47e89 4c8b4320 mov r8, qword ptr [rbx + 0x20]
0x140a47e8d 488b4328 mov rax, qword ptr [rbx + 0x28]
0x140a47e91 4c39c0 cmp rax, r8
0x140a47e94 7339 jae 0x140a47ecf
0x140a47e96 0fb6542429 movzx edx, byte ptr [rsp + 0x29]
0x140a47e9b 4c8b4918 mov r9, qword ptr [rcx + 0x18]
0x140a47e9f 48ffc0 inc rax
0x140a47ea2 49f7d8 neg r8
0x140a47ea5 450fb65401ff movzx r10d, byte ptr [r9 + rax - 1]
0x140a47eab 4983fa3a cmp r10, 0x3a
0x140a47eaf 0f87f4010000 ja 0x140a480a9
0x140a47eb5 4c0fa3d7 bt rdi, r10
0x140a47eb9 7329 jae 0x140a47ee4
0x140a47ebb 48894128 mov qword ptr [rcx + 0x28], rax
0x140a47ebf 4d8d1400 lea r10, [r8 + rax]
0x140a47ec3 49ffc2 inc r10
0x140a47ec6 48ffc0 inc rax
0x140a47ec9 4983fa01 cmp r10, 1
0x140a47ecd 75d6 jne 0x140a47ea5
0x140a47ecf 48c744242803000000 mov qword ptr [rsp + 0x28], 3
0x140a47ed8 488d542428 lea rdx, [rsp + 0x28]
0x140a47edd e83e8ae501 call 0x1428a0920
0x140a47ee2 eb90 jmp 0x140a47e74
0x140a47ee4 4983fa3a cmp r10, 0x3a
0x140a47ee8 0f85bb010000 jne 0x140a480a9
0x140a47eee 48894128 mov qword ptr [rcx + 0x28], rax
0x140a47ef2 89d0 mov eax, edx
0x140a47ef4 488d158d780802 lea rdx, [rip + 0x208788d]
0x140a47efb 48630482 movsxd rax, dword ptr [rdx + rax*4]
0x140a47eff 4801d0 add rax, rdx
0x140a47f02 ffe0 jmp rax
0x140a47f04 e8975a9c00 call 0x14140d9a0
0x140a47f09 4885c0 test rax, rax
0x140a47f0c 0f8562ffffff jne 0x140a47e74
0x140a47f12 b038 mov al, 0x38
0x140a47f14 e9ec000000 jmp 0x140a48005
0x140a47f19 66c706003f mov word ptr [rsi], 0x3f00
0x140a47f1e e958ffffff jmp 0x140a47e7b
0x140a47f23 66c706003c mov word ptr [rsi], 0x3c00
0x140a47f28 e94effffff jmp 0x140a47e7b
0x140a47f2d 66c706003a mov word ptr [rsi], 0x3a00
0x140a47f32 e944ffffff jmp 0x140a47e7b
0x140a47f37 66c706003b mov word ptr [rsi], 0x3b00
0x140a47f3c e93affffff jmp 0x140a47e7b
0x140a47f41 66c7060039 mov word ptr [rsi], 0x3900
0x140a47f46 e930ffffff jmp 0x140a47e7b
0x140a47f4b 66c706003d mov word ptr [rsi], 0x3d00
0x140a47f50 e926ffffff jmp 0x140a47e7b
0x140a47f55 48c74424280a000000 mov qword ptr [rsp + 0x28], 0xa
0x140a47f5e e99cfeffff jmp 0x140a47dff
0x140a47f63 e8385a9c00 call 0x14140d9a0
0x140a47f68 4885c0 test rax, rax
0x140a47f6b 0f8503ffffff jne 0x140a47e74
0x140a47f71 b03c mov al, 0x3c
0x140a47f73 e98d000000 jmp 0x140a48005
0x140a47f78 e8235a9c00 call 0x14140d9a0
0x140a47f7d 4885c0 test rax, rax
0x140a47f80 0f85eefeffff jne 0x140a47e74
0x140a47f86 b03a mov al, 0x3a
0x140a47f88 eb7b jmp 0x140a48005
0x140a47f8a e8115a9c00 call 0x14140d9a0
0x140a47f8f 4885c0 test rax, rax
0x140a47f92 0f85dcfeffff jne 0x140a47e74
0x140a47f98 b03b mov al, 0x3b
0x140a47f9a eb69 jmp 0x140a48005
0x140a47f9c e8ff599c00 call 0x14140d9a0
0x140a47fa1 4885c0 test rax, rax
0x140a47fa4 0f85cafeffff jne 0x140a47e74
0x140a47faa b03f mov al, 0x3f
0x140a47fac eb57 jmp 0x140a48005
0x140a47fae e8ed599c00 call 0x14140d9a0
0x140a47fb3 4885c0 test rax, rax
0x140a47fb6 0f85b8feffff jne 0x140a47e74
0x140a47fbc b039 mov al, 0x39
0x140a47fbe eb45 jmp 0x140a48005
0x140a47fc0 e8db599c00 call 0x14140d9a0
0x140a47fc5 4885c0 test rax, rax
0x140a47fc8 0f85a6feffff jne 0x140a47e74
0x140a47fce b03d mov al, 0x3d
0x140a47fd0 eb33 jmp 0x140a48005
0x140a47fd2 4c8d05476e0802 lea r8, [rip + 0x2086e47]
0x140a47fd9 488d442440 lea rax, [rsp + 0x40]
0x140a47fde 41b901000000 mov r9d, 1
0x140a47fe4 4889ca mov rdx, rcx
0x140a47fe7 4889c1 mov rcx, rax
0x140a47fea e811679c00 call 0x14140e700
0x140a47fef f644244001 test byte ptr [rsp + 0x40], 1
0x140a47ff4 740a je 0x140a48000
0x140a47ff6 488b442448 mov rax, qword ptr [rsp + 0x48]
0x140a47ffb e974feffff jmp 0x140a47e74
0x140a48000 0fb6442441 movzx eax, byte ptr [rsp + 0x41]
0x140a48005 4889d9 mov rcx, rbx
0x140a48008 fe4330 inc byte ptr [rbx + 0x30]
0x140a4800b 4c8b4320 mov r8, qword ptr [rbx + 0x20]
0x140a4800f 488b5328 mov rdx, qword ptr [rbx + 0x28]
0x140a48013 4c39c2 cmp rdx, r8
0x140a48016 7330 jae 0x140a48048
0x140a48018 4c8b4918 mov r9, qword ptr [rcx + 0x18]
0x140a4801c 48ffc2 inc rdx
0x140a4801f 49f7d8 neg r8
0x140a48022 450fb65411ff movzx r10d, byte ptr [r9 + rdx - 1]
0x140a48028 4983fa20 cmp r10, 0x20
0x140a4802c 7735 ja 0x140a48063
0x140a4802e 4c0fa3d7 bt rdi, r10
0x140a48032 732f jae 0x140a48063
0x140a48034 48895128 mov qword ptr [rcx + 0x28], rdx
0x140a48038 4d8d1410 lea r10, [r8 + rdx]
0x140a4803c 49ffc2 inc r10
0x140a4803f 48ffc2 inc rdx
0x140a48042 4983fa01 cmp r10, 1
0x140a48046 75da jne 0x140a48022
0x140a48048 48c744242803000000 mov qword ptr [rsp + 0x28], 3
0x140a48051 488d542428 lea rdx, [rsp + 0x28]
0x140a48056 e8458fe501 call 0x1428a0fa0
0x140a4805b 48894608 mov qword ptr [rsi + 8], rax
0x140a4805f b001 mov al, 1
0x140a48061 eb0f jmp 0x140a48072
0x140a48063 4183fa7d cmp r10d, 0x7d
0x140a48067 754e jne 0x140a480b7
0x140a48069 48895128 mov qword ptr [rcx + 0x28], rdx
0x140a4806d 884601 mov byte ptr [rsi + 1], al
0x140a48070 31c0 xor eax, eax
0x140a48072 8806 mov byte ptr [rsi], al
0x140a48074 e902feffff jmp 0x140a47e7b
0x140a48079 c64424280d mov byte ptr [rsp + 0x28], 0xd
0x140a4807e 488d155b3e2102 lea rdx, [rip + 0x2213e5b]
0x140a48085 4c8d05643e2102 lea r8, [rip + 0x2213e64]
0x140a4808c 488d4c2428 lea rcx, [rsp + 0x28]
0x140a48091 e86a19e801 call 0x1428c9a00
0x140a48096 e9d9fdffff jmp 0x140a47e74
0x140a4809b 48c744242815000000 mov qword ptr [rsp + 0x28], 0x15
0x140a480a4 e956fdffff jmp 0x140a47dff
0x140a480a9 48c744242806000000 mov qword ptr [rsp + 0x28], 6
0x140a480b2 e921feffff jmp 0x140a47ed8
0x140a480b7 48c74424280a000000 mov qword ptr [rsp + 0x28], 0xa
0x140a480c0 eb8f jmp 0x140a48051
