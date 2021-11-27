use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn process_data<'a>(products: &'a [Input<'a>]) -> Vec<Output<'a>> {
    let mut map = HashMap::<&str, Output>::new();
    for p in products {
        map.insert(p.assigned_to, Output::try_from(p).unwrap());
    }

    let mut output: Vec<Output> = map.into_values().collect();
    output.sort_by(|a, b| a.assigned_to.cmp(b.assigned_to));
    output
}

pub fn algorithm(input: &str) {
    if let Ok(products) = serde_json::from_str::<Vec<Input>>(input) {
        let output = process_data(&products);
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
pub struct Input<'a> {
    target: &'a str,
    id: &'a str,
    assigned_to: &'a str,
}

impl<'a> Input<'a> {
    fn new(target: &'a str, id: &'a str, assigned_to: &'a str) -> Self {
        Self {
            target,
            id,
            assigned_to,
        }
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Target<'a> {
    location: &'a str,
    object_id: &'a str,
    code: &'a str,
}

impl<'a> TryFrom<&'a str> for Target<'a> {
    type Error = &'static str;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let segments: Vec<&str> = s.split(':').collect();
        match &segments as &[&str] {
            [location, object_id, code, ..] => Ok(Target {
                location,
                object_id,
                code,
            }),
            _ => Err("Couldn't convert string"),
        }
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Output<'a> {
    target: Target<'a>,
    assigned_to: &'a str,
}

impl<'a> TryFrom<&'a Input<'_>> for Output<'a> {
    type Error = &'static str;
    fn try_from(input: &'a Input) -> Result<Self, Self::Error> {
        match Target::try_from(input.target) {
            Ok(target) => Ok(Self {
                target,
                assigned_to: input.assigned_to,
            }),

            Err(_) => Err("Couldn't convert input to output because of an error"),
        }
    }
}

impl<'a> TryFrom<&'a mut Input<'_>> for Output<'a> {
    type Error = &'static str;
    fn try_from(input: &'a mut Input) -> Result<Self, Self::Error> {
        match Target::try_from(input.target) {
            Ok(target) => Ok(Self {
                target,
                assigned_to: input.assigned_to,
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
        let c_out = Output::try_from(&c).unwrap();
        let d_out = Output::try_from(&d).unwrap();
        let mut input = vec![
            Input::new("x:j:t", "", "a"),
            Input::new("x:t:t", "", "b"),
            c.clone(),
            d.clone(),
        ];

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
            Output::try_from(&a).unwrap(),
            Output::try_from(&b).unwrap(),
            Output::try_from(&c).unwrap(),
        ];

        let output = process_data(&mut input);

        assert_eq!(output, expected);
    }

    #[test]
    fn target_splits_string_correctly() {
        let input = "a:b:c";

        let expected = Target {
            location: "a",
            object_id: "b",
            code: "c",
        };

        let actual = Target::try_from(input).unwrap();

        assert_eq!(actual, expected);
    }
}
