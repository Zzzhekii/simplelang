Simplelang (SL) consists of simple parts:
    - FUNCTIONS
    - VARIABLES
    - KEY WORDS
    - Additional functions

Every variable is i32: [-2147483648; 2147483647]

Key words:
%HERE%                      // If used as argument, replaced with the position of current line
%NEXT%                      // If used as argument, replaced with the position of next line
%AFTER_NEXT%                // If used as argument, replaced with the position of a line after next line

Functions:
PRT 1                       // Prints argument's matching ascii character
INP &variable               // Waits for user input; stores it in &variable
ADD &variable 32            // Stores &variable + 32 in &variable
REV &variable               // Reverses &variable value: Negative => Positive, Positive => Negative
JMP 0 11                    // if [1st argument] is > 0, then jump to [2nd argument] line
CALL &variable              // Resumes execution of program at specified command (line) and writes current position to call stack
RET 0                       // Returns to position from call stack; replaces last i32 in stack with argument; Deletes variable if used as argument.
DEL &variable               // Deletes &variable. After that &variable can not be accessed anymore
POP &variable               // Moves last i32 from the stack to &variable (unless i32 is passed);
                            //      moved i32 is removed from stack
PUSH 0                      // Pushes any i32 to a stack

Variables:
&test 12                    // Declaration of variable &test with 12 as argument

Variables can be redeclared anytime
Variables can be accessed from every part of the code after declarasion unless &variable was deleted.

Additional functions:
#declare_and_skip &variable         // Declare variable pointing at %HERE&+1 and skip to next line that contains #end_of_declaration
#end_of_declaration                 // End of declaration
#print_declarated_variables         // Print list of declarated variables and it's values
