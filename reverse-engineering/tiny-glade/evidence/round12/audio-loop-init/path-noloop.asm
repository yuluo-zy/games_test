1409d08d0: 55 push rbp
1409d08d1: 56 push rsi
1409d08d2: 57 push rdi
1409d08d3: 4881ec50020000 sub rsp, 0x250
1409d08da: 488dac2480000000 lea rbp, [rsp + 0x80]
1409d08e2: 48c785c8010000feffffff mov qword ptr [rbp + 0x1c8], 0xfffffffffffffffe
1409d08ed: 4889d6 mov rsi, rdx
1409d08f0: 488b39 mov rdi, qword ptr [rcx]
1409d08f3: 48c785a001000003000000 mov qword ptr [rbp + 0x1a0], 3
1409d08fe: c785a801000032000000 mov dword ptr [rbp + 0x1a8], 0x32
1409d0908: 488d05eb0c0f02 lea rax, [rip + 0x20f0ceb]
1409d090f: 48898590010000 mov qword ptr [rbp + 0x190], rax
1409d0916: 48c785980100001a000000 mov qword ptr [rbp + 0x198], 0x1a
1409d0921: 488d4d70 lea rcx, [rbp + 0x70]
1409d0925: 4c8d8590010000 lea r8, [rbp + 0x190]
1409d092c: 4889fa mov rdx, rdi
1409d092f: e8ec880800 call 0x140a59220
1409d0934: 488b4d70 mov rcx, qword ptr [rbp + 0x70]
1409d0938: 488b4578 mov rax, qword ptr [rbp + 0x78]
1409d093c: 4889ca mov rdx, rcx
1409d093f: 48f7da neg rdx
1409d0942: 0f809c030000 jo 0x1409d0ce4
1409d0948: 8b9590000000 mov edx, dword ptr [rbp + 0x90]
1409d094e: 4c8b8598000000 mov r8, qword ptr [rbp + 0x98]
1409d0955: 448b8da0000000 mov r9d, dword ptr [rbp + 0xa0]
1409d095c: 440fb795a4000000 movzx r10d, word ptr [rbp + 0xa4]
1409d0964: 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
1409d096b: 48894d38 mov qword ptr [rbp + 0x38], rcx
1409d096f: 48894540 mov qword ptr [rbp + 0x40], rax
1409d0973: 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
1409d0977: 895558 mov dword ptr [rbp + 0x58], edx
1409d097a: 4c894560 mov qword ptr [rbp + 0x60], r8
1409d097e: 44894d68 mov dword ptr [rbp + 0x68], r9d
1409d0982: 664489556c mov word ptr [rbp + 0x6c], r10w
1409d0987: c6456e01 mov byte ptr [rbp + 0x6e], 1
1409d098b: 48c785a001000003000000 mov qword ptr [rbp + 0x1a0], 3
1409d0996: c785a801000032000000 mov dword ptr [rbp + 0x1a8], 0x32
1409d09a0: 488d05b90c0f02 lea rax, [rip + 0x20f0cb9]
1409d09a7: 48898590010000 mov qword ptr [rbp + 0x190], rax
1409d09ae: 48c7859801000014000000 mov qword ptr [rbp + 0x198], 0x14
1409d09b9: 488d4d70 lea rcx, [rbp + 0x70]
1409d09bd: 4c8d8590010000 lea r8, [rbp + 0x190]
1409d09c4: 4889fa mov rdx, rdi
1409d09c7: e854880800 call 0x140a59220
1409d09cc: 488b4d70 mov rcx, qword ptr [rbp + 0x70]
1409d09d0: 488b4578 mov rax, qword ptr [rbp + 0x78]
1409d09d4: 4889ca mov rdx, rcx
1409d09d7: 48f7da neg rdx
1409d09da: 0f8035030000 jo 0x1409d0d15
1409d09e0: 8b9590000000 mov edx, dword ptr [rbp + 0x90]
1409d09e6: 4c8b8598000000 mov r8, qword ptr [rbp + 0x98]
1409d09ed: 448b8da0000000 mov r9d, dword ptr [rbp + 0xa0]
1409d09f4: 440fb795a4000000 movzx r10d, word ptr [rbp + 0xa4]
1409d09fc: 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
1409d0a03: 48894d00 mov qword ptr [rbp], rcx
1409d0a07: 48894508 mov qword ptr [rbp + 8], rax
1409d0a0b: 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
1409d0a0f: 895520 mov dword ptr [rbp + 0x20], edx
1409d0a12: 4c894528 mov qword ptr [rbp + 0x28], r8
1409d0a16: 44894d30 mov dword ptr [rbp + 0x30], r9d
1409d0a1a: 6644895534 mov word ptr [rbp + 0x34], r10w
1409d0a1f: c6453601 mov byte ptr [rbp + 0x36], 1
1409d0a23: 48c785a001000003000000 mov qword ptr [rbp + 0x1a0], 3
1409d0a2e: c785a801000032000000 mov dword ptr [rbp + 0x1a8], 0x32
1409d0a38: 488d05510c0f02 lea rax, [rip + 0x20f0c51]
1409d0a3f: 48898590010000 mov qword ptr [rbp + 0x190], rax
1409d0a46: 48c7859801000015000000 mov qword ptr [rbp + 0x198], 0x15
1409d0a51: 488d4d70 lea rcx, [rbp + 0x70]
1409d0a55: 4c8d8590010000 lea r8, [rbp + 0x190]
1409d0a5c: 4889fa mov rdx, rdi
1409d0a5f: e8bc870800 call 0x140a59220
1409d0a64: 488b4d70 mov rcx, qword ptr [rbp + 0x70]
1409d0a68: 488b4578 mov rax, qword ptr [rbp + 0x78]
1409d0a6c: 4889ca mov rdx, rcx
1409d0a6f: 48f7da neg rdx
1409d0a72: 0f80ce020000 jo 0x1409d0d46
1409d0a78: 8b9590000000 mov edx, dword ptr [rbp + 0x90]
1409d0a7e: 4c8b8598000000 mov r8, qword ptr [rbp + 0x98]
1409d0a85: 448b8da0000000 mov r9d, dword ptr [rbp + 0xa0]
1409d0a8c: 440fb795a4000000 movzx r10d, word ptr [rbp + 0xa4]
1409d0a94: 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
1409d0a9b: 48894dc8 mov qword ptr [rbp - 0x38], rcx
1409d0a9f: 488945d0 mov qword ptr [rbp - 0x30], rax
1409d0aa3: 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
1409d0aa7: 8955e8 mov dword ptr [rbp - 0x18], edx
1409d0aaa: 4c8945f0 mov qword ptr [rbp - 0x10], r8
1409d0aae: 44894df8 mov dword ptr [rbp - 8], r9d
1409d0ab2: 66448955fc mov word ptr [rbp - 4], r10w
1409d0ab7: c645fe01 mov byte ptr [rbp - 2], 1
1409d0abb: 48c785a001000003000000 mov qword ptr [rbp + 0x1a0], 3
1409d0ac6: c785a801000032000000 mov dword ptr [rbp + 0x1a8], 0x32
1409d0ad0: 488d05e90b0f02 lea rax, [rip + 0x20f0be9]
1409d0ad7: 48898590010000 mov qword ptr [rbp + 0x190], rax
1409d0ade: 48c785980100001e000000 mov qword ptr [rbp + 0x198], 0x1e
1409d0ae9: 488d4d70 lea rcx, [rbp + 0x70]
1409d0aed: 4c8d8590010000 lea r8, [rbp + 0x190]
1409d0af4: 4889fa mov rdx, rdi
1409d0af7: e824870800 call 0x140a59220
1409d0afc: 488b4d70 mov rcx, qword ptr [rbp + 0x70]
1409d0b00: 488b4578 mov rax, qword ptr [rbp + 0x78]
1409d0b04: 4889ca mov rdx, rcx
1409d0b07: 48f7da neg rdx
1409d0b0a: 0f8064020000 jo 0x1409d0d74
1409d0b10: 8b9590000000 mov edx, dword ptr [rbp + 0x90]
1409d0b16: 4c8b8598000000 mov r8, qword ptr [rbp + 0x98]
1409d0b1d: 448b8da0000000 mov r9d, dword ptr [rbp + 0xa0]
1409d0b24: 440fb795a4000000 movzx r10d, word ptr [rbp + 0xa4]
1409d0b2c: 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
1409d0b33: 48898d90010000 mov qword ptr [rbp + 0x190], rcx
1409d0b3a: 48898598010000 mov qword ptr [rbp + 0x198], rax
1409d0b41: 0f1185a0010000 movups xmmword ptr [rbp + 0x1a0], xmm0
1409d0b48: 8995b0010000 mov dword ptr [rbp + 0x1b0], edx
1409d0b4e: 4c8985b8010000 mov qword ptr [rbp + 0x1b8], r8
1409d0b55: 44898dc0010000 mov dword ptr [rbp + 0x1c0], r9d
1409d0b5c: 66448995c4010000 mov word ptr [rbp + 0x1c4], r10w
1409d0b64: c685c601000001 mov byte ptr [rbp + 0x1c6], 1
1409d0b6b: 48c745b803000000 mov qword ptr [rbp - 0x48], 3
1409d0b73: c745c032000000 mov dword ptr [rbp - 0x40], 0x32
1409d0b7a: 488d05770b0f02 lea rax, [rip + 0x20f0b77]
1409d0b81: 488945a8 mov qword ptr [rbp - 0x58], rax
1409d0b85: 48c745b01f000000 mov qword ptr [rbp - 0x50], 0x1f
1409d0b8d: 488d4d70 lea rcx, [rbp + 0x70]
1409d0b91: 4c8d45a8 lea r8, [rbp - 0x58]
1409d0b95: 4889fa mov rdx, rdi
1409d0b98: e883860800 call 0x140a59220
1409d0b9d: 488b4d70 mov rcx, qword ptr [rbp + 0x70]
1409d0ba1: 488b4578 mov rax, qword ptr [rbp + 0x78]
1409d0ba5: 4889ca mov rdx, rcx
1409d0ba8: 48f7da neg rdx
1409d0bab: 0f80f1010000 jo 0x1409d0da2
1409d0bb1: 8b9590000000 mov edx, dword ptr [rbp + 0x90]
1409d0bb7: 4c8b8598000000 mov r8, qword ptr [rbp + 0x98]
1409d0bbe: 448b8da0000000 mov r9d, dword ptr [rbp + 0xa0]
1409d0bc5: 440fb795a4000000 movzx r10d, word ptr [rbp + 0xa4]
1409d0bcd: 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
1409d0bd4: 48898d50010000 mov qword ptr [rbp + 0x150], rcx
1409d0bdb: 48898558010000 mov qword ptr [rbp + 0x158], rax
1409d0be2: 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
1409d0be9: 899570010000 mov dword ptr [rbp + 0x170], edx
1409d0bef: 4c898578010000 mov qword ptr [rbp + 0x178], r8
1409d0bf6: 44898d80010000 mov dword ptr [rbp + 0x180], r9d
1409d0bfd: 6644899584010000 mov word ptr [rbp + 0x184], r10w
1409d0c05: c6858601000001 mov byte ptr [rbp + 0x186], 1
1409d0c0c: 0f104538 movups xmm0, xmmword ptr [rbp + 0x38]
1409d0c10: 0f104d48 movups xmm1, xmmword ptr [rbp + 0x48]
1409d0c14: 0f105558 movups xmm2, xmmword ptr [rbp + 0x58]
1409d0c18: 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
1409d0c1c: 0f298d80000000 movaps xmmword ptr [rbp + 0x80], xmm1
1409d0c23: 0f299590000000 movaps xmmword ptr [rbp + 0x90], xmm2
1409d0c2a: 488b4568 mov rax, qword ptr [rbp + 0x68]
1409d0c2e: 488985a0000000 mov qword ptr [rbp + 0xa0], rax
1409d0c35: 488b4530 mov rax, qword ptr [rbp + 0x30]
1409d0c39: 488985d8000000 mov qword ptr [rbp + 0xd8], rax
1409d0c40: 0f104500 movups xmm0, xmmword ptr [rbp]
1409d0c44: 0f104d10 movups xmm1, xmmword ptr [rbp + 0x10]
1409d0c48: 0f105520 movups xmm2, xmmword ptr [rbp + 0x20]
1409d0c4c: 0f1195c8000000 movups xmmword ptr [rbp + 0xc8], xmm2
1409d0c53: 0f118db8000000 movups xmmword ptr [rbp + 0xb8], xmm1
1409d0c5a: 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
1409d0c61: 488b85c0010000 mov rax, qword ptr [rbp + 0x1c0]
1409d0c68: 48898510010000 mov qword ptr [rbp + 0x110], rax
1409d0c6f: 0f108590010000 movups xmm0, xmmword ptr [rbp + 0x190]
1409d0c76: 0f108da0010000 movups xmm1, xmmword ptr [rbp + 0x1a0]
1409d0c7d: 0f1095b0010000 movups xmm2, xmmword ptr [rbp + 0x1b0]
1409d0c84: 0f299500010000 movaps xmmword ptr [rbp + 0x100], xmm2
1409d0c8b: 0f298df0000000 movaps xmmword ptr [rbp + 0xf0], xmm1
1409d0c92: 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
1409d0c99: 488b45f8 mov rax, qword ptr [rbp - 8]
1409d0c9d: 48898548010000 mov qword ptr [rbp + 0x148], rax
1409d0ca4: 0f1045c8 movups xmm0, xmmword ptr [rbp - 0x38]
1409d0ca8: 0f104dd8 movups xmm1, xmmword ptr [rbp - 0x28]
1409d0cac: 0f1055e8 movups xmm2, xmmword ptr [rbp - 0x18]
1409d0cb0: 0f119538010000 movups xmmword ptr [rbp + 0x138], xmm2
1409d0cb7: 0f118d28010000 movups xmmword ptr [rbp + 0x128], xmm1
1409d0cbe: 0f118518010000 movups xmmword ptr [rbp + 0x118], xmm0
1409d0cc5: 4c8d05640a0f02 lea r8, [rip + 0x20f0a64]
1409d0ccc: 488d5570 lea rdx, [rbp + 0x70]
1409d0cd0: 4889f1 mov rcx, rsi
1409d0cd3: e868049100 call 0x1412e1140
1409d0cd8: 90 nop 
1409d0cd9: 4881c450020000 add rsp, 0x250
1409d0ce0: 5f pop rdi
1409d0ce1: 5e pop rsi
1409d0ce2: 5d pop rbp
1409d0ce3: c3 ret 
1409d0ce4: 48894570 mov qword ptr [rbp + 0x70], rax
1409d0ce8: 488d0559090f02 lea rax, [rip + 0x20f0959]
1409d0cef: 4889442420 mov qword ptr [rsp + 0x20], rax
1409d0cf4: 488d0da5060f02 lea rcx, [rip + 0x20f06a5]
1409d0cfb: 4c8d0d7e060f02 lea r9, [rip + 0x20f067e]
1409d0d02: 4c8d4570 lea r8, [rbp + 0x70]
1409d0d06: ba2b000000 mov edx, 0x2b
1409d0d0b: e8508af001 call 0x1428d9760
1409d0d10: e9b9000000 jmp 0x1409d0dce
1409d0d15: 48894570 mov qword ptr [rbp + 0x70], rax
1409d0d19: 488d0558090f02 lea rax, [rip + 0x20f0958]
1409d0d20: 4889442420 mov qword ptr [rsp + 0x20], rax
1409d0d25: 488d0d74060f02 lea rcx, [rip + 0x20f0674]
1409d0d2c: 4c8d0d4d060f02 lea r9, [rip + 0x20f064d]
1409d0d33: 4c8d4570 lea r8, [rbp + 0x70]
1409d0d37: ba2b000000 mov edx, 0x2b
1409d0d3c: e81f8af001 call 0x1428d9760
1409d0d41: e988000000 jmp 0x1409d0dce
1409d0d46: 48894570 mov qword ptr [rbp + 0x70], rax
1409d0d4a: 488d0557090f02 lea rax, [rip + 0x20f0957]
1409d0d51: 4889442420 mov qword ptr [rsp + 0x20], rax
1409d0d56: 488d0d43060f02 lea rcx, [rip + 0x20f0643]
1409d0d5d: 4c8d0d1c060f02 lea r9, [rip + 0x20f061c]
1409d0d64: 4c8d4570 lea r8, [rbp + 0x70]
1409d0d68: ba2b000000 mov edx, 0x2b
1409d0d6d: e8ee89f001 call 0x1428d9760
1409d0d72: eb5a jmp 0x1409d0dce
1409d0d74: 48894570 mov qword ptr [rbp + 0x70], rax
1409d0d78: 488d0561090f02 lea rax, [rip + 0x20f0961]
1409d0d7f: 4889442420 mov qword ptr [rsp + 0x20], rax
1409d0d84: 488d0d15060f02 lea rcx, [rip + 0x20f0615]
1409d0d8b: 4c8d0dee050f02 lea r9, [rip + 0x20f05ee]
1409d0d92: 4c8d4570 lea r8, [rbp + 0x70]
1409d0d96: ba2b000000 mov edx, 0x2b
1409d0d9b: e8c089f001 call 0x1428d9760
1409d0da0: eb2c jmp 0x1409d0dce
1409d0da2: 48894570 mov qword ptr [rbp + 0x70], rax
1409d0da6: 488d056b090f02 lea rax, [rip + 0x20f096b]
1409d0dad: 4889442420 mov qword ptr [rsp + 0x20], rax
1409d0db2: 488d0de7050f02 lea rcx, [rip + 0x20f05e7]
1409d0db9: 4c8d0dc0050f02 lea r9, [rip + 0x20f05c0]
1409d0dc0: 4c8d4570 lea r8, [rbp + 0x70]
1409d0dc4: ba2b000000 mov edx, 0x2b
1409d0dc9: e89289f001 call 0x1428d9760
1409d0dce: 0f0b ud2 
1409d0dd0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d0dd5: 55 push rbp
1409d0dd6: 56 push rsi
1409d0dd7: 57 push rdi
1409d0dd8: 4883ec30 sub rsp, 0x30
1409d0ddc: 488daa80000000 lea rbp, [rdx + 0x80]
1409d0de3: 488d4d38 lea rcx, [rbp + 0x38]
1409d0de7: e85489ecff call 0x140899740
1409d0dec: 90 nop 
1409d0ded: 4883c430 add rsp, 0x30
1409d0df1: 5f pop rdi
1409d0df2: 5e pop rsi
1409d0df3: 5d pop rbp
1409d0df4: c3 ret 
1409d0df5: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409d0e00: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d0e05: 55 push rbp
1409d0e06: 56 push rsi
1409d0e07: 57 push rdi
1409d0e08: 4883ec30 sub rsp, 0x30
1409d0e0c: 488daa80000000 lea rbp, [rdx + 0x80]
1409d0e13: 4889e9 mov rcx, rbp
1409d0e16: e82589ecff call 0x140899740
1409d0e1b: 90 nop 
1409d0e1c: 4883c430 add rsp, 0x30
1409d0e20: 5f pop rdi
1409d0e21: 5e pop rsi
1409d0e22: 5d pop rbp
1409d0e23: c3 ret 
1409d0e24: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409d0e30: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d0e35: 55 push rbp
1409d0e36: 56 push rsi
1409d0e37: 57 push rdi
1409d0e38: 4883ec30 sub rsp, 0x30
1409d0e3c: 488daa80000000 lea rbp, [rdx + 0x80]
1409d0e43: 488d4dc8 lea rcx, [rbp - 0x38]
1409d0e47: e8f488ecff call 0x140899740
1409d0e4c: 90 nop 
1409d0e4d: 4883c430 add rsp, 0x30
1409d0e51: 5f pop rdi
1409d0e52: 5e pop rsi
1409d0e53: 5d pop rbp
1409d0e54: c3 ret 
1409d0e55: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409d0e60: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d0e65: 55 push rbp
1409d0e66: 56 push rsi
1409d0e67: 57 push rdi
1409d0e68: 4883ec30 sub rsp, 0x30
1409d0e6c: 488daa80000000 lea rbp, [rdx + 0x80]
1409d0e73: 488d8d90010000 lea rcx, [rbp + 0x190]
1409d0e7a: e8c188ecff call 0x140899740
1409d0e7f: 90 nop 
1409d0e80: 4883c430 add rsp, 0x30
1409d0e84: 5f pop rdi
1409d0e85: 5e pop rsi
1409d0e86: 5d pop rbp
1409d0e87: c3 ret 
1409d0e88: 0f1f840000000000 nop dword ptr [rax + rax]
1409d0e90: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d0e95: 55 push rbp
1409d0e96: 56 push rsi
1409d0e97: 57 push rdi
1409d0e98: 4883ec30 sub rsp, 0x30
1409d0e9c: 488daa80000000 lea rbp, [rdx + 0x80]
1409d0ea3: 488d4d70 lea rcx, [rbp + 0x70]
1409d0ea7: e884bd3700 call 0x140d4cc30
1409d0eac: 90 nop 
1409d0ead: 4883c430 add rsp, 0x30
1409d0eb1: 5f pop rdi
1409d0eb2: 5e pop rsi
1409d0eb3: 5d pop rbp
1409d0eb4: c3 ret 
1409d0eb5: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409d0ec0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d0ec5: 55 push rbp
1409d0ec6: 56 push rsi
1409d0ec7: 57 push rdi
1409d0ec8: 4883ec30 sub rsp, 0x30
1409d0ecc: 488daa80000000 lea rbp, [rdx + 0x80]
1409d0ed3: 488d4d70 lea rcx, [rbp + 0x70]
1409d0ed7: e854bd3700 call 0x140d4cc30
1409d0edc: 90 nop 
1409d0edd: 4883c430 add rsp, 0x30
1409d0ee1: 5f pop rdi
1409d0ee2: 5e pop rsi
1409d0ee3: 5d pop rbp
1409d0ee4: c3 ret 
1409d0ee5: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409d0ef0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d0ef5: 55 push rbp
1409d0ef6: 56 push rsi
1409d0ef7: 57 push rdi
1409d0ef8: 4883ec30 sub rsp, 0x30
1409d0efc: 488daa80000000 lea rbp, [rdx + 0x80]
1409d0f03: 488d4d70 lea rcx, [rbp + 0x70]
1409d0f07: e824bd3700 call 0x140d4cc30
1409d0f0c: 90 nop 
1409d0f0d: 4883c430 add rsp, 0x30
1409d0f11: 5f pop rdi
1409d0f12: 5e pop rsi
1409d0f13: 5d pop rbp
1409d0f14: c3 ret 
1409d0f15: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409d0f20: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d0f25: 55 push rbp
1409d0f26: 56 push rsi
1409d0f27: 57 push rdi
1409d0f28: 4883ec30 sub rsp, 0x30
1409d0f2c: 488daa80000000 lea rbp, [rdx + 0x80]
1409d0f33: 488d4d70 lea rcx, [rbp + 0x70]
1409d0f37: e8f4bc3700 call 0x140d4cc30
1409d0f3c: 90 nop 
1409d0f3d: 4883c430 add rsp, 0x30
1409d0f41: 5f pop rdi
1409d0f42: 5e pop rsi
1409d0f43: 5d pop rbp
1409d0f44: c3 ret 
1409d0f45: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409d0f50: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409d0f55: 55 push rbp
1409d0f56: 56 push rsi
1409d0f57: 57 push rdi
1409d0f58: 4883ec30 sub rsp, 0x30
1409d0f5c: 488daa80000000 lea rbp, [rdx + 0x80]
1409d0f63: 488d4d70 lea rcx, [rbp + 0x70]
1409d0f67: e8c4bc3700 call 0x140d4cc30
1409d0f6c: 90 nop 
1409d0f6d: 4883c430 add rsp, 0x30
1409d0f71: 5f pop rdi
1409d0f72: 5e pop rsi
1409d0f73: 5d pop rbp
1409d0f74: c3 ret 
