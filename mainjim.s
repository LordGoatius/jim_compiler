.intel_syntax noprefix
.extern printf
.global main

main:
    # id1 = 6
    add rsp, 8
    mov QWORD ptr [rsp - 8], 6
    # id2 = 7
    add rsp, 8
    mov QWORD ptr [rsp - 8], 7
    # id3 = 8
    add rsp, 8
    mov QWORD ptr [rsp - 8], 8
    # id4 = 9
    add rsp, 8
    mov QWORD ptr [rsp - 8], 9
    
    # id8 = id1
    # id9 = id2
    # id7 = id8 + id9
    mov r12, [rsp - 32]
    mov r13, [rsp - 24]
    add r12, r13

    # id10 = id3
    # id6 = id7 - id10
    mov r13, [rsp - 16]
    sub r12, r13

    # id11 = id4
    # id5 = id6 * id11
    mov r13, [rsp - 8]
    imul r12, r13

    # print = id5

    add rsp, 8

    mov rsi, r12
    mov rdi, offset flat:[printint]
    mov rax, 0
    call printf

    sub rsp, 8
     
    # id13 = id4
    mov r12, [rsp - 8]

    # id14 = 1
    add rsp, 8
    mov QWORD ptr [rsp - 8], 1

    # id12 = id13 + id14
    mov r13, [rsp - 8]
    add r12, r13

    # print = id12

    mov rsi, r12
    mov rdi, offset flat:[printint]
    mov rax, 0
    call printf

    # exit
    mov rax, 60
    xor rdi, rdi
    syscall

printfloat:
    .ascii "%g\n\0"

printint:
    .ascii "%d\n\0"

