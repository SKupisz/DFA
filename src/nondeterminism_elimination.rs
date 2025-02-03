extern crate queues;
use queues::*;
use std::collections::HashSet;
fn remove_duplicates(data: &mut Vec<String>) {
    let set: HashSet<_> = data.to_owned().into_iter().collect();
    data.clear();
    let result:Vec<String> = set.into_iter().collect();
    data.extend(result);
}
pub fn eliminate_nondeterminism(initial_state: &String, alphabet: &Vec<String>, states: &mut Vec<String>, 
    accepting_states: &mut Vec<String>, transitions: &mut Vec<(String, String, String)>){
    let mut new_transitions: Vec<(String, String, String)> = [].to_vec();
    let mut analyzed_states: Queue<String> = queue![initial_state.clone()];
    let mut handled_states: Vec<String> = [].to_vec();

    while analyzed_states.size() > 0 {
        let current_state_result = analyzed_states.remove();
        let current_state = match current_state_result {
            Ok(state) => state,
            Err(error) => panic!("Problem with popping from the queue {error}")
        };
        if !handled_states.contains(&current_state) {
            let state_split:Vec<String> = current_state.split(',').map(|v| v.to_string()).collect();
            for i in 0..alphabet.iter().len() {
                let initial_state_transitions:Vec<&(String, String, String)> = transitions
                .iter()
                .filter(|&(ref analyzed_state, ref symbol, _)| 
                state_split.to_owned().contains(analyzed_state) && *symbol == alphabet[i]).collect();
    
                let mut new_state_parts: Vec<String> = initial_state_transitions.iter()
                .map(|(_, _, transition_value)| transition_value.clone()).collect();
                new_state_parts = new_state_parts.iter().flat_map(|transition| transition.split(","))
                .map(|transition| transition.trim().to_string()).collect();
                
                remove_duplicates(&mut new_state_parts);
                new_state_parts.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    
                let new_state = new_state_parts.join(",").to_string();
                if !handled_states.contains(&new_state) {
                    let _ = analyzed_states.add(new_state.clone());
                }
                new_transitions.push((current_state.clone(), alphabet[i].clone(), new_state.clone()));
            }
            handled_states.push(current_state);
        }
    }
    let mut new_accepting_states: Vec<String> = [].to_vec();
    for i in 0..handled_states.len() {
        let currently_analyzed_state = handled_states[i].clone();
        if accepting_states.iter().any(|state| currently_analyzed_state.contains(state)){
            new_accepting_states.push(currently_analyzed_state);
        }
    }
    accepting_states.clear();
    accepting_states.extend(new_accepting_states);
    transitions.clear();
    transitions.extend(new_transitions);
    states.clear();
    states.extend(handled_states);
}