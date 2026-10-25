14206bc10: 55 push rbp
14206bc11: 56 push rsi
14206bc12: 57 push rdi
14206bc13: 53 push rbx
14206bc14: 4881ec980c0000 sub rsp, 0xc98
14206bc1b: 488dac2480000000 lea rbp, [rsp + 0x80]
14206bc23: 48c785100c0000feffffff mov qword ptr [rbp + 0xc10], 0xfffffffffffffffe
14206bc2e: 4889ce mov rsi, rcx
14206bc31: 0fb605d18f5c01 movzx eax, byte ptr [rip + 0x15c8fd1]
14206bc38: b9c0080000 mov ecx, 0x8c0
14206bc3d: ba08000000 mov edx, 8
14206bc42: e8c97f5afe call 0x140613c10 ; unknown
14206bc47: 4885c0 test rax, rax
14206bc4a: 0f848e250000 je 0x14206e1de
14206bc50: 48898560080000 mov qword ptr [rbp + 0x860], rax
14206bc57: 0f57c0 xorps xmm0, xmm0
14206bc5a: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206bc61: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206bc68: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206bc6f: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206bc76: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206bc7d: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206bc87: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206bc8e: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206bc95: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206bc9c: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206bca3: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206bcaa: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206bcb4: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206bcbf: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206bcca: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206bcd5: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206bce0: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206bceb: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206bcf6: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206bd01: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206bd0c: 48bb0000000000000080 movabs rbx, 0x8000000000000000
14206bd16: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206bd1d: 488d054769f600 lea rax, [rip + 0xf66947]
14206bd24: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206bd2b: 48c7851009000043000000 mov qword ptr [rbp + 0x910], 0x43
14206bd36: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206bd41: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206bd4c: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206bd57: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206bd62: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206bd6d: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206bd77: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206bd80: 48899de0090000 mov qword ptr [rbp + 0x9e0], rbx
14206bd87: c785e80a000000000000 mov dword ptr [rbp + 0xae8], 0
14206bd91: 0fb605718e5c01 movzx eax, byte ptr [rip + 0x15c8e71]
14206bd98: b988020000 mov ecx, 0x288
14206bd9d: ba08000000 mov edx, 8
14206bda2: e8697e5afe call 0x140613c10 ; unknown
14206bda7: 4885c0 test rax, rax
14206bdaa: 0f843d240000 je 0x14206e1ed
14206bdb0: 4889c7 mov rdi, rax
14206bdb3: 488d9568080000 lea rdx, [rbp + 0x868]
14206bdba: 41b888020000 mov r8d, 0x288
14206bdc0: 4889c1 mov rcx, rax
14206bdc3: e8387e8b00 call 0x142923c00 ; unknown
14206bdc8: 4c8d05c158f600 lea r8, [rip + 0xf658c1]
14206bdcf: 488d8df0070000 lea rcx, [rbp + 0x7f0]
14206bdd6: 4889fa mov rdx, rdi
14206bdd9: e8328dc7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206bdde: 0f57c0 xorps xmm0, xmm0
14206bde1: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206bde8: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206bdef: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206bdf6: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206bdfd: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206be04: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206be0e: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206be15: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206be1c: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206be23: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206be2a: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206be31: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206be3b: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206be46: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206be51: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206be5c: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206be67: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206be72: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206be7d: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206be88: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206be93: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206be9a: 488d052e6bf600 lea rax, [rip + 0xf66b2e]
14206bea1: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206bea8: 48c785100900004a000000 mov qword ptr [rbp + 0x910], 0x4a
14206beb3: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206bebe: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206bec9: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206bed4: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206bedf: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206beea: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206bef4: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206befd: 48899de0090000 mov qword ptr [rbp + 0x9e0], rbx
14206bf04: c785e80a000000000000 mov dword ptr [rbp + 0xae8], 0
14206bf0e: 0fb605f48c5c01 movzx eax, byte ptr [rip + 0x15c8cf4]
14206bf15: b988020000 mov ecx, 0x288
14206bf1a: ba08000000 mov edx, 8
14206bf1f: e8ec7c5afe call 0x140613c10 ; unknown
14206bf24: 4885c0 test rax, rax
14206bf27: 0f84d4220000 je 0x14206e201
14206bf2d: 4889c7 mov rdi, rax
14206bf30: 488d9568080000 lea rdx, [rbp + 0x868]
14206bf37: 41b888020000 mov r8d, 0x288
14206bf3d: 4889c1 mov rcx, rax
14206bf40: e8bb7c8b00 call 0x142923c00 ; unknown
14206bf45: 4c8d05745ef600 lea r8, [rip + 0xf65e74]
14206bf4c: 488d8d80070000 lea rcx, [rbp + 0x780]
14206bf53: 4889fa mov rdx, rdi
14206bf56: e8b58bc7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206bf5b: 0f57c0 xorps xmm0, xmm0
14206bf5e: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206bf65: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206bf6c: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206bf73: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206bf7a: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206bf81: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206bf8b: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206bf92: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206bf99: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206bfa0: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206bfa7: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206bfae: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206bfb8: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206bfc3: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206bfce: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206bfd9: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206bfe4: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206bfef: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206bffa: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206c005: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206c010: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206c017: 488d05c065f600 lea rax, [rip + 0xf665c0]
14206c01e: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206c025: 48c7851009000050000000 mov qword ptr [rbp + 0x910], 0x50
14206c030: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206c03b: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206c046: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206c051: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206c05c: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206c067: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206c071: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206c07a: 48899de0090000 mov qword ptr [rbp + 0x9e0], rbx
14206c081: c785080b000000000000 mov dword ptr [rbp + 0xb08], 0
14206c08b: 0fb605778b5c01 movzx eax, byte ptr [rip + 0x15c8b77]
14206c092: b9a8020000 mov ecx, 0x2a8
14206c097: ba08000000 mov edx, 8
14206c09c: e86f7b5afe call 0x140613c10 ; unknown
14206c0a1: 4885c0 test rax, rax
14206c0a4: 0f846b210000 je 0x14206e215
14206c0aa: 4889c7 mov rdi, rax
14206c0ad: 488d9568080000 lea rdx, [rbp + 0x868]
14206c0b4: 41b8a8020000 mov r8d, 0x2a8
14206c0ba: 4889c1 mov rcx, rax
14206c0bd: e83e7b8b00 call 0x142923c00 ; unknown
14206c0c2: 4c8d05675ef600 lea r8, [rip + 0xf65e67]
14206c0c9: 488d8d10070000 lea rcx, [rbp + 0x710]
14206c0d0: 4889fa mov rdx, rdi
14206c0d3: e8388ac7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206c0d8: 0f57c0 xorps xmm0, xmm0
14206c0db: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206c0e2: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206c0e9: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206c0f0: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206c0f7: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206c0fe: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206c108: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206c10f: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206c116: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206c11d: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206c124: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206c12b: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206c135: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206c140: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206c14b: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206c156: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206c161: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206c16c: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206c177: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206c182: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206c18d: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206c194: 488d051365f600 lea rax, [rip + 0xf66513]
14206c19b: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206c1a2: 48c785100900004a000000 mov qword ptr [rbp + 0x910], 0x4a
14206c1ad: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206c1b8: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206c1c3: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206c1ce: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206c1d9: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206c1e4: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206c1ee: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206c1f7: 48899d980a0000 mov qword ptr [rbp + 0xa98], rbx
14206c1fe: c785e00a000000000000 mov dword ptr [rbp + 0xae0], 0
14206c208: 0fb605fa895c01 movzx eax, byte ptr [rip + 0x15c89fa]
14206c20f: b980020000 mov ecx, 0x280
14206c214: ba08000000 mov edx, 8
14206c219: e8f2795afe call 0x140613c10 ; unknown
14206c21e: 4885c0 test rax, rax
14206c221: 0f8402200000 je 0x14206e229
14206c227: 4889c7 mov rdi, rax
14206c22a: 488d9568080000 lea rdx, [rbp + 0x868]
14206c231: 41b880020000 mov r8d, 0x280
14206c237: 4889c1 mov rcx, rax
14206c23a: e8c1798b00 call 0x142923c00 ; unknown
14206c23f: 4c8d055a5ef600 lea r8, [rip + 0xf65e5a]
14206c246: 488d8da0060000 lea rcx, [rbp + 0x6a0]
14206c24d: 4889fa mov rdx, rdi
14206c250: e8bb88c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206c255: 0f57c0 xorps xmm0, xmm0
14206c258: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206c25f: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206c266: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206c26d: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206c274: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206c27b: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206c285: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206c28c: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206c293: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206c29a: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206c2a1: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206c2a8: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206c2b2: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206c2bd: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206c2c8: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206c2d3: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206c2de: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206c2e9: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206c2f4: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206c2ff: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206c30a: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206c311: 488d054b64f600 lea rax, [rip + 0xf6644b]
14206c318: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206c31f: 48c7851009000045000000 mov qword ptr [rbp + 0x910], 0x45
14206c32a: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206c335: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206c340: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206c34b: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206c356: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206c361: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206c36b: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206c374: 48899de0090000 mov qword ptr [rbp + 0x9e0], rbx
14206c37b: c785e80a000000000000 mov dword ptr [rbp + 0xae8], 0
14206c385: 0fb6057d885c01 movzx eax, byte ptr [rip + 0x15c887d]
14206c38c: b988020000 mov ecx, 0x288
14206c391: ba08000000 mov edx, 8
14206c396: e875785afe call 0x140613c10 ; unknown
14206c39b: 4885c0 test rax, rax
14206c39e: 0f84991e0000 je 0x14206e23d
14206c3a4: 4889c7 mov rdi, rax
14206c3a7: 488d9568080000 lea rdx, [rbp + 0x868]
14206c3ae: 41b888020000 mov r8d, 0x288
14206c3b4: 4889c1 mov rcx, rax
14206c3b7: e844788b00 call 0x142923c00 ; unknown
14206c3bc: 4c8d051552f600 lea r8, [rip + 0xf65215]
14206c3c3: 488d8d30060000 lea rcx, [rbp + 0x630]
14206c3ca: 4889fa mov rdx, rdi
14206c3cd: e83e87c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206c3d2: 0f57c0 xorps xmm0, xmm0
14206c3d5: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206c3dc: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206c3e3: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206c3ea: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206c3f1: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206c3f8: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206c402: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206c409: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206c410: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206c417: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206c41e: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206c425: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206c42f: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206c43a: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206c445: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206c450: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206c45b: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206c466: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206c471: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206c47c: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206c487: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206c48e: 488d051363f600 lea rax, [rip + 0xf66313]
14206c495: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206c49c: 48c785100900003c000000 mov qword ptr [rbp + 0x910], 0x3c
14206c4a7: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206c4b2: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206c4bd: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206c4c8: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206c4d3: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206c4de: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206c4e8: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206c4f1: 48899de0090000 mov qword ptr [rbp + 0x9e0], rbx
14206c4f8: c785f00a000000000000 mov dword ptr [rbp + 0xaf0], 0
14206c502: 0fb60500875c01 movzx eax, byte ptr [rip + 0x15c8700]
14206c509: b990020000 mov ecx, 0x290
14206c50e: ba08000000 mov edx, 8
14206c513: e8f8765afe call 0x140613c10 ; unknown
14206c518: 4885c0 test rax, rax
14206c51b: 0f84301d0000 je 0x14206e251
14206c521: 4889c7 mov rdi, rax
14206c524: 488d9568080000 lea rdx, [rbp + 0x868]
14206c52b: 41b890020000 mov r8d, 0x290
14206c531: 4889c1 mov rcx, rax
14206c534: e8c7768b00 call 0x142923c00 ; unknown
14206c539: 4c8d05a055f600 lea r8, [rip + 0xf655a0]
14206c540: 488d8dc0050000 lea rcx, [rbp + 0x5c0]
14206c547: 4889fa mov rdx, rdi
14206c54a: e8c185c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206c54f: 0f57c0 xorps xmm0, xmm0
14206c552: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206c559: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206c560: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206c567: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206c56e: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206c575: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206c57f: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206c586: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206c58d: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206c594: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206c59b: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206c5a2: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206c5ac: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206c5b7: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206c5c2: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206c5cd: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206c5d8: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206c5e3: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206c5ee: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206c5f9: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206c604: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206c60b: 488d058f62f600 lea rax, [rip + 0xf6628f]
14206c612: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206c619: 48c7851009000036000000 mov qword ptr [rbp + 0x910], 0x36
14206c624: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206c62f: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206c63a: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206c645: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206c650: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206c65b: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206c665: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206c66e: 48899d980a0000 mov qword ptr [rbp + 0xa98], rbx
14206c675: c785d80a000000000000 mov dword ptr [rbp + 0xad8], 0
14206c67f: 0fb60583855c01 movzx eax, byte ptr [rip + 0x15c8583]
14206c686: b978020000 mov ecx, 0x278
14206c68b: ba08000000 mov edx, 8
14206c690: e87b755afe call 0x140613c10 ; unknown
14206c695: 4885c0 test rax, rax
14206c698: 0f84c71b0000 je 0x14206e265
14206c69e: 4889c7 mov rdi, rax
14206c6a1: 488d9568080000 lea rdx, [rbp + 0x868]
14206c6a8: 41b878020000 mov r8d, 0x278
14206c6ae: 4889c1 mov rcx, rax
14206c6b1: e84a758b00 call 0x142923c00 ; unknown
14206c6b6: 4c8d050b5cf600 lea r8, [rip + 0xf65c0b]
14206c6bd: 488d8d50050000 lea rcx, [rbp + 0x550]
14206c6c4: 4889fa mov rdx, rdi
14206c6c7: e84484c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206c6cc: 0f57c0 xorps xmm0, xmm0
14206c6cf: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206c6d6: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206c6dd: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206c6e4: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206c6eb: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206c6f2: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206c6fc: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206c703: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206c70a: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206c711: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206c718: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206c71f: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206c729: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206c734: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206c73f: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206c74a: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206c755: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206c760: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206c76b: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206c776: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206c781: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206c788: 488d055560f600 lea rax, [rip + 0xf66055]
14206c78f: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206c796: 48c7851009000041000000 mov qword ptr [rbp + 0x910], 0x41
14206c7a1: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206c7ac: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206c7b7: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206c7c2: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206c7cd: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206c7d8: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206c7e2: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206c7eb: 48899d980a0000 mov qword ptr [rbp + 0xa98], rbx
14206c7f2: c785080b000000000000 mov dword ptr [rbp + 0xb08], 0
14206c7fc: 0fb60506845c01 movzx eax, byte ptr [rip + 0x15c8406]
14206c803: b9a8020000 mov ecx, 0x2a8
14206c808: ba08000000 mov edx, 8
14206c80d: e8fe735afe call 0x140613c10 ; unknown
14206c812: 4885c0 test rax, rax
14206c815: 0f845e1a0000 je 0x14206e279
14206c81b: 4889c7 mov rdi, rax
14206c81e: 488d9568080000 lea rdx, [rbp + 0x868]
14206c825: 41b8a8020000 mov r8d, 0x2a8
14206c82b: 4889c1 mov rcx, rax
14206c82e: e8cd738b00 call 0x142923c00 ; unknown
14206c833: 4c8d055e53f600 lea r8, [rip + 0xf6535e]
14206c83a: 488d8de0040000 lea rcx, [rbp + 0x4e0]
14206c841: 4889fa mov rdx, rdi
14206c844: e8c782c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206c849: 0f57c0 xorps xmm0, xmm0
14206c84c: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206c853: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206c85a: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206c861: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206c868: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206c86f: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206c879: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206c880: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206c887: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206c88e: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206c895: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206c89c: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206c8a6: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206c8b1: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206c8bc: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206c8c7: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206c8d2: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206c8dd: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206c8e8: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206c8f3: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206c8fe: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206c905: 488d058c61f600 lea rax, [rip + 0xf6618c]
14206c90c: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206c913: 48c7851009000041000000 mov qword ptr [rbp + 0x910], 0x41
14206c91e: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206c929: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206c934: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206c93f: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206c94a: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206c955: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206c95f: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206c968: 48899d980a0000 mov qword ptr [rbp + 0xa98], rbx
14206c96f: c785e00a000000000000 mov dword ptr [rbp + 0xae0], 0
14206c979: 0fb60589825c01 movzx eax, byte ptr [rip + 0x15c8289]
14206c980: b980020000 mov ecx, 0x280
14206c985: ba08000000 mov edx, 8
14206c98a: e881725afe call 0x140613c10 ; unknown
14206c98f: 4885c0 test rax, rax
14206c992: 0f84f5180000 je 0x14206e28d
14206c998: 4889c7 mov rdi, rax
14206c99b: 488d9568080000 lea rdx, [rbp + 0x868]
14206c9a2: 41b880020000 mov r8d, 0x280
14206c9a8: 4889c1 mov rcx, rax
14206c9ab: e850728b00 call 0x142923c00 ; unknown
14206c9b0: 4c8d05914df600 lea r8, [rip + 0xf64d91]
14206c9b7: 488d8d70040000 lea rcx, [rbp + 0x470]
14206c9be: 4889fa mov rdx, rdi
14206c9c1: e84a81c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206c9c6: 0f57c0 xorps xmm0, xmm0
14206c9c9: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206c9d0: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206c9d7: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206c9de: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206c9e5: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206c9ec: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206c9f6: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206c9fd: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206ca04: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206ca0b: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206ca12: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206ca19: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206ca23: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206ca2e: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206ca39: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206ca44: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206ca4f: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206ca5a: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206ca65: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206ca70: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206ca7b: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206ca82: 488d05c55ff600 lea rax, [rip + 0xf65fc5]
14206ca89: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206ca90: 48c785100900004a000000 mov qword ptr [rbp + 0x910], 0x4a
14206ca9b: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206caa6: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206cab1: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206cabc: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206cac7: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206cad2: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206cadc: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206cae5: 48899d980a0000 mov qword ptr [rbp + 0xa98], rbx
14206caec: c785e00a000000000000 mov dword ptr [rbp + 0xae0], 0
14206caf6: 0fb6050c815c01 movzx eax, byte ptr [rip + 0x15c810c]
14206cafd: b980020000 mov ecx, 0x280
14206cb02: ba08000000 mov edx, 8
14206cb07: e804715afe call 0x140613c10 ; unknown
14206cb0c: 4885c0 test rax, rax
14206cb0f: 0f848c170000 je 0x14206e2a1
14206cb15: 4889c7 mov rdi, rax
14206cb18: 488d9568080000 lea rdx, [rbp + 0x868]
14206cb1f: 41b880020000 mov r8d, 0x280
14206cb25: 4889c1 mov rcx, rax
14206cb28: e8d3708b00 call 0x142923c00 ; unknown
14206cb2d: 4c8d05d451f600 lea r8, [rip + 0xf651d4]
14206cb34: 488d8d00040000 lea rcx, [rbp + 0x400]
14206cb3b: 4889fa mov rdx, rdi
14206cb3e: e8cd7fc7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206cb43: 0f57c0 xorps xmm0, xmm0
14206cb46: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206cb4d: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206cb54: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206cb5b: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206cb62: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206cb69: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206cb73: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206cb7a: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206cb81: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206cb88: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206cb8f: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206cb96: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206cba0: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206cbab: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206cbb6: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206cbc1: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206cbcc: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206cbd7: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206cbe2: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206cbed: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206cbf8: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206cbff: 488d05285bf600 lea rax, [rip + 0xf65b28]
14206cc06: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206cc0d: 48c7851009000035000000 mov qword ptr [rbp + 0x910], 0x35
14206cc18: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206cc23: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206cc2e: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206cc39: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206cc44: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206cc4f: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206cc59: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206cc62: 48899de0090000 mov qword ptr [rbp + 0x9e0], rbx
14206cc69: c785e80a000000000000 mov dword ptr [rbp + 0xae8], 0
14206cc73: 0fb6058f7f5c01 movzx eax, byte ptr [rip + 0x15c7f8f]
14206cc7a: b988020000 mov ecx, 0x288
14206cc7f: ba08000000 mov edx, 8
14206cc84: e8876f5afe call 0x140613c10 ; unknown
14206cc89: 4885c0 test rax, rax
14206cc8c: 0f8423160000 je 0x14206e2b5
14206cc92: 4889c7 mov rdi, rax
14206cc95: 488d9568080000 lea rdx, [rbp + 0x868]
14206cc9c: 41b888020000 mov r8d, 0x288
14206cca2: 4889c1 mov rcx, rax
14206cca5: e8566f8b00 call 0x142923c00 ; unknown
14206ccaa: 4c8d05a754f600 lea r8, [rip + 0xf654a7]
14206ccb1: 488d8d90030000 lea rcx, [rbp + 0x390]
14206ccb8: 4889fa mov rdx, rdi
14206ccbb: e8507ec7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206ccc0: 0f57c0 xorps xmm0, xmm0
14206ccc3: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206ccca: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206ccd1: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206ccd8: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206ccdf: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206cce6: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206ccf0: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206ccf7: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206ccfe: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206cd05: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206cd0c: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206cd13: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206cd1d: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206cd28: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206cd33: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206cd3e: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206cd49: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206cd54: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206cd5f: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206cd6a: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206cd75: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206cd7c: 488d05915bf600 lea rax, [rip + 0xf65b91]
14206cd83: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206cd8a: 48c7851009000041000000 mov qword ptr [rbp + 0x910], 0x41
14206cd95: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206cda0: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206cdab: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206cdb6: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206cdc1: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206cdcc: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206cdd6: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206cddf: 48899de0090000 mov qword ptr [rbp + 0x9e0], rbx
14206cde6: c785f00a000000000000 mov dword ptr [rbp + 0xaf0], 0
14206cdf0: 0fb605127e5c01 movzx eax, byte ptr [rip + 0x15c7e12]
14206cdf7: b990020000 mov ecx, 0x290
14206cdfc: ba08000000 mov edx, 8
14206ce01: e80a6e5afe call 0x140613c10 ; unknown
14206ce06: 4885c0 test rax, rax
14206ce09: 0f84ba140000 je 0x14206e2c9
14206ce0f: 4889c7 mov rdi, rax
14206ce12: 488d9568080000 lea rdx, [rbp + 0x868]
14206ce19: 41b890020000 mov r8d, 0x290
14206ce1f: 4889c1 mov rcx, rax
14206ce22: e8d96d8b00 call 0x142923c00 ; unknown
14206ce27: 4c8d05224ef600 lea r8, [rip + 0xf64e22]
14206ce2e: 488d8d20030000 lea rcx, [rbp + 0x320]
14206ce35: 4889fa mov rdx, rdi
14206ce38: e8d37cc7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206ce3d: 0f57c0 xorps xmm0, xmm0
14206ce40: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206ce47: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206ce4e: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206ce55: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206ce5c: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206ce63: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206ce6d: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206ce74: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206ce7b: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206ce82: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206ce89: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206ce90: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206ce9a: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206cea5: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206ceb0: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206cebb: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206cec6: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206ced1: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206cedc: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206cee7: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206cef2: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206cef9: 488d05555af600 lea rax, [rip + 0xf65a55]
14206cf00: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206cf07: 48c7851009000038000000 mov qword ptr [rbp + 0x910], 0x38
14206cf12: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206cf1d: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206cf28: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206cf33: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206cf3e: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206cf49: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206cf53: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206cf5c: 48899de0090000 mov qword ptr [rbp + 0x9e0], rbx
14206cf63: c785f80a000000000000 mov dword ptr [rbp + 0xaf8], 0
14206cf6d: 0fb605957c5c01 movzx eax, byte ptr [rip + 0x15c7c95]
14206cf74: b998020000 mov ecx, 0x298
14206cf79: ba08000000 mov edx, 8
14206cf7e: e88d6c5afe call 0x140613c10 ; unknown
14206cf83: 4885c0 test rax, rax
14206cf86: 0f8451130000 je 0x14206e2dd
14206cf8c: 4889c7 mov rdi, rax
14206cf8f: 488d9568080000 lea rdx, [rbp + 0x868]
14206cf96: 41b898020000 mov r8d, 0x298
14206cf9c: 4889c1 mov rcx, rax
14206cf9f: e85c6c8b00 call 0x142923c00 ; unknown
14206cfa4: 4c8d056552f600 lea r8, [rip + 0xf65265]
14206cfab: 488d8db0020000 lea rcx, [rbp + 0x2b0]
14206cfb2: 4889fa mov rdx, rdi
14206cfb5: e8567bc7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206cfba: 0f57c0 xorps xmm0, xmm0
14206cfbd: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206cfc4: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206cfcb: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206cfd2: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206cfd9: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206cfe0: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206cfea: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206cff1: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206cff8: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206cfff: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206d006: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206d00d: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206d017: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206d022: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206d02d: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206d038: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206d043: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206d04e: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206d059: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206d064: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206d06f: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206d076: 488d052b55f600 lea rax, [rip + 0xf6552b]
14206d07d: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206d084: 48c7851009000036000000 mov qword ptr [rbp + 0x910], 0x36
14206d08f: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206d09a: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206d0a5: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206d0b0: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206d0bb: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206d0c6: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206d0d0: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206d0d9: 48899de0090000 mov qword ptr [rbp + 0x9e0], rbx
14206d0e0: c785f00a000000000000 mov dword ptr [rbp + 0xaf0], 0
14206d0ea: 0fb605187b5c01 movzx eax, byte ptr [rip + 0x15c7b18]
14206d0f1: b990020000 mov ecx, 0x290
14206d0f6: ba08000000 mov edx, 8
14206d0fb: e8106b5afe call 0x140613c10 ; unknown
14206d100: 4885c0 test rax, rax
14206d103: 0f84e5110000 je 0x14206e2ee
14206d109: 4889c7 mov rdi, rax
14206d10c: 488d9568080000 lea rdx, [rbp + 0x868]
14206d113: 41b890020000 mov r8d, 0x290
14206d119: 4889c1 mov rcx, rax
14206d11c: e8df6a8b00 call 0x142923c00 ; unknown
14206d121: 4c8d055852f600 lea r8, [rip + 0xf65258]
14206d128: 488d8d40020000 lea rcx, [rbp + 0x240]
14206d12f: 4889fa mov rdx, rdi
14206d132: e8d979c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206d137: 0f57c0 xorps xmm0, xmm0
14206d13a: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206d141: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206d148: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206d14f: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206d156: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206d15d: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206d167: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206d16e: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206d175: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206d17c: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206d183: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206d18a: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206d194: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206d19f: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206d1aa: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206d1b5: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206d1c0: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206d1cb: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206d1d6: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206d1e1: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206d1ec: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206d1f3: 488d05fe54f600 lea rax, [rip + 0xf654fe]
14206d1fa: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206d201: 48c7851009000036000000 mov qword ptr [rbp + 0x910], 0x36
14206d20c: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206d217: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206d222: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206d22d: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206d238: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206d243: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206d24d: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206d256: 48899d980a0000 mov qword ptr [rbp + 0xa98], rbx
14206d25d: c785e00a000000000000 mov dword ptr [rbp + 0xae0], 0
14206d267: 0fb6059b795c01 movzx eax, byte ptr [rip + 0x15c799b]
14206d26e: b980020000 mov ecx, 0x280
14206d273: ba08000000 mov edx, 8
14206d278: e893695afe call 0x140613c10 ; unknown
14206d27d: 4885c0 test rax, rax
14206d280: 0f8479100000 je 0x14206e2ff
14206d286: 4889c7 mov rdi, rax
14206d289: 488d9568080000 lea rdx, [rbp + 0x868]
14206d290: 41b880020000 mov r8d, 0x280
14206d296: 4889c1 mov rcx, rax
14206d299: e862698b00 call 0x142923c00 ; unknown
14206d29e: 4c8d055b45f600 lea r8, [rip + 0xf6455b]
14206d2a5: 488d8dd0010000 lea rcx, [rbp + 0x1d0]
14206d2ac: 4889fa mov rdx, rdi
14206d2af: e85c78c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206d2b4: 0f57c0 xorps xmm0, xmm0
14206d2b7: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206d2be: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206d2c5: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206d2cc: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206d2d3: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206d2da: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206d2e4: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206d2eb: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206d2f2: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206d2f9: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206d300: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206d307: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206d311: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206d31c: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206d327: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206d332: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206d33d: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206d348: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206d353: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206d35e: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206d369: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206d370: 488d056055f600 lea rax, [rip + 0xf65560]
14206d377: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206d37e: 48c785100900003d000000 mov qword ptr [rbp + 0x910], 0x3d
14206d389: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206d394: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206d39f: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206d3aa: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206d3b5: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206d3c0: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206d3ca: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206d3d3: 48899de0090000 mov qword ptr [rbp + 0x9e0], rbx
14206d3da: c785e80a000000000000 mov dword ptr [rbp + 0xae8], 0
14206d3e4: 0fb6051e785c01 movzx eax, byte ptr [rip + 0x15c781e]
14206d3eb: b988020000 mov ecx, 0x288
14206d3f0: ba08000000 mov edx, 8
14206d3f5: e816685afe call 0x140613c10 ; unknown
14206d3fa: 4885c0 test rax, rax
14206d3fd: 0f840d0f0000 je 0x14206e310
14206d403: 4889c7 mov rdi, rax
14206d406: 488d9568080000 lea rdx, [rbp + 0x868]
14206d40d: 41b888020000 mov r8d, 0x288
14206d413: 4889c1 mov rcx, rax
14206d416: e8e5678b00 call 0x142923c00 ; unknown
14206d41b: 4c8d050646f600 lea r8, [rip + 0xf64606]
14206d422: 488d8d60010000 lea rcx, [rbp + 0x160]
14206d429: 4889fa mov rdx, rdi
14206d42c: e8df76c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206d431: 0f57c0 xorps xmm0, xmm0
14206d434: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206d43b: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206d442: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206d449: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206d450: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206d457: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206d461: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206d468: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206d46f: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206d476: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206d47d: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206d484: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206d48e: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206d499: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206d4a4: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206d4af: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206d4ba: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206d4c5: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206d4d0: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206d4db: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206d4e6: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206d4ed: 488d052555f600 lea rax, [rip + 0xf65525]
14206d4f4: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206d4fb: 48c7851009000035000000 mov qword ptr [rbp + 0x910], 0x35
14206d506: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206d511: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206d51c: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206d527: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206d532: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206d53d: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206d547: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206d550: 48899de0090000 mov qword ptr [rbp + 0x9e0], rbx
14206d557: c785f00a000000000000 mov dword ptr [rbp + 0xaf0], 0
14206d561: 0fb605a1765c01 movzx eax, byte ptr [rip + 0x15c76a1]
14206d568: b990020000 mov ecx, 0x290
14206d56d: ba08000000 mov edx, 8
14206d572: e899665afe call 0x140613c10 ; unknown
14206d577: 4885c0 test rax, rax
14206d57a: 0f84a10d0000 je 0x14206e321
14206d580: 4889c7 mov rdi, rax
14206d583: 488d9568080000 lea rdx, [rbp + 0x868]
14206d58a: 41b890020000 mov r8d, 0x290
14206d590: 4889c1 mov rcx, rax
14206d593: e868668b00 call 0x142923c00 ; unknown
14206d598: 4c8d05d948f600 lea r8, [rip + 0xf648d9]
14206d59f: 488d8df0000000 lea rcx, [rbp + 0xf0]
14206d5a6: 4889fa mov rdx, rdi
14206d5a9: e86275c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206d5ae: 0f57c0 xorps xmm0, xmm0
14206d5b1: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206d5b8: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206d5bf: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206d5c6: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206d5cd: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206d5d4: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206d5de: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206d5e5: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206d5ec: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206d5f3: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206d5fa: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206d601: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206d60b: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206d616: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206d621: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206d62c: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206d637: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206d642: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206d64d: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206d658: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206d663: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206d66a: 488d051c53f600 lea rax, [rip + 0xf6531c]
14206d671: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206d678: 48c7851009000042000000 mov qword ptr [rbp + 0x910], 0x42
14206d683: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206d68e: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206d699: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206d6a4: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206d6af: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206d6ba: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206d6c4: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206d6cd: 48899d900a0000 mov qword ptr [rbp + 0xa90], rbx
14206d6d4: c785080c000000000000 mov dword ptr [rbp + 0xc08], 0
14206d6de: 0fb60524755c01 movzx eax, byte ptr [rip + 0x15c7524]
14206d6e5: b9a8030000 mov ecx, 0x3a8
14206d6ea: ba08000000 mov edx, 8
14206d6ef: e81c655afe call 0x140613c10 ; unknown
14206d6f4: 4885c0 test rax, rax
14206d6f7: 0f84350c0000 je 0x14206e332
14206d6fd: 4889c7 mov rdi, rax
14206d700: 488d9568080000 lea rdx, [rbp + 0x868]
14206d707: 41b8a8030000 mov r8d, 0x3a8
14206d70d: 4889c1 mov rcx, rax
14206d710: e8eb648b00 call 0x142923c00 ; unknown
14206d715: 4c8d059c41f600 lea r8, [rip + 0xf6419c]
14206d71c: 488d8d80000000 lea rcx, [rbp + 0x80]
14206d723: 4889fa mov rdx, rdi
14206d726: e8e573c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206d72b: 0f57c0 xorps xmm0, xmm0
14206d72e: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206d735: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206d73c: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206d743: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206d74a: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206d751: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206d75b: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206d762: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206d769: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206d770: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206d777: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206d77e: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206d788: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206d793: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206d79e: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206d7a9: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206d7b4: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206d7bf: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206d7ca: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206d7d5: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206d7e0: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206d7e7: 488d05eb52f600 lea rax, [rip + 0xf652eb]
14206d7ee: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206d7f5: 48c785100900003e000000 mov qword ptr [rbp + 0x910], 0x3e
14206d800: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206d80b: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206d816: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206d821: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206d82c: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206d837: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206d841: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206d84a: 48899d980a0000 mov qword ptr [rbp + 0xa98], rbx
14206d851: c785e00a000000000000 mov dword ptr [rbp + 0xae0], 0
14206d85b: 0fb605a7735c01 movzx eax, byte ptr [rip + 0x15c73a7]
14206d862: b980020000 mov ecx, 0x280
14206d867: ba08000000 mov edx, 8
14206d86c: e89f635afe call 0x140613c10 ; unknown
14206d871: 4885c0 test rax, rax
14206d874: 0f84c90a0000 je 0x14206e343
14206d87a: 4889c7 mov rdi, rax
14206d87d: 488d9568080000 lea rdx, [rbp + 0x868]
14206d884: 41b880020000 mov r8d, 0x280
14206d88a: 4889c1 mov rcx, rax
14206d88d: e86e638b00 call 0x142923c00 ; unknown
14206d892: 4c8d059f4bf600 lea r8, [rip + 0xf64b9f]
14206d899: 488d4d10 lea rcx, [rbp + 0x10]
14206d89d: 4889fa mov rdx, rdi
14206d8a0: e86b72c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206d8a5: 0f57c0 xorps xmm0, xmm0
14206d8a8: 0f118588080000 movups xmmword ptr [rbp + 0x888], xmm0
14206d8af: 0f1185a0080000 movups xmmword ptr [rbp + 0x8a0], xmm0
14206d8b6: 0f1185b8080000 movups xmmword ptr [rbp + 0x8b8], xmm0
14206d8bd: 0f1185d0080000 movups xmmword ptr [rbp + 0x8d0], xmm0
14206d8c4: 0f1185e8080000 movups xmmword ptr [rbp + 0x8e8], xmm0
14206d8cb: c785f808000000000000 mov dword ptr [rbp + 0x8f8], 0
14206d8d5: 0f118520090000 movups xmmword ptr [rbp + 0x920], xmm0
14206d8dc: 0f118538090000 movups xmmword ptr [rbp + 0x938], xmm0
14206d8e3: 0f118550090000 movups xmmword ptr [rbp + 0x950], xmm0
14206d8ea: 0f118568090000 movups xmmword ptr [rbp + 0x968], xmm0
14206d8f1: 0f118580090000 movups xmmword ptr [rbp + 0x980], xmm0
14206d8f8: c7859009000000000000 mov dword ptr [rbp + 0x990], 0
14206d902: 48c7856808000000000000 mov qword ptr [rbp + 0x868], 0
14206d90d: 48c7857008000008000000 mov qword ptr [rbp + 0x870], 8
14206d918: 48c7857808000000000000 mov qword ptr [rbp + 0x878], 0
14206d923: 48c7858008000010000000 mov qword ptr [rbp + 0x880], 0x10
14206d92e: 48c7859808000010000000 mov qword ptr [rbp + 0x898], 0x10
14206d939: 48c785b008000010000000 mov qword ptr [rbp + 0x8b0], 0x10
14206d944: 48c785c808000010000000 mov qword ptr [rbp + 0x8c8], 0x10
14206d94f: 48c785e008000010000000 mov qword ptr [rbp + 0x8e0], 0x10
14206d95a: 48899d00090000 mov qword ptr [rbp + 0x900], rbx
14206d961: 488d05c64cf600 lea rax, [rip + 0xf64cc6]
14206d968: 48898508090000 mov qword ptr [rbp + 0x908], rax
14206d96f: 48c785100900003d000000 mov qword ptr [rbp + 0x910], 0x3d
14206d97a: 48c7851809000010000000 mov qword ptr [rbp + 0x918], 0x10
14206d985: 48c7853009000010000000 mov qword ptr [rbp + 0x930], 0x10
14206d990: 48c7854809000010000000 mov qword ptr [rbp + 0x948], 0x10
14206d99b: 48c7856009000010000000 mov qword ptr [rbp + 0x960], 0x10
14206d9a6: 48c7857809000010000000 mov qword ptr [rbp + 0x978], 0x10
14206d9b1: c7859809000000000000 mov dword ptr [rbp + 0x998], 0
14206d9bb: 66c7859c0900000100 mov word ptr [rbp + 0x99c], 1
14206d9c4: 48899de0090000 mov qword ptr [rbp + 0x9e0], rbx
14206d9cb: c785e80a000000000000 mov dword ptr [rbp + 0xae8], 0
14206d9d5: 0fb6052d725c01 movzx eax, byte ptr [rip + 0x15c722d]
14206d9dc: b988020000 mov ecx, 0x288
14206d9e1: ba08000000 mov edx, 8
14206d9e6: e825625afe call 0x140613c10 ; unknown
14206d9eb: 4885c0 test rax, rax
14206d9ee: 0f8460090000 je 0x14206e354
14206d9f4: 4889c7 mov rdi, rax
14206d9f7: 488d9568080000 lea rdx, [rbp + 0x868]
14206d9fe: 41b888020000 mov r8d, 0x288
14206da04: 4889c1 mov rcx, rax
14206da07: e8f4618b00 call 0x142923c00 ; unknown
14206da0c: 4c8d055d3ff600 lea r8, [rip + 0xf63f5d]
14206da13: 488d4da0 lea rcx, [rbp - 0x60]
14206da17: 4889fa mov rdx, rdi
14206da1a: e8f170c7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206da1f: 0f108550080000 movups xmm0, xmmword ptr [rbp + 0x850]
14206da26: 488b8560080000 mov rax, qword ptr [rbp + 0x860]
14206da2d: 0f114060 movups xmmword ptr [rax + 0x60], xmm0
14206da31: 0f108540080000 movups xmm0, xmmword ptr [rbp + 0x840]
14206da38: 0f114050 movups xmmword ptr [rax + 0x50], xmm0
14206da3c: 0f108530080000 movups xmm0, xmmword ptr [rbp + 0x830]
14206da43: 0f114040 movups xmmword ptr [rax + 0x40], xmm0
14206da47: 0f1085f0070000 movups xmm0, xmmword ptr [rbp + 0x7f0]
14206da4e: 0f108d00080000 movups xmm1, xmmword ptr [rbp + 0x800]
14206da55: 0f109510080000 movups xmm2, xmmword ptr [rbp + 0x810]
14206da5c: 0f109d20080000 movups xmm3, xmmword ptr [rbp + 0x820]
14206da63: 0f115830 movups xmmword ptr [rax + 0x30], xmm3
14206da67: 0f115020 movups xmmword ptr [rax + 0x20], xmm2
14206da6b: 0f114810 movups xmmword ptr [rax + 0x10], xmm1
14206da6f: 0f1100 movups xmmword ptr [rax], xmm0
14206da72: 0f108580070000 movups xmm0, xmmword ptr [rbp + 0x780]
14206da79: 0f108d90070000 movups xmm1, xmmword ptr [rbp + 0x790]
14206da80: 0f1095a0070000 movups xmm2, xmmword ptr [rbp + 0x7a0]
14206da87: 0f109db0070000 movups xmm3, xmmword ptr [rbp + 0x7b0]
14206da8e: 0f114070 movups xmmword ptr [rax + 0x70], xmm0
14206da92: 0f118880000000 movups xmmword ptr [rax + 0x80], xmm1
14206da99: 0f119090000000 movups xmmword ptr [rax + 0x90], xmm2
14206daa0: 0f1198a0000000 movups xmmword ptr [rax + 0xa0], xmm3
14206daa7: 0f1085c0070000 movups xmm0, xmmword ptr [rbp + 0x7c0]
14206daae: 0f1180b0000000 movups xmmword ptr [rax + 0xb0], xmm0
14206dab5: 0f1085d0070000 movups xmm0, xmmword ptr [rbp + 0x7d0]
14206dabc: 0f1180c0000000 movups xmmword ptr [rax + 0xc0], xmm0
14206dac3: 0f1085e0070000 movups xmm0, xmmword ptr [rbp + 0x7e0]
14206daca: 0f1180d0000000 movups xmmword ptr [rax + 0xd0], xmm0
14206dad1: 0f108570070000 movups xmm0, xmmword ptr [rbp + 0x770]
14206dad8: 0f118040010000 movups xmmword ptr [rax + 0x140], xmm0
14206dadf: 0f108560070000 movups xmm0, xmmword ptr [rbp + 0x760]
14206dae6: 0f118030010000 movups xmmword ptr [rax + 0x130], xmm0
14206daed: 0f108550070000 movups xmm0, xmmword ptr [rbp + 0x750]
14206daf4: 0f118020010000 movups xmmword ptr [rax + 0x120], xmm0
14206dafb: 0f108510070000 movups xmm0, xmmword ptr [rbp + 0x710]
14206db02: 0f108d20070000 movups xmm1, xmmword ptr [rbp + 0x720]
14206db09: 0f109530070000 movups xmm2, xmmword ptr [rbp + 0x730]
14206db10: 0f109d40070000 movups xmm3, xmmword ptr [rbp + 0x740]
14206db17: 0f119810010000 movups xmmword ptr [rax + 0x110], xmm3
14206db1e: 0f119000010000 movups xmmword ptr [rax + 0x100], xmm2
14206db25: 0f1188f0000000 movups xmmword ptr [rax + 0xf0], xmm1
14206db2c: 0f1180e0000000 movups xmmword ptr [rax + 0xe0], xmm0
14206db33: 0f1085a0060000 movups xmm0, xmmword ptr [rbp + 0x6a0]
14206db3a: 0f108db0060000 movups xmm1, xmmword ptr [rbp + 0x6b0]
14206db41: 0f1095c0060000 movups xmm2, xmmword ptr [rbp + 0x6c0]
14206db48: 0f109dd0060000 movups xmm3, xmmword ptr [rbp + 0x6d0]
14206db4f: 0f118050010000 movups xmmword ptr [rax + 0x150], xmm0
14206db56: 0f118860010000 movups xmmword ptr [rax + 0x160], xmm1
14206db5d: 0f119070010000 movups xmmword ptr [rax + 0x170], xmm2
14206db64: 0f119880010000 movups xmmword ptr [rax + 0x180], xmm3
14206db6b: 0f1085e0060000 movups xmm0, xmmword ptr [rbp + 0x6e0]
14206db72: 0f118090010000 movups xmmword ptr [rax + 0x190], xmm0
14206db79: 0f1085f0060000 movups xmm0, xmmword ptr [rbp + 0x6f0]
14206db80: 0f1180a0010000 movups xmmword ptr [rax + 0x1a0], xmm0
14206db87: 0f108500070000 movups xmm0, xmmword ptr [rbp + 0x700]
14206db8e: 0f1180b0010000 movups xmmword ptr [rax + 0x1b0], xmm0
14206db95: 0f108530060000 movups xmm0, xmmword ptr [rbp + 0x630]
14206db9c: 0f108d40060000 movups xmm1, xmmword ptr [rbp + 0x640]
14206dba3: 0f109550060000 movups xmm2, xmmword ptr [rbp + 0x650]
14206dbaa: 0f109d60060000 movups xmm3, xmmword ptr [rbp + 0x660]
14206dbb1: 0f1180c0010000 movups xmmword ptr [rax + 0x1c0], xmm0
14206dbb8: 0f1188d0010000 movups xmmword ptr [rax + 0x1d0], xmm1
14206dbbf: 0f1190e0010000 movups xmmword ptr [rax + 0x1e0], xmm2
14206dbc6: 0f1198f0010000 movups xmmword ptr [rax + 0x1f0], xmm3
14206dbcd: 0f108570060000 movups xmm0, xmmword ptr [rbp + 0x670]
14206dbd4: 0f118000020000 movups xmmword ptr [rax + 0x200], xmm0
14206dbdb: 0f108590060000 movups xmm0, xmmword ptr [rbp + 0x690]
14206dbe2: 0f118020020000 movups xmmword ptr [rax + 0x220], xmm0
14206dbe9: 0f108580060000 movups xmm0, xmmword ptr [rbp + 0x680]
14206dbf0: 0f118010020000 movups xmmword ptr [rax + 0x210], xmm0
14206dbf7: 0f108520060000 movups xmm0, xmmword ptr [rbp + 0x620]
14206dbfe: 0f118090020000 movups xmmword ptr [rax + 0x290], xmm0
14206dc05: 0f108510060000 movups xmm0, xmmword ptr [rbp + 0x610]
14206dc0c: 0f118080020000 movups xmmword ptr [rax + 0x280], xmm0
14206dc13: 0f108500060000 movups xmm0, xmmword ptr [rbp + 0x600]
14206dc1a: 0f118070020000 movups xmmword ptr [rax + 0x270], xmm0
14206dc21: 0f1085c0050000 movups xmm0, xmmword ptr [rbp + 0x5c0]
14206dc28: 0f108dd0050000 movups xmm1, xmmword ptr [rbp + 0x5d0]
14206dc2f: 0f1095e0050000 movups xmm2, xmmword ptr [rbp + 0x5e0]
14206dc36: 0f109df0050000 movups xmm3, xmmword ptr [rbp + 0x5f0]
14206dc3d: 0f119860020000 movups xmmword ptr [rax + 0x260], xmm3
14206dc44: 0f119050020000 movups xmmword ptr [rax + 0x250], xmm2
14206dc4b: 0f118840020000 movups xmmword ptr [rax + 0x240], xmm1
14206dc52: 0f118030020000 movups xmmword ptr [rax + 0x230], xmm0
14206dc59: 0f108550050000 movups xmm0, xmmword ptr [rbp + 0x550]
14206dc60: 0f108d60050000 movups xmm1, xmmword ptr [rbp + 0x560]
14206dc67: 0f109570050000 movups xmm2, xmmword ptr [rbp + 0x570]
14206dc6e: 0f109d80050000 movups xmm3, xmmword ptr [rbp + 0x580]
14206dc75: 0f1180a0020000 movups xmmword ptr [rax + 0x2a0], xmm0
14206dc7c: 0f1188b0020000 movups xmmword ptr [rax + 0x2b0], xmm1
14206dc83: 0f1190c0020000 movups xmmword ptr [rax + 0x2c0], xmm2
14206dc8a: 0f1198d0020000 movups xmmword ptr [rax + 0x2d0], xmm3
14206dc91: 0f108590050000 movups xmm0, xmmword ptr [rbp + 0x590]
14206dc98: 0f1180e0020000 movups xmmword ptr [rax + 0x2e0], xmm0
14206dc9f: 0f1085a0050000 movups xmm0, xmmword ptr [rbp + 0x5a0]
14206dca6: 0f1180f0020000 movups xmmword ptr [rax + 0x2f0], xmm0
14206dcad: 0f1085b0050000 movups xmm0, xmmword ptr [rbp + 0x5b0]
14206dcb4: 0f118000030000 movups xmmword ptr [rax + 0x300], xmm0
14206dcbb: 0f1085e0040000 movups xmm0, xmmword ptr [rbp + 0x4e0]
14206dcc2: 0f108df0040000 movups xmm1, xmmword ptr [rbp + 0x4f0]
14206dcc9: 0f109500050000 movups xmm2, xmmword ptr [rbp + 0x500]
14206dcd0: 0f109d10050000 movups xmm3, xmmword ptr [rbp + 0x510]
14206dcd7: 0f118010030000 movups xmmword ptr [rax + 0x310], xmm0
14206dcde: 0f118820030000 movups xmmword ptr [rax + 0x320], xmm1
14206dce5: 0f119030030000 movups xmmword ptr [rax + 0x330], xmm2
14206dcec: 0f119840030000 movups xmmword ptr [rax + 0x340], xmm3
14206dcf3: 0f108520050000 movups xmm0, xmmword ptr [rbp + 0x520]
14206dcfa: 0f118050030000 movups xmmword ptr [rax + 0x350], xmm0
14206dd01: 0f108530050000 movups xmm0, xmmword ptr [rbp + 0x530]
14206dd08: 0f118060030000 movups xmmword ptr [rax + 0x360], xmm0
14206dd0f: 0f108540050000 movups xmm0, xmmword ptr [rbp + 0x540]
14206dd16: 0f118070030000 movups xmmword ptr [rax + 0x370], xmm0
14206dd1d: 0f108570040000 movups xmm0, xmmword ptr [rbp + 0x470]
14206dd24: 0f108d80040000 movups xmm1, xmmword ptr [rbp + 0x480]
14206dd2b: 0f109590040000 movups xmm2, xmmword ptr [rbp + 0x490]
14206dd32: 0f109da0040000 movups xmm3, xmmword ptr [rbp + 0x4a0]
14206dd39: 0f118080030000 movups xmmword ptr [rax + 0x380], xmm0
14206dd40: 0f118890030000 movups xmmword ptr [rax + 0x390], xmm1
14206dd47: 0f1190a0030000 movups xmmword ptr [rax + 0x3a0], xmm2
14206dd4e: 0f1198b0030000 movups xmmword ptr [rax + 0x3b0], xmm3
14206dd55: 0f1085b0040000 movups xmm0, xmmword ptr [rbp + 0x4b0]
14206dd5c: 0f1180c0030000 movups xmmword ptr [rax + 0x3c0], xmm0
14206dd63: 0f1085c0040000 movups xmm0, xmmword ptr [rbp + 0x4c0]
14206dd6a: 0f1180d0030000 movups xmmword ptr [rax + 0x3d0], xmm0
14206dd71: 0f1085d0040000 movups xmm0, xmmword ptr [rbp + 0x4d0]
14206dd78: 0f1180e0030000 movups xmmword ptr [rax + 0x3e0], xmm0
14206dd7f: 0f108500040000 movups xmm0, xmmword ptr [rbp + 0x400]
14206dd86: 0f108d10040000 movups xmm1, xmmword ptr [rbp + 0x410]
14206dd8d: 0f109520040000 movups xmm2, xmmword ptr [rbp + 0x420]
14206dd94: 0f109d30040000 movups xmm3, xmmword ptr [rbp + 0x430]
14206dd9b: 0f1180f0030000 movups xmmword ptr [rax + 0x3f0], xmm0
14206dda2: 0f118800040000 movups xmmword ptr [rax + 0x400], xmm1
14206dda9: 0f119010040000 movups xmmword ptr [rax + 0x410], xmm2
14206ddb0: 0f119820040000 movups xmmword ptr [rax + 0x420], xmm3
14206ddb7: 0f108540040000 movups xmm0, xmmword ptr [rbp + 0x440]
14206ddbe: 0f118030040000 movups xmmword ptr [rax + 0x430], xmm0
14206ddc5: 0f108550040000 movups xmm0, xmmword ptr [rbp + 0x450]
14206ddcc: 0f118040040000 movups xmmword ptr [rax + 0x440], xmm0
14206ddd3: 0f108560040000 movups xmm0, xmmword ptr [rbp + 0x460]
14206ddda: 0f118050040000 movups xmmword ptr [rax + 0x450], xmm0
14206dde1: 0f108590030000 movups xmm0, xmmword ptr [rbp + 0x390]
14206dde8: 0f108da0030000 movups xmm1, xmmword ptr [rbp + 0x3a0]
14206ddef: 0f1095b0030000 movups xmm2, xmmword ptr [rbp + 0x3b0]
14206ddf6: 0f109dc0030000 movups xmm3, xmmword ptr [rbp + 0x3c0]
14206ddfd: 0f118060040000 movups xmmword ptr [rax + 0x460], xmm0
14206de04: 0f118870040000 movups xmmword ptr [rax + 0x470], xmm1
14206de0b: 0f119080040000 movups xmmword ptr [rax + 0x480], xmm2
14206de12: 0f119890040000 movups xmmword ptr [rax + 0x490], xmm3
14206de19: 0f1085d0030000 movups xmm0, xmmword ptr [rbp + 0x3d0]
14206de20: 0f1180a0040000 movups xmmword ptr [rax + 0x4a0], xmm0
14206de27: 0f1085e0030000 movups xmm0, xmmword ptr [rbp + 0x3e0]
14206de2e: 0f1180b0040000 movups xmmword ptr [rax + 0x4b0], xmm0
14206de35: 0f1085f0030000 movups xmm0, xmmword ptr [rbp + 0x3f0]
14206de3c: 0f1180c0040000 movups xmmword ptr [rax + 0x4c0], xmm0
14206de43: 0f108520030000 movups xmm0, xmmword ptr [rbp + 0x320]
14206de4a: 0f108d30030000 movups xmm1, xmmword ptr [rbp + 0x330]
14206de51: 0f109540030000 movups xmm2, xmmword ptr [rbp + 0x340]
14206de58: 0f109d50030000 movups xmm3, xmmword ptr [rbp + 0x350]
14206de5f: 0f1180d0040000 movups xmmword ptr [rax + 0x4d0], xmm0
14206de66: 0f1188e0040000 movups xmmword ptr [rax + 0x4e0], xmm1
14206de6d: 0f1190f0040000 movups xmmword ptr [rax + 0x4f0], xmm2
14206de74: 0f119800050000 movups xmmword ptr [rax + 0x500], xmm3
14206de7b: 0f108560030000 movups xmm0, xmmword ptr [rbp + 0x360]
14206de82: 0f118010050000 movups xmmword ptr [rax + 0x510], xmm0
14206de89: 0f108570030000 movups xmm0, xmmword ptr [rbp + 0x370]
14206de90: 0f118020050000 movups xmmword ptr [rax + 0x520], xmm0
14206de97: 0f108580030000 movups xmm0, xmmword ptr [rbp + 0x380]
14206de9e: 0f118030050000 movups xmmword ptr [rax + 0x530], xmm0
14206dea5: 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
14206deac: 0f108dc0020000 movups xmm1, xmmword ptr [rbp + 0x2c0]
14206deb3: 0f1095d0020000 movups xmm2, xmmword ptr [rbp + 0x2d0]
14206deba: 0f109de0020000 movups xmm3, xmmword ptr [rbp + 0x2e0]
14206dec1: 0f118040050000 movups xmmword ptr [rax + 0x540], xmm0
14206dec8: 0f118850050000 movups xmmword ptr [rax + 0x550], xmm1
14206decf: 0f119060050000 movups xmmword ptr [rax + 0x560], xmm2
14206ded6: 0f119870050000 movups xmmword ptr [rax + 0x570], xmm3
14206dedd: 0f1085f0020000 movups xmm0, xmmword ptr [rbp + 0x2f0]
14206dee4: 0f118080050000 movups xmmword ptr [rax + 0x580], xmm0
14206deeb: 0f108500030000 movups xmm0, xmmword ptr [rbp + 0x300]
14206def2: 0f118090050000 movups xmmword ptr [rax + 0x590], xmm0
14206def9: 0f108510030000 movups xmm0, xmmword ptr [rbp + 0x310]
14206df00: 0f1180a0050000 movups xmmword ptr [rax + 0x5a0], xmm0
14206df07: 0f108540020000 movups xmm0, xmmword ptr [rbp + 0x240]
14206df0e: 0f108d50020000 movups xmm1, xmmword ptr [rbp + 0x250]
14206df15: 0f109560020000 movups xmm2, xmmword ptr [rbp + 0x260]
14206df1c: 0f109d70020000 movups xmm3, xmmword ptr [rbp + 0x270]
14206df23: 0f1180b0050000 movups xmmword ptr [rax + 0x5b0], xmm0
14206df2a: 0f1188c0050000 movups xmmword ptr [rax + 0x5c0], xmm1
14206df31: 0f1190d0050000 movups xmmword ptr [rax + 0x5d0], xmm2
14206df38: 0f1198e0050000 movups xmmword ptr [rax + 0x5e0], xmm3
14206df3f: 0f108580020000 movups xmm0, xmmword ptr [rbp + 0x280]
14206df46: 0f1180f0050000 movups xmmword ptr [rax + 0x5f0], xmm0
14206df4d: 0f108590020000 movups xmm0, xmmword ptr [rbp + 0x290]
14206df54: 0f118000060000 movups xmmword ptr [rax + 0x600], xmm0
14206df5b: 0f1085a0020000 movups xmm0, xmmword ptr [rbp + 0x2a0]
14206df62: 0f118010060000 movups xmmword ptr [rax + 0x610], xmm0
14206df69: 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
14206df70: 0f108de0010000 movups xmm1, xmmword ptr [rbp + 0x1e0]
14206df77: 0f1095f0010000 movups xmm2, xmmword ptr [rbp + 0x1f0]
14206df7e: 0f109d00020000 movups xmm3, xmmword ptr [rbp + 0x200]
14206df85: 0f118020060000 movups xmmword ptr [rax + 0x620], xmm0
14206df8c: 0f118830060000 movups xmmword ptr [rax + 0x630], xmm1
14206df93: 0f119040060000 movups xmmword ptr [rax + 0x640], xmm2
14206df9a: 0f119850060000 movups xmmword ptr [rax + 0x650], xmm3
14206dfa1: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
14206dfa8: 0f118060060000 movups xmmword ptr [rax + 0x660], xmm0
14206dfaf: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
14206dfb6: 0f118070060000 movups xmmword ptr [rax + 0x670], xmm0
14206dfbd: 0f108530020000 movups xmm0, xmmword ptr [rbp + 0x230]
14206dfc4: 0f118080060000 movups xmmword ptr [rax + 0x680], xmm0
14206dfcb: 0f108560010000 movups xmm0, xmmword ptr [rbp + 0x160]
14206dfd2: 0f108d70010000 movups xmm1, xmmword ptr [rbp + 0x170]
14206dfd9: 0f109580010000 movups xmm2, xmmword ptr [rbp + 0x180]
14206dfe0: 0f109d90010000 movups xmm3, xmmword ptr [rbp + 0x190]
14206dfe7: 0f118090060000 movups xmmword ptr [rax + 0x690], xmm0
14206dfee: 0f1188a0060000 movups xmmword ptr [rax + 0x6a0], xmm1
14206dff5: 0f1190b0060000 movups xmmword ptr [rax + 0x6b0], xmm2
14206dffc: 0f1198c0060000 movups xmmword ptr [rax + 0x6c0], xmm3
14206e003: 0f1085a0010000 movups xmm0, xmmword ptr [rbp + 0x1a0]
14206e00a: 0f1180d0060000 movups xmmword ptr [rax + 0x6d0], xmm0
14206e011: 0f1085b0010000 movups xmm0, xmmword ptr [rbp + 0x1b0]
14206e018: 0f1180e0060000 movups xmmword ptr [rax + 0x6e0], xmm0
14206e01f: 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
14206e026: 0f1180f0060000 movups xmmword ptr [rax + 0x6f0], xmm0
14206e02d: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
14206e034: 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
14206e03b: 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
14206e042: 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
14206e049: 0f118000070000 movups xmmword ptr [rax + 0x700], xmm0
14206e050: 0f118810070000 movups xmmword ptr [rax + 0x710], xmm1
14206e057: 0f119020070000 movups xmmword ptr [rax + 0x720], xmm2
14206e05e: 0f119830070000 movups xmmword ptr [rax + 0x730], xmm3
14206e065: 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
14206e06c: 0f118040070000 movups xmmword ptr [rax + 0x740], xmm0
14206e073: 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
14206e07a: 0f118050070000 movups xmmword ptr [rax + 0x750], xmm0
14206e081: 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
14206e088: 0f118060070000 movups xmmword ptr [rax + 0x760], xmm0
14206e08f: 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
14206e096: 0f108d90000000 movups xmm1, xmmword ptr [rbp + 0x90]
14206e09d: 0f1095a0000000 movups xmm2, xmmword ptr [rbp + 0xa0]
14206e0a4: 0f109db0000000 movups xmm3, xmmword ptr [rbp + 0xb0]
14206e0ab: 0f118070070000 movups xmmword ptr [rax + 0x770], xmm0
14206e0b2: 0f118880070000 movups xmmword ptr [rax + 0x780], xmm1
14206e0b9: 0f119090070000 movups xmmword ptr [rax + 0x790], xmm2
14206e0c0: 0f1198a0070000 movups xmmword ptr [rax + 0x7a0], xmm3
14206e0c7: 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
14206e0ce: 0f1180b0070000 movups xmmword ptr [rax + 0x7b0], xmm0
14206e0d5: 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
14206e0dc: 0f1180c0070000 movups xmmword ptr [rax + 0x7c0], xmm0
14206e0e3: 0f1085e0000000 movups xmm0, xmmword ptr [rbp + 0xe0]
14206e0ea: 0f1180d0070000 movups xmmword ptr [rax + 0x7d0], xmm0
14206e0f1: 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
14206e0f5: 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
14206e0f9: 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
14206e0fd: 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
14206e101: 0f1180e0070000 movups xmmword ptr [rax + 0x7e0], xmm0
14206e108: 0f1188f0070000 movups xmmword ptr [rax + 0x7f0], xmm1
14206e10f: 0f119000080000 movups xmmword ptr [rax + 0x800], xmm2
14206e116: 0f119810080000 movups xmmword ptr [rax + 0x810], xmm3
14206e11d: 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
14206e121: 0f118020080000 movups xmmword ptr [rax + 0x820], xmm0
14206e128: 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
14206e12c: 0f118030080000 movups xmmword ptr [rax + 0x830], xmm0
14206e133: 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
14206e137: 0f118040080000 movups xmmword ptr [rax + 0x840], xmm0
14206e13e: 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
14206e142: 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
14206e146: 0f1055c0 movups xmm2, xmmword ptr [rbp - 0x40]
14206e14a: 0f105dd0 movups xmm3, xmmword ptr [rbp - 0x30]
14206e14e: 0f118050080000 movups xmmword ptr [rax + 0x850], xmm0
14206e155: 0f118860080000 movups xmmword ptr [rax + 0x860], xmm1
14206e15c: 0f119070080000 movups xmmword ptr [rax + 0x870], xmm2
14206e163: 0f119880080000 movups xmmword ptr [rax + 0x880], xmm3
14206e16a: 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
14206e16e: 0f118090080000 movups xmmword ptr [rax + 0x890], xmm0
14206e175: 0f104500 movups xmm0, xmmword ptr [rbp]
14206e179: 0f1180b0080000 movups xmmword ptr [rax + 0x8b0], xmm0
14206e180: 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
14206e184: 0f1180a0080000 movups xmmword ptr [rax + 0x8a0], xmm0
14206e18b: 48c7460814000000 mov qword ptr [rsi + 8], 0x14
14206e193: 48894610 mov qword ptr [rsi + 0x10], rax
14206e197: 48c7461814000000 mov qword ptr [rsi + 0x18], 0x14
14206e19f: 48c7462000000000 mov qword ptr [rsi + 0x20], 0
14206e1a7: 48c7462808000000 mov qword ptr [rsi + 0x28], 8
14206e1af: 0f57c0 xorps xmm0, xmm0
14206e1b2: 0f114630 movups xmmword ptr [rsi + 0x30], xmm0
14206e1b6: 0f108568080000 movups xmm0, xmmword ptr [rbp + 0x868]
14206e1bd: 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
14206e1c1: 488b8578080000 mov rax, qword ptr [rbp + 0x878]
14206e1c8: 48894650 mov qword ptr [rsi + 0x50], rax
14206e1cc: 48891e mov qword ptr [rsi], rbx
14206e1cf: 4889f0 mov rax, rsi
14206e1d2: 4881c4980c0000 add rsp, 0xc98
14206e1d9: 5b pop rbx
14206e1da: 5f pop rdi
14206e1db: 5e pop rsi
14206e1dc: 5d pop rbp
14206e1dd: c3 ret 
14206e1de: b908000000 mov ecx, 8
14206e1e3: bac0080000 mov edx, 0x8c0
14206e1e8: e813ae8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e1ed: b908000000 mov ecx, 8
14206e1f2: ba88020000 mov edx, 0x288
14206e1f7: e804ae8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e1fc: e962010000 jmp 0x14206e363 ; unknown
14206e201: b908000000 mov ecx, 8
14206e206: ba88020000 mov edx, 0x288
14206e20b: e8f0ad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e210: e94e010000 jmp 0x14206e363 ; unknown
14206e215: b908000000 mov ecx, 8
14206e21a: baa8020000 mov edx, 0x2a8
14206e21f: e8dcad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e224: e93a010000 jmp 0x14206e363 ; unknown
14206e229: b908000000 mov ecx, 8
14206e22e: ba80020000 mov edx, 0x280
14206e233: e8c8ad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e238: e926010000 jmp 0x14206e363 ; unknown
14206e23d: b908000000 mov ecx, 8
14206e242: ba88020000 mov edx, 0x288
14206e247: e8b4ad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e24c: e912010000 jmp 0x14206e363 ; unknown
14206e251: b908000000 mov ecx, 8
14206e256: ba90020000 mov edx, 0x290
14206e25b: e8a0ad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e260: e9fe000000 jmp 0x14206e363 ; unknown
14206e265: b908000000 mov ecx, 8
14206e26a: ba78020000 mov edx, 0x278
14206e26f: e88cad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e274: e9ea000000 jmp 0x14206e363 ; unknown
14206e279: b908000000 mov ecx, 8
14206e27e: baa8020000 mov edx, 0x2a8
14206e283: e878ad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e288: e9d6000000 jmp 0x14206e363 ; unknown
14206e28d: b908000000 mov ecx, 8
14206e292: ba80020000 mov edx, 0x280
14206e297: e864ad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e29c: e9c2000000 jmp 0x14206e363 ; unknown
14206e2a1: b908000000 mov ecx, 8
14206e2a6: ba80020000 mov edx, 0x280
14206e2ab: e850ad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e2b0: e9ae000000 jmp 0x14206e363 ; unknown
14206e2b5: b908000000 mov ecx, 8
14206e2ba: ba88020000 mov edx, 0x288
14206e2bf: e83cad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e2c4: e99a000000 jmp 0x14206e363 ; unknown
14206e2c9: b908000000 mov ecx, 8
14206e2ce: ba90020000 mov edx, 0x290
14206e2d3: e828ad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e2d8: e986000000 jmp 0x14206e363 ; unknown
14206e2dd: b908000000 mov ecx, 8
14206e2e2: ba98020000 mov edx, 0x298
14206e2e7: e814ad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e2ec: eb75 jmp 0x14206e363 ; unknown
14206e2ee: b908000000 mov ecx, 8
14206e2f3: ba90020000 mov edx, 0x290
14206e2f8: e803ad8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e2fd: eb64 jmp 0x14206e363 ; unknown
14206e2ff: b908000000 mov ecx, 8
14206e304: ba80020000 mov edx, 0x280
14206e309: e8f2ac8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e30e: eb53 jmp 0x14206e363 ; unknown
14206e310: b908000000 mov ecx, 8
14206e315: ba88020000 mov edx, 0x288
14206e31a: e8e1ac8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e31f: eb42 jmp 0x14206e363 ; unknown
14206e321: b908000000 mov ecx, 8
14206e326: ba90020000 mov edx, 0x290
14206e32b: e8d0ac8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e330: eb31 jmp 0x14206e363 ; unknown
14206e332: b908000000 mov ecx, 8
14206e337: baa8030000 mov edx, 0x3a8
14206e33c: e8bfac8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e341: eb20 jmp 0x14206e363 ; unknown
14206e343: b908000000 mov ecx, 8
14206e348: ba80020000 mov edx, 0x280
14206e34d: e8aeac8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e352: eb0f jmp 0x14206e363 ; unknown
14206e354: b908000000 mov ecx, 8
14206e359: ba88020000 mov edx, 0x288
14206e35e: e89dac8600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206e363: 0f0b ud2 
14206e365: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
14206e370: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e375: 55 push rbp
14206e376: 56 push rsi
14206e377: 57 push rdi
14206e378: 53 push rbx
14206e379: 4883ec28 sub rsp, 0x28
14206e37d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e384: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e38b: e8508e80fe call 0x1408771e0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206e390: 90 nop 
14206e391: 4883c428 add rsp, 0x28
14206e395: 5b pop rbx
14206e396: 5f pop rdi
14206e397: 5e pop rsi
14206e398: 5d pop rbp
14206e399: c3 ret 
14206e39a: 660f1f440000 nop word ptr [rax + rax]
14206e3a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e3a5: 55 push rbp
14206e3a6: 56 push rsi
14206e3a7: 57 push rdi
14206e3a8: 53 push rbx
14206e3a9: 4883ec28 sub rsp, 0x28
14206e3ad: 488daa80000000 lea rbp, [rdx + 0x80]
14206e3b4: bac0080000 mov edx, 0x8c0
14206e3b9: 41b808000000 mov r8d, 8
14206e3bf: 488b8d60080000 mov rcx, qword ptr [rbp + 0x860]
14206e3c6: e855585afe call 0x140613c20 ; unknown
14206e3cb: 90 nop 
14206e3cc: 4883c428 add rsp, 0x28
14206e3d0: 5b pop rbx
14206e3d1: 5f pop rdi
14206e3d2: 5e pop rsi
14206e3d3: 5d pop rbp
14206e3d4: c3 ret 
14206e3d5: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
14206e3e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e3e5: 55 push rbp
14206e3e6: 56 push rsi
14206e3e7: 57 push rdi
14206e3e8: 53 push rbx
14206e3e9: 4883ec28 sub rsp, 0x28
14206e3ed: 488daa80000000 lea rbp, [rdx + 0x80]
14206e3f4: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e3fb: e8e08d80fe call 0x1408771e0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206e400: 90 nop 
14206e401: 4883c428 add rsp, 0x28
14206e405: 5b pop rbx
14206e406: 5f pop rdi
14206e407: 5e pop rsi
14206e408: 5d pop rbp
14206e409: c3 ret 
14206e40a: 660f1f440000 nop word ptr [rax + rax]
14206e410: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e415: 55 push rbp
14206e416: 56 push rsi
14206e417: 57 push rdi
14206e418: 53 push rbx
14206e419: 4883ec28 sub rsp, 0x28
14206e41d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e424: 488d8df0070000 lea rcx, [rbp + 0x7f0]
14206e42b: e850b4fbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e430: 90 nop 
14206e431: 4883c428 add rsp, 0x28
14206e435: 5b pop rbx
14206e436: 5f pop rdi
14206e437: 5e pop rsi
14206e438: 5d pop rbp
14206e439: c3 ret 
14206e43a: 660f1f440000 nop word ptr [rax + rax]
14206e440: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e445: 55 push rbp
14206e446: 56 push rsi
14206e447: 57 push rdi
14206e448: 53 push rbx
14206e449: 4883ec28 sub rsp, 0x28
14206e44d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e454: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e45b: e8808d80fe call 0x1408771e0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206e460: 90 nop 
14206e461: 4883c428 add rsp, 0x28
14206e465: 5b pop rbx
14206e466: 5f pop rdi
14206e467: 5e pop rsi
14206e468: 5d pop rbp
14206e469: c3 ret 
14206e46a: 660f1f440000 nop word ptr [rax + rax]
14206e470: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e475: 55 push rbp
14206e476: 56 push rsi
14206e477: 57 push rdi
14206e478: 53 push rbx
14206e479: 4883ec28 sub rsp, 0x28
14206e47d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e484: 488d8d80070000 lea rcx, [rbp + 0x780]
14206e48b: e8f0b3fbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e490: 90 nop 
14206e491: 4883c428 add rsp, 0x28
14206e495: 5b pop rbx
14206e496: 5f pop rdi
14206e497: 5e pop rsi
14206e498: 5d pop rbp
14206e499: c3 ret 
14206e49a: 660f1f440000 nop word ptr [rax + rax]
14206e4a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e4a5: 55 push rbp
14206e4a6: 56 push rsi
14206e4a7: 57 push rdi
14206e4a8: 53 push rbx
14206e4a9: 4883ec28 sub rsp, 0x28
14206e4ad: 488daa80000000 lea rbp, [rdx + 0x80]
14206e4b4: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e4bb: e8f00e56fe call 0x1405cf3b0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<count
14206e4c0: 90 nop 
14206e4c1: 4883c428 add rsp, 0x28
14206e4c5: 5b pop rbx
14206e4c6: 5f pop rdi
14206e4c7: 5e pop rsi
14206e4c8: 5d pop rbp
14206e4c9: c3 ret 
14206e4ca: 660f1f440000 nop word ptr [rax + rax]
14206e4d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e4d5: 55 push rbp
14206e4d6: 56 push rsi
14206e4d7: 57 push rdi
14206e4d8: 53 push rbx
14206e4d9: 4883ec28 sub rsp, 0x28
14206e4dd: 488daa80000000 lea rbp, [rdx + 0x80]
14206e4e4: 488d8d10070000 lea rcx, [rbp + 0x710]
14206e4eb: e890b3fbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e4f0: 90 nop 
14206e4f1: 4883c428 add rsp, 0x28
14206e4f5: 5b pop rbx
14206e4f6: 5f pop rdi
14206e4f7: 5e pop rsi
14206e4f8: 5d pop rbp
14206e4f9: c3 ret 
14206e4fa: 660f1f440000 nop word ptr [rax + rax]
14206e500: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e505: 55 push rbp
14206e506: 56 push rsi
14206e507: 57 push rdi
14206e508: 53 push rbx
14206e509: 4883ec28 sub rsp, 0x28
14206e50d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e514: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e51b: e8c08c80fe call 0x1408771e0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206e520: 90 nop 
14206e521: 4883c428 add rsp, 0x28
14206e525: 5b pop rbx
14206e526: 5f pop rdi
14206e527: 5e pop rsi
14206e528: 5d pop rbp
14206e529: c3 ret 
14206e52a: 660f1f440000 nop word ptr [rax + rax]
14206e530: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e535: 55 push rbp
14206e536: 56 push rsi
14206e537: 57 push rdi
14206e538: 53 push rbx
14206e539: 4883ec28 sub rsp, 0x28
14206e53d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e544: 488d8da0060000 lea rcx, [rbp + 0x6a0]
14206e54b: e830b3fbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e550: 90 nop 
14206e551: 4883c428 add rsp, 0x28
14206e555: 5b pop rbx
14206e556: 5f pop rdi
14206e557: 5e pop rsi
14206e558: 5d pop rbp
14206e559: c3 ret 
14206e55a: 660f1f440000 nop word ptr [rax + rax]
14206e560: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e565: 55 push rbp
14206e566: 56 push rsi
14206e567: 57 push rdi
14206e568: 53 push rbx
14206e569: 4883ec28 sub rsp, 0x28
14206e56d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e574: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e57b: e8608c80fe call 0x1408771e0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206e580: 90 nop 
14206e581: 4883c428 add rsp, 0x28
14206e585: 5b pop rbx
14206e586: 5f pop rdi
14206e587: 5e pop rsi
14206e588: 5d pop rbp
14206e589: c3 ret 
14206e58a: 660f1f440000 nop word ptr [rax + rax]
14206e590: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e595: 55 push rbp
14206e596: 56 push rsi
14206e597: 57 push rdi
14206e598: 53 push rbx
14206e599: 4883ec28 sub rsp, 0x28
14206e59d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e5a4: 488d8d30060000 lea rcx, [rbp + 0x630]
14206e5ab: e8d0b2fbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e5b0: 90 nop 
14206e5b1: 4883c428 add rsp, 0x28
14206e5b5: 5b pop rbx
14206e5b6: 5f pop rdi
14206e5b7: 5e pop rsi
14206e5b8: 5d pop rbp
14206e5b9: c3 ret 
14206e5ba: 660f1f440000 nop word ptr [rax + rax]
14206e5c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e5c5: 55 push rbp
14206e5c6: 56 push rsi
14206e5c7: 57 push rdi
14206e5c8: 53 push rbx
14206e5c9: 4883ec28 sub rsp, 0x28
14206e5cd: 488daa80000000 lea rbp, [rdx + 0x80]
14206e5d4: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e5db: e8d00d56fe call 0x1405cf3b0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<count
14206e5e0: 90 nop 
14206e5e1: 4883c428 add rsp, 0x28
14206e5e5: 5b pop rbx
14206e5e6: 5f pop rdi
14206e5e7: 5e pop rsi
14206e5e8: 5d pop rbp
14206e5e9: c3 ret 
14206e5ea: 660f1f440000 nop word ptr [rax + rax]
14206e5f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e5f5: 55 push rbp
14206e5f6: 56 push rsi
14206e5f7: 57 push rdi
14206e5f8: 53 push rbx
14206e5f9: 4883ec28 sub rsp, 0x28
14206e5fd: 488daa80000000 lea rbp, [rdx + 0x80]
14206e604: 488d8dc0050000 lea rcx, [rbp + 0x5c0]
14206e60b: e870b2fbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e610: 90 nop 
14206e611: 4883c428 add rsp, 0x28
14206e615: 5b pop rbx
14206e616: 5f pop rdi
14206e617: 5e pop rsi
14206e618: 5d pop rbp
14206e619: c3 ret 
14206e61a: 660f1f440000 nop word ptr [rax + rax]
14206e620: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e625: 55 push rbp
14206e626: 56 push rsi
14206e627: 57 push rdi
14206e628: 53 push rbx
14206e629: 4883ec28 sub rsp, 0x28
14206e62d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e634: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e63b: e8700d56fe call 0x1405cf3b0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<count
14206e640: 90 nop 
14206e641: 4883c428 add rsp, 0x28
14206e645: 5b pop rbx
14206e646: 5f pop rdi
14206e647: 5e pop rsi
14206e648: 5d pop rbp
14206e649: c3 ret 
14206e64a: 660f1f440000 nop word ptr [rax + rax]
14206e650: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e655: 55 push rbp
14206e656: 56 push rsi
14206e657: 57 push rdi
14206e658: 53 push rbx
14206e659: 4883ec28 sub rsp, 0x28
14206e65d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e664: 488d8d50050000 lea rcx, [rbp + 0x550]
14206e66b: e810b2fbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e670: 90 nop 
14206e671: 4883c428 add rsp, 0x28
14206e675: 5b pop rbx
14206e676: 5f pop rdi
14206e677: 5e pop rsi
14206e678: 5d pop rbp
14206e679: c3 ret 
14206e67a: 660f1f440000 nop word ptr [rax + rax]
14206e680: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e685: 55 push rbp
14206e686: 56 push rsi
14206e687: 57 push rdi
14206e688: 53 push rbx
14206e689: 4883ec28 sub rsp, 0x28
14206e68d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e694: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e69b: e8100d56fe call 0x1405cf3b0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<count
14206e6a0: 90 nop 
14206e6a1: 4883c428 add rsp, 0x28
14206e6a5: 5b pop rbx
14206e6a6: 5f pop rdi
14206e6a7: 5e pop rsi
14206e6a8: 5d pop rbp
14206e6a9: c3 ret 
14206e6aa: 660f1f440000 nop word ptr [rax + rax]
14206e6b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e6b5: 55 push rbp
14206e6b6: 56 push rsi
14206e6b7: 57 push rdi
14206e6b8: 53 push rbx
14206e6b9: 4883ec28 sub rsp, 0x28
14206e6bd: 488daa80000000 lea rbp, [rdx + 0x80]
14206e6c4: 488d8de0040000 lea rcx, [rbp + 0x4e0]
14206e6cb: e8b0b1fbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e6d0: 90 nop 
14206e6d1: 4883c428 add rsp, 0x28
14206e6d5: 5b pop rbx
14206e6d6: 5f pop rdi
14206e6d7: 5e pop rsi
14206e6d8: 5d pop rbp
14206e6d9: c3 ret 
14206e6da: 660f1f440000 nop word ptr [rax + rax]
14206e6e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e6e5: 55 push rbp
14206e6e6: 56 push rsi
14206e6e7: 57 push rdi
14206e6e8: 53 push rbx
14206e6e9: 4883ec28 sub rsp, 0x28
14206e6ed: 488daa80000000 lea rbp, [rdx + 0x80]
14206e6f4: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e6fb: e8b00c56fe call 0x1405cf3b0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<count
14206e700: 90 nop 
14206e701: 4883c428 add rsp, 0x28
14206e705: 5b pop rbx
14206e706: 5f pop rdi
14206e707: 5e pop rsi
14206e708: 5d pop rbp
14206e709: c3 ret 
14206e70a: 660f1f440000 nop word ptr [rax + rax]
14206e710: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e715: 55 push rbp
14206e716: 56 push rsi
14206e717: 57 push rdi
14206e718: 53 push rbx
14206e719: 4883ec28 sub rsp, 0x28
14206e71d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e724: 488d8d70040000 lea rcx, [rbp + 0x470]
14206e72b: e850b1fbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e730: 90 nop 
14206e731: 4883c428 add rsp, 0x28
14206e735: 5b pop rbx
14206e736: 5f pop rdi
14206e737: 5e pop rsi
14206e738: 5d pop rbp
14206e739: c3 ret 
14206e73a: 660f1f440000 nop word ptr [rax + rax]
14206e740: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e745: 55 push rbp
14206e746: 56 push rsi
14206e747: 57 push rdi
14206e748: 53 push rbx
14206e749: 4883ec28 sub rsp, 0x28
14206e74d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e754: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e75b: e8808a80fe call 0x1408771e0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206e760: 90 nop 
14206e761: 4883c428 add rsp, 0x28
14206e765: 5b pop rbx
14206e766: 5f pop rdi
14206e767: 5e pop rsi
14206e768: 5d pop rbp
14206e769: c3 ret 
14206e76a: 660f1f440000 nop word ptr [rax + rax]
14206e770: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e775: 55 push rbp
14206e776: 56 push rsi
14206e777: 57 push rdi
14206e778: 53 push rbx
14206e779: 4883ec28 sub rsp, 0x28
14206e77d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e784: 488d8d00040000 lea rcx, [rbp + 0x400]
14206e78b: e8f0b0fbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e790: 90 nop 
14206e791: 4883c428 add rsp, 0x28
14206e795: 5b pop rbx
14206e796: 5f pop rdi
14206e797: 5e pop rsi
14206e798: 5d pop rbp
14206e799: c3 ret 
14206e79a: 660f1f440000 nop word ptr [rax + rax]
14206e7a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e7a5: 55 push rbp
14206e7a6: 56 push rsi
14206e7a7: 57 push rdi
14206e7a8: 53 push rbx
14206e7a9: 4883ec28 sub rsp, 0x28
14206e7ad: 488daa80000000 lea rbp, [rdx + 0x80]
14206e7b4: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e7bb: e8208a80fe call 0x1408771e0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206e7c0: 90 nop 
14206e7c1: 4883c428 add rsp, 0x28
14206e7c5: 5b pop rbx
14206e7c6: 5f pop rdi
14206e7c7: 5e pop rsi
14206e7c8: 5d pop rbp
14206e7c9: c3 ret 
14206e7ca: 660f1f440000 nop word ptr [rax + rax]
14206e7d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e7d5: 55 push rbp
14206e7d6: 56 push rsi
14206e7d7: 57 push rdi
14206e7d8: 53 push rbx
14206e7d9: 4883ec28 sub rsp, 0x28
14206e7dd: 488daa80000000 lea rbp, [rdx + 0x80]
14206e7e4: 488d8d90030000 lea rcx, [rbp + 0x390]
14206e7eb: e890b0fbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e7f0: 90 nop 
14206e7f1: 4883c428 add rsp, 0x28
14206e7f5: 5b pop rbx
14206e7f6: 5f pop rdi
14206e7f7: 5e pop rsi
14206e7f8: 5d pop rbp
14206e7f9: c3 ret 
14206e7fa: 660f1f440000 nop word ptr [rax + rax]
14206e800: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e805: 55 push rbp
14206e806: 56 push rsi
14206e807: 57 push rdi
14206e808: 53 push rbx
14206e809: 4883ec28 sub rsp, 0x28
14206e80d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e814: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e81b: e8c08980fe call 0x1408771e0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206e820: 90 nop 
14206e821: 4883c428 add rsp, 0x28
14206e825: 5b pop rbx
14206e826: 5f pop rdi
14206e827: 5e pop rsi
14206e828: 5d pop rbp
14206e829: c3 ret 
14206e82a: 660f1f440000 nop word ptr [rax + rax]
14206e830: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e835: 55 push rbp
14206e836: 56 push rsi
14206e837: 57 push rdi
14206e838: 53 push rbx
14206e839: 4883ec28 sub rsp, 0x28
14206e83d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e844: 488d8d20030000 lea rcx, [rbp + 0x320]
14206e84b: e830b0fbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e850: 90 nop 
14206e851: 4883c428 add rsp, 0x28
14206e855: 5b pop rbx
14206e856: 5f pop rdi
14206e857: 5e pop rsi
14206e858: 5d pop rbp
14206e859: c3 ret 
14206e85a: 660f1f440000 nop word ptr [rax + rax]
14206e860: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e865: 55 push rbp
14206e866: 56 push rsi
14206e867: 57 push rdi
14206e868: 53 push rbx
14206e869: 4883ec28 sub rsp, 0x28
14206e86d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e874: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e87b: e8608980fe call 0x1408771e0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206e880: 90 nop 
14206e881: 4883c428 add rsp, 0x28
14206e885: 5b pop rbx
14206e886: 5f pop rdi
14206e887: 5e pop rsi
14206e888: 5d pop rbp
14206e889: c3 ret 
14206e88a: 660f1f440000 nop word ptr [rax + rax]
14206e890: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e895: 55 push rbp
14206e896: 56 push rsi
14206e897: 57 push rdi
14206e898: 53 push rbx
14206e899: 4883ec28 sub rsp, 0x28
14206e89d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e8a4: 488d8db0020000 lea rcx, [rbp + 0x2b0]
14206e8ab: e8d0affbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e8b0: 90 nop 
14206e8b1: 4883c428 add rsp, 0x28
14206e8b5: 5b pop rbx
14206e8b6: 5f pop rdi
14206e8b7: 5e pop rsi
14206e8b8: 5d pop rbp
14206e8b9: c3 ret 
14206e8ba: 660f1f440000 nop word ptr [rax + rax]
14206e8c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e8c5: 55 push rbp
14206e8c6: 56 push rsi
14206e8c7: 57 push rdi
14206e8c8: 53 push rbx
14206e8c9: 4883ec28 sub rsp, 0x28
14206e8cd: 488daa80000000 lea rbp, [rdx + 0x80]
14206e8d4: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e8db: e8d00a56fe call 0x1405cf3b0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<count
14206e8e0: 90 nop 
14206e8e1: 4883c428 add rsp, 0x28
14206e8e5: 5b pop rbx
14206e8e6: 5f pop rdi
14206e8e7: 5e pop rsi
14206e8e8: 5d pop rbp
14206e8e9: c3 ret 
14206e8ea: 660f1f440000 nop word ptr [rax + rax]
14206e8f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e8f5: 55 push rbp
14206e8f6: 56 push rsi
14206e8f7: 57 push rdi
14206e8f8: 53 push rbx
14206e8f9: 4883ec28 sub rsp, 0x28
14206e8fd: 488daa80000000 lea rbp, [rdx + 0x80]
14206e904: 488d8d40020000 lea rcx, [rbp + 0x240]
14206e90b: e870affbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e910: 90 nop 
14206e911: 4883c428 add rsp, 0x28
14206e915: 5b pop rbx
14206e916: 5f pop rdi
14206e917: 5e pop rsi
14206e918: 5d pop rbp
14206e919: c3 ret 
14206e91a: 660f1f440000 nop word ptr [rax + rax]
14206e920: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e925: 55 push rbp
14206e926: 56 push rsi
14206e927: 57 push rdi
14206e928: 53 push rbx
14206e929: 4883ec28 sub rsp, 0x28
14206e92d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e934: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e93b: e8a08880fe call 0x1408771e0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206e940: 90 nop 
14206e941: 4883c428 add rsp, 0x28
14206e945: 5b pop rbx
14206e946: 5f pop rdi
14206e947: 5e pop rsi
14206e948: 5d pop rbp
14206e949: c3 ret 
14206e94a: 660f1f440000 nop word ptr [rax + rax]
14206e950: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e955: 55 push rbp
14206e956: 56 push rsi
14206e957: 57 push rdi
14206e958: 53 push rbx
14206e959: 4883ec28 sub rsp, 0x28
14206e95d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e964: 488d8dd0010000 lea rcx, [rbp + 0x1d0]
14206e96b: e810affbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e970: 90 nop 
14206e971: 4883c428 add rsp, 0x28
14206e975: 5b pop rbx
14206e976: 5f pop rdi
14206e977: 5e pop rsi
14206e978: 5d pop rbp
14206e979: c3 ret 
14206e97a: 660f1f440000 nop word ptr [rax + rax]
14206e980: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e985: 55 push rbp
14206e986: 56 push rsi
14206e987: 57 push rdi
14206e988: 53 push rbx
14206e989: 4883ec28 sub rsp, 0x28
14206e98d: 488daa80000000 lea rbp, [rdx + 0x80]
14206e994: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e99b: e8408880fe call 0x1408771e0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206e9a0: 90 nop 
14206e9a1: 4883c428 add rsp, 0x28
14206e9a5: 5b pop rbx
14206e9a6: 5f pop rdi
14206e9a7: 5e pop rsi
14206e9a8: 5d pop rbp
14206e9a9: c3 ret 
14206e9aa: 660f1f440000 nop word ptr [rax + rax]
14206e9b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e9b5: 55 push rbp
14206e9b6: 56 push rsi
14206e9b7: 57 push rdi
14206e9b8: 53 push rbx
14206e9b9: 4883ec28 sub rsp, 0x28
14206e9bd: 488daa80000000 lea rbp, [rdx + 0x80]
14206e9c4: 488d8d60010000 lea rcx, [rbp + 0x160]
14206e9cb: e8b0aefbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206e9d0: 90 nop 
14206e9d1: 4883c428 add rsp, 0x28
14206e9d5: 5b pop rbx
14206e9d6: 5f pop rdi
14206e9d7: 5e pop rsi
14206e9d8: 5d pop rbp
14206e9d9: c3 ret 
14206e9da: 660f1f440000 nop word ptr [rax + rax]
14206e9e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206e9e5: 55 push rbp
14206e9e6: 56 push rsi
14206e9e7: 57 push rdi
14206e9e8: 53 push rbx
14206e9e9: 4883ec28 sub rsp, 0x28
14206e9ed: 488daa80000000 lea rbp, [rdx + 0x80]
14206e9f4: 488d8d68080000 lea rcx, [rbp + 0x868]
14206e9fb: e8607a80fe call 0x140876460 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206ea00: 90 nop 
14206ea01: 4883c428 add rsp, 0x28
14206ea05: 5b pop rbx
14206ea06: 5f pop rdi
14206ea07: 5e pop rsi
14206ea08: 5d pop rbp
14206ea09: c3 ret 
14206ea0a: 660f1f440000 nop word ptr [rax + rax]
14206ea10: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206ea15: 55 push rbp
14206ea16: 56 push rsi
14206ea17: 57 push rdi
14206ea18: 53 push rbx
14206ea19: 4883ec28 sub rsp, 0x28
14206ea1d: 488daa80000000 lea rbp, [rdx + 0x80]
14206ea24: 488d8df0000000 lea rcx, [rbp + 0xf0]
14206ea2b: e850aefbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206ea30: 90 nop 
14206ea31: 4883c428 add rsp, 0x28
14206ea35: 5b pop rbx
14206ea36: 5f pop rdi
14206ea37: 5e pop rsi
14206ea38: 5d pop rbp
14206ea39: c3 ret 
14206ea3a: 660f1f440000 nop word ptr [rax + rax]
14206ea40: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206ea45: 55 push rbp
14206ea46: 56 push rsi
14206ea47: 57 push rdi
14206ea48: 53 push rbx
14206ea49: 4883ec28 sub rsp, 0x28
14206ea4d: 488daa80000000 lea rbp, [rdx + 0x80]
14206ea54: 488d8d68080000 lea rcx, [rbp + 0x868]
14206ea5b: e8500956fe call 0x1405cf3b0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<count
14206ea60: 90 nop 
14206ea61: 4883c428 add rsp, 0x28
14206ea65: 5b pop rbx
14206ea66: 5f pop rdi
14206ea67: 5e pop rsi
14206ea68: 5d pop rbp
14206ea69: c3 ret 
14206ea6a: 660f1f440000 nop word ptr [rax + rax]
14206ea70: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206ea75: 55 push rbp
14206ea76: 56 push rsi
14206ea77: 57 push rdi
14206ea78: 53 push rbx
14206ea79: 4883ec28 sub rsp, 0x28
14206ea7d: 488daa80000000 lea rbp, [rdx + 0x80]
14206ea84: 488d8d80000000 lea rcx, [rbp + 0x80]
14206ea8b: e8f0adfbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206ea90: 90 nop 
14206ea91: 4883c428 add rsp, 0x28
14206ea95: 5b pop rbx
14206ea96: 5f pop rdi
14206ea97: 5e pop rsi
14206ea98: 5d pop rbp
14206ea99: c3 ret 
14206ea9a: 660f1f440000 nop word ptr [rax + rax]
14206eaa0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206eaa5: 55 push rbp
14206eaa6: 56 push rsi
14206eaa7: 57 push rdi
14206eaa8: 53 push rbx
14206eaa9: 4883ec28 sub rsp, 0x28
14206eaad: 488daa80000000 lea rbp, [rdx + 0x80]
14206eab4: 488d8d68080000 lea rcx, [rbp + 0x868]
14206eabb: e8208780fe call 0x1408771e0 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206eac0: 90 nop 
14206eac1: 4883c428 add rsp, 0x28
14206eac5: 5b pop rbx
14206eac6: 5f pop rdi
14206eac7: 5e pop rsi
14206eac8: 5d pop rbp
14206eac9: c3 ret 
14206eaca: 660f1f440000 nop word ptr [rax + rax]
14206ead0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206ead5: 55 push rbp
14206ead6: 56 push rsi
14206ead7: 57 push rdi
14206ead8: 53 push rbx
14206ead9: 4883ec28 sub rsp, 0x28
14206eadd: 488daa80000000 lea rbp, [rdx + 0x80]
14206eae4: 488d4d10 lea rcx, [rbp + 0x10]
14206eae8: e893adfbfd call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::res
14206eaed: 90 nop 
14206eaee: 4883c428 add rsp, 0x28
14206eaf2: 5b pop rbx
14206eaf3: 5f pop rdi
14206eaf4: 5e pop rsi
14206eaf5: 5d pop rbp
14206eaf6: c3 ret 
