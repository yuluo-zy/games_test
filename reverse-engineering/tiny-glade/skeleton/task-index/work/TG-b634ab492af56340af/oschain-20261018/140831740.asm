140831740: push rbp
140831741: push rsi
140831742: push rdi
140831743: sub rsp, 0xd0
14083174a: lea rbp, [rsp + 0x80]
140831752: mov qword ptr [rbp + 0x48], 0xfffffffffffffffe
14083175a: mov rax, r8
14083175d: mov rdi, rdx
140831760: mov rsi, rcx
140831763: mov qword ptr [rsp + 0x20], 0x28
14083176c: lea r8, [rip + 0x2260ac5]
140831773: lea rcx, [rbp - 0x10]
140831777: mov r9d, 0xb
14083177d: mov rdx, rax
140831780: call 0x1413132c0
140831785: movzx eax, byte ptr [rbp - 0x10]
140831789: cmp al, 0x32
14083178b: jne 0x14083200d
140831791: mov rax, qword ptr [rbp + 8]
140831795: movups xmm0, xmmword ptr [rbp - 8]
140831799: movaps xmmword ptr [rbp - 0x30], xmm0
14083179d: mov qword ptr [rbp - 0x20], rax
1408317a1: mov byte ptr [rbp + 0x47], 1
1408317a5: mov qword ptr [rsp + 0x20], rdi
1408317aa: lea r8, [rip + 0x2260a92]
1408317b1: lea rcx, [rbp - 0x10]
1408317b5: lea rdx, [rbp - 0x30]
1408317b9: mov r9d, 0xc
1408317bf: call 0x1413115f0
1408317c4: movzx eax, byte ptr [rbp - 0x10]
1408317c8: cmp al, 0x32
1408317ca: jne 0x140832049
1408317d0: lea rax, [rdi + 0xc]
1408317d4: mov byte ptr [rbp + 0x47], 1
1408317d8: mov qword ptr [rsp + 0x20], rax
1408317dd: lea r8, [rip + 0x2260a6b]
1408317e4: lea rcx, [rbp - 0x10]
1408317e8: lea rdx, [rbp - 0x30]
1408317ec: mov r9d, 9
1408317f2: call 0x1413115f0
1408317f7: movzx eax, byte ptr [rbp - 0x10]
1408317fb: cmp al, 0x32
1408317fd: jne 0x140832049
140831803: lea rax, [rdi + 0x18]
140831807: mov byte ptr [rbp + 0x47], 1
14083180b: mov qword ptr [rsp + 0x20], rax
140831810: lea r8, [rip + 0x2260a41]
140831817: lea rcx, [rbp - 0x10]
14083181b: lea rdx, [rbp - 0x30]
14083181f: mov r9d, 9
140831825: call 0x1413115f0
14083182a: movzx eax, byte ptr [rbp - 0x10]
14083182e: cmp al, 0x32
140831830: jne 0x140832049
140831836: lea rax, [rdi + 0x24]
14083183a: mov byte ptr [rbp + 0x47], 1
14083183e: mov qword ptr [rsp + 0x20], rax
140831843: lea r8, [rip + 0x2260a17]
14083184a: lea rcx, [rbp - 0x10]
14083184e: lea rdx, [rbp - 0x30]
140831852: mov r9d, 0xa
140831858: call 0x1413115f0
14083185d: movzx eax, byte ptr [rbp - 0x10]
140831861: cmp al, 0x32
140831863: jne 0x140832049
140831869: lea rax, [rdi + 0x30]
14083186d: mov byte ptr [rbp + 0x47], 1
140831871: mov qword ptr [rsp + 0x20], rax
140831876: lea r8, [rip + 0x22609ee]
14083187d: lea rcx, [rbp - 0x10]
140831881: lea rdx, [rbp - 0x30]
140831885: mov r9d, 7
14083188b: call 0x1413115f0
140831890: movzx eax, byte ptr [rbp - 0x10]
140831894: cmp al, 0x32
140831896: jne 0x140832049
14083189c: lea rax, [rdi + 0x3c]
1408318a0: mov byte ptr [rbp + 0x47], 1
1408318a4: mov qword ptr [rsp + 0x20], rax
1408318a9: lea r8, [rip + 0x22609c2]
1408318b0: lea rcx, [rbp - 0x10]
1408318b4: lea rdx, [rbp - 0x30]
1408318b8: mov r9d, 9
1408318be: call 0x1413115f0
1408318c3: movzx eax, byte ptr [rbp - 0x10]
1408318c7: cmp al, 0x32
1408318c9: jne 0x140832049
1408318cf: lea rax, [rdi + 0x48]
1408318d3: mov byte ptr [rbp + 0x47], 1
1408318d7: mov qword ptr [rsp + 0x20], rax
1408318dc: lea r8, [rip + 0x2260998]
1408318e3: lea rcx, [rbp - 0x10]
1408318e7: lea rdx, [rbp - 0x30]
1408318eb: mov r9d, 0xb
1408318f1: call 0x1413115f0
1408318f6: movzx eax, byte ptr [rbp - 0x10]
1408318fa: cmp al, 0x32
1408318fc: jne 0x140832049
140831902: lea rax, [rdi + 0x54]
140831906: mov byte ptr [rbp + 0x47], 1
14083190a: mov qword ptr [rsp + 0x20], rax
14083190f: lea r8, [rip + 0x2260970]
140831916: lea rcx, [rbp - 0x10]
14083191a: lea rdx, [rbp - 0x30]
14083191e: mov r9d, 0xc
140831924: call 0x1413115f0
140831929: movzx eax, byte ptr [rbp - 0x10]
14083192d: cmp al, 0x32
14083192f: jne 0x140832049
140831935: lea rax, [rdi + 0x60]
140831939: mov byte ptr [rbp + 0x47], 1
14083193d: mov qword ptr [rsp + 0x20], rax
140831942: lea r8, [rip + 0x2260949]
140831949: lea rcx, [rbp - 0x10]
14083194d: lea rdx, [rbp - 0x30]
140831951: mov r9d, 7
140831957: call 0x1413115f0
14083195c: movzx eax, byte ptr [rbp - 0x10]
140831960: cmp al, 0x32
140831962: jne 0x140832049
140831968: lea rax, [rdi + 0x6c]
14083196c: mov byte ptr [rbp + 0x47], 1
140831970: mov qword ptr [rsp + 0x20], rax
140831975: lea r8, [rip + 0x226091d]
14083197c: lea rcx, [rbp - 0x10]
140831980: lea rdx, [rbp - 0x30]
140831984: mov r9d, 8
14083198a: call 0x1413115f0
14083198f: movzx eax, byte ptr [rbp - 0x10]
140831993: cmp al, 0x32
140831995: jne 0x140832049
14083199b: lea rax, [rdi + 0x78]
14083199f: mov byte ptr [rbp + 0x47], 1
1408319a3: mov qword ptr [rsp + 0x20], rax
1408319a8: lea r8, [rip + 0x22608f2]
1408319af: lea rcx, [rbp - 0x10]
1408319b3: lea rdx, [rbp - 0x30]
1408319b7: mov r9d, 8
1408319bd: call 0x1413115f0
1408319c2: movzx eax, byte ptr [rbp - 0x10]
1408319c6: cmp al, 0x32
1408319c8: jne 0x140832049
1408319ce: lea rax, [rdi + 0x84]
1408319d5: mov byte ptr [rbp + 0x47], 1
1408319d9: mov qword ptr [rsp + 0x20], rax
1408319de: lea r8, [rip + 0x22608c4]
1408319e5: lea rcx, [rbp - 0x10]
1408319e9: lea rdx, [rbp - 0x30]
1408319ed: mov r9d, 4
1408319f3: call 0x1413115f0
1408319f8: movzx eax, byte ptr [rbp - 0x10]
1408319fc: cmp al, 0x32
1408319fe: jne 0x140832049
140831a04: lea rax, [rdi + 0x90]
140831a0b: mov byte ptr [rbp + 0x47], 1
140831a0f: mov qword ptr [rsp + 0x20], rax
140831a14: lea r8, [rip + 0x2260892]
140831a1b: lea rcx, [rbp - 0x10]
140831a1f: lea rdx, [rbp - 0x30]
140831a23: mov r9d, 0xa
140831a29: call 0x1413115f0
140831a2e: movzx eax, byte ptr [rbp - 0x10]
140831a32: cmp al, 0x32
140831a34: jne 0x140832049
140831a3a: lea rax, [rdi + 0x9c]
140831a41: mov byte ptr [rbp + 0x47], 1
140831a45: mov qword ptr [rsp + 0x20], rax
140831a4a: lea r8, [rip + 0x2260866]
140831a51: lea rcx, [rbp - 0x10]
140831a55: lea rdx, [rbp - 0x30]
140831a59: mov r9d, 9
140831a5f: call 0x1413115f0
140831a64: movzx eax, byte ptr [rbp - 0x10]
140831a68: cmp al, 0x32
140831a6a: jne 0x140832049
140831a70: lea rax, [rdi + 0xa8]
140831a77: mov byte ptr [rbp + 0x47], 1
140831a7b: mov qword ptr [rsp + 0x20], rax
140831a80: lea r8, [rip + 0x2260839]
140831a87: lea rcx, [rbp - 0x10]
140831a8b: lea rdx, [rbp - 0x30]
140831a8f: mov r9d, 6
140831a95: call 0x1413115f0
140831a9a: movzx eax, byte ptr [rbp - 0x10]
140831a9e: cmp al, 0x32
140831aa0: jne 0x140832049
140831aa6: lea rax, [rdi + 0xb4]
140831aad: mov byte ptr [rbp + 0x47], 1
140831ab1: mov qword ptr [rsp + 0x20], rax
140831ab6: lea r8, [rip + 0x2260809]
140831abd: lea rcx, [rbp - 0x10]
140831ac1: lea rdx, [rbp - 0x30]
140831ac5: mov r9d, 7
140831acb: call 0x1413115f0
140831ad0: movzx eax, byte ptr [rbp - 0x10]
140831ad4: cmp al, 0x32
140831ad6: jne 0x140832049
140831adc: lea rax, [rdi + 0xc0]
140831ae3: mov byte ptr [rbp + 0x47], 1
140831ae7: mov qword ptr [rsp + 0x20], rax
140831aec: lea r8, [rip + 0x22607da]
140831af3: lea rcx, [rbp - 0x10]
140831af7: lea rdx, [rbp - 0x30]
140831afb: mov r9d, 6
140831b01: call 0x1413115f0
140831b06: movzx eax, byte ptr [rbp - 0x10]
140831b0a: cmp al, 0x32
140831b0c: jne 0x140832049
140831b12: lea rax, [rdi + 0xcc]
140831b19: mov byte ptr [rbp + 0x47], 1
140831b1d: mov qword ptr [rsp + 0x20], rax
140831b22: lea r8, [rip + 0x22607aa]
140831b29: lea rcx, [rbp - 0x10]
140831b2d: lea rdx, [rbp - 0x30]
140831b31: mov r9d, 4
140831b37: call 0x1413115f0
140831b3c: movzx eax, byte ptr [rbp - 0x10]
140831b40: cmp al, 0x32
140831b42: jne 0x140832049
140831b48: lea rax, [rdi + 0xd8]
140831b4f: mov byte ptr [rbp + 0x47], 1
140831b53: mov qword ptr [rsp + 0x20], rax
140831b58: lea r8, [rip + 0x2260778]
140831b5f: lea rcx, [rbp - 0x10]
140831b63: lea rdx, [rbp - 0x30]
140831b67: mov r9d, 5
140831b6d: call 0x1413115f0
140831b72: movzx eax, byte ptr [rbp - 0x10]
140831b76: cmp al, 0x32
140831b78: jne 0x140832049
140831b7e: lea rax, [rdi + 0xe4]
140831b85: mov byte ptr [rbp + 0x47], 1
140831b89: mov qword ptr [rsp + 0x20], rax
140831b8e: lea r8, [rip + 0x2260747]
140831b95: lea rcx, [rbp - 0x10]
140831b99: lea rdx, [rbp - 0x30]
140831b9d: mov r9d, 9
140831ba3: call 0x1413115f0
140831ba8: movzx eax, byte ptr [rbp - 0x10]
140831bac: cmp al, 0x32
140831bae: jne 0x140832049
140831bb4: lea rax, [rdi + 0xf0]
140831bbb: mov byte ptr [rbp + 0x47], 1
140831bbf: mov qword ptr [rsp + 0x20], rax
140831bc4: lea r8, [rip + 0x226071a]
140831bcb: lea rcx, [rbp - 0x10]
140831bcf: lea rdx, [rbp - 0x30]
140831bd3: mov r9d, 0xb
140831bd9: call 0x1413115f0
140831bde: movzx eax, byte ptr [rbp - 0x10]
140831be2: cmp al, 0x32
140831be4: jne 0x140832049
140831bea: lea rax, [rdi + 0xfc]
140831bf1: mov byte ptr [rbp + 0x47], 1
140831bf5: mov qword ptr [rsp + 0x20], rax
140831bfa: lea r8, [rip + 0x22606ef]
140831c01: lea rcx, [rbp - 0x10]
140831c05: lea rdx, [rbp - 0x30]
140831c09: mov r9d, 4
140831c0f: call 0x1413115f0
140831c14: movzx eax, byte ptr [rbp - 0x10]
140831c18: cmp al, 0x32
140831c1a: jne 0x140832049
140831c20: lea rax, [rdi + 0x108]
140831c27: mov byte ptr [rbp + 0x47], 1
140831c2b: mov qword ptr [rsp + 0x20], rax
140831c30: lea r8, [rip + 0x22606bd]
140831c37: lea rcx, [rbp - 0x10]
140831c3b: lea rdx, [rbp - 0x30]
140831c3f: mov r9d, 4
140831c45: call 0x1413115f0
140831c4a: movzx eax, byte ptr [rbp - 0x10]
140831c4e: cmp al, 0x32
140831c50: jne 0x140832049
140831c56: lea rax, [rdi + 0x114]
140831c5d: mov byte ptr [rbp + 0x47], 1
140831c61: mov qword ptr [rsp + 0x20], rax
140831c66: lea r8, [rip + 0x226068b]
140831c6d: lea rcx, [rbp - 0x10]
140831c71: lea rdx, [rbp - 0x30]
140831c75: mov r9d, 0xa
140831c7b: call 0x1413115f0
140831c80: movzx eax, byte ptr [rbp - 0x10]
140831c84: cmp al, 0x32
140831c86: jne 0x140832049
140831c8c: lea rax, [rdi + 0x120]
140831c93: mov byte ptr [rbp + 0x47], 1
140831c97: mov qword ptr [rsp + 0x20], rax
140831c9c: lea r8, [rip + 0x226065f]
140831ca3: lea rcx, [rbp - 0x10]
140831ca7: lea rdx, [rbp - 0x30]
140831cab: mov r9d, 0x10
140831cb1: call 0x1413115f0
140831cb6: movzx eax, byte ptr [rbp - 0x10]
140831cba: cmp al, 0x32
140831cbc: jne 0x140832049
140831cc2: lea rax, [rdi + 0x12c]
140831cc9: mov byte ptr [rbp + 0x47], 1
140831ccd: mov qword ptr [rsp + 0x20], rax
140831cd2: lea r8, [rip + 0x2260639]
140831cd9: lea rcx, [rbp - 0x10]
140831cdd: lea rdx, [rbp - 0x30]
140831ce1: mov r9d, 0xa
140831ce7: call 0x1413115f0
140831cec: movzx eax, byte ptr [rbp - 0x10]
140831cf0: cmp al, 0x32
140831cf2: jne 0x140832049
140831cf8: lea rax, [rdi + 0x138]
140831cff: mov byte ptr [rbp + 0x47], 1
140831d03: mov qword ptr [rsp + 0x20], rax
140831d08: lea r8, [rip + 0x226060d]
140831d0f: lea rcx, [rbp - 0x10]
140831d13: lea rdx, [rbp - 0x30]
140831d17: mov r9d, 8
140831d1d: call 0x1413115f0
140831d22: movzx eax, byte ptr [rbp - 0x10]
140831d26: cmp al, 0x32
140831d28: jne 0x140832049
140831d2e: lea rax, [rdi + 0x144]
140831d35: mov byte ptr [rbp + 0x47], 1
140831d39: mov qword ptr [rsp + 0x20], rax
140831d3e: lea r8, [rip + 0x22605df]
140831d45: lea rcx, [rbp - 0x10]
140831d49: lea rdx, [rbp - 0x30]
140831d4d: mov r9d, 0x11
140831d53: call 0x1413115f0
140831d58: movzx eax, byte ptr [rbp - 0x10]
140831d5c: cmp al, 0x32
140831d5e: jne 0x140832049
140831d64: lea rax, [rdi + 0x150]
140831d6b: mov byte ptr [rbp + 0x47], 1
140831d6f: mov qword ptr [rsp + 0x20], rax
140831d74: lea r8, [rip + 0x22605ba]
140831d7b: lea rcx, [rbp - 0x10]
140831d7f: lea rdx, [rbp - 0x30]
140831d83: mov r9d, 9
140831d89: call 0x1413115f0
140831d8e: movzx eax, byte ptr [rbp - 0x10]
140831d92: cmp al, 0x32
140831d94: jne 0x140832049
140831d9a: lea rax, [rdi + 0x15c]
140831da1: mov byte ptr [rbp + 0x47], 1
140831da5: mov qword ptr [rsp + 0x20], rax
140831daa: lea r8, [rip + 0x226058d]
140831db1: lea rcx, [rbp - 0x10]
140831db5: lea rdx, [rbp - 0x30]
140831db9: mov r9d, 0xa
140831dbf: call 0x1413115f0
140831dc4: movzx eax, byte ptr [rbp - 0x10]
140831dc8: cmp al, 0x32
140831dca: jne 0x140832049
140831dd0: lea rax, [rdi + 0x168]
140831dd7: mov byte ptr [rbp + 0x47], 1
140831ddb: mov qword ptr [rsp + 0x20], rax
140831de0: lea r8, [rip + 0x2260561]
140831de7: lea rcx, [rbp - 0x10]
140831deb: lea rdx, [rbp - 0x30]
140831def: mov r9d, 7
140831df5: call 0x1413115f0
140831dfa: movzx eax, byte ptr [rbp - 0x10]
140831dfe: cmp al, 0x32
140831e00: jne 0x140832049
140831e06: lea rax, [rdi + 0x174]
140831e0d: mov byte ptr [rbp + 0x47], 1
140831e11: mov qword ptr [rsp + 0x20], rax
140831e16: lea r8, [rip + 0x2260532]
140831e1d: lea rcx, [rbp - 0x10]
140831e21: lea rdx, [rbp - 0x30]
140831e25: mov r9d, 8
140831e2b: call 0x1413115f0
140831e30: movzx eax, byte ptr [rbp - 0x10]
140831e34: cmp al, 0x32
140831e36: jne 0x140832049
140831e3c: lea rax, [rdi + 0x180]
140831e43: mov byte ptr [rbp + 0x47], 1
140831e47: mov qword ptr [rsp + 0x20], rax
140831e4c: lea r8, [rip + 0x2260504]
140831e53: lea rcx, [rbp - 0x10]
140831e57: lea rdx, [rbp - 0x30]
140831e5b: mov r9d, 7
140831e61: call 0x1413115f0
140831e66: movzx eax, byte ptr [rbp - 0x10]
140831e6a: cmp al, 0x32
140831e6c: jne 0x140832049
140831e72: lea rax, [rdi + 0x18c]
140831e79: mov byte ptr [rbp + 0x47], 1
140831e7d: mov qword ptr [rsp + 0x20], rax
140831e82: lea r8, [rip + 0x22604d5]
140831e89: lea rcx, [rbp - 0x10]
140831e8d: lea rdx, [rbp - 0x30]
140831e91: mov r9d, 5
140831e97: call 0x1413115f0
140831e9c: movzx eax, byte ptr [rbp - 0x10]
140831ea0: cmp al, 0x32
140831ea2: jne 0x140832049
140831ea8: lea rax, [rdi + 0x198]
140831eaf: mov byte ptr [rbp + 0x47], 1
140831eb3: mov qword ptr [rsp + 0x20], rax
140831eb8: lea r8, [rip + 0x22604a4]
140831ebf: lea rcx, [rbp - 0x10]
140831ec3: lea rdx, [rbp - 0x30]
140831ec7: mov r9d, 0xb
140831ecd: call 0x1413115f0
140831ed2: movzx eax, byte ptr [rbp - 0x10]
140831ed6: cmp al, 0x32
140831ed8: jne 0x140832049
140831ede: lea rax, [rdi + 0x1a4]
140831ee5: mov byte ptr [rbp + 0x47], 1
140831ee9: mov qword ptr [rsp + 0x20], rax
140831eee: lea r8, [rip + 0x2260479]
140831ef5: lea rcx, [rbp - 0x10]
140831ef9: lea rdx, [rbp - 0x30]
140831efd: mov r9d, 0xb
140831f03: call 0x1413115f0
140831f08: movzx eax, byte ptr [rbp - 0x10]
140831f0c: cmp al, 0x32
140831f0e: jne 0x140832049
140831f14: lea rax, [rdi + 0x1bc]
140831f1b: mov byte ptr [rbp + 0x47], 1
140831f1f: mov qword ptr [rsp + 0x20], rax
140831f24: lea r8, [rip + 0x226044e]
140831f2b: lea rcx, [rbp - 0x10]
140831f2f: lea rdx, [rbp - 0x30]
140831f33: mov r9d, 0xc
140831f39: call 0x141312760
140831f3e: movzx eax, byte ptr [rbp - 0x10]
140831f42: cmp al, 0x32
140831f44: jne 0x140832049
140831f4a: lea rax, [rdi + 0x1c0]
140831f51: mov byte ptr [rbp + 0x47], 1
140831f55: mov qword ptr [rsp + 0x20], rax
140831f5a: lea r8, [rip + 0x2260424]
140831f61: lea rcx, [rbp - 0x10]
140831f65: lea rdx, [rbp - 0x30]
140831f69: mov r9d, 9
140831f6f: call 0x141312760
140831f74: movzx eax, byte ptr [rbp - 0x10]
140831f78: cmp al, 0x32
140831f7a: jne 0x140832049
140831f80: lea rax, [rdi + 0x1c4]
140831f87: mov byte ptr [rbp + 0x47], 1
140831f8b: mov qword ptr [rsp + 0x20], rax
140831f90: lea r8, [rip + 0x22603f7]
140831f97: lea rcx, [rbp - 0x10]
140831f9b: lea rdx, [rbp - 0x30]
140831f9f: mov r9d, 0x10
140831fa5: call 0x141312760
140831faa: movzx eax, byte ptr [rbp - 0x10]
140831fae: cmp al, 0x32
140831fb0: jne 0x140832049
140831fb6: add rdi, 0x1b0
140831fbd: mov byte ptr [rbp + 0x47], 1
140831fc1: mov qword ptr [rsp + 0x20], rdi
140831fc6: lea r8, [rip + 0x22603d1]
140831fcd: lea rcx, [rbp - 0x10]
140831fd1: lea rdx, [rbp - 0x30]
140831fd5: mov r9d, 0x13
140831fdb: call 0x1413115f0
140831fe0: movzx eax, byte ptr [rbp - 0x10]
140831fe4: cmp al, 0x32
140831fe6: jne 0x140832049
140831fe8: mov rax, qword ptr [rbp - 0x20]
140831fec: mov qword ptr [rbp], rax
140831ff0: movaps xmm0, xmmword ptr [rbp - 0x30]
140831ff4: movaps xmmword ptr [rbp - 0x10], xmm0
140831ff8: mov byte ptr [rbp + 0x47], 0
140831ffc: lea rdx, [rbp - 0x10]
140832000: mov rcx, rsi
140832003: call 0x141312a40
140832008: jmp 0x140832092
14083200d: movups xmm0, xmmword ptr [rbp - 0xf]
140832011: movups xmm1, xmmword ptr [rbp]
140832015: movups xmm2, xmmword ptr [rbp + 0x10]
140832019: movups xmm3, xmmword ptr [rbp + 0x20]
14083201d: movups xmmword ptr [rbp - 0x41], xmm1
140832021: movaps xmmword ptr [rbp - 0x50], xmm0
140832025: mov rcx, qword ptr [rbp + 0x30]
140832029: mov qword ptr [rsi + 0x40], rcx
14083202d: movups xmmword ptr [rsi + 0x30], xmm3
140832031: movups xmmword ptr [rsi + 0x20], xmm2
140832035: movups xmm0, xmmword ptr [rbp - 0x41]
140832039: movups xmmword ptr [rsi + 0x10], xmm0
14083203d: movaps xmm0, xmmword ptr [rbp - 0x50]
140832041: movups xmmword ptr [rsi + 1], xmm0
140832045: mov byte ptr [rsi], al
140832047: jmp 0x140832092
140832049: mov rcx, qword ptr [rbp + 0x30]
14083204d: mov qword ptr [rsi + 0x40], rcx
140832051: movups xmm0, xmmword ptr [rbp - 0xf]
140832055: movups xmm1, xmmword ptr [rbp + 1]
140832059: movups xmm2, xmmword ptr [rbp + 0x11]
14083205d: movups xmm3, xmmword ptr [rbp + 0x21]
140832061: movups xmmword ptr [rsi + 0x31], xmm3
140832065: movups xmmword ptr [rsi + 0x21], xmm2
140832069: movups xmmword ptr [rsi + 0x11], xmm1
14083206d: movups xmmword ptr [rsi + 1], xmm0
140832071: mov byte ptr [rsi], al
140832073: mov rax, qword ptr [rbp - 0x28]
140832077: cmp dword ptr [rax], 1
14083207a: jne 0x140832092
14083207c: mov rcx, qword ptr [rax + 8]
140832080: inc rcx
140832083: mov rdx, 0xffffffffffffffff
14083208a: cmovne rdx, rcx
14083208e: mov qword ptr [rax + 8], rdx
140832092: mov rax, rsi
140832095: add rsp, 0xd0
14083209c: pop rdi
14083209d: pop rsi
14083209e: pop rbp
14083209f: ret 
1408320a0: mov qword ptr [rsp + 0x10], rdx
1408320a5: push rbp
1408320a6: push rsi
1408320a7: push rdi
1408320a8: sub rsp, 0x30
1408320ac: lea rbp, [rdx + 0x80]
1408320b3: cmp byte ptr [rbp + 0x47], 0
1408320b7: je 0x1408320d8
1408320b9: mov rax, qword ptr [rbp - 0x28]
1408320bd: cmp dword ptr [rax], 1
1408320c0: jne 0x1408320d8
1408320c2: mov rcx, qword ptr [rax + 8]
1408320c6: inc rcx
1408320c9: mov rdx, 0xffffffffffffffff
1408320d0: cmovne rdx, rcx
1408320d4: mov qword ptr [rax + 8], rdx
1408320d8: add rsp, 0x30
1408320dc: pop rdi
1408320dd: pop rsi
1408320de: pop rbp
1408320df: ret 
