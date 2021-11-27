# Simplelang (SLG) is my small esolang.
_I've made it as a little rust project since I've started to learn rust recently._

##### Every variable and value in SLG are i32 (from -2147483648 to 2147483647)
### Simplelang consists of 4 simple parts:
    - Commands
    - Variables
    - Key worlds
    - Additional commands

### Simplelang interpreter must have:
    - The stack
    - Hashmap of variables
    - An ability to execute SLG code

### Key words:
```
%HERE%              If used as an argument, replaced with the position of current line
%NEXT%              If used as an argument, replaced with the position of next line
%AFTER_NEXT%        If used as an argument, replaced with the position of a line after next line
```

### Commands:
```
PRT 1                   Prints argument's matching ascii character
INP &variable           Waits for user input; stores it in &variable
ADD &variable 32        Stores &variable + 32 in &variable
REV &variable           Reverses &variable value: Negative => Positive, Positive => Negative
JMP 0 11                If [1st argument] is > 0, then jump to [2nd argument] line
CALL &variable          Resumes execution of program at specified command (line) and writes current position to call stack
RET 0                       - Returns to the position from call stack;
                            - Replaces the last integer in the stack with argument;
                            - Deletes variable if used as argument.
DEL &variable           Deletes &variable. After that &variable can not be accessed anymore
POP &variable               - Moves last integer from the stack to &variable
                                (If argument was a variable, otherwise POP will only remove last number)
                            - Removes last number from stack
PUSH 0                  Pushes any number to the stack
```

### Variables:
```
&test 12            Declaration of a variable &test with 12 as argument
&test &adv          Redeclaration of a variable &test with value of &adv as argument
```

#### Variables can be redeclared anytime
##### Any variable can be accessed from every part of the code after it was declarated unless it was deleted.

### Additional functions:
```
#declare_and_skip &variable         Declare variable pointing at %HERE&+1 and skip to next line that contains
#end_of_declaration                 End of declaration
#print_declarated_variables         Print list of declarated variables and it's values
```

# Example code:
```
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

// Multiply &a by &b and write the result to a &c
// Then delete &Multiply_ab (for clear output)
// And print every variable
&a 9
&b 3
CALL &Multiply_ab
&c 0
POP &c

DEL &Multiply_ab
#print_declarated_variables
```
