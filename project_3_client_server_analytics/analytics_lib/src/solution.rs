use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn check(dataset: &Dataset, row: &Row, condition: &Condition) -> bool { // this is the new function we added to return the true or false, passes by ref
    match condition {
        Condition::Equal(column, value) => {
            let index = dataset.column_index(column);
            row.get_value(index) == value
        }

        Condition::And(left, right) => {
            check(dataset, row, left) && check(dataset, row, right) // recursively called
        }

        Condition::Or(left, right) => {
            check(dataset, row, left) || check(dataset, row, right)
        }
        Condition::Not(inner) => {
            !check(dataset, row, inner)
        }
        // the condition parameter has 4 options the equal, and, or, and not
    }
}


pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut temp = Dataset::new(dataset.columns().clone()); // Creates a new empty dataset with the same columns as the original dataset.

    for row in dataset.iter() {
        if check(dataset, row, filter) { // calling the new function, if true it adds to filered
            temp.add_row(row.clone()); // pass by clone because we want to write the columns we want without deleting them from the origional dataset
        }
    }

    temp // called temp because scope is only in function
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let mut temp = HashMap::new();

    let column_index = dataset.column_index(group_by_column);
    let columns = dataset.columns().clone();

    for row in dataset.into_iter() {
        let i = row.get_value(column_index).clone();

        temp
            .entry(i)// finds the hashmap slot for i
            .or_insert(Dataset::new(columns.clone()))// giving the dataset stored there
            .add_row(row); // adding the row to the dataset
    }

    temp
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
  // you are given ownership of the hashmap, so you can take do the for loop w/o passing by ref
   // place to store the final answer
    let mut result = HashMap::new();

    //iterate through each group produced by the group_by_dataset function
    for (group_value, group_dataset) in dataset {
        // compute the aggregated value 
        let aggregated_value = compute_aggregation_for_group(&group_dataset, aggregation);
       // store the result for the group
        result.insert(group_value, aggregated_value);
    }
    // return the result 
    result
}


fn compute_aggregation_for_group(dataset: &Dataset, aggregation: &Aggregation) -> Value {
  // helper function that takes care of count, sum, and average 
    match aggregation {
        // if the aggregation is count, then you just count the number of rows 
        Aggregation::Count(_column_name) => {
            Value::Integer(dataset.len() as i32)
        }

        // if the aggregation is sum, then you take the specific column and add all the int values
        Aggregation::Sum(column_name) => {
            //find the col index
            let column_index = dataset.column_index(column_name);
            let mut total = 0;

            //loop through the column 
            for row in dataset.iter() {
                match row.get_value(column_index) {
                    Value::Integer(num) => total += *num,
                    //used chatgpt here, what to do if the column value is not a number  
                    Value::String(_) => panic!("SUM requires an integer column"),
                }
            }

            Value::Integer(total)
        }
        // what if the aggregation is a sum....
        Aggregation::Average(column_name) => {
            let column_index = dataset.column_index(column_name);
            let mut total = 0;
            let mut count = 0;
            // first collect the sum, similar to the sum aggregation 
            // then count the rows 
            for row in dataset.iter() {
                match row.get_value(column_index) {
                    Value::Integer(num) => {
                        total += *num;
                        count += 1;
                    }
                    Value::String(_) => panic!("AVERAGE requires an integer column"),
                }
            }
            //check if the count is zero, so you avoid division by zero just in case 
            if count == 0 {
                Value::Integer(0)
            } else {
                Value::Integer(total / count)
            }
        }
    }
}



pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}