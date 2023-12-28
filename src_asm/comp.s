.global main
.extern printf
.intel_syntax noprefix

main:
    sub rsp, 8
    mov QWORD ptr [rsp + 8], 6
    sub rsp, 8
    mov QWORD ptr [rsp + 8], 7
    sub rsp, 8
    mov QWORD ptr [rsp + 8], 8
    sub rsp, 8
    mov QWORD ptr [rsp + 8], 9
    mov r12, [rsp + 32]
    mov r13, [rsp + 24]
    add r12, r13
    mov r13, [rsp + 16]
    sub r12, r13
    mov r13, [rsp + 8]
    imul r12, r13
    sub rsp, 8
    mov rsi, r12
    mov rdi, offset flat:[printint]
    mov rax, 0
    call printf
    sub rsp, 8
    mov r12, [rsp + 8]
    sub rsp, 8
    mov QWORD ptr [rsp + 8], 1
    mov r13, [rsp + 8]
    add r12, r13
    mov rsi, r12
    mov rdi, offset flat:[printint]
    mov rax, 0
    call printf
    mov rax, 60
    xor rdi, rdi
    syscall

printfloat:
    .ascii "%g\n\0"

printint:
    .ascii "%d\n\0"

