use serde::{Deserialize, Serialize};
use serde_json;

fn process_data(products: &mut Vec<Input>) -> Vec<Output> {
    products.reverse();
    let mut output: Vec<Output> = vec![];
    for p in products {
        if !output.iter().any(|x| x.assigned_to == p.assigned_to) {
            output.push(Output::try_from(p.clone()).unwrap())
        }
    }

    output.sort_by(|a, b| a.assigned_to.cmp(&b.assigned_to));
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

#[derive(Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Target {
    location: String,
    object_id: String,
    code: String,
}

impl TryFrom<&str> for Target {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let segments: Vec<&str> = s.split(':').collect();
        match &segments as &[&str] {
            [loc, id, code, ..] => Ok(Target {
                location: loc.to_string(),
                object_id: id.to_string(),
                code: code.to_string(),
            }),
            _ => Err("Couldn't convert string"),
        }
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Output {
    target: Target,
    assigned_to: String,
}

impl TryFrom<Input> for Output {
    type Error = &'static str;
    fn try_from(input: Input) -> Result<Self, Self::Error> {
        match Target::try_from(input.target.as_str()) {
            Ok(target) => Ok(Self {
                target,
                assigned_to: input.assigned_to.clone(),
            }),

            Err(_) => Err("Couldn't convert input to output because of an error"),
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
            Input::new("a:bt:na", "", "a"),
            Input::new("a:bt:na", "", "b"),
            Input::new("a:bt:na", "", "a"),
            Input::new("a:bt:na", "", "b"),
        ];

        let output = process_data(&mut input);

        assert_eq!(output.len(), 2);
    }

    #[test]
    fn it_removes_the_right_elements() {
        let c = Input::new("c:j:t", "", "a");
        let d = Input::new("d:x:z", "", "b");
        let c_out = Output::try_from(c.clone()).unwrap();
        let d_out = Output::try_from(d.clone()).unwrap();
        let mut input = vec![Input::new("x", "", "a"), Input::new("x", "", "b"), c, d];

        let output = process_data(&mut input);

        assert_eq!(output.len(), 2);
        assert!(output.contains(&c_out));
        assert!(output.contains(&d_out));
    }

    #[test]
    fn it_sorts_by_assigned_to() {
        let a = Input::new("a:bA:c", "", "a");
        let b = Input::new("b:bc:c", "", "b");
        let c = Input::new("c:aeouht:n", "", "c");
        let mut input = vec![b.clone(), c.clone(), a.clone()];

        let expected = vec![
            Output::try_from(a.clone()).unwrap(),
            Output::try_from(b.clone()).unwrap(),
            Output::try_from(c.clone()).unwrap(),
        ];

        let output = process_data(&mut input);

        assert_eq!(output, expected);
    }

    #[test]
    fn target_splits_string_correctly() {
        let input = "a:b:c";

        let expected = Target {
            location: "a".to_string(),
            object_id: "b".to_string(),
            code: "c".to_string(),
        };

        let actual = Target::try_from(input).unwrap();

        assert_eq!(actual, expected);
    }
}
