mod nondeterminism_elimination;

use std::env;

use nondeterminism_elimination::eliminate_nondeterminism;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let analyzed_word = &args[2];
    let type_of_automaton = &args[3];
    if type_of_automaton != "dfa" && type_of_automaton != "nfa" {
        println!("ERROR: unrecognized automaton requested");
        return;
    }
    let contents = std::fs::read_to_string(file_path).expect("Failed to load the file");
    let mut automaton_result:MachineData = process_contents(&contents, type_of_automaton);
    let mut current_state = automaton_result.initial_state;

    if type_of_automaton == "nfa" {
        eliminate_nondeterminism(&current_state,&mut automaton_result.alphabet, &mut automaton_result.states, 
            &mut automaton_result.accepting_states,  &mut automaton_result.transitions);        
    }

    for i in 0..analyzed_word.len() {
        let current_letter = analyzed_word.chars().nth(i).unwrap().to_string();
        if !automaton_result.alphabet.iter().any(|s| s.to_string() == current_letter) {
            println!("ERROR: unrecognized symbol: {}", current_letter);
            break;
        }
        if let Some(transition) =  automaton_result.transitions.iter()
        .find(|&(ref state, ref symbol, _)| state.to_string() == current_state && symbol.to_string() == current_letter){
            let next_state = transition.2.clone();
            if automaton_result.states.iter().any(|s| s.to_string() == next_state) {
                current_state = transition.2.clone();
            } else {
                println!("ERROR: unrecognized state: {}", next_state);
                break;
            }
        } else {
            println!("No matching transition!");
            break;
        }
    }
    if let Some(_state) = automaton_result.accepting_states.iter()
    .find(|&s| s.to_string() == current_state){
        println!("Input accepted");
    } else {
        println!("Input rejected");
    }
}

struct MachineData {
    states: Vec<String>,
    alphabet: Vec<String>,
    transitions: Vec<(String, String, String)>,
    initial_state: String,
    accepting_states: Vec<String>
}

fn process_contents(contents: &String, type_of_automaton: &String) -> MachineData {
    let parts: Vec<&str> = contents.split('\n').collect();
    let states_raw: Vec<&str> = parts[0].split(' ').collect();
    let states = states_raw.iter().map(|&s| s.to_string()).collect();
    let alphabet_raw: Vec<&str> = parts[1].split(' ').collect();
    let alphabet: Vec<String> = alphabet_raw.iter().map(|&s| s.to_string()).collect();
    let mut transitions: Vec<(String, String, String)> = [].to_vec();
    let mut i = 2;
    loop {
        let row: Vec<&str> = parts[i].trim().split(' ').collect();
        if row.len() < 3 || (type_of_automaton == "dfa" && row.len() > 3){
            break;
        } else if row.len() == 3 {
            transitions.push((row[0].to_owned(), row[1].to_owned(), row[2].to_owned()));
        } else {
            transitions.push((row[0].to_owned(), row[1].to_owned(), row[2..].join(",")));
        }
        i+=1;
    }
    let initial_state = parts[i].to_string();
    i+=1;
    let accepting_states_raw: Vec<&str> = parts[i].split(' ').collect();
    let accepting_states: Vec<String> = accepting_states_raw.iter().map(|&s| s.to_string()).collect();


    MachineData {
        states,
        alphabet,
        transitions,
        initial_state,
        accepting_states,
    }
}
