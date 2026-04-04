use analytics_lib::{dataset::Dataset, query::Query};

pub fn hello() -> String {
    println!("hello called");
    return String::from("hello");
}

pub fn slow_rpc(input_dataset: &Dataset) -> Dataset {
    println!("slow_rpc called");
    //need a clone bc the input dataset is a reference
    return input_dataset.clone();
}

pub fn fast_rpc(input_dataset: &Dataset, query: Query) -> Dataset {
    println!("fast_rpc called");
    todo!("implement");
}