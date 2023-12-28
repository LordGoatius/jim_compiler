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
	add r12, r13	mov r12, [rsp + 16]
	sub r12, r13	mov r12, [rsp + 8]
	imul r12, r13	mov r12, [rsp + 8]
	sub rsp, 8
	mov QWORD ptr [rsp + 8], 1
	add r12, r13    
    mov rax, 60  
    mov rdi, 0   
    syscall

pntfloat:
    .ascii "%g\n\0"

pntint: 
    .ascii "%d\n\0"

