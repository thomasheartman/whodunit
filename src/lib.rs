use serde::{Deserialize, Serialize};
use serde_json;

pub fn algorithm(s: &str) {
    if let Ok(output) =
        serde_json::from_str::<Vec<Input>>(s).and_then(|products| serde_json::to_string(&products))
    {
        std::fs::write("output", output).unwrap();
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    target: String,
    #[serde(skip_serializing)]
    id: String,
    assigned_to: String,
}

// transform and sort, use the ID for lookup

// switch from slice to hashmap or collect into one <- if you need to look things up multiple times

// how do you sort? <-

// sku consists of a number of parts. break it up. first as Strings, later as &strs

// write to file <- can this be done for each element?

// Original goal: transform to JSON, write to output file

// Issue! Turns out there might be duplicates in the input. We don't want that in the output. (Use set or a map). Only keep the last one. Also, can we sort the output? Makes it easier.
