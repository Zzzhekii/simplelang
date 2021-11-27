#declare_and_skip &Multiply_ab
&tmp_a &a
&tmp_b &b
ADD &tmp_b -1
&tmp_jmp %HERE%
ADD &tmp_b -1
ADD &tmp_a &a
JMP &tmp_b &tmp_jmp
DEL &tmp_jmp
DEL &tmp_b
RET &tmp_a
#end_of_declaration

#declare_and_skip &Divide_ab
&tmp_a &a
&tmp_b &b

&tmp_jmp %HERE%
ADD &tmp_a 
JMP &tmp_a &tmp_jmp
ADD &tmp_a &tmp_b

RET &tmp_a
#end_of_declaration

#print_declarated_variables
