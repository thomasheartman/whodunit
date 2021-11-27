use serde::{Deserialize, Serialize};
use serde_json;

fn process_data(products: &mut Vec<Input>) -> Vec<Input> {
    products.reverse();
    let mut output: Vec<Input> = vec![];
    for p in products {
        if !output.iter().any(|x| x.assigned_to == p.assigned_to) {
            output.push(p.clone())
        }
    }
    output
}

pub fn algorithm(input: &str) {
    if let Ok(mut products) = serde_json::from_str::<Vec<Input>>(input) {
        let output = process_data(&mut products);
        if let Ok(out) = serde_json::to_string(&output) {
            match std::fs::write("output.json", &out) {
                Ok(_) => {}
                Err(e) => println!("Failed to write output: {e}"),
            }
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    target: String,
    #[serde(skip_serializing)]
    id: String,
    assigned_to: String,
}

impl Input {
    pub fn new(target: &str, id: &str, assigned_to: &str) -> Self {
        Self {
            target: target.to_string(),
            id: id.to_string(),
            assigned_to: assigned_to.to_string(),
        }
    }
}

// transform and sort, use the ID for lookup

// switch from slice to hashmap or collect into one <- if you need to look things up multiple times

// how do you sort? <-

// sku consists of a number of parts. break it up. first as Strings, later as &strs

// write to file <- can this be done for each element?

// Original goal: transform to JSON, write to output file

// Issue! Turns out there might be duplicates in the input. We don't want that in the output. (Use set or a map). Only keep the last one. Also, can we sort the output? Makes it easier.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_removes_duplicates() {
        let mut input = vec![
            Input::new("x", "", "a"),
            Input::new("x", "", "b"),
            Input::new("_", "", "a"),
            Input::new("_", "", "b"),
        ];

        let output = process_data(&mut input);

        assert_eq!(output.len(), 2);
    }

    #[test]
    fn it_removes_the_right_elements() {
        let c = Input::new("c", "", "a");
        let d = Input::new("d", "", "b");
        let mut input = vec![
            Input::new("x", "", "a"),
            Input::new("x", "", "b"),
            c.clone(),
            d.clone(),
        ];

        let output = process_data(&mut input);

        assert_eq!(output.len(), 2);
        assert!(output.contains(&c));
        assert!(output.contains(&d));
    }
}
