1416f8470: push rbp
1416f8471: push rsi
1416f8472: push rdi
1416f8473: push rbx
1416f8474: sub rsp, 0x58
1416f8478: lea rbp, [rsp + 0x50]
1416f847d: mov qword ptr [rbp], 0xfffffffffffffffe
1416f8485: mov rdi, rdx
1416f8488: mov rsi, rcx
1416f848b: movzx eax, byte ptr [rip + 0x1f3c777] ; 0x143634c09 b''
1416f8492: mov ecx, 0x80
1416f8497: mov edx, 1
1416f849c: call 0x140613c10
1416f84a1: test rax, rax
1416f84a4: je 0x1416f8627
1416f84aa: mov qword ptr [rbp - 0x28], 0x80
1416f84b2: mov qword ptr [rbp - 0x20], rax
1416f84b6: lea rcx, [rbp - 0x28]
1416f84ba: mov qword ptr [rbp - 0x30], rcx
1416f84be: mov byte ptr [rax], 0x7b
1416f84c1: mov qword ptr [rbp - 0x18], 1
1416f84c9: lea rax, [rbp - 0x30]
1416f84cd: mov qword ptr [rbp - 0x10], rax
1416f84d1: mov byte ptr [rbp - 8], 1
1416f84d5: lea rdx, [rip + 0x1599d02] ; 0x142c921de b'yawpitchcamera_rig_arm_lengthpitch_offsetyaw_offsetyaw_cachepitc'
1416f84dc: lea rcx, [rbp - 0x10]
1416f84e0: mov r8d, 3
1416f84e6: mov r9, rdi
1416f84e9: call 0x141586090
1416f84ee: mov rbx, rax
1416f84f1: test rax, rax
1416f84f4: jne 0x1416f85ab
1416f84fa: lea r9, [rdi + 4]
1416f84fe: lea rdx, [rip + 0x1599cdc] ; 0x142c921e1 b'pitchcamera_rig_arm_lengthpitch_offsetyaw_offsetyaw_cachepitch_c'
1416f8505: lea rcx, [rbp - 0x10]
1416f8509: mov r8d, 5
1416f850f: call 0x141586090
1416f8514: mov rbx, rax
1416f8517: test rax, rax
1416f851a: jne 0x1416f85ab
1416f8520: lea r9, [rdi + 8]
1416f8524: lea rdx, [rip + 0x1599a21] ; 0x142c91f4c b'positiongoalmove_state\x00\x00\x00\x00\x00\x00L\x1f\xc9B\x01\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00T\x1f\xc9B\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00X\x1f\xc9B'
1416f852b: lea rcx, [rbp - 0x10]
1416f852f: mov r8d, 8
1416f8535: call 0x141583cf0
1416f853a: mov rbx, rax
1416f853d: test rax, rax
1416f8540: jne 0x1416f85ab
1416f8542: lea r9, [rdi + 0x14]
1416f8546: lea rdx, [rip + 0x1599c99] ; 0x142c921e6 b'camera_rig_arm_lengthpitch_offsetyaw_offsetyaw_cachepitch_cachep'
1416f854d: lea rcx, [rbp - 0x10]
1416f8551: mov r8d, 0x15
1416f8557: call 0x141586090
1416f855c: mov rbx, rax
1416f855f: test rax, rax
1416f8562: jne 0x1416f85ab
1416f8564: lea r9, [rdi + 0x1c]
1416f8568: lea rdx, [rip + 0x1599c8c] ; 0x142c921fb b'pitch_offsetyaw_offsetyaw_cachepitch_cacheposition_cache\x00\x00\x00\x00\x00\xde!\xc9'
1416f856f: lea rcx, [rbp - 0x10]
1416f8573: mov r8d, 0xc
1416f8579: call 0x141586090
1416f857e: mov rbx, rax
1416f8581: test rax, rax
1416f8584: jne 0x1416f85ab
1416f8586: add rdi, 0x20
1416f858a: lea rdx, [rip + 0x1599c76] ; 0x142c92207 b'yaw_offsetyaw_cachepitch_cacheposition_cache\x00\x00\x00\x00\x00\xde!\xc9B\x01\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00'
1416f8591: lea rcx, [rbp - 0x10]
1416f8595: mov r8d, 0xa
1416f859b: mov r9, rdi
1416f859e: call 0x141586090
1416f85a3: mov rbx, rax
1416f85a6: test rax, rax
1416f85a9: je 0x1416f85e0
1416f85ab: mov rdx, qword ptr [rbp - 0x28]
1416f85af: test rdx, rdx
1416f85b2: je 0x1416f85c3
1416f85b4: mov rcx, qword ptr [rbp - 0x20]
1416f85b8: mov r8d, 1
1416f85be: call 0x140613c20
1416f85c3: mov qword ptr [rsi + 8], rbx
1416f85c7: movabs rax, 0x8000000000000000
1416f85d1: mov qword ptr [rsi], rax
1416f85d4: mov rax, rsi
1416f85d7: add rsp, 0x58
1416f85db: pop rbx
1416f85dc: pop rdi
1416f85dd: pop rsi
1416f85de: pop rbp
1416f85df: ret 
1416f85e0: cmp byte ptr [rbp - 8], 0
1416f85e4: je 0x1416f8606
1416f85e6: mov rax, qword ptr [rbp - 0x10]
1416f85ea: mov rcx, qword ptr [rax]
1416f85ed: lea rdx, [rip + 0x15993f4] ; 0x142c919e8 b'}map with a single key\x00\x00\xe9\x19\xc9B\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00'
1416f85f4: lea r9, [rip + 0x15994a5] ; 0x142c91aa0 b'P\x1a\xc9B\x01\x00\x00\x00K\x00\x00\x00\x00\x00\x00\x00\xee\x01\x00\x00\x0e\x00\x00\x000001020304050607080910111213141516171819'
1416f85fb: mov r8d, 1
1416f8601: call 0x1412f4520
1416f8606: mov rax, qword ptr [rbp - 0x28]
1416f860a: mov rbx, qword ptr [rbp - 0x20]
1416f860e: mov rcx, rax
1416f8611: neg rcx
1416f8614: jo 0x1416f85c3
1416f8616: mov rcx, qword ptr [rbp - 0x18]
1416f861a: mov qword ptr [rsi], rax
1416f861d: mov qword ptr [rsi + 8], rbx
1416f8621: mov qword ptr [rsi + 0x10], rcx
1416f8625: jmp 0x1416f85d4
1416f8627: lea r8, [rip + 0x15993a2] ; 0x142c919d0 b'p\x19\xc9B\x01\x00\x00\x00Z\x00\x00\x00\x00\x00\x00\x00\x8d\x08\x00\x00\x16\x00\x00\x00}map with a single key\x00\x00\xe9\x19\xc9B\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00'
1416f862e: mov ecx, 1
1416f8633: mov edx, 0x80
1416f8638: call 0x1428d8fe3
