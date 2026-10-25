1409c08d0: 55                       push rbp
1409c08d1: 4157                     push r15
1409c08d3: 4156                     push r14
1409c08d5: 4155                     push r13
1409c08d7: 4154                     push r12
1409c08d9: 56                       push rsi
1409c08da: 57                       push rdi
1409c08db: 53                       push rbx
1409c08dc: 4881ec280e0000           sub rsp, 0xe28
1409c08e3: 488dac2480000000         lea rbp, [rsp + 0x80]
1409c08eb: 48c785a00d0000feffffff   mov qword ptr [rbp + 0xda0], 0xfffffffffffffffe
1409c08f6: 4d89cf                   mov r15, r9
1409c08f9: 4d89c6                   mov r14, r8
1409c08fc: 4989d5                   mov r13, rdx
1409c08ff: 4989cc                   mov r12, rcx
1409c0902: 488b9d100e0000           mov rbx, qword ptr [rbp + 0xe10]
1409c0909: 498d7004                 lea rsi, [r8 + 4]
1409c090d: 4889f1                   mov rcx, rsi
1409c0910: e87b4e2100               call 0x140bd5790
1409c0915: 4889c7                   mov rdi, rax
1409c0918: 4889f1                   mov rcx, rsi
1409c091b: e8704e2100               call 0x140bd5790
1409c0920: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c0927: 4889c2                   mov rdx, rax
1409c092a: 41b809000000             mov r8d, 9
1409c0930: e86b492100               call 0x140bd52a0
1409c0935: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c093c: 0f108df00b0000           movups xmm1, xmmword ptr [rbp + 0xbf0]
1409c0943: 0f298dc0020000           movaps xmmword ptr [rbp + 0x2c0], xmm1
1409c094a: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c0951: 488d8dc0070000           lea rcx, [rbp + 0x7c0]
1409c0958: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c095f: e87c632100               call 0x140bd6ce0
1409c0964: 488d051ef30f02           lea rax, [rip + 0x20ff31e]
1409c096b: 488985b0070000           mov qword ptr [rbp + 0x7b0], rax
1409c0972: 48c785b807000011000000   mov qword ptr [rbp + 0x7b8], 0x11
1409c097d: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c0984: 4c89e1                   mov rcx, r12
1409c0987: e8448b2200               call 0x140be94d0
1409c098c: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c0993: 0f108df00b0000           movups xmm1, xmmword ptr [rbp + 0xbf0]
1409c099a: 0f118510020000           movups xmmword ptr [rbp + 0x210], xmm0
1409c09a1: 0f118d20020000           movups xmmword ptr [rbp + 0x220], xmm1
1409c09a8: 89850c020000             mov dword ptr [rbp + 0x20c], eax
1409c09ae: 4889f1                   mov rcx, rsi
1409c09b1: e8da4d2100               call 0x140bd5790
1409c09b6: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c09bd: 4889c2                   mov rdx, rax
1409c09c0: 41b862000000             mov r8d, 0x62
1409c09c6: e8d5482100               call 0x140bd52a0
1409c09cb: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c09d2: 0f108df00b0000           movups xmm1, xmmword ptr [rbp + 0xbf0]
1409c09d9: 0f298dc0020000           movaps xmmword ptr [rbp + 0x2c0], xmm1
1409c09e0: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c09e7: 488d8dc0070000           lea rcx, [rbp + 0x7c0]
1409c09ee: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c09f5: e8e6622100               call 0x140bd6ce0
1409c09fa: 488d0599f20f02           lea rax, [rip + 0x20ff299]
1409c0a01: 488985b0070000           mov qword ptr [rbp + 0x7b0], rax
1409c0a08: 48c785b80700000d000000   mov qword ptr [rbp + 0x7b8], 0xd
1409c0a13: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c0a1a: 4c89e1                   mov rcx, r12
1409c0a1d: e8ae8a2200               call 0x140be94d0
1409c0a22: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c0a29: 0f108df00b0000           movups xmm1, xmmword ptr [rbp + 0xbf0]
1409c0a30: 0f1185a4010000           movups xmmword ptr [rbp + 0x1a4], xmm0
1409c0a37: 0f118db4010000           movups xmmword ptr [rbp + 0x1b4], xmm1
1409c0a3e: 8985a0010000             mov dword ptr [rbp + 0x1a0], eax
1409c0a44: 4889b5780d0000           mov qword ptr [rbp + 0xd78], rsi
1409c0a4b: 4889f1                   mov rcx, rsi
1409c0a4e: e83d4d2100               call 0x140bd5790
1409c0a53: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c0a5a: 4889f1                   mov rcx, rsi
1409c0a5d: 4889c2                   mov rdx, rax
1409c0a60: 41b862000000             mov r8d, 0x62
1409c0a66: e835482100               call 0x140bd52a0
1409c0a6b: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c0a72: 4889f2                   mov rdx, rsi
1409c0a75: e896492100               call 0x140bd5410
1409c0a7a: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c0a81: 0f108df00b0000           movups xmm1, xmmword ptr [rbp + 0xbf0]
1409c0a88: 0f298dc0020000           movaps xmmword ptr [rbp + 0x2c0], xmm1
1409c0a8f: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c0a96: 488d8dc0070000           lea rcx, [rbp + 0x7c0]
1409c0a9d: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c0aa4: e837622100               call 0x140bd6ce0
1409c0aa9: 488d05f7f10f02           lea rax, [rip + 0x20ff1f7]
1409c0ab0: 488985b0070000           mov qword ptr [rbp + 0x7b0], rax
1409c0ab7: 48c785b807000015000000   mov qword ptr [rbp + 0x7b8], 0x15
1409c0ac2: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c0ac9: 4c89e1                   mov rcx, r12
1409c0acc: e8ff892200               call 0x140be94d0
1409c0ad1: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c0ad8: 0f108df00b0000           movups xmm1, xmmword ptr [rbp + 0xbf0]
1409c0adf: 0f118580010000           movups xmmword ptr [rbp + 0x180], xmm0
1409c0ae6: 0f118d90010000           movups xmmword ptr [rbp + 0x190], xmm1
1409c0aed: 89857c010000             mov dword ptr [rbp + 0x17c], eax
1409c0af3: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c0afa: ba10000000               mov edx, 0x10
1409c0aff: 41b820000000             mov r8d, 0x20
1409c0b05: e816432100               call 0x140bd4e20
1409c0b0a: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c0b11: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c0b18: 488d8dc0070000           lea rcx, [rbp + 0x7c0]
1409c0b1f: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c0b26: e8d5612100               call 0x140bd6d00
1409c0b2b: 488d058af10f02           lea rax, [rip + 0x20ff18a]
1409c0b32: 488985b0070000           mov qword ptr [rbp + 0x7b0], rax
1409c0b39: 48c785b807000015000000   mov qword ptr [rbp + 0x7b8], 0x15
1409c0b44: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c0b4b: 4c89e1                   mov rcx, r12
1409c0b4e: e87d892200               call 0x140be94d0
1409c0b53: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c0b5a: 0f2985c00b0000           movaps xmmword ptr [rbp + 0xbc0], xmm0
1409c0b61: 8985d00b0000             mov dword ptr [rbp + 0xbd0], eax
1409c0b67: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c0b6e: ba10000000               mov edx, 0x10
1409c0b73: 41b820000000             mov r8d, 0x20
1409c0b79: e8a2422100               call 0x140bd4e20
1409c0b7e: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c0b85: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c0b8c: 488d8dc0070000           lea rcx, [rbp + 0x7c0]
1409c0b93: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c0b9a: e861612100               call 0x140bd6d00
1409c0b9f: 488d052bf10f02           lea rax, [rip + 0x20ff12b]
1409c0ba6: 488985b0070000           mov qword ptr [rbp + 0x7b0], rax
1409c0bad: 48c785b807000013000000   mov qword ptr [rbp + 0x7b8], 0x13
1409c0bb8: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c0bbf: 4c89e1                   mov rcx, r12
1409c0bc2: e809892200               call 0x140be94d0
1409c0bc7: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c0bce: 0f2985a0060000           movaps xmmword ptr [rbp + 0x6a0], xmm0
1409c0bd5: 8985b0060000             mov dword ptr [rbp + 0x6b0], eax
1409c0bdb: 48c744242004000000       mov qword ptr [rsp + 0x20], 4
1409c0be4: 488d35f9f00f02           lea rsi, [rip + 0x20ff0f9]
1409c0beb: 4c8d85c00b0000           lea r8, [rbp + 0xbc0]
1409c0bf2: 4c89e1                   mov rcx, r12
1409c0bf5: 4c89ea                   mov rdx, r13
1409c0bf8: 4989f1                   mov r9, rsi
1409c0bfb: e8a0cdf4ff               call 0x14090d9a0
1409c0c00: 48c744242004000000       mov qword ptr [rsp + 0x20], 4
1409c0c09: 4c8d85a0060000           lea r8, [rbp + 0x6a0]
1409c0c10: 4c89e1                   mov rcx, r12
1409c0c13: 4c89ea                   mov rdx, r13
1409c0c16: 4989f1                   mov r9, rsi
1409c0c19: e882cdf4ff               call 0x14090d9a0
1409c0c1e: 488d4b04                 lea rcx, [rbx + 4]
1409c0c22: e8694b2100               call 0x140bd5790
1409c0c27: 4889c1                   mov rcx, rax
1409c0c2a: 48c1e920                 shr rcx, 0x20
1409c0c2e: 0fafc1                   imul eax, ecx
1409c0c31: 89c6                     mov esi, eax
1409c0c33: c1ee04                   shr esi, 4
1409c0c36: 01c6                     add esi, eax
1409c0c38: 488d14b500000000         lea rdx, [rsi*4]
1409c0c40: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c0c47: 41b820000000             mov r8d, 0x20
1409c0c4d: e8ce412100               call 0x140bd4e20
1409c0c52: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c0c59: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c0c60: 488d8dc0070000           lea rcx, [rbp + 0x7c0]
1409c0c67: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c0c6e: e88d602100               call 0x140bd6d00
1409c0c73: 488d057af00f02           lea rax, [rip + 0x20ff07a]
1409c0c7a: 488985b0070000           mov qword ptr [rbp + 0x7b0], rax
1409c0c81: 48c785b807000012000000   mov qword ptr [rbp + 0x7b8], 0x12
1409c0c8c: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c0c93: 4c89e1                   mov rcx, r12
1409c0c96: e835882200               call 0x140be94d0
1409c0c9b: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c0ca2: 0f298560020000           movaps xmmword ptr [rbp + 0x260], xmm0
1409c0ca9: 898570020000             mov dword ptr [rbp + 0x270], eax
1409c0caf: 48c1e604                 shl rsi, 4
1409c0cb3: 488d9de00b0000           lea rbx, [rbp + 0xbe0]
1409c0cba: 4889d9                   mov rcx, rbx
1409c0cbd: 4889f2                   mov rdx, rsi
1409c0cc0: 41b820000000             mov r8d, 0x20
1409c0cc6: e855412100               call 0x140bd4e20
1409c0ccb: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c0cd2: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c0cd9: 488d8dc0070000           lea rcx, [rbp + 0x7c0]
1409c0ce0: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c0ce7: e814602100               call 0x140bd6d00
1409c0cec: 488d0513f00f02           lea rax, [rip + 0x20ff013]
1409c0cf3: 488985b0070000           mov qword ptr [rbp + 0x7b0], rax
1409c0cfa: 48c785b807000011000000   mov qword ptr [rbp + 0x7b8], 0x11
1409c0d05: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c0d0c: 4c89e1                   mov rcx, r12
1409c0d0f: e8bc872200               call 0x140be94d0
1409c0d14: 0f1085e00b0000           movups xmm0, xmmword ptr [rbp + 0xbe0]
1409c0d1b: 0f298560010000           movaps xmmword ptr [rbp + 0x160], xmm0
1409c0d22: 898570010000             mov dword ptr [rbp + 0x170], eax
1409c0d28: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c0d2f: 4889f1                   mov rcx, rsi
1409c0d32: 4889bda80b0000           mov qword ptr [rbp + 0xba8], rdi
1409c0d39: 4889fa                   mov rdx, rdi
1409c0d3c: 41b809000000             mov r8d, 9
1409c0d42: e859452100               call 0x140bd52a0
1409c0d47: 488dbdb0020000           lea rdi, [rbp + 0x2b0]
1409c0d4e: 4889f9                   mov rcx, rdi
1409c0d51: 4889f2                   mov rdx, rsi
1409c0d54: e817462100               call 0x140bd5370
1409c0d59: 4889d9                   mov rcx, rbx
1409c0d5c: 4889fa                   mov rdx, rdi
1409c0d5f: e86c462100               call 0x140bd53d0
1409c0d64: 48895c2420               mov qword ptr [rsp + 0x20], rbx
1409c0d69: 4c8d05a7ef0f02           lea r8, [rip + 0x20fefa7]
1409c0d70: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c0d77: 41b90b000000             mov r9d, 0xb
1409c0d7d: 4c89e2                   mov rdx, r12
1409c0d80: e80bc7faff               call 0x14096d490
1409c0d85: 83bdb007000001           cmp dword ptr [rbp + 0x7b0], 1
1409c0d8c: 0f84471d0000             je 0x1409c2ad9
1409c0d92: 488b85f4070000           mov rax, qword ptr [rbp + 0x7f4]
1409c0d99: 488985f0020000           mov qword ptr [rbp + 0x2f0], rax
1409c0da0: 0f1085b4070000           movups xmm0, xmmword ptr [rbp + 0x7b4]
1409c0da7: 0f108dc4070000           movups xmm1, xmmword ptr [rbp + 0x7c4]
1409c0dae: 0f1095d4070000           movups xmm2, xmmword ptr [rbp + 0x7d4]
1409c0db5: 0f109de4070000           movups xmm3, xmmword ptr [rbp + 0x7e4]
1409c0dbc: 0f299de0020000           movaps xmmword ptr [rbp + 0x2e0], xmm3
1409c0dc3: 0f2995d0020000           movaps xmmword ptr [rbp + 0x2d0], xmm2
1409c0dca: 0f298dc0020000           movaps xmmword ptr [rbp + 0x2c0], xmm1
1409c0dd1: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c0dd8: 0fb685fc070000           movzx eax, byte ptr [rbp + 0x7fc]
1409c0ddf: 89858c0d0000             mov dword ptr [rbp + 0xd8c], eax
1409c0de5: 8b85d0020000             mov eax, dword ptr [rbp + 0x2d0]
1409c0deb: 898550010000             mov dword ptr [rbp + 0x150], eax
1409c0df1: 0f298d40010000           movaps xmmword ptr [rbp + 0x140], xmm1
1409c0df8: 0f298530010000           movaps xmmword ptr [rbp + 0x130], xmm0
1409c0dff: 0f1085e4020000           movups xmm0, xmmword ptr [rbp + 0x2e4]
1409c0e06: 0f298510010000           movaps xmmword ptr [rbp + 0x110], xmm0
1409c0e0d: 0f1085d4020000           movups xmm0, xmmword ptr [rbp + 0x2d4]
1409c0e14: 0f298500010000           movaps xmmword ptr [rbp + 0x100], xmm0
1409c0e1b: 8b85f4020000             mov eax, dword ptr [rbp + 0x2f4]
1409c0e21: 898520010000             mov dword ptr [rbp + 0x120], eax
1409c0e27: 498d4f04                 lea rcx, [r15 + 4]
1409c0e2b: e860492100               call 0x140bd5790
1409c0e30: c785b007000000000000     mov dword ptr [rbp + 0x7b0], 0
1409c0e3a: 4c89b5b80b0000           mov qword ptr [rbp + 0xbb8], r14
1409c0e41: 4c89b5b8070000           mov qword ptr [rbp + 0x7b8], r14
1409c0e48: c785d007000001000000     mov dword ptr [rbp + 0x7d0], 1
1409c0e52: 0f2805d74af601           movaps xmm0, xmmword ptr [rip + 0x1f64ad7]
1409c0e59: 0f1185d4070000           movups xmmword ptr [rbp + 0x7d4], xmm0
1409c0e60: 488d8d0c020000           lea rcx, [rbp + 0x20c]
1409c0e67: 48898de8070000           mov qword ptr [rbp + 0x7e8], rcx
1409c0e6e: c785f007000001000000     mov dword ptr [rbp + 0x7f0], 1
1409c0e78: 0f57c0                   xorps xmm0, xmm0
1409c0e7b: 0f1185f4070000           movups xmmword ptr [rbp + 0x7f4], xmm0
1409c0e82: 488d8da0010000           lea rcx, [rbp + 0x1a0]
1409c0e89: 48898d08080000           mov qword ptr [rbp + 0x808], rcx
1409c0e90: c7851008000002000000     mov dword ptr [rbp + 0x810], 2
1409c0e9a: 488d8d7c010000           lea rcx, [rbp + 0x17c]
1409c0ea1: 48898d18080000           mov qword ptr [rbp + 0x818], rcx
1409c0ea8: c7853008000001000000     mov dword ptr [rbp + 0x830], 1
1409c0eb2: 0f118534080000           movups xmmword ptr [rbp + 0x834], xmm0
1409c0eb9: 488d8d30010000           lea rcx, [rbp + 0x130]
1409c0ec0: 48898d48080000           mov qword ptr [rbp + 0x848], rcx
1409c0ec7: 48c785e00b000000000000   mov qword ptr [rbp + 0xbe0], 0
1409c0ed2: 4c89bdb00b0000           mov qword ptr [rbp + 0xbb0], r15
1409c0ed9: 4c89bde80b0000           mov qword ptr [rbp + 0xbe8], r15
1409c0ee0: 48c785f00b000000000000   mov qword ptr [rbp + 0xbf0], 0
1409c0eeb: 4c89a5800d0000           mov qword ptr [rbp + 0xd80], r12
1409c0ef2: 4c89642440               mov qword ptr [rsp + 0x40], r12
1409c0ef7: 4c896c2438               mov qword ptr [rsp + 0x38], r13
1409c0efc: 4889442430               mov qword ptr [rsp + 0x30], rax
1409c0f01: 48895c2428               mov qword ptr [rsp + 0x28], rbx
1409c0f06: 48c744242005000000       mov qword ptr [rsp + 0x20], 5
1409c0f0f: 488d1572ee0f02           lea rdx, [rip + 0x20fee72]
1409c0f16: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c0f1d: 4c8da5b0070000           lea r12, [rbp + 0x7b0]
1409c0f24: 41b809000000             mov r8d, 9
1409c0f2a: 4d89e1                   mov r9, r12
1409c0f2d: e83e6aeaff               call 0x140867970
1409c0f32: 4c89ad900d0000           mov qword ptr [rbp + 0xd90], r13
1409c0f39: 498d4d38                 lea rcx, [r13 + 0x38]
1409c0f3d: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c0f44: 48ba29f7805b1962c34b     movabs rdx, 0x4bc362195b80f729
1409c0f4e: 49b8fbcecb80ce982798     movabs r8, 0x982798ce80cbcefb
1409c0f58: e893c771ff               call 0x1400dd6f0
1409c0f5d: 4885c0                   test rax, rax
1409c0f60: 488b8d900d0000           mov rcx, qword ptr [rbp + 0xd90]
1409c0f67: 0f84541b0000             je 0x1409c2ac1
1409c0f6d: 48399118020000           cmp qword ptr [rcx + 0x218], rdx
1409c0f74: 0f86471b0000             jbe 0x1409c2ac1
1409c0f7a: 488b8110020000           mov rax, qword ptr [rcx + 0x210]
1409c0f81: 488b04d0                 mov rax, qword ptr [rax + rdx*8]
1409c0f85: 4885c0                   test rax, rax
1409c0f88: 0f84331b0000             je 0x1409c2ac1
1409c0f8e: 488b89e0010000           mov rcx, qword ptr [rcx + 0x1e0]
1409c0f95: 48f7d0                   not rax
1409c0f98: 488d1440                 lea rdx, [rax + rax*2]
1409c0f9c: 48c1e205                 shl rdx, 5
1409c0fa0: 48837c112000             cmp qword ptr [rcx + rdx + 0x20], 0
1409c0fa6: 0f84151b0000             je 0x1409c2ac1
1409c0fac: 488b85380e0000           mov rax, qword ptr [rbp + 0xe38]
1409c0fb3: 4801d1                   add rcx, rdx
1409c0fb6: 488b4910                 mov rcx, qword ptr [rcx + 0x10]
1409c0fba: 488b11                   mov rdx, qword ptr [rcx]
1409c0fbd: 4885d2                   test rdx, rdx
1409c0fc0: 780a                     js 0x1409c0fcc
1409c0fc2: 0f57c0                   xorps xmm0, xmm0
1409c0fc5: f3480f2ac2               cvtsi2ss xmm0, rdx
1409c0fca: eb18                     jmp 0x1409c0fe4
1409c0fcc: 4989d0                   mov r8, rdx
1409c0fcf: 49d1e8                   shr r8, 1
1409c0fd2: 83e201                   and edx, 1
1409c0fd5: 4c09c2                   or rdx, r8
1409c0fd8: 0f57c0                   xorps xmm0, xmm0
1409c0fdb: f3480f2ac2               cvtsi2ss xmm0, rdx
1409c0fe0: f30f58c0                 addss xmm0, xmm0
1409c0fe4: f30f2a4908               cvtsi2ss xmm1, dword ptr [rcx + 8]
1409c0fe9: f30f5e0d4f66fe01         divss xmm1, dword ptr [rip + 0x1fe664f]
1409c0ff1: f30f58c8                 addss xmm1, xmm0
1409c0ff5: f30f118d80020000         movss dword ptr [rbp + 0x280], xmm1
1409c0ffd: 488bb870010000           mov rdi, qword ptr [rax + 0x170]
1409c1004: 488b8078010000           mov rax, qword ptr [rax + 0x178]
1409c100b: 48c1e007                 shl rax, 7
1409c100f: 4801f8                   add rax, rdi
1409c1012: 488985a00b0000           mov qword ptr [rbp + 0xba0], rax
1409c1019: 31c0                     xor eax, eax
1409c101b: 80bd200e000000           cmp byte ptr [rbp + 0xe20], 0
1409c1022: 8b8d8c0d0000             mov ecx, dword ptr [rbp + 0xd8c]
1409c1028: 0f44c8                   cmove ecx, eax
1409c102b: 898d8c0d0000             mov dword ptr [rbp + 0xd8c], ecx
1409c1031: c6858402000000           mov byte ptr [rbp + 0x284], 0
1409c1038: 4c8db5b8060000           lea r14, [rbp + 0x6b8]
1409c103f: 483bbda00b0000           cmp rdi, qword ptr [rbp + 0xba0]
1409c1046: 0f84d5030000             je 0x1409c1421
1409c104c: 0f1f4000                 nop dword ptr [rax]
1409c1050: 4c8b7f50                 mov r15, qword ptr [rdi + 0x50]
1409c1054: 4c8b6f58                 mov r13, qword ptr [rdi + 0x58]
1409c1058: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c105f: e8ec562200               call 0x140be6750
1409c1064: 89c6                     mov esi, eax
1409c1066: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c106d: 31c9                     xor ecx, ecx
1409c106f: e8ac8a71ff               call 0x1400d9b20
1409c1074: 4885c0                   test rax, rax
1409c1077: 0f842c1a0000             je 0x1409c2aa9
1409c107d: 89b560070000             mov dword ptr [rbp + 0x760], esi
1409c1083: 66c785680700000000       mov word ptr [rbp + 0x768], 0
1409c108c: c785640700000000803f     mov dword ptr [rbp + 0x764], 0x3f800000
1409c1096: 48c785b806000001000000   mov qword ptr [rbp + 0x6b8], 1
1409c10a1: 4c89bdc0060000           mov qword ptr [rbp + 0x6c0], r15
1409c10a8: 4c89ad58070000           mov qword ptr [rbp + 0x758], r13
1409c10af: 48c785c806000000000000   mov qword ptr [rbp + 0x6c8], 0
1409c10ba: 48c785d006000004000000   mov qword ptr [rbp + 0x6d0], 4
1409c10c5: 48c785d806000000000000   mov qword ptr [rbp + 0x6d8], 0
1409c10d0: 0f1000                   movups xmm0, xmmword ptr [rax]
1409c10d3: 48ff00                   inc qword ptr [rax]
1409c10d6: 0f100d53560d02           movups xmm1, xmmword ptr [rip + 0x20d5653]
1409c10dd: 488d8528070000           lea rax, [rbp + 0x728]
1409c10e4: 0f114810                 movups xmmword ptr [rax + 0x10], xmm1
1409c10e8: 0f100d31560d02           movups xmm1, xmmword ptr [rip + 0x20d5631]
1409c10ef: 0f1108                   movups xmmword ptr [rax], xmm1
1409c10f2: 0f118548070000           movups xmmword ptr [rbp + 0x748], xmm0
1409c10f9: 48b80300000001000000     movabs rax, 0x100000003
1409c1103: 488985f8060000           mov qword ptr [rbp + 0x6f8], rax
1409c110a: 48c785e006000000000000   mov qword ptr [rbp + 0x6e0], 0
1409c1115: 48c785e806000001000000   mov qword ptr [rbp + 0x6e8], 1
1409c1120: 48c785f006000000000000   mov qword ptr [rbp + 0x6f0], 0
1409c112b: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c1132: 4c89e1                   mov rcx, r12
1409c1135: 4c89f2                   mov rdx, r14
1409c1138: 4c8d85b0020000           lea r8, [rbp + 0x2b0]
1409c113f: 4531c9                   xor r9d, r9d
1409c1142: e8994af4ff               call 0x140905be0
1409c1147: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c114e: 4889d9                   mov rcx, rbx
1409c1151: 4c89e2                   mov rdx, r12
1409c1154: 4c8d8580020000           lea r8, [rbp + 0x280]
1409c115b: 4989f9                   mov r9, rdi
1409c115e: e8fda2f4ff               call 0x14090b460
1409c1163: 80bdbe0c000008           cmp byte ptr [rbp + 0xcbe], 8
1409c116a: 7428                     je 0x1409c1194
1409c116c: 488b95c80c0000           mov rdx, qword ptr [rbp + 0xcc8]
1409c1173: b8e8000000               mov eax, 0xe8
1409c1178: 4801c2                   add rdx, rax
1409c117b: 488d8d980c0000           lea rcx, [rbp + 0xc98]
1409c1182: 49b84ca5591f44988a41     movabs r8, 0x418a98441f59a54c
1409c118c: 41b101                   mov r9b, 1
1409c118f: e8bca6faff               call 0x14096b850
1409c1194: 41b8f8000000             mov r8d, 0xf8
1409c119a: 4c89e1                   mov rcx, r12
1409c119d: 4889da                   mov rdx, rbx
1409c11a0: e85b2af601               call 0x142923c00
1409c11a5: c6853002000011           mov byte ptr [rbp + 0x230], 0x11
1409c11ac: c785b806000002000000     mov dword ptr [rbp + 0x6b8], 2
1409c11b6: 4c89742420               mov qword ptr [rsp + 0x20], r14
1409c11bb: 488d8d30020000           lea rcx, [rbp + 0x230]
1409c11c2: 4c89e2                   mov rdx, r12
1409c11c5: 49b8a41a84c94d1f6511     movabs r8, 0x11651f4dc9841aa4
1409c11cf: 41b11d                   mov r9b, 0x1d
1409c11d2: e8797af4ff               call 0x140908c50
1409c11d7: 41b8f8000000             mov r8d, 0xf8
1409c11dd: 4c89f1                   mov rcx, r14
1409c11e0: 4c89e2                   mov rdx, r12
1409c11e3: e8182af601               call 0x142923c00
1409c11e8: 488b85100e0000           mov rax, qword ptr [rbp + 0xe10]
1409c11ef: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c11f6: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c1200: 4c89642420               mov qword ptr [rsp + 0x20], r12
1409c1205: 4889d9                   mov rcx, rbx
1409c1208: 4c89f2                   mov rdx, r14
1409c120b: 49b8308202283a3c332a     movabs r8, 0x2a333c3a28028230
1409c1215: 41b11d                   mov r9b, 0x1d
1409c1218: e8e3d5f2ff               call 0x1408ee800
1409c121d: 41b8f8000000             mov r8d, 0xf8
1409c1223: 4c89e1                   mov rcx, r12
1409c1226: 4c89f2                   mov rdx, r14
1409c1229: e8d229f601               call 0x142923c00
1409c122e: 488b85180e0000           mov rax, qword ptr [rbp + 0xe18]
1409c1235: 488985b8060000           mov qword ptr [rbp + 0x6b8], rax
1409c123c: c785e00b000002000000     mov dword ptr [rbp + 0xbe0], 2
1409c1246: 48895c2420               mov qword ptr [rsp + 0x20], rbx
1409c124b: 4c89f1                   mov rcx, r14
1409c124e: 4c89e2                   mov rdx, r12
1409c1251: 49b8d0096c8285511704     movabs r8, 0x4175185826c09d0
1409c125b: 41b11d                   mov r9b, 0x1d
1409c125e: e89dd5f2ff               call 0x1408ee800
1409c1263: 41b8f8000000             mov r8d, 0xf8
1409c1269: 4889d9                   mov rcx, rbx
1409c126c: 4c89e2                   mov rdx, r12
1409c126f: e88c29f601               call 0x142923c00
1409c1274: 488d8500010000           lea rax, [rbp + 0x100]
1409c127b: 488985b8060000           mov qword ptr [rbp + 0x6b8], rax
1409c1282: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c128c: 4c89642420               mov qword ptr [rsp + 0x20], r12
1409c1291: 4c89f1                   mov rcx, r14
1409c1294: 4889da                   mov rdx, rbx
1409c1297: 49b81bde698451b2fafd     movabs r8, 0xfdfab2518469de1b
1409c12a1: 41b11d                   mov r9b, 0x1d
1409c12a4: e8a7e3faff               call 0x14096f650
1409c12a9: 41b8f8000000             mov r8d, 0xf8
1409c12af: 488d4dd0                 lea rcx, [rbp - 0x30]
1409c12b3: 4889da                   mov rdx, rbx
1409c12b6: e84529f601               call 0x142923c00
1409c12bb: 80bdae00000008           cmp byte ptr [rbp + 0xae], 8
1409c12c2: 742c                     je 0x1409c12f0
1409c12c4: 488b95b8000000           mov rdx, qword ptr [rbp + 0xb8]
1409c12cb: b8e8000000               mov eax, 0xe8
1409c12d0: 4801c2                   add rdx, rax
1409c12d3: 488d8d88000000           lea rcx, [rbp + 0x88]
1409c12da: 49b802d9ee60b709a1a9     movabs r8, 0xa9a109b760eed902
1409c12e4: 448b8d8c0d0000           mov r9d, dword ptr [rbp + 0xd8c]
1409c12eb: e880a2faff               call 0x14096b570
1409c12f0: 41b8f8000000             mov r8d, 0xf8
1409c12f6: 4c89e1                   mov rcx, r12
1409c12f9: 488d55d0                 lea rdx, [rbp - 0x30]
1409c12fd: e8fe28f601               call 0x142923c00
1409c1302: 488b85300e0000           mov rax, qword ptr [rbp + 0xe30]
1409c1309: 488985b8060000           mov qword ptr [rbp + 0x6b8], rax
1409c1310: c785e00b000002000000     mov dword ptr [rbp + 0xbe0], 2
1409c131a: 48895c2420               mov qword ptr [rsp + 0x20], rbx
1409c131f: 4c89f1                   mov rcx, r14
1409c1322: 4c89e2                   mov rdx, r12
1409c1325: 49b8e36b5a83cd4ca7b8     movabs r8, 0xb8a74ccd835a6be3
1409c132f: 41b11d                   mov r9b, 0x1d
1409c1332: e8c9d4f2ff               call 0x1408ee800
1409c1337: 41b8f8000000             mov r8d, 0xf8
1409c133d: 4889d9                   mov rcx, rbx
1409c1340: 4c89e2                   mov rdx, r12
1409c1343: e8b828f601               call 0x142923c00
1409c1348: 488b85280e0000           mov rax, qword ptr [rbp + 0xe28]
1409c134f: 488985b8060000           mov qword ptr [rbp + 0x6b8], rax
1409c1356: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c1360: 4c89642420               mov qword ptr [rsp + 0x20], r12
1409c1365: 4c89f1                   mov rcx, r14
1409c1368: 4889da                   mov rdx, rbx
1409c136b: 49b83c4e88288a8e20e9     movabs r8, 0xe9208e8a28884e3c
1409c1375: 41b11d                   mov r9b, 0x1d
1409c1378: e883d4f2ff               call 0x1408ee800
1409c137d: 41b8f8000000             mov r8d, 0xf8
1409c1383: 4c89e1                   mov rcx, r12
1409c1386: 4889da                   mov rdx, rbx
1409c1389: e87228f601               call 0x142923c00
1409c138e: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c1395: 4889d9                   mov rcx, rbx
1409c1398: 4c89e2                   mov rdx, r12
1409c139b: 49b829dee8b2ea15342a     movabs r8, 0x2a3415eab2e8de29
1409c13a5: 4c8d8dc00b0000           lea r9, [rbp + 0xbc0]
1409c13ac: e82f54f4ff               call 0x1409067e0
1409c13b1: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c13b8: 4c89f1                   mov rcx, r14
1409c13bb: 4889da                   mov rdx, rbx
1409c13be: 49b852065fb7352090fc     movabs r8, 0xfc902035b75f0652
1409c13c8: 4c8d8d60020000           lea r9, [rbp + 0x260]
1409c13cf: e80c54f4ff               call 0x1409067e0
1409c13d4: 0fb68563070000           movzx eax, byte ptr [rbp + 0x763]
1409c13db: 249f                     and al, 0x9f
1409c13dd: 0c40                     or al, 0x40
1409c13df: 888563070000             mov byte ptr [rbp + 0x763], al
1409c13e5: 488b7770                 mov rsi, qword ptr [rdi + 0x70]
1409c13e9: 41b8f8000000             mov r8d, 0xf8
1409c13ef: 4c89e1                   mov rcx, r12
1409c13f2: 4c89f2                   mov rdx, r14
1409c13f5: e80628f601               call 0x142923c00
1409c13fa: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c1401: 4c89e1                   mov rcx, r12
1409c1404: 4889fa                   mov rdx, rdi
1409c1407: 4c8b85900d0000           mov r8, qword ptr [rbp + 0xd90]
1409c140e: ffd6                     call rsi
1409c1410: 4883ef80                 sub rdi, -0x80
1409c1414: 483bbda00b0000           cmp rdi, qword ptr [rbp + 0xba0]
1409c141b: 0f852ffcffff             jne 0x1409c1050
1409c1421: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c1428: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c142f: 41b8e8030000             mov r8d, 0x3e8
1409c1435: 4889f1                   mov rcx, rsi
1409c1438: e8c327f601               call 0x142923c00
1409c143d: c6859f0d000000           mov byte ptr [rbp + 0xd9f], 0
1409c1444: 4889f1                   mov rcx, rsi
1409c1447: e8246feaff               call 0x140868370
1409c144c: 488b9d780d0000           mov rbx, qword ptr [rbp + 0xd78]
1409c1453: 0f1003                   movups xmm0, xmmword ptr [rbx]
1409c1456: 0f104b10                 movups xmm1, xmmword ptr [rbx + 0x10]
1409c145a: 0f298dc0070000           movaps xmmword ptr [rbp + 0x7c0], xmm1
1409c1461: 0f2985b0070000           movaps xmmword ptr [rbp + 0x7b0], xmm0
1409c1468: 488db5e00b0000           lea rsi, [rbp + 0xbe0]
1409c146f: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c1476: 4889f1                   mov rcx, rsi
1409c1479: e8b2422100               call 0x140bd5730
1409c147e: 4889f1                   mov rcx, rsi
1409c1481: e80a432100               call 0x140bd5790
1409c1486: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c148d: 4889f1                   mov rcx, rsi
1409c1490: 4889c2                   mov rdx, rax
1409c1493: 41b862000000             mov r8d, 0x62
1409c1499: e8023e2100               call 0x140bd52a0
1409c149e: 488d8db8060000           lea rcx, [rbp + 0x6b8]
1409c14a5: 4889f2                   mov rdx, rsi
1409c14a8: e8633f2100               call 0x140bd5410
1409c14ad: 0f1085b8060000           movups xmm0, xmmword ptr [rbp + 0x6b8]
1409c14b4: 0f108dc8060000           movups xmm1, xmmword ptr [rbp + 0x6c8]
1409c14bb: 0f298dc0020000           movaps xmmword ptr [rbp + 0x2c0], xmm1
1409c14c2: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c14c9: 488d8dc0070000           lea rcx, [rbp + 0x7c0]
1409c14d0: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c14d7: e804582100               call 0x140bd6ce0
1409c14dc: 488d05aee80f02           lea rax, [rip + 0x20fe8ae]
1409c14e3: 488985b0070000           mov qword ptr [rbp + 0x7b0], rax
1409c14ea: 48c785b807000017000000   mov qword ptr [rbp + 0x7b8], 0x17
1409c14f5: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c14fc: 488bbd800d0000           mov rdi, qword ptr [rbp + 0xd80]
1409c1503: 4889f9                   mov rcx, rdi
1409c1506: e8c57f2200               call 0x140be94d0
1409c150b: 0f1085b8060000           movups xmm0, xmmword ptr [rbp + 0x6b8]
1409c1512: 0f108dc8060000           movups xmm1, xmmword ptr [rbp + 0x6c8]
1409c1519: 0f1185ec010000           movups xmmword ptr [rbp + 0x1ec], xmm0
1409c1520: 0f118dfc010000           movups xmmword ptr [rbp + 0x1fc], xmm1
1409c1527: 8985e8010000             mov dword ptr [rbp + 0x1e8], eax
1409c152d: 0f1003                   movups xmm0, xmmword ptr [rbx]
1409c1530: 0f104b10                 movups xmm1, xmmword ptr [rbx + 0x10]
1409c1534: 0f298dc0070000           movaps xmmword ptr [rbp + 0x7c0], xmm1
1409c153b: 0f2985b0070000           movaps xmmword ptr [rbp + 0x7b0], xmm0
1409c1542: 488db5e00b0000           lea rsi, [rbp + 0xbe0]
1409c1549: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c1550: 4889f1                   mov rcx, rsi
1409c1553: e8d8412100               call 0x140bd5730
1409c1558: 4889f1                   mov rcx, rsi
1409c155b: e830422100               call 0x140bd5790
1409c1560: 488d8db8060000           lea rcx, [rbp + 0x6b8]
1409c1567: 4889c2                   mov rdx, rax
1409c156a: 41b853000000             mov r8d, 0x53
1409c1570: e82b3d2100               call 0x140bd52a0
1409c1575: 0f1085b8060000           movups xmm0, xmmword ptr [rbp + 0x6b8]
1409c157c: 0f108dc8060000           movups xmm1, xmmword ptr [rbp + 0x6c8]
1409c1583: 0f298dc0020000           movaps xmmword ptr [rbp + 0x2c0], xmm1
1409c158a: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c1591: 488d8dc0070000           lea rcx, [rbp + 0x7c0]
1409c1598: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c159f: e83c572100               call 0x140bd6ce0
1409c15a4: 488d05fde70f02           lea rax, [rip + 0x20fe7fd]
1409c15ab: 488985b0070000           mov qword ptr [rbp + 0x7b0], rax
1409c15b2: 48c785b80700000f000000   mov qword ptr [rbp + 0x7b8], 0xf
1409c15bd: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c15c4: 4889f9                   mov rcx, rdi
1409c15c7: e8047f2200               call 0x140be94d0
1409c15cc: 0f1085b8060000           movups xmm0, xmmword ptr [rbp + 0x6b8]
1409c15d3: 0f108dc8060000           movups xmm1, xmmword ptr [rbp + 0x6c8]
1409c15da: 0f1185c8010000           movups xmmword ptr [rbp + 0x1c8], xmm0
1409c15e1: 0f118dd8010000           movups xmmword ptr [rbp + 0x1d8], xmm1
1409c15e8: 8985c4010000             mov dword ptr [rbp + 0x1c4], eax
1409c15ee: 4c8da5c4010000           lea r12, [rbp + 0x1c4]
1409c15f5: 4889f9                   mov rcx, rdi
1409c15f8: 4c89e2                   mov rdx, r12
1409c15fb: e890c80100               call 0x1409dde90
1409c1600: 48897c2430               mov qword ptr [rsp + 0x30], rdi
1409c1605: 4c8bbd900d0000           mov r15, qword ptr [rbp + 0xd90]
1409c160c: 4c897c2428               mov qword ptr [rsp + 0x28], r15
1409c1611: 48c74424201d000000       mov qword ptr [rsp + 0x20], 0x1d
1409c161a: 488d1596e70f02           lea rdx, [rip + 0x20fe796]
1409c1621: 4c8d0d9de70f02           lea r9, [rip + 0x20fe79d]
1409c1628: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c162f: 41b80e000000             mov r8d, 0xe
1409c1635: 4889f1                   mov rcx, rsi
1409c1638: e8631cc400               call 0x1416032a0
1409c163d: 488dbdb0020000           lea rdi, [rbp + 0x2b0]
1409c1644: 49b829dee8b2ea15342a     movabs r8, 0x2a3415eab2e8de29
1409c164e: 4c8d8dc00b0000           lea r9, [rbp + 0xbc0]
1409c1655: 4889f9                   mov rcx, rdi
1409c1658: 4889f2                   mov rdx, rsi
1409c165b: e860c4faff               call 0x14096dac0
1409c1660: 49b852065fb7352090fc     movabs r8, 0xfc902035b75f0652
1409c166a: 4c8d8d60020000           lea r9, [rbp + 0x260]
1409c1671: 4889f1                   mov rcx, rsi
1409c1674: 4889fa                   mov rdx, rdi
1409c1677: e844c4faff               call 0x14096dac0
1409c167c: 488b85100e0000           mov rax, qword ptr [rbp + 0xe10]
1409c1683: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c168a: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c1694: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c1699: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c16a0: 49b8308202283a3c332a     movabs r8, 0x2a333c3a28028230
1409c16aa: 4889f2                   mov rdx, rsi
1409c16ad: 41b119                   mov r9b, 0x19
1409c16b0: e88bcbf2ff               call 0x1408ee240
1409c16b5: 488db5b0020000           lea rsi, [rbp + 0x2b0]
1409c16bc: 488dbdb0070000           lea rdi, [rbp + 0x7b0]
1409c16c3: 41b890010000             mov r8d, 0x190
1409c16c9: 4889f1                   mov rcx, rsi
1409c16cc: 4889fa                   mov rdx, rdi
1409c16cf: e82c25f601               call 0x142923c00
1409c16d4: 488b85180e0000           mov rax, qword ptr [rbp + 0xe18]
1409c16db: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c16e2: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c16ec: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c16f1: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c16f8: 49b8d0096c8285511704     movabs r8, 0x4175185826c09d0
1409c1702: 4889f2                   mov rdx, rsi
1409c1705: 41b119                   mov r9b, 0x19
1409c1708: e833cbf2ff               call 0x1408ee240
1409c170d: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c1714: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c171b: 41b890010000             mov r8d, 0x190
1409c1721: e8da24f601               call 0x142923c00
1409c1726: 0fb6b5200e0000           movzx esi, byte ptr [rbp + 0xe20]
1409c172d: 80bde608000008           cmp byte ptr [rbp + 0x8e6], 8
1409c1734: 7425                     je 0x1409c175b
1409c1736: 488d8dc0080000           lea rcx, [rbp + 0x8c0]
1409c173d: bae8000000               mov edx, 0xe8
1409c1742: 480395e8080000           add rdx, qword ptr [rbp + 0x8e8]
1409c1749: 49b802d9ee60b709a1a9     movabs r8, 0xa9a109b760eed902
1409c1753: 4189f1                   mov r9d, esi
1409c1756: e8159efaff               call 0x14096b570
1409c175b: 488dbdb0020000           lea rdi, [rbp + 0x2b0]
1409c1762: 488d9db0070000           lea rbx, [rbp + 0x7b0]
1409c1769: 41b890010000             mov r8d, 0x190
1409c176f: 4889f9                   mov rcx, rdi
1409c1772: 4889da                   mov rdx, rbx
1409c1775: e88624f601               call 0x142923c00
1409c177a: 488b85300e0000           mov rax, qword ptr [rbp + 0xe30]
1409c1781: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c1788: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c1792: 48895c2420               mov qword ptr [rsp + 0x20], rbx
1409c1797: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c179e: 49b8e36b5a83cd4ca7b8     movabs r8, 0xb8a74ccd835a6be3
1409c17a8: 4889fa                   mov rdx, rdi
1409c17ab: 41b119                   mov r9b, 0x19
1409c17ae: e88dcaf2ff               call 0x1408ee240
1409c17b3: 488dbdb0070000           lea rdi, [rbp + 0x7b0]
1409c17ba: 488d9db0020000           lea rbx, [rbp + 0x2b0]
1409c17c1: 41b890010000             mov r8d, 0x190
1409c17c7: 4889f9                   mov rcx, rdi
1409c17ca: 4889da                   mov rdx, rbx
1409c17cd: e82e24f601               call 0x142923c00
1409c17d2: 48b86518a091ab399f57     movabs rax, 0x579f39ab91a01865
1409c17dc: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c17e3: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c17ed: 48895c2420               mov qword ptr [rsp + 0x20], rbx
1409c17f2: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c17f9: 49b88c97666903f55b81     movabs r8, 0x815bf5036966978c
1409c1803: 4889fa                   mov rdx, rdi
1409c1806: 41b119                   mov r9b, 0x19
1409c1809: e8c23ef4ff               call 0x1409056d0
1409c180e: 41b890010000             mov r8d, 0x190
1409c1814: 4889d9                   mov rcx, rbx
1409c1817: 4889fa                   mov rdx, rdi
1409c181a: e8e123f601               call 0x142923c00
1409c181f: 488d850c020000           lea rax, [rbp + 0x20c]
1409c1826: 488985b8060000           mov qword ptr [rbp + 0x6b8], rax
1409c182d: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c1837: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c183c: 488d8db8060000           lea rcx, [rbp + 0x6b8]
1409c1843: 49b883b85ae4eb2b9053     movabs r8, 0x53902bebe45ab883
1409c184d: 4889da                   mov rdx, rbx
1409c1850: 41b119                   mov r9b, 0x19
1409c1853: e8e8c9f2ff               call 0x1408ee240
1409c1858: 488d9de00b0000           lea rbx, [rbp + 0xbe0]
1409c185f: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c1866: 41b890010000             mov r8d, 0x190
1409c186c: 4889d9                   mov rcx, rbx
1409c186f: e88c23f601               call 0x142923c00
1409c1874: c685b002000009           mov byte ptr [rbp + 0x2b0], 9
1409c187b: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c1885: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c188a: 488dbdb0020000           lea rdi, [rbp + 0x2b0]
1409c1891: 49b8f2c570045e8bb061     movabs r8, 0x61b08b5e0470c5f2
1409c189b: 4889f9                   mov rcx, rdi
1409c189e: 4889da                   mov rdx, rbx
1409c18a1: 41b119                   mov r9b, 0x19
1409c18a4: e8b739f4ff               call 0x140905260
1409c18a9: 488d9db0070000           lea rbx, [rbp + 0x7b0]
1409c18b0: 488d95e00b0000           lea rdx, [rbp + 0xbe0]
1409c18b7: 41b890010000             mov r8d, 0x190
1409c18bd: 4889d9                   mov rcx, rbx
1409c18c0: e83b23f601               call 0x142923c00
1409c18c5: c685b80600000a           mov byte ptr [rbp + 0x6b8], 0xa
1409c18cc: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c18d6: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c18db: 488d8db8060000           lea rcx, [rbp + 0x6b8]
1409c18e2: 49b857f6e8da4c1e7d29     movabs r8, 0x297d1e4cdae8f657
1409c18ec: 4889da                   mov rdx, rbx
1409c18ef: 41b119                   mov r9b, 0x19
1409c18f2: e86939f4ff               call 0x140905260
1409c18f7: 41b890010000             mov r8d, 0x190
1409c18fd: 4889f9                   mov rcx, rdi
1409c1900: 4889da                   mov rdx, rbx
1409c1903: e8f822f601               call 0x142923c00
1409c1908: c685e00b000008           mov byte ptr [rbp + 0xbe0], 8
1409c190f: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c1919: 48895c2420               mov qword ptr [rsp + 0x20], rbx
1409c191e: 488d9de00b0000           lea rbx, [rbp + 0xbe0]
1409c1925: 49b83fca8dac13011677     movabs r8, 0x77160113ac8dca3f
1409c192f: 4889d9                   mov rcx, rbx
1409c1932: 4889fa                   mov rdx, rdi
1409c1935: 41b119                   mov r9b, 0x19
1409c1938: e82339f4ff               call 0x140905260
1409c193d: 4c8db5b0070000           lea r14, [rbp + 0x7b0]
1409c1944: 41b890010000             mov r8d, 0x190
1409c194a: 4c89f1                   mov rcx, r14
1409c194d: 4889fa                   mov rdx, rdi
1409c1950: e8ab22f601               call 0x142923c00
1409c1955: 48b86518a091ab399f57     movabs rax, 0x579f39ab91a01865
1409c195f: 488985b8060000           mov qword ptr [rbp + 0x6b8], rax
1409c1966: c785e00b000002000000     mov dword ptr [rbp + 0xbe0], 2
1409c1970: 48895c2420               mov qword ptr [rsp + 0x20], rbx
1409c1975: 488d8db8060000           lea rcx, [rbp + 0x6b8]
1409c197c: 49b88c97666903f55b81     movabs r8, 0x815bf5036966978c
1409c1986: 4c89f2                   mov rdx, r14
1409c1989: 41b119                   mov r9b, 0x19
1409c198c: e83f3df4ff               call 0x1409056d0
1409c1991: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c1998: 41b890010000             mov r8d, 0x190
1409c199e: 4889d9                   mov rcx, rbx
1409c19a1: e85a22f601               call 0x142923c00
1409c19a6: 4c89f1                   mov rcx, r14
1409c19a9: 4889da                   mov rdx, rbx
1409c19ac: 4d89f8                   mov r8, r15
1409c19af: e87ca8c300               call 0x1415fc230
1409c19b4: 4c89a5e00b0000           mov qword ptr [rbp + 0xbe0], r12
1409c19bb: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c19c5: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c19ca: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c19d1: 49b8d66b7f6b2701592c     movabs r8, 0x2c5901276b7f6bd6
1409c19db: 4c89f2                   mov rdx, r14
1409c19de: 41b132                   mov r9b, 0x32
1409c19e1: e81ac3f2ff               call 0x1408edd00
1409c19e6: 488d9db0070000           lea rbx, [rbp + 0x7b0]
1409c19ed: 41b890010000             mov r8d, 0x190
1409c19f3: 4889f9                   mov rcx, rdi
1409c19f6: 4889da                   mov rdx, rbx
1409c19f9: e80222f601               call 0x142923c00
1409c19fe: 488d85e8010000           lea rax, [rbp + 0x1e8]
1409c1a05: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c1a0c: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c1a16: 48895c2420               mov qword ptr [rsp + 0x20], rbx
1409c1a1b: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c1a22: 49b842b3aad108f9c9ec     movabs r8, 0xecc9f908d1aab342
1409c1a2c: 4889fa                   mov rdx, rdi
1409c1a2f: 41b132                   mov r9b, 0x32
1409c1a32: e8c9c2f2ff               call 0x1408edd00
1409c1a37: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c1a3e: 41b890010000             mov r8d, 0x190
1409c1a44: 4c89f1                   mov rcx, r14
1409c1a47: e8b421f601               call 0x142923c00
1409c1a4c: 4c8da5b0020000           lea r12, [rbp + 0x2b0]
1409c1a53: 48bf3ee93994882b51f0     movabs rdi, 0xf0512b889439e93e
1409c1a5d: 488d9da0060000           lea rbx, [rbp + 0x6a0]
1409c1a64: 4c89e1                   mov rcx, r12
1409c1a67: 4c89f2                   mov rdx, r14
1409c1a6a: 4989f8                   mov r8, rdi
1409c1a6d: 4989d9                   mov r9, rbx
1409c1a70: e8ebc2faff               call 0x14096dd60
1409c1a75: 4c8dadb0070000           lea r13, [rbp + 0x7b0]
1409c1a7c: 49beab53f2fd6c1d5406     movabs r14, 0x6541d6cfdf253ab
1409c1a86: 4c8dbd60010000           lea r15, [rbp + 0x160]
1409c1a8d: 4c89e9                   mov rcx, r13
1409c1a90: 4c89e2                   mov rdx, r12
1409c1a93: 4d89f0                   mov r8, r14
1409c1a96: 4d89f9                   mov r9, r15
1409c1a99: e8c2c2faff               call 0x14096dd60
1409c1a9e: 488d95c00b0000           lea rdx, [rbp + 0xbc0]
1409c1aa5: 4c89e9                   mov rcx, r13
1409c1aa8: 4531c0                   xor r8d, r8d
1409c1aab: e8d0cdfaff               call 0x14096e880
1409c1ab0: 488b85800d0000           mov rax, qword ptr [rbp + 0xd80]
1409c1ab7: 4889442430               mov qword ptr [rsp + 0x30], rax
1409c1abc: 488b85900d0000           mov rax, qword ptr [rbp + 0xd90]
1409c1ac3: 4889442428               mov qword ptr [rsp + 0x28], rax
1409c1ac8: 48c74424201e000000       mov qword ptr [rsp + 0x20], 0x1e
1409c1ad1: 488d150ae30f02           lea rdx, [rip + 0x20fe30a]
1409c1ad8: 4c8d0d11e30f02           lea r9, [rip + 0x20fe311]
1409c1adf: 4c8da5b0070000           lea r12, [rbp + 0x7b0]
1409c1ae6: 41b80e000000             mov r8d, 0xe
1409c1aec: 4c89e1                   mov rcx, r12
1409c1aef: e8ac17c400               call 0x1416032a0
1409c1af4: 4c8dadb0020000           lea r13, [rbp + 0x2b0]
1409c1afb: 4c89e9                   mov rcx, r13
1409c1afe: 4c89e2                   mov rdx, r12
1409c1b01: 4989f8                   mov r8, rdi
1409c1b04: 4989d9                   mov r9, rbx
1409c1b07: e8b4bffaff               call 0x14096dac0
1409c1b0c: 488dbdb0070000           lea rdi, [rbp + 0x7b0]
1409c1b13: 4889f9                   mov rcx, rdi
1409c1b16: 4c89ea                   mov rdx, r13
1409c1b19: 4d89f0                   mov r8, r14
1409c1b1c: 4d89f9                   mov r9, r15
1409c1b1f: e89cbffaff               call 0x14096dac0
1409c1b24: 488b85100e0000           mov rax, qword ptr [rbp + 0xe10]
1409c1b2b: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c1b32: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c1b3c: 4c896c2420               mov qword ptr [rsp + 0x20], r13
1409c1b41: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c1b48: 49b8308202283a3c332a     movabs r8, 0x2a333c3a28028230
1409c1b52: 4889fa                   mov rdx, rdi
1409c1b55: 41b119                   mov r9b, 0x19
1409c1b58: e8e3c6f2ff               call 0x1408ee240
1409c1b5d: 488dbdb0020000           lea rdi, [rbp + 0x2b0]
1409c1b64: 488d9db0070000           lea rbx, [rbp + 0x7b0]
1409c1b6b: 41b890010000             mov r8d, 0x190
1409c1b71: 4889f9                   mov rcx, rdi
1409c1b74: 4889da                   mov rdx, rbx
1409c1b77: e88420f601               call 0x142923c00
1409c1b7c: 488b85180e0000           mov rax, qword ptr [rbp + 0xe18]
1409c1b83: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c1b8a: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c1b94: 48895c2420               mov qword ptr [rsp + 0x20], rbx
1409c1b99: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c1ba0: 49b8d0096c8285511704     movabs r8, 0x4175185826c09d0
1409c1baa: 4889fa                   mov rdx, rdi
1409c1bad: 41b119                   mov r9b, 0x19
1409c1bb0: e88bc6f2ff               call 0x1408ee240
1409c1bb5: 4c8ba5a80b0000           mov r12, qword ptr [rbp + 0xba8]
1409c1bbc: 4c8badb00b0000           mov r13, qword ptr [rbp + 0xbb0]
1409c1bc3: 4c8bb5780d0000           mov r14, qword ptr [rbp + 0xd78]
1409c1bca: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c1bd1: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c1bd8: 41b890010000             mov r8d, 0x190
1409c1bde: e81d20f601               call 0x142923c00
1409c1be3: 80bde608000008           cmp byte ptr [rbp + 0x8e6], 8
1409c1bea: 4c8bbd800d0000           mov r15, qword ptr [rbp + 0xd80]
1409c1bf1: 7425                     je 0x1409c1c18
1409c1bf3: 488d8dc0080000           lea rcx, [rbp + 0x8c0]
1409c1bfa: bae8000000               mov edx, 0xe8
1409c1bff: 480395e8080000           add rdx, qword ptr [rbp + 0x8e8]
1409c1c06: 49b802d9ee60b709a1a9     movabs r8, 0xa9a109b760eed902
1409c1c10: 4189f1                   mov r9d, esi
1409c1c13: e85899faff               call 0x14096b570
1409c1c18: 488db5b0020000           lea rsi, [rbp + 0x2b0]
1409c1c1f: 488dbdb0070000           lea rdi, [rbp + 0x7b0]
1409c1c26: 41b890010000             mov r8d, 0x190
1409c1c2c: 4889f1                   mov rcx, rsi
1409c1c2f: 4889fa                   mov rdx, rdi
1409c1c32: e8c91ff601               call 0x142923c00
1409c1c37: 488b85300e0000           mov rax, qword ptr [rbp + 0xe30]
1409c1c3e: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c1c45: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c1c4f: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c1c54: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c1c5b: 49b8e36b5a83cd4ca7b8     movabs r8, 0xb8a74ccd835a6be3
1409c1c65: 4889f2                   mov rdx, rsi
1409c1c68: 41b119                   mov r9b, 0x19
1409c1c6b: e8d0c5f2ff               call 0x1408ee240
1409c1c70: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c1c77: 488dbdb0020000           lea rdi, [rbp + 0x2b0]
1409c1c7e: 41b890010000             mov r8d, 0x190
1409c1c84: 4889f1                   mov rcx, rsi
1409c1c87: 4889fa                   mov rdx, rdi
1409c1c8a: e8711ff601               call 0x142923c00
1409c1c8f: 48b86518a091ab399f57     movabs rax, 0x579f39ab91a01865
1409c1c99: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c1ca0: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c1caa: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c1caf: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c1cb6: 49b88c97666903f55b81     movabs r8, 0x815bf5036966978c
1409c1cc0: 4889f2                   mov rdx, rsi
1409c1cc3: 41b119                   mov r9b, 0x19
1409c1cc6: e8053af4ff               call 0x1409056d0
1409c1ccb: 41b890010000             mov r8d, 0x190
1409c1cd1: 4889f9                   mov rcx, rdi
1409c1cd4: 4889f2                   mov rdx, rsi
1409c1cd7: e8241ff601               call 0x142923c00
1409c1cdc: c685e00b000009           mov byte ptr [rbp + 0xbe0], 9
1409c1ce3: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c1ced: 4889742420               mov qword ptr [rsp + 0x20], rsi
1409c1cf2: 488db5e00b0000           lea rsi, [rbp + 0xbe0]
1409c1cf9: 49b8f2c570045e8bb061     movabs r8, 0x61b08b5e0470c5f2
1409c1d03: 4889f1                   mov rcx, rsi
1409c1d06: 4889fa                   mov rdx, rdi
1409c1d09: 41b119                   mov r9b, 0x19
1409c1d0c: e84f35f4ff               call 0x140905260
1409c1d11: 488dbdb0070000           lea rdi, [rbp + 0x7b0]
1409c1d18: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c1d1f: 41b890010000             mov r8d, 0x190
1409c1d25: 4889f9                   mov rcx, rdi
1409c1d28: e8d31ef601               call 0x142923c00
1409c1d2d: c685b80600000a           mov byte ptr [rbp + 0x6b8], 0xa
1409c1d34: c785e00b000002000000     mov dword ptr [rbp + 0xbe0], 2
1409c1d3e: 4889742420               mov qword ptr [rsp + 0x20], rsi
1409c1d43: 488d8db8060000           lea rcx, [rbp + 0x6b8]
1409c1d4a: 49b857f6e8da4c1e7d29     movabs r8, 0x297d1e4cdae8f657
1409c1d54: 4889fa                   mov rdx, rdi
1409c1d57: 41b119                   mov r9b, 0x19
1409c1d5a: e80135f4ff               call 0x140905260
1409c1d5f: 488dbde00b0000           lea rdi, [rbp + 0xbe0]
1409c1d66: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c1d6d: 41b890010000             mov r8d, 0x190
1409c1d73: 4889f9                   mov rcx, rdi
1409c1d76: 4889f2                   mov rdx, rsi
1409c1d79: e8821ef601               call 0x142923c00
1409c1d7e: c685b002000008           mov byte ptr [rbp + 0x2b0], 8
1409c1d85: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c1d8f: 4889742420               mov qword ptr [rsp + 0x20], rsi
1409c1d94: 488db5b0020000           lea rsi, [rbp + 0x2b0]
1409c1d9b: 49b83fca8dac13011677     movabs r8, 0x77160113ac8dca3f
1409c1da5: 4889f1                   mov rcx, rsi
1409c1da8: 4889fa                   mov rdx, rdi
1409c1dab: 41b119                   mov r9b, 0x19
1409c1dae: e8ad34f4ff               call 0x140905260
1409c1db3: 488dbdb0070000           lea rdi, [rbp + 0x7b0]
1409c1dba: 488d95e00b0000           lea rdx, [rbp + 0xbe0]
1409c1dc1: 41b890010000             mov r8d, 0x190
1409c1dc7: 4889f9                   mov rcx, rdi
1409c1dca: e8311ef601               call 0x142923c00
1409c1dcf: 48b86518a091ab399f57     movabs rax, 0x579f39ab91a01865
1409c1dd9: 488985b8060000           mov qword ptr [rbp + 0x6b8], rax
1409c1de0: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c1dea: 4889742420               mov qword ptr [rsp + 0x20], rsi
1409c1def: 488d8db8060000           lea rcx, [rbp + 0x6b8]
1409c1df6: 49b88c97666903f55b81     movabs r8, 0x815bf5036966978c
1409c1e00: 4889fa                   mov rdx, rdi
1409c1e03: 41b119                   mov r9b, 0x19
1409c1e06: e8c538f4ff               call 0x1409056d0
1409c1e0b: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c1e12: 41b890010000             mov r8d, 0x190
1409c1e18: 4889f1                   mov rcx, rsi
1409c1e1b: e8e01df601               call 0x142923c00
1409c1e20: 4889f9                   mov rcx, rdi
1409c1e23: 4889f2                   mov rdx, rsi
1409c1e26: 4c8b85900d0000           mov r8, qword ptr [rbp + 0xd90]
1409c1e2d: e8fea3c300               call 0x1415fc230
1409c1e32: 488d85c4010000           lea rax, [rbp + 0x1c4]
1409c1e39: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c1e40: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c1e4a: 4889742420               mov qword ptr [rsp + 0x20], rsi
1409c1e4f: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c1e56: 49b8d66b7f6b2701592c     movabs r8, 0x2c5901276b7f6bd6
1409c1e60: 4889fa                   mov rdx, rdi
1409c1e63: 41b132                   mov r9b, 0x32
1409c1e66: e895bef2ff               call 0x1408edd00
1409c1e6b: 488dbdb0020000           lea rdi, [rbp + 0x2b0]
1409c1e72: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c1e79: 41b890010000             mov r8d, 0x190
1409c1e7f: 4889f9                   mov rcx, rdi
1409c1e82: 4889f2                   mov rdx, rsi
1409c1e85: e8761df601               call 0x142923c00
1409c1e8a: 488d85e8010000           lea rax, [rbp + 0x1e8]
1409c1e91: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c1e98: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c1ea2: 4889742420               mov qword ptr [rsp + 0x20], rsi
1409c1ea7: 488db5e00b0000           lea rsi, [rbp + 0xbe0]
1409c1eae: 49b842b3aad108f9c9ec     movabs r8, 0xecc9f908d1aab342
1409c1eb8: 4889f1                   mov rcx, rsi
1409c1ebb: 4889fa                   mov rdx, rdi
1409c1ebe: 41b132                   mov r9b, 0x32
1409c1ec1: e83abef2ff               call 0x1408edd00
1409c1ec6: 488dbdb0070000           lea rdi, [rbp + 0x7b0]
1409c1ecd: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c1ed4: 41b890010000             mov r8d, 0x190
1409c1eda: 4889f9                   mov rcx, rdi
1409c1edd: e81e1df601               call 0x142923c00
1409c1ee2: 488d95a0060000           lea rdx, [rbp + 0x6a0]
1409c1ee9: 4889f9                   mov rcx, rdi
1409c1eec: 4531c0                   xor r8d, r8d
1409c1eef: e88cc9faff               call 0x14096e880
1409c1ef4: 4c89f1                   mov rcx, r14
1409c1ef7: e894382100               call 0x140bd5790
1409c1efc: 4889f9                   mov rcx, rdi
1409c1eff: 4889c2                   mov rdx, rax
1409c1f02: 41b87a000000             mov r8d, 0x7a
1409c1f08: e893332100               call 0x140bd52a0
1409c1f0d: 488d9db0020000           lea rbx, [rbp + 0x2b0]
1409c1f14: 4889d9                   mov rcx, rbx
1409c1f17: 4889fa                   mov rdx, rdi
1409c1f1a: e851342100               call 0x140bd5370
1409c1f1f: 4889f1                   mov rcx, rsi
1409c1f22: 4889da                   mov rdx, rbx
1409c1f25: e866342100               call 0x140bd5390
1409c1f2a: 4889742420               mov qword ptr [rsp + 0x20], rsi
1409c1f2f: 4c8d05d8de0f02           lea r8, [rip + 0x20fded8]
1409c1f36: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c1f3d: 41b913000000             mov r9d, 0x13
1409c1f43: 4c89fa                   mov rdx, r15
1409c1f46: e845b5faff               call 0x14096d490
1409c1f4b: 83bdb007000001           cmp dword ptr [rbp + 0x7b0], 1
1409c1f52: 0f84bc0b0000             je 0x1409c2b14
1409c1f58: 488b85f4070000           mov rax, qword ptr [rbp + 0x7f4]
1409c1f5f: 488985f0020000           mov qword ptr [rbp + 0x2f0], rax
1409c1f66: 0f1085b4070000           movups xmm0, xmmword ptr [rbp + 0x7b4]
1409c1f6d: 0f108dc4070000           movups xmm1, xmmword ptr [rbp + 0x7c4]
1409c1f74: 0f1095d4070000           movups xmm2, xmmword ptr [rbp + 0x7d4]
1409c1f7b: 0f109de4070000           movups xmm3, xmmword ptr [rbp + 0x7e4]
1409c1f82: 0f299de0020000           movaps xmmword ptr [rbp + 0x2e0], xmm3
1409c1f89: 0f2995d0020000           movaps xmmword ptr [rbp + 0x2d0], xmm2
1409c1f90: 0f298dc0020000           movaps xmmword ptr [rbp + 0x2c0], xmm1
1409c1f97: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c1f9e: 0fb69dfc070000           movzx ebx, byte ptr [rbp + 0x7fc]
1409c1fa5: 8b85d0020000             mov eax, dword ptr [rbp + 0x2d0]
1409c1fab: 8985f0000000             mov dword ptr [rbp + 0xf0], eax
1409c1fb1: 0f298de0000000           movaps xmmword ptr [rbp + 0xe0], xmm1
1409c1fb8: 0f2985d0000000           movaps xmmword ptr [rbp + 0xd0], xmm0
1409c1fbf: 0f1085e4020000           movups xmm0, xmmword ptr [rbp + 0x2e4]
1409c1fc6: 0f298540020000           movaps xmmword ptr [rbp + 0x240], xmm0
1409c1fcd: 0f1085d4020000           movups xmm0, xmmword ptr [rbp + 0x2d4]
1409c1fd4: 0f298530020000           movaps xmmword ptr [rbp + 0x230], xmm0
1409c1fdb: 8b85f4020000             mov eax, dword ptr [rbp + 0x2f4]
1409c1fe1: 898550020000             mov dword ptr [rbp + 0x250], eax
1409c1fe7: 4c89f1                   mov rcx, r14
1409c1fea: e8a1372100               call 0x140bd5790
1409c1fef: 4c8db5b0070000           lea r14, [rbp + 0x7b0]
1409c1ff6: 4c89f1                   mov rcx, r14
1409c1ff9: 4889c2                   mov rdx, rax
1409c1ffc: 41b853000000             mov r8d, 0x53
1409c2002: e899322100               call 0x140bd52a0
1409c2007: 488dbdb0020000           lea rdi, [rbp + 0x2b0]
1409c200e: 4889f9                   mov rcx, rdi
1409c2011: 4c89f2                   mov rdx, r14
1409c2014: e857332100               call 0x140bd5370
1409c2019: 4889f1                   mov rcx, rsi
1409c201c: 4889fa                   mov rdx, rdi
1409c201f: e86c332100               call 0x140bd5390
1409c2024: 4889742420               mov qword ptr [rsp + 0x20], rsi
1409c2029: 4c8d0510de0f02           lea r8, [rip + 0x20fde10]
1409c2030: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2037: 41b910000000             mov r9d, 0x10
1409c203d: 4c89fa                   mov rdx, r15
1409c2040: e84bb4faff               call 0x14096d490
1409c2045: 83bdb007000001           cmp dword ptr [rbp + 0x7b0], 1
1409c204c: 0f84fd0a0000             je 0x1409c2b4f
1409c2052: 239dfc070000             and ebx, dword ptr [rbp + 0x7fc]
1409c2058: 488b85f4070000           mov rax, qword ptr [rbp + 0x7f4]
1409c205f: 488985f0020000           mov qword ptr [rbp + 0x2f0], rax
1409c2066: 0f1085b4070000           movups xmm0, xmmword ptr [rbp + 0x7b4]
1409c206d: 0f108dc4070000           movups xmm1, xmmword ptr [rbp + 0x7c4]
1409c2074: 0f1095d4070000           movups xmm2, xmmword ptr [rbp + 0x7d4]
1409c207b: 0f109de4070000           movups xmm3, xmmword ptr [rbp + 0x7e4]
1409c2082: 0f299de0020000           movaps xmmword ptr [rbp + 0x2e0], xmm3
1409c2089: 0f2995d0020000           movaps xmmword ptr [rbp + 0x2d0], xmm2
1409c2090: 0f298dc0020000           movaps xmmword ptr [rbp + 0x2c0], xmm1
1409c2097: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c209e: 8b85d0020000             mov eax, dword ptr [rbp + 0x2d0]
1409c20a4: 8985a0020000             mov dword ptr [rbp + 0x2a0], eax
1409c20aa: 0f298d90020000           movaps xmmword ptr [rbp + 0x290], xmm1
1409c20b1: 0f298580020000           movaps xmmword ptr [rbp + 0x280], xmm0
1409c20b8: 0f1085e4020000           movups xmm0, xmmword ptr [rbp + 0x2e4]
1409c20bf: 0f2945e0                 movaps xmmword ptr [rbp - 0x20], xmm0
1409c20c3: 0f1085d4020000           movups xmm0, xmmword ptr [rbp + 0x2d4]
1409c20ca: 0f2945d0                 movaps xmmword ptr [rbp - 0x30], xmm0
1409c20ce: 8b85f4020000             mov eax, dword ptr [rbp + 0x2f4]
1409c20d4: 8945f0                   mov dword ptr [rbp - 0x10], eax
1409c20d7: 4c897c2430               mov qword ptr [rsp + 0x30], r15
1409c20dc: 488b85900d0000           mov rax, qword ptr [rbp + 0xd90]
1409c20e3: 4889442428               mov qword ptr [rsp + 0x28], rax
1409c20e8: 48c74424201f000000       mov qword ptr [rsp + 0x20], 0x1f
1409c20f1: 488d1570dd0f02           lea rdx, [rip + 0x20fdd70]
1409c20f8: 4c8d0d79dd0f02           lea r9, [rip + 0x20fdd79]
1409c20ff: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c2106: 41b810000000             mov r8d, 0x10
1409c210c: 4889f1                   mov rcx, rsi
1409c210f: e88c11c400               call 0x1416032a0
1409c2114: 488d85a0010000           lea rax, [rbp + 0x1a0]
1409c211b: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c2122: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c212c: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c2131: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c2138: 49b8e4bb15d200f82811     movabs r8, 0x1128f800d215bbe4
1409c2142: 4889f2                   mov rdx, rsi
1409c2145: 41b119                   mov r9b, 0x19
1409c2148: e8f3c0f2ff               call 0x1408ee240
1409c214d: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c2154: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c215b: 41b890010000             mov r8d, 0x190
1409c2161: e89a1af601               call 0x142923c00
1409c2166: c6859e0d000001           mov byte ptr [rbp + 0xd9e], 1
1409c216d: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2174: e827362100               call 0x140bd57a0
1409c2179: c6859e0d000001           mov byte ptr [rbp + 0xd9e], 1
1409c2180: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c2187: 4889f2                   mov rdx, rsi
1409c218a: 41b87b000000             mov r8d, 0x7b
1409c2190: e8cb362100               call 0x140bd5860
1409c2195: c6859e0d000001           mov byte ptr [rbp + 0xd9e], 1
1409c219c: 488d8db8060000           lea rcx, [rbp + 0x6b8]
1409c21a3: 488d95e00b0000           lea rdx, [rbp + 0xbe0]
1409c21aa: 41b804000000             mov r8d, 4
1409c21b0: e86b362100               call 0x140bd5820
1409c21b5: 488d857c010000           lea rax, [rbp + 0x17c]
1409c21bc: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c21c3: 0f1085b8060000           movups xmm0, xmmword ptr [rbp + 0x6b8]
1409c21ca: 0f108dc8060000           movups xmm1, xmmword ptr [rbp + 0x6c8]
1409c21d1: 0f1095d8060000           movups xmm2, xmmword ptr [rbp + 0x6d8]
1409c21d8: 0f109de4060000           movups xmm3, xmmword ptr [rbp + 0x6e4]
1409c21df: 0f119ddc070000           movups xmmword ptr [rbp + 0x7dc], xmm3
1409c21e6: 0f2995d0070000           movaps xmmword ptr [rbp + 0x7d0], xmm2
1409c21ed: 0f298dc0070000           movaps xmmword ptr [rbp + 0x7c0], xmm1
1409c21f4: 0f2985b0070000           movaps xmmword ptr [rbp + 0x7b0], xmm0
1409c21fb: 4889742420               mov qword ptr [rsp + 0x20], rsi
1409c2200: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c2207: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c220e: 49b8b4791173079e47de     movabs r8, 0xde479e07731179b4
1409c2218: 41b119                   mov r9b, 0x19
1409c221b: e820c0f2ff               call 0x1408ee240
1409c2220: 41b890010000             mov r8d, 0x190
1409c2226: 4889f1                   mov rcx, rsi
1409c2229: 4889fa                   mov rdx, rdi
1409c222c: e8cf19f601               call 0x142923c00
1409c2231: 4c89ade00b0000           mov qword ptr [rbp + 0xbe0], r13
1409c2238: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c2242: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c2247: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c224e: 49b8fcfb49a09d576148     movabs r8, 0x4861579da049fbfc
1409c2258: 4889f2                   mov rdx, rsi
1409c225b: 41b119                   mov r9b, 0x19
1409c225e: e8ddbff2ff               call 0x1408ee240
1409c2263: 488db5b0020000           lea rsi, [rbp + 0x2b0]
1409c226a: 488dbdb0070000           lea rdi, [rbp + 0x7b0]
1409c2271: 41b890010000             mov r8d, 0x190
1409c2277: 4889f1                   mov rcx, rsi
1409c227a: 4889fa                   mov rdx, rdi
1409c227d: e87e19f601               call 0x142923c00
1409c2282: 4c8dbdd0000000           lea r15, [rbp + 0xd0]
1409c2289: 4c89bde00b0000           mov qword ptr [rbp + 0xbe0], r15
1409c2290: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c229a: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c229f: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c22a6: 49b8ab9cdfc1c760c2b8     movabs r8, 0xb8c260c7c1df9cab
1409c22b0: 4889f2                   mov rdx, rsi
1409c22b3: 41b132                   mov r9b, 0x32
1409c22b6: e845baf2ff               call 0x1408edd00
1409c22bb: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c22c2: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c22c9: 41b890010000             mov r8d, 0x190
1409c22cf: e82c19f601               call 0x142923c00
1409c22d4: 4d89e6                   mov r14, r12
1409c22d7: 49c1ee20                 shr r14, 0x20
1409c22db: 0f108550080000           movups xmm0, xmmword ptr [rbp + 0x850]
1409c22e2: 0f2985200c0000           movaps xmmword ptr [rbp + 0xc20], xmm0
1409c22e9: 0f108540080000           movups xmm0, xmmword ptr [rbp + 0x840]
1409c22f0: 0f2985100c0000           movaps xmmword ptr [rbp + 0xc10], xmm0
1409c22f7: 0f108530080000           movups xmm0, xmmword ptr [rbp + 0x830]
1409c22fe: 0f2985000c0000           movaps xmmword ptr [rbp + 0xc00], xmm0
1409c2305: 0f108520080000           movups xmm0, xmmword ptr [rbp + 0x820]
1409c230c: 0f2985f00b0000           movaps xmmword ptr [rbp + 0xbf0], xmm0
1409c2313: 0f108510080000           movups xmm0, xmmword ptr [rbp + 0x810]
1409c231a: 0f2985e00b0000           movaps xmmword ptr [rbp + 0xbe0], xmm0
1409c2321: 488b85e8080000           mov rax, qword ptr [rbp + 0x8e8]
1409c2328: 488b8d30090000           mov rcx, qword ptr [rbp + 0x930]
1409c232f: 488b95c0070000           mov rdx, qword ptr [rbp + 0x7c0]
1409c2336: 488995c0020000           mov qword ptr [rbp + 0x2c0], rdx
1409c233d: 0f1085b0070000           movups xmm0, xmmword ptr [rbp + 0x7b0]
1409c2344: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c234b: 0f1085c8070000           movups xmm0, xmmword ptr [rbp + 0x7c8]
1409c2352: 0f2985e0020000           movaps xmmword ptr [rbp + 0x2e0], xmm0
1409c2359: 488b95d8070000           mov rdx, qword ptr [rbp + 0x7d8]
1409c2360: 488995f0020000           mov qword ptr [rbp + 0x2f0], rdx
1409c2367: 0f1085e0070000           movups xmm0, xmmword ptr [rbp + 0x7e0]
1409c236e: 0f1185c8020000           movups xmmword ptr [rbp + 0x2c8], xmm0
1409c2375: 488b95f0070000           mov rdx, qword ptr [rbp + 0x7f0]
1409c237c: 488995d8020000           mov qword ptr [rbp + 0x2d8], rdx
1409c2383: 0f108560080000           movups xmm0, xmmword ptr [rbp + 0x860]
1409c238a: 0f108d70080000           movups xmm1, xmmword ptr [rbp + 0x870]
1409c2391: 0f109580080000           movups xmm2, xmmword ptr [rbp + 0x880]
1409c2398: 0f109d90080000           movups xmm3, xmmword ptr [rbp + 0x890]
1409c239f: 0f118528030000           movups xmmword ptr [rbp + 0x328], xmm0
1409c23a6: 0f118d38030000           movups xmmword ptr [rbp + 0x338], xmm1
1409c23ad: 0f119548030000           movups xmmword ptr [rbp + 0x348], xmm2
1409c23b4: 0f119d58030000           movups xmmword ptr [rbp + 0x358], xmm3
1409c23bb: 0f1085a0080000           movups xmm0, xmmword ptr [rbp + 0x8a0]
1409c23c2: 0f118568030000           movups xmmword ptr [rbp + 0x368], xmm0
1409c23c9: 0f1085b0080000           movups xmm0, xmmword ptr [rbp + 0x8b0]
1409c23d0: 0f118578030000           movups xmmword ptr [rbp + 0x378], xmm0
1409c23d7: 488b9508080000           mov rdx, qword ptr [rbp + 0x808]
1409c23de: 48899508030000           mov qword ptr [rbp + 0x308], rdx
1409c23e5: 0f1085f8070000           movups xmm0, xmmword ptr [rbp + 0x7f8]
1409c23ec: 0f1185f8020000           movups xmmword ptr [rbp + 0x2f8], xmm0
1409c23f3: 48898d90030000           mov qword ptr [rbp + 0x390], rcx
1409c23fa: 48898588030000           mov qword ptr [rbp + 0x388], rax
1409c2401: c7851003000001000000     mov dword ptr [rbp + 0x310], 1
1409c240b: 4489a514030000           mov dword ptr [rbp + 0x314], r12d
1409c2412: 4489b518030000           mov dword ptr [rbp + 0x318], r14d
1409c2419: c7851c03000001000000     mov dword ptr [rbp + 0x31c], 1
1409c2423: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c242a: 4889f2                   mov rdx, rsi
1409c242d: e89eb00100               call 0x1409dd4d0
1409c2432: 80bde608000008           cmp byte ptr [rbp + 0x8e6], 8
1409c2439: 740c                     je 0x1409c2447
1409c243b: 488d8dd0080000           lea rcx, [rbp + 0x8d0]
1409c2442: e869002200               call 0x140be24b0
1409c2447: 488b85800d0000           mov rax, qword ptr [rbp + 0xd80]
1409c244e: 4889442430               mov qword ptr [rsp + 0x30], rax
1409c2453: 488b85900d0000           mov rax, qword ptr [rbp + 0xd90]
1409c245a: 4889442428               mov qword ptr [rsp + 0x28], rax
1409c245f: 48c74424201f000000       mov qword ptr [rsp + 0x20], 0x1f
1409c2468: 488d1528da0f02           lea rdx, [rip + 0x20fda28]
1409c246f: 4c8d0d31da0f02           lea r9, [rip + 0x20fda31]
1409c2476: 41b810000000             mov r8d, 0x10
1409c247c: 4889f9                   mov rcx, rdi
1409c247f: e81c0ec400               call 0x1416032a0
1409c2484: c685e00b000000           mov byte ptr [rbp + 0xbe0], 0
1409c248b: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c2495: 4889742420               mov qword ptr [rsp + 0x20], rsi
1409c249a: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c24a1: 49b84541256a8a1a9db7     movabs r8, 0xb79d1a8a6a254145
1409c24ab: 4889fa                   mov rdx, rdi
1409c24ae: 41b119                   mov r9b, 0x19
1409c24b1: e8aa2df4ff               call 0x140905260
1409c24b6: 41b890010000             mov r8d, 0x190
1409c24bc: 4889f1                   mov rcx, rsi
1409c24bf: 4889fa                   mov rdx, rdi
1409c24c2: e83917f601               call 0x142923c00
1409c24c7: 488d85c4010000           lea rax, [rbp + 0x1c4]
1409c24ce: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c24d5: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c24df: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c24e4: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c24eb: 49b8d66b7f6b2701592c     movabs r8, 0x2c5901276b7f6bd6
1409c24f5: 4889f2                   mov rdx, rsi
1409c24f8: 41b119                   mov r9b, 0x19
1409c24fb: e840bdf2ff               call 0x1408ee240
1409c2500: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2507: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c250e: 41b890010000             mov r8d, 0x190
1409c2514: e8e716f601               call 0x142923c00
1409c2519: c6859d0d000001           mov byte ptr [rbp + 0xd9d], 1
1409c2520: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c2527: e874322100               call 0x140bd57a0
1409c252c: c6859d0d000001           mov byte ptr [rbp + 0xd9d], 1
1409c2533: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c253a: 4889f2                   mov rdx, rsi
1409c253d: 41b87b000000             mov r8d, 0x7b
1409c2543: e818332100               call 0x140bd5860
1409c2548: c6859d0d000001           mov byte ptr [rbp + 0xd9d], 1
1409c254f: 488d8db8060000           lea rcx, [rbp + 0x6b8]
1409c2556: 488d95e00b0000           lea rdx, [rbp + 0xbe0]
1409c255d: 41b804000000             mov r8d, 4
1409c2563: e8b8322100               call 0x140bd5820
1409c2568: 488d85e8010000           lea rax, [rbp + 0x1e8]
1409c256f: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c2576: 0f1085b8060000           movups xmm0, xmmword ptr [rbp + 0x6b8]
1409c257d: 0f108dc8060000           movups xmm1, xmmword ptr [rbp + 0x6c8]
1409c2584: 0f1095d8060000           movups xmm2, xmmword ptr [rbp + 0x6d8]
1409c258b: 0f109de4060000           movups xmm3, xmmword ptr [rbp + 0x6e4]
1409c2592: 0f119ddc020000           movups xmmword ptr [rbp + 0x2dc], xmm3
1409c2599: 0f2995d0020000           movaps xmmword ptr [rbp + 0x2d0], xmm2
1409c25a0: 0f298dc0020000           movaps xmmword ptr [rbp + 0x2c0], xmm1
1409c25a7: 0f2985b0020000           movaps xmmword ptr [rbp + 0x2b0], xmm0
1409c25ae: 4889742420               mov qword ptr [rsp + 0x20], rsi
1409c25b3: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c25ba: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c25c1: 49b842b3aad108f9c9ec     movabs r8, 0xecc9f908d1aab342
1409c25cb: 41b119                   mov r9b, 0x19
1409c25ce: e86dbcf2ff               call 0x1408ee240
1409c25d3: 41b890010000             mov r8d, 0x190
1409c25d9: 4889f1                   mov rcx, rsi
1409c25dc: 4889fa                   mov rdx, rdi
1409c25df: e81c16f601               call 0x142923c00
1409c25e4: 4c89ade00b0000           mov qword ptr [rbp + 0xbe0], r13
1409c25eb: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c25f5: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c25fa: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c2601: 49b8fcfb49a09d576148     movabs r8, 0x4861579da049fbfc
1409c260b: 4889f2                   mov rdx, rsi
1409c260e: 41b119                   mov r9b, 0x19
1409c2611: e82abcf2ff               call 0x1408ee240
1409c2616: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c261d: 488dbdb0020000           lea rdi, [rbp + 0x2b0]
1409c2624: 41b890010000             mov r8d, 0x190
1409c262a: 4889f1                   mov rcx, rsi
1409c262d: 4889fa                   mov rdx, rdi
1409c2630: e8cb15f601               call 0x142923c00
1409c2635: 48b86518a091ab399f57     movabs rax, 0x579f39ab91a01865
1409c263f: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c2646: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c2650: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c2655: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c265c: 49b88c97666903f55b81     movabs r8, 0x815bf5036966978c
1409c2666: 4889f2                   mov rdx, rsi
1409c2669: 41b119                   mov r9b, 0x19
1409c266c: e85f30f4ff               call 0x1409056d0
1409c2671: 488db5b0020000           lea rsi, [rbp + 0x2b0]
1409c2678: 488dbdb0070000           lea rdi, [rbp + 0x7b0]
1409c267f: 41b890010000             mov r8d, 0x190
1409c2685: 4889f1                   mov rcx, rsi
1409c2688: 4889fa                   mov rdx, rdi
1409c268b: e87015f601               call 0x142923c00
1409c2690: 488b85300e0000           mov rax, qword ptr [rbp + 0xe30]
1409c2697: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c269e: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c26a8: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c26ad: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c26b4: 49b8e36b5a83cd4ca7b8     movabs r8, 0xb8a74ccd835a6be3
1409c26be: 4889f2                   mov rdx, rsi
1409c26c1: 41b119                   mov r9b, 0x19
1409c26c4: e877bbf2ff               call 0x1408ee240
1409c26c9: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c26d0: 488dbdb0020000           lea rdi, [rbp + 0x2b0]
1409c26d7: 41b890010000             mov r8d, 0x190
1409c26dd: 4889f1                   mov rcx, rsi
1409c26e0: 4889fa                   mov rdx, rdi
1409c26e3: e81815f601               call 0x142923c00
1409c26e8: 488d850c020000           lea rax, [rbp + 0x20c]
1409c26ef: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c26f6: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c2700: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c2705: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c270c: 49b883b85ae4eb2b9053     movabs r8, 0x53902bebe45ab883
1409c2716: 4889f2                   mov rdx, rsi
1409c2719: 41b119                   mov r9b, 0x19
1409c271c: e81fbbf2ff               call 0x1408ee240
1409c2721: 488db5b0020000           lea rsi, [rbp + 0x2b0]
1409c2728: 488dbdb0070000           lea rdi, [rbp + 0x7b0]
1409c272f: 41b890010000             mov r8d, 0x190
1409c2735: 4889f1                   mov rcx, rsi
1409c2738: 4889fa                   mov rdx, rdi
1409c273b: e8c014f601               call 0x142923c00
1409c2740: 488b85b80b0000           mov rax, qword ptr [rbp + 0xbb8]
1409c2747: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c274e: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c2758: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c275d: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c2764: 49b85ea9e91c548e97d9     movabs r8, 0xd9978e541ce9a95e
1409c276e: 4889f2                   mov rdx, rsi
1409c2771: 41b132                   mov r9b, 0x32
1409c2774: e887b5f2ff               call 0x1408edd00
1409c2779: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2780: 488d95b0020000           lea rdx, [rbp + 0x2b0]
1409c2787: 41b890010000             mov r8d, 0x190
1409c278d: e86e14f601               call 0x142923c00
1409c2792: 80bde608000008           cmp byte ptr [rbp + 0x8e6], 8
1409c2799: 7428                     je 0x1409c27c3
1409c279b: 83e301                   and ebx, 1
1409c279e: 488d8dc0080000           lea rcx, [rbp + 0x8c0]
1409c27a5: bae8000000               mov edx, 0xe8
1409c27aa: 480395e8080000           add rdx, qword ptr [rbp + 0x8e8]
1409c27b1: 49b802d9ee60b709a1a9     movabs r8, 0xa9a109b760eed902
1409c27bb: 4189d9                   mov r9d, ebx
1409c27be: e8ad8dfaff               call 0x14096b570
1409c27c3: 41b890010000             mov r8d, 0x190
1409c27c9: 4889f1                   mov rcx, rsi
1409c27cc: 4889fa                   mov rdx, rdi
1409c27cf: e82c14f601               call 0x142923c00
1409c27d4: 488d8530020000           lea rax, [rbp + 0x230]
1409c27db: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c27e2: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c27ec: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c27f1: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c27f8: 49b893d23cb84f9759f4     movabs r8, 0xf459974fb83cd293
1409c2802: 4889f2                   mov rdx, rsi
1409c2805: 41b119                   mov r9b, 0x19
1409c2808: e883c8faff               call 0x14096f090
1409c280d: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c2814: 488dbdb0020000           lea rdi, [rbp + 0x2b0]
1409c281b: 41b890010000             mov r8d, 0x190
1409c2821: 4889f1                   mov rcx, rsi
1409c2824: 4889fa                   mov rdx, rdi
1409c2827: e8d413f601               call 0x142923c00
1409c282c: 4c89bde00b0000           mov qword ptr [rbp + 0xbe0], r15
1409c2833: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c283d: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c2842: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c2849: 49b8ab9cdfc1c760c2b8     movabs r8, 0xb8c260c7c1df9cab
1409c2853: 4889f2                   mov rdx, rsi
1409c2856: 41b132                   mov r9b, 0x32
1409c2859: e8a2b4f2ff               call 0x1408edd00
1409c285e: 488db5b0020000           lea rsi, [rbp + 0x2b0]
1409c2865: 488dbdb0070000           lea rdi, [rbp + 0x7b0]
1409c286c: 41b890010000             mov r8d, 0x190
1409c2872: 4889f1                   mov rcx, rsi
1409c2875: 4889fa                   mov rdx, rdi
1409c2878: e88313f601               call 0x142923c00
1409c287d: 488d45d0                 lea rax, [rbp - 0x30]
1409c2881: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c2888: c785b007000002000000     mov dword ptr [rbp + 0x7b0], 2
1409c2892: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c2897: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c289e: 49b816402f5ab7bdd769     movabs r8, 0x69d7bdb75a2f4016
1409c28a8: 4889f2                   mov rdx, rsi
1409c28ab: 41b119                   mov r9b, 0x19
1409c28ae: e8ddc7faff               call 0x14096f090
1409c28b3: 488db5b0070000           lea rsi, [rbp + 0x7b0]
1409c28ba: 488dbdb0020000           lea rdi, [rbp + 0x2b0]
1409c28c1: 41b890010000             mov r8d, 0x190
1409c28c7: 4889f1                   mov rcx, rsi
1409c28ca: 4889fa                   mov rdx, rdi
1409c28cd: e82e13f601               call 0x142923c00
1409c28d2: 488d8580020000           lea rax, [rbp + 0x280]
1409c28d9: 488985e00b0000           mov qword ptr [rbp + 0xbe0], rax
1409c28e0: c785b002000002000000     mov dword ptr [rbp + 0x2b0], 2
1409c28ea: 48897c2420               mov qword ptr [rsp + 0x20], rdi
1409c28ef: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c28f6: 49b8fae8ea543da4c796     movabs r8, 0x96c7a43d54eae8fa
1409c2900: 4889f2                   mov rdx, rsi
1409c2903: 41b132                   mov r9b, 0x32
1409c2906: e8f5b3f2ff               call 0x1408edd00
1409c290b: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c2912: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c2919: 41b890010000             mov r8d, 0x190
1409c291f: e8dc12f601               call 0x142923c00
1409c2924: 0f108550030000           movups xmm0, xmmword ptr [rbp + 0x350]
1409c292b: 0f2985200c0000           movaps xmmword ptr [rbp + 0xc20], xmm0
1409c2932: 0f108540030000           movups xmm0, xmmword ptr [rbp + 0x340]
1409c2939: 0f2985100c0000           movaps xmmword ptr [rbp + 0xc10], xmm0
1409c2940: 0f108530030000           movups xmm0, xmmword ptr [rbp + 0x330]
1409c2947: 0f2985000c0000           movaps xmmword ptr [rbp + 0xc00], xmm0
1409c294e: 0f108520030000           movups xmm0, xmmword ptr [rbp + 0x320]
1409c2955: 0f2985f00b0000           movaps xmmword ptr [rbp + 0xbf0], xmm0
1409c295c: 0f108510030000           movups xmm0, xmmword ptr [rbp + 0x310]
1409c2963: 0f2985e00b0000           movaps xmmword ptr [rbp + 0xbe0], xmm0
1409c296a: 488b85e8030000           mov rax, qword ptr [rbp + 0x3e8]
1409c2971: 488b8d30040000           mov rcx, qword ptr [rbp + 0x430]
1409c2978: 488b95c0020000           mov rdx, qword ptr [rbp + 0x2c0]
1409c297f: 488995c0070000           mov qword ptr [rbp + 0x7c0], rdx
1409c2986: 0f1085b0020000           movups xmm0, xmmword ptr [rbp + 0x2b0]
1409c298d: 0f2985b0070000           movaps xmmword ptr [rbp + 0x7b0], xmm0
1409c2994: 0f1085c8020000           movups xmm0, xmmword ptr [rbp + 0x2c8]
1409c299b: 0f2985e0070000           movaps xmmword ptr [rbp + 0x7e0], xmm0
1409c29a2: 488b95d8020000           mov rdx, qword ptr [rbp + 0x2d8]
1409c29a9: 488995f0070000           mov qword ptr [rbp + 0x7f0], rdx
1409c29b0: 0f1085e0020000           movups xmm0, xmmword ptr [rbp + 0x2e0]
1409c29b7: 0f1185c8070000           movups xmmword ptr [rbp + 0x7c8], xmm0
1409c29be: 488b95f0020000           mov rdx, qword ptr [rbp + 0x2f0]
1409c29c5: 488995d8070000           mov qword ptr [rbp + 0x7d8], rdx
1409c29cc: 0f108560030000           movups xmm0, xmmword ptr [rbp + 0x360]
1409c29d3: 0f108d70030000           movups xmm1, xmmword ptr [rbp + 0x370]
1409c29da: 0f109580030000           movups xmm2, xmmword ptr [rbp + 0x380]
1409c29e1: 0f109d90030000           movups xmm3, xmmword ptr [rbp + 0x390]
1409c29e8: 0f118528080000           movups xmmword ptr [rbp + 0x828], xmm0
1409c29ef: 0f118d38080000           movups xmmword ptr [rbp + 0x838], xmm1
1409c29f6: 0f119548080000           movups xmmword ptr [rbp + 0x848], xmm2
1409c29fd: 0f119d58080000           movups xmmword ptr [rbp + 0x858], xmm3
1409c2a04: 0f1085a0030000           movups xmm0, xmmword ptr [rbp + 0x3a0]
1409c2a0b: 0f118568080000           movups xmmword ptr [rbp + 0x868], xmm0
1409c2a12: 0f1085b0030000           movups xmm0, xmmword ptr [rbp + 0x3b0]
1409c2a19: 0f118578080000           movups xmmword ptr [rbp + 0x878], xmm0
1409c2a20: 488b9508030000           mov rdx, qword ptr [rbp + 0x308]
1409c2a27: 48899508080000           mov qword ptr [rbp + 0x808], rdx
1409c2a2e: 0f1085f8020000           movups xmm0, xmmword ptr [rbp + 0x2f8]
1409c2a35: 0f1185f8070000           movups xmmword ptr [rbp + 0x7f8], xmm0
1409c2a3c: 48898d90080000           mov qword ptr [rbp + 0x890], rcx
1409c2a43: 48898588080000           mov qword ptr [rbp + 0x888], rax
1409c2a4a: c7851008000001000000     mov dword ptr [rbp + 0x810], 1
1409c2a54: 4489a514080000           mov dword ptr [rbp + 0x814], r12d
1409c2a5b: 4489b518080000           mov dword ptr [rbp + 0x818], r14d
1409c2a62: c7851c08000001000000     mov dword ptr [rbp + 0x81c], 1
1409c2a6c: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c2a73: 488d95b0070000           lea rdx, [rbp + 0x7b0]
1409c2a7a: e851aa0100               call 0x1409dd4d0
1409c2a7f: 80bde603000008           cmp byte ptr [rbp + 0x3e6], 8
1409c2a86: 740c                     je 0x1409c2a94
1409c2a88: 488d8dd0030000           lea rcx, [rbp + 0x3d0]
1409c2a8f: e81cfa2100               call 0x140be24b0
1409c2a94: 90                       nop 
1409c2a95: 4881c4280e0000           add rsp, 0xe28
1409c2a9c: 5b                       pop rbx
1409c2a9d: 5f                       pop rdi
1409c2a9e: 5e                       pop rsi
1409c2a9f: 415c                     pop r12
1409c2aa1: 415d                     pop r13
1409c2aa3: 415e                     pop r14
1409c2aa5: 415f                     pop r15
1409c2aa7: 5d                       pop rbp
1409c2aa8: c3                       ret 
1409c2aa9: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c2ab0: 488d0de97f2702           lea rcx, [rip + 0x2277fe9]
1409c2ab7: e81456f101               call 0x1428d80d0
1409c2abc: e9c7000000               jmp 0x1409c2b88
1409c2ac1: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c2ac8: 488d0d09310e02           lea rcx, [rip + 0x20e3109]
1409c2acf: e83c68f101               call 0x1428d9310
1409c2ad4: e9af000000               jmp 0x1409c2b88
1409c2ad9: 488b85b8070000           mov rax, qword ptr [rbp + 0x7b8]
1409c2ae0: 488985b0020000           mov qword ptr [rbp + 0x2b0], rax
1409c2ae7: 488d0582d20f02           lea rax, [rip + 0x20fd282]
1409c2aee: 4889442420               mov qword ptr [rsp + 0x20], rax
1409c2af3: 488d0dd6ce0f02           lea rcx, [rip + 0x20fced6]
1409c2afa: 4c8d0dafce0f02           lea r9, [rip + 0x20fceaf]
1409c2b01: 4c8d85b0020000           lea r8, [rbp + 0x2b0]
1409c2b08: ba2b000000               mov edx, 0x2b
1409c2b0d: e84e6cf101               call 0x1428d9760
1409c2b12: eb74                     jmp 0x1409c2b88
1409c2b14: 488b85b8070000           mov rax, qword ptr [rbp + 0x7b8]
1409c2b1b: 488985b0020000           mov qword ptr [rbp + 0x2b0], rax
1409c2b22: 488d05ffd20f02           lea rax, [rip + 0x20fd2ff]
1409c2b29: 4889442420               mov qword ptr [rsp + 0x20], rax
1409c2b2e: 488d0d9bce0f02           lea rcx, [rip + 0x20fce9b]
1409c2b35: 4c8d0d74ce0f02           lea r9, [rip + 0x20fce74]
1409c2b3c: 4c8d85b0020000           lea r8, [rbp + 0x2b0]
1409c2b43: ba2b000000               mov edx, 0x2b
1409c2b48: e8136cf101               call 0x1428d9760
1409c2b4d: eb39                     jmp 0x1409c2b88
1409c2b4f: 488b85b8070000           mov rax, qword ptr [rbp + 0x7b8]
1409c2b56: 488985b0020000           mov qword ptr [rbp + 0x2b0], rax
1409c2b5d: 488d05ecd20f02           lea rax, [rip + 0x20fd2ec]
1409c2b64: 4889442420               mov qword ptr [rsp + 0x20], rax
1409c2b69: 488d0d60ce0f02           lea rcx, [rip + 0x20fce60]
1409c2b70: 4c8d0d39ce0f02           lea r9, [rip + 0x20fce39]
1409c2b77: 4c8d85b0020000           lea r8, [rbp + 0x2b0]
1409c2b7e: ba2b000000               mov edx, 0x2b
1409c2b83: e8d86bf101               call 0x1428d9760
1409c2b88: 0f0b                     ud2 
1409c2b8a: 660f1f440000             nop word ptr [rax + rax]
1409c2b90: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2b95: 55                       push rbp
1409c2b96: 4157                     push r15
1409c2b98: 4156                     push r14
1409c2b9a: 4155                     push r13
1409c2b9c: 4154                     push r12
1409c2b9e: 56                       push rsi
1409c2b9f: 57                       push rdi
1409c2ba0: 53                       push rbx
1409c2ba1: 4883ec48                 sub rsp, 0x48
1409c2ba5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2bac: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2bb3: e8f843edff               call 0x140896fb0
1409c2bb8: 90                       nop 
1409c2bb9: 4883c448                 add rsp, 0x48
1409c2bbd: 5b                       pop rbx
1409c2bbe: 5f                       pop rdi
1409c2bbf: 5e                       pop rsi
1409c2bc0: 415c                     pop r12
1409c2bc2: 415d                     pop r13
1409c2bc4: 415e                     pop r14
1409c2bc6: 415f                     pop r15
1409c2bc8: 5d                       pop rbp
1409c2bc9: c3                       ret 
1409c2bca: 660f1f440000             nop word ptr [rax + rax]
1409c2bd0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2bd5: 55                       push rbp
1409c2bd6: 4157                     push r15
1409c2bd8: 4156                     push r14
1409c2bda: 4155                     push r13
1409c2bdc: 4154                     push r12
1409c2bde: 56                       push rsi
1409c2bdf: 57                       push rdi
1409c2be0: 53                       push rbx
1409c2be1: 4883ec48                 sub rsp, 0x48
1409c2be5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2bec: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c2bf3: e8b843edff               call 0x140896fb0
1409c2bf8: 90                       nop 
1409c2bf9: 4883c448                 add rsp, 0x48
1409c2bfd: 5b                       pop rbx
1409c2bfe: 5f                       pop rdi
1409c2bff: 5e                       pop rsi
1409c2c00: 415c                     pop r12
1409c2c02: 415d                     pop r13
1409c2c04: 415e                     pop r14
1409c2c06: 415f                     pop r15
1409c2c08: 5d                       pop rbp
1409c2c09: c3                       ret 
1409c2c0a: 660f1f440000             nop word ptr [rax + rax]
1409c2c10: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2c15: 55                       push rbp
1409c2c16: 4157                     push r15
1409c2c18: 4156                     push r14
1409c2c1a: 4155                     push r13
1409c2c1c: 4154                     push r12
1409c2c1e: 56                       push rsi
1409c2c1f: 57                       push rdi
1409c2c20: 53                       push rbx
1409c2c21: 4883ec48                 sub rsp, 0x48
1409c2c25: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2c2c: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2c33: e87843edff               call 0x140896fb0
1409c2c38: 90                       nop 
1409c2c39: 4883c448                 add rsp, 0x48
1409c2c3d: 5b                       pop rbx
1409c2c3e: 5f                       pop rdi
1409c2c3f: 5e                       pop rsi
1409c2c40: 415c                     pop r12
1409c2c42: 415d                     pop r13
1409c2c44: 415e                     pop r14
1409c2c46: 415f                     pop r15
1409c2c48: 5d                       pop rbp
1409c2c49: c3                       ret 
1409c2c4a: 660f1f440000             nop word ptr [rax + rax]
1409c2c50: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2c55: 55                       push rbp
1409c2c56: 4157                     push r15
1409c2c58: 4156                     push r14
1409c2c5a: 4155                     push r13
1409c2c5c: 4154                     push r12
1409c2c5e: 56                       push rsi
1409c2c5f: 57                       push rdi
1409c2c60: 53                       push rbx
1409c2c61: 4883ec48                 sub rsp, 0x48
1409c2c65: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2c6c: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c2c73: e83843edff               call 0x140896fb0
1409c2c78: 90                       nop 
1409c2c79: 4883c448                 add rsp, 0x48
1409c2c7d: 5b                       pop rbx
1409c2c7e: 5f                       pop rdi
1409c2c7f: 5e                       pop rsi
1409c2c80: 415c                     pop r12
1409c2c82: 415d                     pop r13
1409c2c84: 415e                     pop r14
1409c2c86: 415f                     pop r15
1409c2c88: 5d                       pop rbp
1409c2c89: c3                       ret 
1409c2c8a: 660f1f440000             nop word ptr [rax + rax]
1409c2c90: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2c95: 55                       push rbp
1409c2c96: 4157                     push r15
1409c2c98: 4156                     push r14
1409c2c9a: 4155                     push r13
1409c2c9c: 4154                     push r12
1409c2c9e: 56                       push rsi
1409c2c9f: 57                       push rdi
1409c2ca0: 53                       push rbx
1409c2ca1: 4883ec48                 sub rsp, 0x48
1409c2ca5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2cac: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2cb3: e8f842edff               call 0x140896fb0
1409c2cb8: 90                       nop 
1409c2cb9: 4883c448                 add rsp, 0x48
1409c2cbd: 5b                       pop rbx
1409c2cbe: 5f                       pop rdi
1409c2cbf: 5e                       pop rsi
1409c2cc0: 415c                     pop r12
1409c2cc2: 415d                     pop r13
1409c2cc4: 415e                     pop r14
1409c2cc6: 415f                     pop r15
1409c2cc8: 5d                       pop rbp
1409c2cc9: c3                       ret 
1409c2cca: 660f1f440000             nop word ptr [rax + rax]
1409c2cd0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2cd5: 55                       push rbp
1409c2cd6: 4157                     push r15
1409c2cd8: 4156                     push r14
1409c2cda: 4155                     push r13
1409c2cdc: 4154                     push r12
1409c2cde: 56                       push rsi
1409c2cdf: 57                       push rdi
1409c2ce0: 53                       push rbx
1409c2ce1: 4883ec48                 sub rsp, 0x48
1409c2ce5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2cec: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c2cf3: e8b842edff               call 0x140896fb0
1409c2cf8: 90                       nop 
1409c2cf9: 4883c448                 add rsp, 0x48
1409c2cfd: 5b                       pop rbx
1409c2cfe: 5f                       pop rdi
1409c2cff: 5e                       pop rsi
1409c2d00: 415c                     pop r12
1409c2d02: 415d                     pop r13
1409c2d04: 415e                     pop r14
1409c2d06: 415f                     pop r15
1409c2d08: 5d                       pop rbp
1409c2d09: c3                       ret 
1409c2d0a: 660f1f440000             nop word ptr [rax + rax]
1409c2d10: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2d15: 55                       push rbp
1409c2d16: 4157                     push r15
1409c2d18: 4156                     push r14
1409c2d1a: 4155                     push r13
1409c2d1c: 4154                     push r12
1409c2d1e: 56                       push rsi
1409c2d1f: 57                       push rdi
1409c2d20: 53                       push rbx
1409c2d21: 4883ec48                 sub rsp, 0x48
1409c2d25: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2d2c: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c2d33: e87842edff               call 0x140896fb0
1409c2d38: 90                       nop 
1409c2d39: 4883c448                 add rsp, 0x48
1409c2d3d: 5b                       pop rbx
1409c2d3e: 5f                       pop rdi
1409c2d3f: 5e                       pop rsi
1409c2d40: 415c                     pop r12
1409c2d42: 415d                     pop r13
1409c2d44: 415e                     pop r14
1409c2d46: 415f                     pop r15
1409c2d48: 5d                       pop rbp
1409c2d49: c3                       ret 
1409c2d4a: 660f1f440000             nop word ptr [rax + rax]
1409c2d50: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2d55: 55                       push rbp
1409c2d56: 4157                     push r15
1409c2d58: 4156                     push r14
1409c2d5a: 4155                     push r13
1409c2d5c: 4154                     push r12
1409c2d5e: 56                       push rsi
1409c2d5f: 57                       push rdi
1409c2d60: 53                       push rbx
1409c2d61: 4883ec48                 sub rsp, 0x48
1409c2d65: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2d6c: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2d73: e83842edff               call 0x140896fb0
1409c2d78: 90                       nop 
1409c2d79: 4883c448                 add rsp, 0x48
1409c2d7d: 5b                       pop rbx
1409c2d7e: 5f                       pop rdi
1409c2d7f: 5e                       pop rsi
1409c2d80: 415c                     pop r12
1409c2d82: 415d                     pop r13
1409c2d84: 415e                     pop r14
1409c2d86: 415f                     pop r15
1409c2d88: 5d                       pop rbp
1409c2d89: c3                       ret 
1409c2d8a: 660f1f440000             nop word ptr [rax + rax]
1409c2d90: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2d95: 55                       push rbp
1409c2d96: 4157                     push r15
1409c2d98: 4156                     push r14
1409c2d9a: 4155                     push r13
1409c2d9c: 4154                     push r12
1409c2d9e: 56                       push rsi
1409c2d9f: 57                       push rdi
1409c2da0: 53                       push rbx
1409c2da1: 4883ec48                 sub rsp, 0x48
1409c2da5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2dac: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c2db3: e8f841edff               call 0x140896fb0
1409c2db8: 90                       nop 
1409c2db9: 4883c448                 add rsp, 0x48
1409c2dbd: 5b                       pop rbx
1409c2dbe: 5f                       pop rdi
1409c2dbf: 5e                       pop rsi
1409c2dc0: 415c                     pop r12
1409c2dc2: 415d                     pop r13
1409c2dc4: 415e                     pop r14
1409c2dc6: 415f                     pop r15
1409c2dc8: 5d                       pop rbp
1409c2dc9: c3                       ret 
1409c2dca: 660f1f440000             nop word ptr [rax + rax]
1409c2dd0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2dd5: 55                       push rbp
1409c2dd6: 4157                     push r15
1409c2dd8: 4156                     push r14
1409c2dda: 4155                     push r13
1409c2ddc: 4154                     push r12
1409c2dde: 56                       push rsi
1409c2ddf: 57                       push rdi
1409c2de0: 53                       push rbx
1409c2de1: 4883ec48                 sub rsp, 0x48
1409c2de5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2dec: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2df3: e8b841edff               call 0x140896fb0
1409c2df8: 90                       nop 
1409c2df9: 4883c448                 add rsp, 0x48
1409c2dfd: 5b                       pop rbx
1409c2dfe: 5f                       pop rdi
1409c2dff: 5e                       pop rsi
1409c2e00: 415c                     pop r12
1409c2e02: 415d                     pop r13
1409c2e04: 415e                     pop r14
1409c2e06: 415f                     pop r15
1409c2e08: 5d                       pop rbp
1409c2e09: c3                       ret 
1409c2e0a: 660f1f440000             nop word ptr [rax + rax]
1409c2e10: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2e15: 55                       push rbp
1409c2e16: 4157                     push r15
1409c2e18: 4156                     push r14
1409c2e1a: 4155                     push r13
1409c2e1c: 4154                     push r12
1409c2e1e: 56                       push rsi
1409c2e1f: 57                       push rdi
1409c2e20: 53                       push rbx
1409c2e21: 4883ec48                 sub rsp, 0x48
1409c2e25: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2e2c: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2e33: e87841edff               call 0x140896fb0
1409c2e38: 90                       nop 
1409c2e39: 4883c448                 add rsp, 0x48
1409c2e3d: 5b                       pop rbx
1409c2e3e: 5f                       pop rdi
1409c2e3f: 5e                       pop rsi
1409c2e40: 415c                     pop r12
1409c2e42: 415d                     pop r13
1409c2e44: 415e                     pop r14
1409c2e46: 415f                     pop r15
1409c2e48: 5d                       pop rbp
1409c2e49: c3                       ret 
1409c2e4a: 660f1f440000             nop word ptr [rax + rax]
1409c2e50: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2e55: 55                       push rbp
1409c2e56: 4157                     push r15
1409c2e58: 4156                     push r14
1409c2e5a: 4155                     push r13
1409c2e5c: 4154                     push r12
1409c2e5e: 56                       push rsi
1409c2e5f: 57                       push rdi
1409c2e60: 53                       push rbx
1409c2e61: 4883ec48                 sub rsp, 0x48
1409c2e65: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2e6c: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c2e73: e83841edff               call 0x140896fb0
1409c2e78: 90                       nop 
1409c2e79: 4883c448                 add rsp, 0x48
1409c2e7d: 5b                       pop rbx
1409c2e7e: 5f                       pop rdi
1409c2e7f: 5e                       pop rsi
1409c2e80: 415c                     pop r12
1409c2e82: 415d                     pop r13
1409c2e84: 415e                     pop r14
1409c2e86: 415f                     pop r15
1409c2e88: 5d                       pop rbp
1409c2e89: c3                       ret 
1409c2e8a: 660f1f440000             nop word ptr [rax + rax]
1409c2e90: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2e95: 55                       push rbp
1409c2e96: 4157                     push r15
1409c2e98: 4156                     push r14
1409c2e9a: 4155                     push r13
1409c2e9c: 4154                     push r12
1409c2e9e: 56                       push rsi
1409c2e9f: 57                       push rdi
1409c2ea0: 53                       push rbx
1409c2ea1: 4883ec48                 sub rsp, 0x48
1409c2ea5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2eac: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2eb3: e8f840edff               call 0x140896fb0
1409c2eb8: 90                       nop 
1409c2eb9: 4883c448                 add rsp, 0x48
1409c2ebd: 5b                       pop rbx
1409c2ebe: 5f                       pop rdi
1409c2ebf: 5e                       pop rsi
1409c2ec0: 415c                     pop r12
1409c2ec2: 415d                     pop r13
1409c2ec4: 415e                     pop r14
1409c2ec6: 415f                     pop r15
1409c2ec8: 5d                       pop rbp
1409c2ec9: c3                       ret 
1409c2eca: 660f1f440000             nop word ptr [rax + rax]
1409c2ed0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2ed5: 55                       push rbp
1409c2ed6: 4157                     push r15
1409c2ed8: 4156                     push r14
1409c2eda: 4155                     push r13
1409c2edc: 4154                     push r12
1409c2ede: 56                       push rsi
1409c2edf: 57                       push rdi
1409c2ee0: 53                       push rbx
1409c2ee1: 4883ec48                 sub rsp, 0x48
1409c2ee5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2eec: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c2ef3: e8b840edff               call 0x140896fb0
1409c2ef8: 90                       nop 
1409c2ef9: 4883c448                 add rsp, 0x48
1409c2efd: 5b                       pop rbx
1409c2efe: 5f                       pop rdi
1409c2eff: 5e                       pop rsi
1409c2f00: 415c                     pop r12
1409c2f02: 415d                     pop r13
1409c2f04: 415e                     pop r14
1409c2f06: 415f                     pop r15
1409c2f08: 5d                       pop rbp
1409c2f09: c3                       ret 
1409c2f0a: 660f1f440000             nop word ptr [rax + rax]
1409c2f10: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2f15: 55                       push rbp
1409c2f16: 4157                     push r15
1409c2f18: 4156                     push r14
1409c2f1a: 4155                     push r13
1409c2f1c: 4154                     push r12
1409c2f1e: 56                       push rsi
1409c2f1f: 57                       push rdi
1409c2f20: 53                       push rbx
1409c2f21: 4883ec48                 sub rsp, 0x48
1409c2f25: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2f2c: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2f33: e87840edff               call 0x140896fb0
1409c2f38: 90                       nop 
1409c2f39: 4883c448                 add rsp, 0x48
1409c2f3d: 5b                       pop rbx
1409c2f3e: 5f                       pop rdi
1409c2f3f: 5e                       pop rsi
1409c2f40: 415c                     pop r12
1409c2f42: 415d                     pop r13
1409c2f44: 415e                     pop r14
1409c2f46: 415f                     pop r15
1409c2f48: 5d                       pop rbp
1409c2f49: c3                       ret 
1409c2f4a: 660f1f440000             nop word ptr [rax + rax]
1409c2f50: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2f55: 55                       push rbp
1409c2f56: 4157                     push r15
1409c2f58: 4156                     push r14
1409c2f5a: 4155                     push r13
1409c2f5c: 4154                     push r12
1409c2f5e: 56                       push rsi
1409c2f5f: 57                       push rdi
1409c2f60: 53                       push rbx
1409c2f61: 4883ec48                 sub rsp, 0x48
1409c2f65: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2f6c: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c2f73: e83840edff               call 0x140896fb0
1409c2f78: 90                       nop 
1409c2f79: 4883c448                 add rsp, 0x48
1409c2f7d: 5b                       pop rbx
1409c2f7e: 5f                       pop rdi
1409c2f7f: 5e                       pop rsi
1409c2f80: 415c                     pop r12
1409c2f82: 415d                     pop r13
1409c2f84: 415e                     pop r14
1409c2f86: 415f                     pop r15
1409c2f88: 5d                       pop rbp
1409c2f89: c3                       ret 
1409c2f8a: 660f1f440000             nop word ptr [rax + rax]
1409c2f90: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2f95: 55                       push rbp
1409c2f96: 4157                     push r15
1409c2f98: 4156                     push r14
1409c2f9a: 4155                     push r13
1409c2f9c: 4154                     push r12
1409c2f9e: 56                       push rsi
1409c2f9f: 57                       push rdi
1409c2fa0: 53                       push rbx
1409c2fa1: 4883ec48                 sub rsp, 0x48
1409c2fa5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2fac: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c2fb3: e8f83fedff               call 0x140896fb0
1409c2fb8: 90                       nop 
1409c2fb9: 4883c448                 add rsp, 0x48
1409c2fbd: 5b                       pop rbx
1409c2fbe: 5f                       pop rdi
1409c2fbf: 5e                       pop rsi
1409c2fc0: 415c                     pop r12
1409c2fc2: 415d                     pop r13
1409c2fc4: 415e                     pop r14
1409c2fc6: 415f                     pop r15
1409c2fc8: 5d                       pop rbp
1409c2fc9: c3                       ret 
1409c2fca: 660f1f440000             nop word ptr [rax + rax]
1409c2fd0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c2fd5: 55                       push rbp
1409c2fd6: 4157                     push r15
1409c2fd8: 4156                     push r14
1409c2fda: 4155                     push r13
1409c2fdc: 4154                     push r12
1409c2fde: 56                       push rsi
1409c2fdf: 57                       push rdi
1409c2fe0: 53                       push rbx
1409c2fe1: 4883ec48                 sub rsp, 0x48
1409c2fe5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c2fec: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c2ff3: e8b83fedff               call 0x140896fb0
1409c2ff8: 90                       nop 
1409c2ff9: 4883c448                 add rsp, 0x48
1409c2ffd: 5b                       pop rbx
1409c2ffe: 5f                       pop rdi
1409c2fff: 5e                       pop rsi
1409c3000: 415c                     pop r12
1409c3002: 415d                     pop r13
1409c3004: 415e                     pop r14
1409c3006: 415f                     pop r15
1409c3008: 5d                       pop rbp
1409c3009: c3                       ret 
1409c300a: 660f1f440000             nop word ptr [rax + rax]
1409c3010: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3015: 55                       push rbp
1409c3016: 4157                     push r15
1409c3018: 4156                     push r14
1409c301a: 4155                     push r13
1409c301c: 4154                     push r12
1409c301e: 56                       push rsi
1409c301f: 57                       push rdi
1409c3020: 53                       push rbx
1409c3021: 4883ec48                 sub rsp, 0x48
1409c3025: 488daa80000000           lea rbp, [rdx + 0x80]
1409c302c: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c3033: e8783fedff               call 0x140896fb0
1409c3038: 90                       nop 
1409c3039: 4883c448                 add rsp, 0x48
1409c303d: 5b                       pop rbx
1409c303e: 5f                       pop rdi
1409c303f: 5e                       pop rsi
1409c3040: 415c                     pop r12
1409c3042: 415d                     pop r13
1409c3044: 415e                     pop r14
1409c3046: 415f                     pop r15
1409c3048: 5d                       pop rbp
1409c3049: c3                       ret 
1409c304a: 660f1f440000             nop word ptr [rax + rax]
1409c3050: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3055: 55                       push rbp
1409c3056: 4157                     push r15
1409c3058: 4156                     push r14
1409c305a: 4155                     push r13
1409c305c: 4154                     push r12
1409c305e: 56                       push rsi
1409c305f: 57                       push rdi
1409c3060: 53                       push rbx
1409c3061: 4883ec48                 sub rsp, 0x48
1409c3065: 488daa80000000           lea rbp, [rdx + 0x80]
1409c306c: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c3073: e8383fedff               call 0x140896fb0
1409c3078: 90                       nop 
1409c3079: 4883c448                 add rsp, 0x48
1409c307d: 5b                       pop rbx
1409c307e: 5f                       pop rdi
1409c307f: 5e                       pop rsi
1409c3080: 415c                     pop r12
1409c3082: 415d                     pop r13
1409c3084: 415e                     pop r14
1409c3086: 415f                     pop r15
1409c3088: 5d                       pop rbp
1409c3089: c3                       ret 
1409c308a: 660f1f440000             nop word ptr [rax + rax]
1409c3090: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3095: 55                       push rbp
1409c3096: 4157                     push r15
1409c3098: 4156                     push r14
1409c309a: 4155                     push r13
1409c309c: 4154                     push r12
1409c309e: 56                       push rsi
1409c309f: 57                       push rdi
1409c30a0: 53                       push rbx
1409c30a1: 4883ec48                 sub rsp, 0x48
1409c30a5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c30ac: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c30b3: e8f83eedff               call 0x140896fb0
1409c30b8: 90                       nop 
1409c30b9: 4883c448                 add rsp, 0x48
1409c30bd: 5b                       pop rbx
1409c30be: 5f                       pop rdi
1409c30bf: 5e                       pop rsi
1409c30c0: 415c                     pop r12
1409c30c2: 415d                     pop r13
1409c30c4: 415e                     pop r14
1409c30c6: 415f                     pop r15
1409c30c8: 5d                       pop rbp
1409c30c9: c3                       ret 
1409c30ca: 660f1f440000             nop word ptr [rax + rax]
1409c30d0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c30d5: 55                       push rbp
1409c30d6: 4157                     push r15
1409c30d8: 4156                     push r14
1409c30da: 4155                     push r13
1409c30dc: 4154                     push r12
1409c30de: 56                       push rsi
1409c30df: 57                       push rdi
1409c30e0: 53                       push rbx
1409c30e1: 4883ec48                 sub rsp, 0x48
1409c30e5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c30ec: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c30f3: e8b83eedff               call 0x140896fb0
1409c30f8: 90                       nop 
1409c30f9: 4883c448                 add rsp, 0x48
1409c30fd: 5b                       pop rbx
1409c30fe: 5f                       pop rdi
1409c30ff: 5e                       pop rsi
1409c3100: 415c                     pop r12
1409c3102: 415d                     pop r13
1409c3104: 415e                     pop r14
1409c3106: 415f                     pop r15
1409c3108: 5d                       pop rbp
1409c3109: c3                       ret 
1409c310a: 660f1f440000             nop word ptr [rax + rax]
1409c3110: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3115: 55                       push rbp
1409c3116: 4157                     push r15
1409c3118: 4156                     push r14
1409c311a: 4155                     push r13
1409c311c: 4154                     push r12
1409c311e: 56                       push rsi
1409c311f: 57                       push rdi
1409c3120: 53                       push rbx
1409c3121: 4883ec48                 sub rsp, 0x48
1409c3125: 488daa80000000           lea rbp, [rdx + 0x80]
1409c312c: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c3133: e8783eedff               call 0x140896fb0
1409c3138: 90                       nop 
1409c3139: 4883c448                 add rsp, 0x48
1409c313d: 5b                       pop rbx
1409c313e: 5f                       pop rdi
1409c313f: 5e                       pop rsi
1409c3140: 415c                     pop r12
1409c3142: 415d                     pop r13
1409c3144: 415e                     pop r14
1409c3146: 415f                     pop r15
1409c3148: 5d                       pop rbp
1409c3149: c3                       ret 
1409c314a: 660f1f440000             nop word ptr [rax + rax]
1409c3150: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3155: 55                       push rbp
1409c3156: 4157                     push r15
1409c3158: 4156                     push r14
1409c315a: 4155                     push r13
1409c315c: 4154                     push r12
1409c315e: 56                       push rsi
1409c315f: 57                       push rdi
1409c3160: 53                       push rbx
1409c3161: 4883ec48                 sub rsp, 0x48
1409c3165: 488daa80000000           lea rbp, [rdx + 0x80]
1409c316c: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c3173: e8b89a3800               call 0x140d4cc30
1409c3178: 90                       nop 
1409c3179: 4883c448                 add rsp, 0x48
1409c317d: 5b                       pop rbx
1409c317e: 5f                       pop rdi
1409c317f: 5e                       pop rsi
1409c3180: 415c                     pop r12
1409c3182: 415d                     pop r13
1409c3184: 415e                     pop r14
1409c3186: 415f                     pop r15
1409c3188: 5d                       pop rbp
1409c3189: c3                       ret 
1409c318a: 660f1f440000             nop word ptr [rax + rax]
1409c3190: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3195: 55                       push rbp
1409c3196: 4157                     push r15
1409c3198: 4156                     push r14
1409c319a: 4155                     push r13
1409c319c: 4154                     push r12
1409c319e: 56                       push rsi
1409c319f: 57                       push rdi
1409c31a0: 53                       push rbx
1409c31a1: 4883ec48                 sub rsp, 0x48
1409c31a5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c31ac: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c31b3: e8f83dedff               call 0x140896fb0
1409c31b8: 90                       nop 
1409c31b9: 4883c448                 add rsp, 0x48
1409c31bd: 5b                       pop rbx
1409c31be: 5f                       pop rdi
1409c31bf: 5e                       pop rsi
1409c31c0: 415c                     pop r12
1409c31c2: 415d                     pop r13
1409c31c4: 415e                     pop r14
1409c31c6: 415f                     pop r15
1409c31c8: 5d                       pop rbp
1409c31c9: c3                       ret 
1409c31ca: 660f1f440000             nop word ptr [rax + rax]
1409c31d0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c31d5: 55                       push rbp
1409c31d6: 4157                     push r15
1409c31d8: 4156                     push r14
1409c31da: 4155                     push r13
1409c31dc: 4154                     push r12
1409c31de: 56                       push rsi
1409c31df: 57                       push rdi
1409c31e0: 53                       push rbx
1409c31e1: 4883ec48                 sub rsp, 0x48
1409c31e5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c31ec: 80bde608000008           cmp byte ptr [rbp + 0x8e6], 8
1409c31f3: 740c                     je 0x1409c3201
1409c31f5: 488d8dd0080000           lea rcx, [rbp + 0x8d0]
1409c31fc: e8aff22100               call 0x140be24b0
1409c3201: 90                       nop 
1409c3202: 4883c448                 add rsp, 0x48
1409c3206: 5b                       pop rbx
1409c3207: 5f                       pop rdi
1409c3208: 5e                       pop rsi
1409c3209: 415c                     pop r12
1409c320b: 415d                     pop r13
1409c320d: 415e                     pop r14
1409c320f: 415f                     pop r15
1409c3211: 5d                       pop rbp
1409c3212: c3                       ret 
1409c3213: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409c3220: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3225: 55                       push rbp
1409c3226: 4157                     push r15
1409c3228: 4156                     push r14
1409c322a: 4155                     push r13
1409c322c: 4154                     push r12
1409c322e: 56                       push rsi
1409c322f: 57                       push rdi
1409c3230: 53                       push rbx
1409c3231: 4883ec48                 sub rsp, 0x48
1409c3235: 488daa80000000           lea rbp, [rdx + 0x80]
1409c323c: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c3243: e8683dedff               call 0x140896fb0
1409c3248: 90                       nop 
1409c3249: 4883c448                 add rsp, 0x48
1409c324d: 5b                       pop rbx
1409c324e: 5f                       pop rdi
1409c324f: 5e                       pop rsi
1409c3250: 415c                     pop r12
1409c3252: 415d                     pop r13
1409c3254: 415e                     pop r14
1409c3256: 415f                     pop r15
1409c3258: 5d                       pop rbp
1409c3259: c3                       ret 
1409c325a: 660f1f440000             nop word ptr [rax + rax]
1409c3260: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3265: 55                       push rbp
1409c3266: 4157                     push r15
1409c3268: 4156                     push r14
1409c326a: 4155                     push r13
1409c326c: 4154                     push r12
1409c326e: 56                       push rsi
1409c326f: 57                       push rdi
1409c3270: 53                       push rbx
1409c3271: 4883ec48                 sub rsp, 0x48
1409c3275: 488daa80000000           lea rbp, [rdx + 0x80]
1409c327c: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c3283: e8283dedff               call 0x140896fb0
1409c3288: 90                       nop 
1409c3289: 4883c448                 add rsp, 0x48
1409c328d: 5b                       pop rbx
1409c328e: 5f                       pop rdi
1409c328f: 5e                       pop rsi
1409c3290: 415c                     pop r12
1409c3292: 415d                     pop r13
1409c3294: 415e                     pop r14
1409c3296: 415f                     pop r15
1409c3298: 5d                       pop rbp
1409c3299: c3                       ret 
1409c329a: 660f1f440000             nop word ptr [rax + rax]
1409c32a0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c32a5: 55                       push rbp
1409c32a6: 4157                     push r15
1409c32a8: 4156                     push r14
1409c32aa: 4155                     push r13
1409c32ac: 4154                     push r12
1409c32ae: 56                       push rsi
1409c32af: 57                       push rdi
1409c32b0: 53                       push rbx
1409c32b1: 4883ec48                 sub rsp, 0x48
1409c32b5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c32bc: c6859d0d000000           mov byte ptr [rbp + 0xd9d], 0
1409c32c3: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c32ca: e8e13cedff               call 0x140896fb0
1409c32cf: c6859d0d000000           mov byte ptr [rbp + 0xd9d], 0
1409c32d6: 4883c448                 add rsp, 0x48
1409c32da: 5b                       pop rbx
1409c32db: 5f                       pop rdi
1409c32dc: 5e                       pop rsi
1409c32dd: 415c                     pop r12
1409c32df: 415d                     pop r13
1409c32e1: 415e                     pop r14
1409c32e3: 415f                     pop r15
1409c32e5: 5d                       pop rbp
1409c32e6: c3                       ret 
1409c32e7: 660f1f840000000000       nop word ptr [rax + rax]
1409c32f0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c32f5: 55                       push rbp
1409c32f6: 4157                     push r15
1409c32f8: 4156                     push r14
1409c32fa: 4155                     push r13
1409c32fc: 4154                     push r12
1409c32fe: 56                       push rsi
1409c32ff: 57                       push rdi
1409c3300: 53                       push rbx
1409c3301: 4883ec48                 sub rsp, 0x48
1409c3305: 488daa80000000           lea rbp, [rdx + 0x80]
1409c330c: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c3313: e8983cedff               call 0x140896fb0
1409c3318: 90                       nop 
1409c3319: 4883c448                 add rsp, 0x48
1409c331d: 5b                       pop rbx
1409c331e: 5f                       pop rdi
1409c331f: 5e                       pop rsi
1409c3320: 415c                     pop r12
1409c3322: 415d                     pop r13
1409c3324: 415e                     pop r14
1409c3326: 415f                     pop r15
1409c3328: 5d                       pop rbp
1409c3329: c3                       ret 
1409c332a: 660f1f440000             nop word ptr [rax + rax]
1409c3330: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3335: 55                       push rbp
1409c3336: 4157                     push r15
1409c3338: 4156                     push r14
1409c333a: 4155                     push r13
1409c333c: 4154                     push r12
1409c333e: 56                       push rsi
1409c333f: 57                       push rdi
1409c3340: 53                       push rbx
1409c3341: 4883ec48                 sub rsp, 0x48
1409c3345: 488daa80000000           lea rbp, [rdx + 0x80]
1409c334c: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c3353: e8583cedff               call 0x140896fb0
1409c3358: 90                       nop 
1409c3359: 4883c448                 add rsp, 0x48
1409c335d: 5b                       pop rbx
1409c335e: 5f                       pop rdi
1409c335f: 5e                       pop rsi
1409c3360: 415c                     pop r12
1409c3362: 415d                     pop r13
1409c3364: 415e                     pop r14
1409c3366: 415f                     pop r15
1409c3368: 5d                       pop rbp
1409c3369: c3                       ret 
1409c336a: 660f1f440000             nop word ptr [rax + rax]
1409c3370: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3375: 55                       push rbp
1409c3376: 4157                     push r15
1409c3378: 4156                     push r14
1409c337a: 4155                     push r13
1409c337c: 4154                     push r12
1409c337e: 56                       push rsi
1409c337f: 57                       push rdi
1409c3380: 53                       push rbx
1409c3381: 4883ec48                 sub rsp, 0x48
1409c3385: 488daa80000000           lea rbp, [rdx + 0x80]
1409c338c: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c3393: e8183cedff               call 0x140896fb0
1409c3398: 90                       nop 
1409c3399: 4883c448                 add rsp, 0x48
1409c339d: 5b                       pop rbx
1409c339e: 5f                       pop rdi
1409c339f: 5e                       pop rsi
1409c33a0: 415c                     pop r12
1409c33a2: 415d                     pop r13
1409c33a4: 415e                     pop r14
1409c33a6: 415f                     pop r15
1409c33a8: 5d                       pop rbp
1409c33a9: c3                       ret 
1409c33aa: 660f1f440000             nop word ptr [rax + rax]
1409c33b0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c33b5: 55                       push rbp
1409c33b6: 4157                     push r15
1409c33b8: 4156                     push r14
1409c33ba: 4155                     push r13
1409c33bc: 4154                     push r12
1409c33be: 56                       push rsi
1409c33bf: 57                       push rdi
1409c33c0: 53                       push rbx
1409c33c1: 4883ec48                 sub rsp, 0x48
1409c33c5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c33cc: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c33d3: e8d83bedff               call 0x140896fb0
1409c33d8: 90                       nop 
1409c33d9: 4883c448                 add rsp, 0x48
1409c33dd: 5b                       pop rbx
1409c33de: 5f                       pop rdi
1409c33df: 5e                       pop rsi
1409c33e0: 415c                     pop r12
1409c33e2: 415d                     pop r13
1409c33e4: 415e                     pop r14
1409c33e6: 415f                     pop r15
1409c33e8: 5d                       pop rbp
1409c33e9: c3                       ret 
1409c33ea: 660f1f440000             nop word ptr [rax + rax]
1409c33f0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c33f5: 55                       push rbp
1409c33f6: 4157                     push r15
1409c33f8: 4156                     push r14
1409c33fa: 4155                     push r13
1409c33fc: 4154                     push r12
1409c33fe: 56                       push rsi
1409c33ff: 57                       push rdi
1409c3400: 53                       push rbx
1409c3401: 4883ec48                 sub rsp, 0x48
1409c3405: 488daa80000000           lea rbp, [rdx + 0x80]
1409c340c: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c3413: e8983bedff               call 0x140896fb0
1409c3418: 90                       nop 
1409c3419: 4883c448                 add rsp, 0x48
1409c341d: 5b                       pop rbx
1409c341e: 5f                       pop rdi
1409c341f: 5e                       pop rsi
1409c3420: 415c                     pop r12
1409c3422: 415d                     pop r13
1409c3424: 415e                     pop r14
1409c3426: 415f                     pop r15
1409c3428: 5d                       pop rbp
1409c3429: c3                       ret 
1409c342a: 660f1f440000             nop word ptr [rax + rax]
1409c3430: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3435: 55                       push rbp
1409c3436: 4157                     push r15
1409c3438: 4156                     push r14
1409c343a: 4155                     push r13
1409c343c: 4154                     push r12
1409c343e: 56                       push rsi
1409c343f: 57                       push rdi
1409c3440: 53                       push rbx
1409c3441: 4883ec48                 sub rsp, 0x48
1409c3445: 488daa80000000           lea rbp, [rdx + 0x80]
1409c344c: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c3453: e8583bedff               call 0x140896fb0
1409c3458: 90                       nop 
1409c3459: 4883c448                 add rsp, 0x48
1409c345d: 5b                       pop rbx
1409c345e: 5f                       pop rdi
1409c345f: 5e                       pop rsi
1409c3460: 415c                     pop r12
1409c3462: 415d                     pop r13
1409c3464: 415e                     pop r14
1409c3466: 415f                     pop r15
1409c3468: 5d                       pop rbp
1409c3469: c3                       ret 
1409c346a: 660f1f440000             nop word ptr [rax + rax]
1409c3470: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3475: 55                       push rbp
1409c3476: 4157                     push r15
1409c3478: 4156                     push r14
1409c347a: 4155                     push r13
1409c347c: 4154                     push r12
1409c347e: 56                       push rsi
1409c347f: 57                       push rdi
1409c3480: 53                       push rbx
1409c3481: 4883ec48                 sub rsp, 0x48
1409c3485: 488daa80000000           lea rbp, [rdx + 0x80]
1409c348c: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c3493: e8183bedff               call 0x140896fb0
1409c3498: 90                       nop 
1409c3499: 4883c448                 add rsp, 0x48
1409c349d: 5b                       pop rbx
1409c349e: 5f                       pop rdi
1409c349f: 5e                       pop rsi
1409c34a0: 415c                     pop r12
1409c34a2: 415d                     pop r13
1409c34a4: 415e                     pop r14
1409c34a6: 415f                     pop r15
1409c34a8: 5d                       pop rbp
1409c34a9: c3                       ret 
1409c34aa: 660f1f440000             nop word ptr [rax + rax]
1409c34b0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c34b5: 55                       push rbp
1409c34b6: 4157                     push r15
1409c34b8: 4156                     push r14
1409c34ba: 4155                     push r13
1409c34bc: 4154                     push r12
1409c34be: 56                       push rsi
1409c34bf: 57                       push rdi
1409c34c0: 53                       push rbx
1409c34c1: 4883ec48                 sub rsp, 0x48
1409c34c5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c34cc: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c34d3: e8d83aedff               call 0x140896fb0
1409c34d8: 90                       nop 
1409c34d9: 4883c448                 add rsp, 0x48
1409c34dd: 5b                       pop rbx
1409c34de: 5f                       pop rdi
1409c34df: 5e                       pop rsi
1409c34e0: 415c                     pop r12
1409c34e2: 415d                     pop r13
1409c34e4: 415e                     pop r14
1409c34e6: 415f                     pop r15
1409c34e8: 5d                       pop rbp
1409c34e9: c3                       ret 
1409c34ea: 660f1f440000             nop word ptr [rax + rax]
1409c34f0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c34f5: 55                       push rbp
1409c34f6: 4157                     push r15
1409c34f8: 4156                     push r14
1409c34fa: 4155                     push r13
1409c34fc: 4154                     push r12
1409c34fe: 56                       push rsi
1409c34ff: 57                       push rdi
1409c3500: 53                       push rbx
1409c3501: 4883ec48                 sub rsp, 0x48
1409c3505: 488daa80000000           lea rbp, [rdx + 0x80]
1409c350c: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c3513: e8983aedff               call 0x140896fb0
1409c3518: 90                       nop 
1409c3519: 4883c448                 add rsp, 0x48
1409c351d: 5b                       pop rbx
1409c351e: 5f                       pop rdi
1409c351f: 5e                       pop rsi
1409c3520: 415c                     pop r12
1409c3522: 415d                     pop r13
1409c3524: 415e                     pop r14
1409c3526: 415f                     pop r15
1409c3528: 5d                       pop rbp
1409c3529: c3                       ret 
1409c352a: 660f1f440000             nop word ptr [rax + rax]
1409c3530: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3535: 55                       push rbp
1409c3536: 4157                     push r15
1409c3538: 4156                     push r14
1409c353a: 4155                     push r13
1409c353c: 4154                     push r12
1409c353e: 56                       push rsi
1409c353f: 57                       push rdi
1409c3540: 53                       push rbx
1409c3541: 4883ec48                 sub rsp, 0x48
1409c3545: 488daa80000000           lea rbp, [rdx + 0x80]
1409c354c: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c3553: e8583aedff               call 0x140896fb0
1409c3558: 90                       nop 
1409c3559: 4883c448                 add rsp, 0x48
1409c355d: 5b                       pop rbx
1409c355e: 5f                       pop rdi
1409c355f: 5e                       pop rsi
1409c3560: 415c                     pop r12
1409c3562: 415d                     pop r13
1409c3564: 415e                     pop r14
1409c3566: 415f                     pop r15
1409c3568: 5d                       pop rbp
1409c3569: c3                       ret 
1409c356a: 660f1f440000             nop word ptr [rax + rax]
1409c3570: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3575: 55                       push rbp
1409c3576: 4157                     push r15
1409c3578: 4156                     push r14
1409c357a: 4155                     push r13
1409c357c: 4154                     push r12
1409c357e: 56                       push rsi
1409c357f: 57                       push rdi
1409c3580: 53                       push rbx
1409c3581: 4883ec48                 sub rsp, 0x48
1409c3585: 488daa80000000           lea rbp, [rdx + 0x80]
1409c358c: 80bde603000008           cmp byte ptr [rbp + 0x3e6], 8
1409c3593: 740c                     je 0x1409c35a1
1409c3595: 488d8dd0030000           lea rcx, [rbp + 0x3d0]
1409c359c: e80fef2100               call 0x140be24b0
1409c35a1: 90                       nop 
1409c35a2: 4883c448                 add rsp, 0x48
1409c35a6: 5b                       pop rbx
1409c35a7: 5f                       pop rdi
1409c35a8: 5e                       pop rsi
1409c35a9: 415c                     pop r12
1409c35ab: 415d                     pop r13
1409c35ad: 415e                     pop r14
1409c35af: 415f                     pop r15
1409c35b1: 5d                       pop rbp
1409c35b2: c3                       ret 
1409c35b3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409c35c0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c35c5: 55                       push rbp
1409c35c6: 4157                     push r15
1409c35c8: 4156                     push r14
1409c35ca: 4155                     push r13
1409c35cc: 4154                     push r12
1409c35ce: 56                       push rsi
1409c35cf: 57                       push rdi
1409c35d0: 53                       push rbx
1409c35d1: 4883ec48                 sub rsp, 0x48
1409c35d5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c35dc: 80bd9d0d000000           cmp byte ptr [rbp + 0xd9d], 0
1409c35e3: 7511                     jne 0x1409c35f6
1409c35e5: 4883c448                 add rsp, 0x48
1409c35e9: 5b                       pop rbx
1409c35ea: 5f                       pop rdi
1409c35eb: 5e                       pop rsi
1409c35ec: 415c                     pop r12
1409c35ee: 415d                     pop r13
1409c35f0: 415e                     pop r14
1409c35f2: 415f                     pop r15
1409c35f4: 5d                       pop rbp
1409c35f5: c3                       ret 
1409c35f6: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c35fd: e8ae39edff               call 0x140896fb0
1409c3602: ebe1                     jmp 0x1409c35e5
1409c3604: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409c3610: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3615: 55                       push rbp
1409c3616: 4157                     push r15
1409c3618: 4156                     push r14
1409c361a: 4155                     push r13
1409c361c: 4154                     push r12
1409c361e: 56                       push rsi
1409c361f: 57                       push rdi
1409c3620: 53                       push rbx
1409c3621: 4883ec48                 sub rsp, 0x48
1409c3625: 488daa80000000           lea rbp, [rdx + 0x80]
1409c362c: 80bd9e0d000000           cmp byte ptr [rbp + 0xd9e], 0
1409c3633: 7511                     jne 0x1409c3646
1409c3635: 4883c448                 add rsp, 0x48
1409c3639: 5b                       pop rbx
1409c363a: 5f                       pop rdi
1409c363b: 5e                       pop rsi
1409c363c: 415c                     pop r12
1409c363e: 415d                     pop r13
1409c3640: 415e                     pop r14
1409c3642: 415f                     pop r15
1409c3644: 5d                       pop rbp
1409c3645: c3                       ret 
1409c3646: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c364d: e85e39edff               call 0x140896fb0
1409c3652: ebe1                     jmp 0x1409c3635
1409c3654: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409c3660: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3665: 55                       push rbp
1409c3666: 4157                     push r15
1409c3668: 4156                     push r14
1409c366a: 4155                     push r13
1409c366c: 4154                     push r12
1409c366e: 56                       push rsi
1409c366f: 57                       push rdi
1409c3670: 53                       push rbx
1409c3671: 4883ec48                 sub rsp, 0x48
1409c3675: 488daa80000000           lea rbp, [rdx + 0x80]
1409c367c: 80bd9f0d000000           cmp byte ptr [rbp + 0xd9f], 0
1409c3683: 7511                     jne 0x1409c3696
1409c3685: 4883c448                 add rsp, 0x48
1409c3689: 5b                       pop rbx
1409c368a: 5f                       pop rdi
1409c368b: 5e                       pop rsi
1409c368c: 415c                     pop r12
1409c368e: 415d                     pop r13
1409c3690: 415e                     pop r14
1409c3692: 415f                     pop r15
1409c3694: 5d                       pop rbp
1409c3695: c3                       ret 
1409c3696: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c369d: e81eedecff               call 0x1408923c0
1409c36a2: ebe1                     jmp 0x1409c3685
1409c36a4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409c36b0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c36b5: 55                       push rbp
1409c36b6: 4157                     push r15
1409c36b8: 4156                     push r14
1409c36ba: 4155                     push r13
1409c36bc: 4154                     push r12
1409c36be: 56                       push rsi
1409c36bf: 57                       push rdi
1409c36c0: 53                       push rbx
1409c36c1: 4883ec48                 sub rsp, 0x48
1409c36c5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c36cc: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c36d3: e858953800               call 0x140d4cc30
1409c36d8: 90                       nop 
1409c36d9: 4883c448                 add rsp, 0x48
1409c36dd: 5b                       pop rbx
1409c36de: 5f                       pop rdi
1409c36df: 5e                       pop rsi
1409c36e0: 415c                     pop r12
1409c36e2: 415d                     pop r13
1409c36e4: 415e                     pop r14
1409c36e6: 415f                     pop r15
1409c36e8: 5d                       pop rbp
1409c36e9: c3                       ret 
1409c36ea: 660f1f440000             nop word ptr [rax + rax]
1409c36f0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c36f5: 55                       push rbp
1409c36f6: 4157                     push r15
1409c36f8: 4156                     push r14
1409c36fa: 4155                     push r13
1409c36fc: 4154                     push r12
1409c36fe: 56                       push rsi
1409c36ff: 57                       push rdi
1409c3700: 53                       push rbx
1409c3701: 4883ec48                 sub rsp, 0x48
1409c3705: 488daa80000000           lea rbp, [rdx + 0x80]
1409c370c: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c3713: e818953800               call 0x140d4cc30
1409c3718: 90                       nop 
1409c3719: 4883c448                 add rsp, 0x48
1409c371d: 5b                       pop rbx
1409c371e: 5f                       pop rdi
1409c371f: 5e                       pop rsi
1409c3720: 415c                     pop r12
1409c3722: 415d                     pop r13
1409c3724: 415e                     pop r14
1409c3726: 415f                     pop r15
1409c3728: 5d                       pop rbp
1409c3729: c3                       ret 
1409c372a: 660f1f440000             nop word ptr [rax + rax]
1409c3730: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3735: 55                       push rbp
1409c3736: 4157                     push r15
1409c3738: 4156                     push r14
1409c373a: 4155                     push r13
1409c373c: 4154                     push r12
1409c373e: 56                       push rsi
1409c373f: 57                       push rdi
1409c3740: 53                       push rbx
1409c3741: 4883ec48                 sub rsp, 0x48
1409c3745: 488daa80000000           lea rbp, [rdx + 0x80]
1409c374c: c6859e0d000000           mov byte ptr [rbp + 0xd9e], 0
1409c3753: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c375a: e85138edff               call 0x140896fb0
1409c375f: c6859e0d000000           mov byte ptr [rbp + 0xd9e], 0
1409c3766: 4883c448                 add rsp, 0x48
1409c376a: 5b                       pop rbx
1409c376b: 5f                       pop rdi
1409c376c: 5e                       pop rsi
1409c376d: 415c                     pop r12
1409c376f: 415d                     pop r13
1409c3771: 415e                     pop r14
1409c3773: 415f                     pop r15
1409c3775: 5d                       pop rbp
1409c3776: c3                       ret 
1409c3777: 660f1f840000000000       nop word ptr [rax + rax]
1409c3780: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3785: 55                       push rbp
1409c3786: 4157                     push r15
1409c3788: 4156                     push r14
1409c378a: 4155                     push r13
1409c378c: 4154                     push r12
1409c378e: 56                       push rsi
1409c378f: 57                       push rdi
1409c3790: 53                       push rbx
1409c3791: 4883ec48                 sub rsp, 0x48
1409c3795: 488daa80000000           lea rbp, [rdx + 0x80]
1409c379c: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c37a3: e80838edff               call 0x140896fb0
1409c37a8: 90                       nop 
1409c37a9: 4883c448                 add rsp, 0x48
1409c37ad: 5b                       pop rbx
1409c37ae: 5f                       pop rdi
1409c37af: 5e                       pop rsi
1409c37b0: 415c                     pop r12
1409c37b2: 415d                     pop r13
1409c37b4: 415e                     pop r14
1409c37b6: 415f                     pop r15
1409c37b8: 5d                       pop rbp
1409c37b9: c3                       ret 
1409c37ba: 660f1f440000             nop word ptr [rax + rax]
1409c37c0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c37c5: 55                       push rbp
1409c37c6: 4157                     push r15
1409c37c8: 4156                     push r14
1409c37ca: 4155                     push r13
1409c37cc: 4154                     push r12
1409c37ce: 56                       push rsi
1409c37cf: 57                       push rdi
1409c37d0: 53                       push rbx
1409c37d1: 4883ec48                 sub rsp, 0x48
1409c37d5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c37dc: 488d8db0020000           lea rcx, [rbp + 0x2b0]
1409c37e3: e8c837edff               call 0x140896fb0
1409c37e8: 90                       nop 
1409c37e9: 4883c448                 add rsp, 0x48
1409c37ed: 5b                       pop rbx
1409c37ee: 5f                       pop rdi
1409c37ef: 5e                       pop rsi
1409c37f0: 415c                     pop r12
1409c37f2: 415d                     pop r13
1409c37f4: 415e                     pop r14
1409c37f6: 415f                     pop r15
1409c37f8: 5d                       pop rbp
1409c37f9: c3                       ret 
1409c37fa: 660f1f440000             nop word ptr [rax + rax]
1409c3800: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3805: 55                       push rbp
1409c3806: 4157                     push r15
1409c3808: 4156                     push r14
1409c380a: 4155                     push r13
1409c380c: 4154                     push r12
1409c380e: 56                       push rsi
1409c380f: 57                       push rdi
1409c3810: 53                       push rbx
1409c3811: 4883ec48                 sub rsp, 0x48
1409c3815: 488daa80000000           lea rbp, [rdx + 0x80]
1409c381c: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c3823: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c382a: e891fdecff               call 0x1408935c0
1409c382f: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c3836: 4883c448                 add rsp, 0x48
1409c383a: 5b                       pop rbx
1409c383b: 5f                       pop rdi
1409c383c: 5e                       pop rsi
1409c383d: 415c                     pop r12
1409c383f: 415d                     pop r13
1409c3841: 415e                     pop r14
1409c3843: 415f                     pop r15
1409c3845: 5d                       pop rbp
1409c3846: c3                       ret 
1409c3847: 660f1f840000000000       nop word ptr [rax + rax]
1409c3850: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3855: 55                       push rbp
1409c3856: 4157                     push r15
1409c3858: 4156                     push r14
1409c385a: 4155                     push r13
1409c385c: 4154                     push r12
1409c385e: 56                       push rsi
1409c385f: 57                       push rdi
1409c3860: 53                       push rbx
1409c3861: 4883ec48                 sub rsp, 0x48
1409c3865: 488daa80000000           lea rbp, [rdx + 0x80]
1409c386c: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c3873: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c387a: e841fdecff               call 0x1408935c0
1409c387f: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c3886: 4883c448                 add rsp, 0x48
1409c388a: 5b                       pop rbx
1409c388b: 5f                       pop rdi
1409c388c: 5e                       pop rsi
1409c388d: 415c                     pop r12
1409c388f: 415d                     pop r13
1409c3891: 415e                     pop r14
1409c3893: 415f                     pop r15
1409c3895: 5d                       pop rbp
1409c3896: c3                       ret 
1409c3897: 660f1f840000000000       nop word ptr [rax + rax]
1409c38a0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c38a5: 55                       push rbp
1409c38a6: 4157                     push r15
1409c38a8: 4156                     push r14
1409c38aa: 4155                     push r13
1409c38ac: 4154                     push r12
1409c38ae: 56                       push rsi
1409c38af: 57                       push rdi
1409c38b0: 53                       push rbx
1409c38b1: 4883ec48                 sub rsp, 0x48
1409c38b5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c38bc: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c38c3: 488d8db8060000           lea rcx, [rbp + 0x6b8]
1409c38ca: e8f1fcecff               call 0x1408935c0
1409c38cf: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c38d6: 4883c448                 add rsp, 0x48
1409c38da: 5b                       pop rbx
1409c38db: 5f                       pop rdi
1409c38dc: 5e                       pop rsi
1409c38dd: 415c                     pop r12
1409c38df: 415d                     pop r13
1409c38e1: 415e                     pop r14
1409c38e3: 415f                     pop r15
1409c38e5: 5d                       pop rbp
1409c38e6: c3                       ret 
1409c38e7: 660f1f840000000000       nop word ptr [rax + rax]
1409c38f0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c38f5: 55                       push rbp
1409c38f6: 4157                     push r15
1409c38f8: 4156                     push r14
1409c38fa: 4155                     push r13
1409c38fc: 4154                     push r12
1409c38fe: 56                       push rsi
1409c38ff: 57                       push rdi
1409c3900: 53                       push rbx
1409c3901: 4883ec48                 sub rsp, 0x48
1409c3905: 488daa80000000           lea rbp, [rdx + 0x80]
1409c390c: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c3913: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c391a: e8a1fcecff               call 0x1408935c0
1409c391f: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c3926: 4883c448                 add rsp, 0x48
1409c392a: 5b                       pop rbx
1409c392b: 5f                       pop rdi
1409c392c: 5e                       pop rsi
1409c392d: 415c                     pop r12
1409c392f: 415d                     pop r13
1409c3931: 415e                     pop r14
1409c3933: 415f                     pop r15
1409c3935: 5d                       pop rbp
1409c3936: c3                       ret 
1409c3937: 660f1f840000000000       nop word ptr [rax + rax]
1409c3940: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3945: 55                       push rbp
1409c3946: 4157                     push r15
1409c3948: 4156                     push r14
1409c394a: 4155                     push r13
1409c394c: 4154                     push r12
1409c394e: 56                       push rsi
1409c394f: 57                       push rdi
1409c3950: 53                       push rbx
1409c3951: 4883ec48                 sub rsp, 0x48
1409c3955: 488daa80000000           lea rbp, [rdx + 0x80]
1409c395c: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c3963: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c396a: e851fcecff               call 0x1408935c0
1409c396f: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c3976: 4883c448                 add rsp, 0x48
1409c397a: 5b                       pop rbx
1409c397b: 5f                       pop rdi
1409c397c: 5e                       pop rsi
1409c397d: 415c                     pop r12
1409c397f: 415d                     pop r13
1409c3981: 415e                     pop r14
1409c3983: 415f                     pop r15
1409c3985: 5d                       pop rbp
1409c3986: c3                       ret 
1409c3987: 660f1f840000000000       nop word ptr [rax + rax]
1409c3990: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3995: 55                       push rbp
1409c3996: 4157                     push r15
1409c3998: 4156                     push r14
1409c399a: 4155                     push r13
1409c399c: 4154                     push r12
1409c399e: 56                       push rsi
1409c399f: 57                       push rdi
1409c39a0: 53                       push rbx
1409c39a1: 4883ec48                 sub rsp, 0x48
1409c39a5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c39ac: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c39b3: 488d4dd0                 lea rcx, [rbp - 0x30]
1409c39b7: e804fcecff               call 0x1408935c0
1409c39bc: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c39c3: 4883c448                 add rsp, 0x48
1409c39c7: 5b                       pop rbx
1409c39c8: 5f                       pop rdi
1409c39c9: 5e                       pop rsi
1409c39ca: 415c                     pop r12
1409c39cc: 415d                     pop r13
1409c39ce: 415e                     pop r14
1409c39d0: 415f                     pop r15
1409c39d2: 5d                       pop rbp
1409c39d3: c3                       ret 
1409c39d4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409c39e0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c39e5: 55                       push rbp
1409c39e6: 4157                     push r15
1409c39e8: 4156                     push r14
1409c39ea: 4155                     push r13
1409c39ec: 4154                     push r12
1409c39ee: 56                       push rsi
1409c39ef: 57                       push rdi
1409c39f0: 53                       push rbx
1409c39f1: 4883ec48                 sub rsp, 0x48
1409c39f5: 488daa80000000           lea rbp, [rdx + 0x80]
1409c39fc: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c3a03: 488d8db0070000           lea rcx, [rbp + 0x7b0]
1409c3a0a: e8b1fbecff               call 0x1408935c0
1409c3a0f: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c3a16: 4883c448                 add rsp, 0x48
1409c3a1a: 5b                       pop rbx
1409c3a1b: 5f                       pop rdi
1409c3a1c: 5e                       pop rsi
1409c3a1d: 415c                     pop r12
1409c3a1f: 415d                     pop r13
1409c3a21: 415e                     pop r14
1409c3a23: 415f                     pop r15
1409c3a25: 5d                       pop rbp
1409c3a26: c3                       ret 
1409c3a27: 660f1f840000000000       nop word ptr [rax + rax]
1409c3a30: 4889542410               mov qword ptr [rsp + 0x10], rdx
1409c3a35: 55                       push rbp
1409c3a36: 4157                     push r15
1409c3a38: 4156                     push r14
1409c3a3a: 4155                     push r13
1409c3a3c: 4154                     push r12
1409c3a3e: 56                       push rsi
1409c3a3f: 57                       push rdi
1409c3a40: 53                       push rbx
1409c3a41: 4883ec48                 sub rsp, 0x48
1409c3a45: 488daa80000000           lea rbp, [rdx + 0x80]
1409c3a4c: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c3a53: 488d8de00b0000           lea rcx, [rbp + 0xbe0]
1409c3a5a: e861fbecff               call 0x1408935c0
1409c3a5f: c6859f0d000001           mov byte ptr [rbp + 0xd9f], 1
1409c3a66: 4883c448                 add rsp, 0x48
1409c3a6a: 5b                       pop rbx
1409c3a6b: 5f                       pop rdi
1409c3a6c: 5e                       pop rsi
1409c3a6d: 415c                     pop r12
1409c3a6f: 415d                     pop r13
1409c3a71: 415e                     pop r14
1409c3a73: 415f                     pop r15
1409c3a75: 5d                       pop rbp
1409c3a76: c3                       ret 
