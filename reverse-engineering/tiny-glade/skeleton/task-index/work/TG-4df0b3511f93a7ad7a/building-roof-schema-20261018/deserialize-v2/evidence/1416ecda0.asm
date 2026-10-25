0x1416ecda0 56 push rsi
0x1416ecda1 4883ec70 sub rsp, 0x70
0x1416ecda5 4889ce mov rsi, rcx
0x1416ecda8 488d4c2438 lea rcx, [rsp + 0x38]
0x1416ecdad e89e5db9ff call 0x141282b50
0x1416ecdb2 0fb6442438 movzx eax, byte ptr [rsp + 0x38]
0x1416ecdb7 488b542440 mov rdx, qword ptr [rsp + 0x40]
0x1416ecdbc 3c02 cmp al, 2
0x1416ecdbe 750f jne 0x1416ecdcf
0x1416ecdc0 48895608 mov qword ptr [rsi + 8], rdx
0x1416ecdc4 c70601000000 mov dword ptr [rsi], 1
0x1416ecdca e9c2000000 jmp 0x1416ece91
0x1416ecdcf a801 test al, 1
0x1416ecdd1 7443 je 0x1416ece16
0x1416ecdd3 4885d2 test rdx, rdx
0x1416ecdd6 0f84be000000 je 0x1416ece9a
0x1416ecddc 488d05f5a65801 lea rax, [rip + 0x158a6f5]
0x1416ecde3 4889442420 mov qword ptr [rsp + 0x20], rax
0x1416ecde8 48c744242805000000 mov qword ptr [rsp + 0x28], 5
0x1416ecdf1 4c8d05d0a65801 lea r8, [rip + 0x158a6d0]
0x1416ecdf8 488d4c2438 lea rcx, [rsp + 0x38]
0x1416ecdfd 41b90b000000 mov r9d, 0xb
0x1416ece03 e84872b9ff call 0x141284050
0x1416ece08 f644243801 test byte ptr [rsp + 0x38], 1
0x1416ece0d 7453 je 0x1416ece62
0x1416ece0f 488b442440 mov rax, qword ptr [rsp + 0x40]
0x1416ece14 eb41 jmp 0x1416ece57
0x1416ece16 4885d2 test rdx, rdx
0x1416ece19 0f84a2000000 je 0x1416ecec1
0x1416ece1f 488d0542265a01 lea rax, [rip + 0x15a2642]
0x1416ece26 4889442420 mov qword ptr [rsp + 0x20], rax
0x1416ece2b 48c744242803000000 mov qword ptr [rsp + 0x28], 3
0x1416ece34 4c8d05cd255a01 lea r8, [rip + 0x15a25cd]
0x1416ece3b 488d4c2458 lea rcx, [rsp + 0x58]
0x1416ece40 41b908000000 mov r9d, 8
0x1416ece46 e85562b9ff call 0x1412830a0
0x1416ece4b f644245801 test byte ptr [rsp + 0x58], 1
0x1416ece50 742b je 0x1416ece7d
0x1416ece52 488b442460 mov rax, qword ptr [rsp + 0x60]
0x1416ece57 48894608 mov qword ptr [rsi + 8], rax
0x1416ece5b b801000000 mov eax, 1
0x1416ece60 eb2d jmp 0x1416ece8f
0x1416ece62 488b44244c mov rax, qword ptr [rsp + 0x4c]
0x1416ece67 48894618 mov qword ptr [rsi + 0x18], rax
0x1416ece6b 0f1044243c movups xmm0, xmmword ptr [rsp + 0x3c]
0x1416ece70 0f114608 movups xmmword ptr [rsi + 8], xmm0
0x1416ece74 c7460401000000 mov dword ptr [rsi + 4], 1
0x1416ece7b eb10 jmp 0x1416ece8d
0x1416ece7d 0f1044245c movups xmm0, xmmword ptr [rsp + 0x5c]
0x1416ece82 0f114608 movups xmmword ptr [rsi + 8], xmm0
0x1416ece86 c7460400000000 mov dword ptr [rsi + 4], 0
0x1416ece8d 31c0 xor eax, eax
0x1416ece8f 8906 mov dword ptr [rsi], eax
0x1416ece91 4889f0 mov rax, rsi
0x1416ece94 4883c470 add rsp, 0x70
0x1416ece98 5e pop rsi
0x1416ece99 c3 ret 
0x1416ece9a c64424380d mov byte ptr [rsp + 0x38], 0xd
0x1416ece9f 488d15caa75301 lea rdx, [rip + 0x153a7ca]
0x1416ecea6 4c8d050ba55301 lea r8, [rip + 0x153a50b]
0x1416ecead 488d4c2438 lea rcx, [rsp + 0x38]
0x1416eceb2 e849cb1d01 call 0x1428c9a00
0x1416eceb7 4889442440 mov qword ptr [rsp + 0x40], rax
0x1416ecebc e94effffff jmp 0x1416ece0f
0x1416ecec1 c64424380d mov byte ptr [rsp + 0x38], 0xd
0x1416ecec6 488d15a3a75301 lea rdx, [rip + 0x153a7a3]
0x1416ececd 4c8d05e4a45301 lea r8, [rip + 0x153a4e4]
0x1416eced4 488d4c2438 lea rcx, [rsp + 0x38]
0x1416eced9 e822cb1d01 call 0x1428c9a00
0x1416ecede 4889442460 mov qword ptr [rsp + 0x60], rax
0x1416ecee3 e96affffff jmp 0x1416ece52
