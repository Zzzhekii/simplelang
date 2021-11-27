use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        println!("\tUsage: simplelang [path_to_the_source].");
        return
    }
    if arguments[1] == "-h" || arguments[1] == "--help" {
        println!("\tUsage: simplelang [path_to_the_source].");
        return
    }

    let source = fs::read_to_string(&arguments[1])
        .expect(format!("\tError: Couldn't read source file. Provided path:\n\t{}", arguments[1]).as_str());

    match run(&source.as_str()) {
        Ok(()) => println!("\nProgram has ended successfully."),
        Err(e) => println!("\n\tAN ERROR OCCURED:\n{}", e)
    };
}

fn run(source: &str) -> Result<(), String> {
    let mut variables: HashMap<&str, i32> = HashMap::new();
    let mut stack: Vec<i32> = Vec::new();

    let program: Vec<&str> = source.lines().collect();
    let mut iterator = 0;

    if program.len() == 0 {
        println!("\tCode file is blank");
        return Ok(());
    }

    loop {
        if iterator as usize >= program.len() { break }
        let item: &str = program[iterator as usize];

        let split_item: Vec<&str> = item.split_whitespace().collect();

        // For debugging
        //println!("{}: {}", iterator, item);

        if split_item.len() != 0
        {
            // Variable declaration
            match is_variable(split_item[0]) {
                Ok(s) => if s {
                    if split_item.len() != 2 { return Err(format!("Error: variable defenision requiers one argument:\n{}: {}", iterator, item)) }
                    
                    match get_argument_value(split_item[1], &variables, iterator) {
                        Ok(s) => {
                            if variables.contains_key(split_item[0]) {
                                variables.remove(split_item[0]);
                                variables.insert(split_item[0], s);
                            } else {
                                variables.insert(split_item[0], s);
                            }
                        },
                        Err(e) => return Err(format!("{}:\n{}", e, item))
                    };
                },
                Err(_) => return Err(format!("Error: wrong definision of variable:\n{}: {}", iterator, item))
            }

            // Function execution
            match split_item[0] {
                "PRT" => {
                    if split_item.len() != 2 { return Err(format!("Error: PRT requires 1 argument:\n{}: {}", iterator, item)) }

                    let arg1 = match get_argument_value(split_item[1], &variables, iterator) { Ok(s) => s, Err(e) =>
                        return Err(format!("{}:\n{}: {}", e, iterator, item)) };

                    print!("{}", to_ascii(arg1));
                },
                "INP" => (),
                "ADD" => {
                    if split_item.len() != 3 { return Err(format!("Error: ADD requires 2 arguments:\n{}: {}", iterator, item)) }
                    if !is_variable_unwrap(split_item[1]) || split_item[1].len() < 2 { return Err(format!(
                                "Error: ADD's first argument must be a variable:\n{}: {}", iterator, item))}

                    let arg1 = match get_argument_value(split_item[1], &variables, iterator) { Ok(s) => s, Err(e) =>
                        return Err(format!("{}:\n{}: {}", e, iterator, item)) };
                    let arg2 = match get_argument_value(split_item[2], &variables, iterator) { Ok(s) => s, Err(e) =>
                        return Err(format!("{}:\n{}: {}", e, iterator, item)) };
                    variables.remove(split_item[1]);
                    variables.insert(split_item[1], arg1 + arg2);
                },
                "JMP" => {
                    if split_item.len() != 3 { return Err(format!("Error: JMP requires 2 arguments:\n{}: {}", iterator, item)) }

                    let arg1 = match get_argument_value(split_item[1], &variables, iterator) { Ok(s) => s, Err(e) =>
                        return Err(format!("{}:\n{}: {}", e, iterator, item)) };
                    let arg2 = match get_argument_value(split_item[2], &variables, iterator) { Ok(s) => s, Err(e) =>
                        return Err(format!("{}:\n{}: {}", e, iterator, item)) };
                    if arg2 < 0 { return Err(format!("Error: JMP's second argument can't be < 0:\n{}: {}", iterator, item)) }

                    if arg1 > 0 {
                        iterator = arg2-1;   // We need to substract 1 because iterator increments after 
                                             // execution of the command
                    }
                },
                "CALL" => { 
                    if split_item.len() != 2 { return Err(format!("Error: CAL requires 1 argument:\n{}: {}", iterator, item)) }

                    let arg1 = match get_argument_value(split_item[1], &variables, iterator) { Ok(s) => s, Err(e) =>
                        return Err(format!("{}:\n{}: {}", e, iterator, item)) };
                    if arg1 < 0 { return Err(format!("Error: CAL can't accept negative value as argument:\n{}: {}", iterator, item)) }
                    
                    stack.push(iterator);
                    iterator = arg1-1;  // We need to substract 1 because iterator increments after 
                                        // execution of the command
                }
                "RET" => {
                    if split_item.len() != 2 { return Err(format!("Error: RET requires 1 argument:\n{}: {}", iterator, item)) }

                    let arg1 = match get_argument_value(split_item[1], &variables, iterator) { Ok(s) => s, Err(e) =>
                        return Err(format!("{}:\n{}: {}", e, iterator, item)) };

                    if is_variable_unwrap(split_item[1]) {
                        variables.remove(split_item[1]);
                    }

                    if let Some(s) = stack.pop() { iterator = s };
                    stack.push(arg1);
                },
                "DEL" => {
                    if split_item.len() != 2 { return Err(format!("Error: DEL requires 1 argument:\n{}: {}", iterator, item)) }

                    let arg1 = split_item[1];
                    if !is_variable_unwrap(arg1) { return Err(format!("Error: DEL's argument must be a variable:\n{}: {}", iterator, item)) }

                    if variables.contains_key(arg1) {
                        variables.remove(arg1);
                    } else { return Err(format!("Error: variable \"{}\" doesn't exist:\n{}: {}", arg1, iterator, item)) }
                },
                "POP" => {
                    if split_item.len() != 2 { return Err(format!("Error: POP requires 1 argument:\n{}: {}", iterator, item)) }

                    let arg1 = split_item[1];

                    let pop = match stack.pop() {
                        Some(s) => s,
                        None => return Err(format!("Error: stack is empty:\n{}: {}", iterator, item))
                    };

                    if is_variable_unwrap(arg1) {
                        if variables.contains_key(arg1) {
                            variables.remove(arg1);
                            variables.insert(arg1, pop);
                        } else { return Err(format!("Error: variable \"{}\" doesn't exist:\n{}:{}", arg1, iterator, item)) }
                    }
                },
                "PUSH" => {
                    if split_item.len() != 2 { return Err(format!("Error: PSH requires 1 argument:\n{}:{}", iterator, item)) }

                    let arg1 = match get_argument_value(split_item[1], &variables, iterator) { Ok(s) => s, Err(e) =>
                        return Err(format!("{}:\n{}", e, item)) };

                    stack.push(arg1);
                },
                "REV" => {
                    if split_item.len() != 2 { return Err(format!("Error: REV requires 1 argument:\n{}:{}", iterator, item)) }

                    let arg1 = split_item[1];
                    if !is_variable_unwrap(arg1) { return Err(format!("Error: REV's argument must be a variable:\n{}: {}", iterator, item)) }

                    variables.remove(arg1);
                    variables.insert(arg1, match get_argument_value(split_item[1], &variables, iterator) { Ok(s) => s, Err(e) =>
                        return Err(format!("{}:\n{}: {}", e, iterator, item)) });
                },

                "#declare_and_skip" => {
                    if split_item.len() != 2 { return Err(format!("Error: #declare_and_skip requires 1 argument:\n{}: {}", iterator, item)) }

                    let arg1 = split_item[1];
                    if !is_variable_unwrap(arg1) { return Err(format!("Error: #declare_and_skip argument must be a variable:\n{}: {}", iterator, item)) }

                    if variables.contains_key(arg1) {
                        variables.remove(arg1);
                    }   variables.insert(arg1, iterator+1);
                    
                    let mut i = iterator;
                    while (i as usize) < program.len() {
                        if program[i as usize] == "#end_of_declaration" { break }
                        i += 1;
                    } if i + 1 >= program.len() as i32 {
                        return Err(format!("Error: #end_of_declaration was not found after #declare_end_skip:\n{}: {}", iterator, item))
                    }
                    iterator = i;
                },
                "#print_declarated_variables" => {
                    if split_item.len() != 1 { return Err(format!("Error: command doesn't require any arguments:\n{}: {}", iterator, item)) }
                    println!("{:?}", variables);
                },
                _ => ()
            };
        }
        iterator += 1;
    }

    Ok(())
}

fn is_variable(string: &str) -> Result<bool, ()> {
    if string.len() < 1 { return Ok(false) }
    if string.chars().nth(0).unwrap() != '&' { return Ok(false) }
    if string.len() < 2 { return Err(()) }

    return Ok(true)
}

fn is_variable_unwrap(string: &str) -> bool {
    if string.len() < 1 { return false }
    if string.chars().nth(0).unwrap() != '&' { return false }
    if string.len() < 2 { return false }

    return true
}

fn get_argument_value(argument: &str, variables: &HashMap<&str, i32>, current_line: i32) -> Result<i32, String>
{
    // Check if keyword
    match argument {
        "%HERE%" => return Ok(current_line),
        "%NEXT%" => return Ok(current_line+1),
        "%AFTER_NEXT%" => return Ok(current_line+2),
        _ => ()
    }

    if let Ok(s) = argument.to_string().parse::<i32>() {
        return Ok(s);
    }
    
    if is_variable_unwrap(argument) {
        match variables.get(argument) {
            Some(s) => Ok(*s),
            None => Err(String::from("Error: use of undeclarated variable"))
        }
    } else {
        Err(String::from(format!("Error: argument can be either variable or integer.\n\t(Did you want to use &{}?)", argument)))
    }
}

fn to_ascii(i: i32) -> char {
    match i {
        x@0..=127 => format!("{}", x as u8 as char).chars().next().unwrap(),
        _ => '\0'
    }
}
