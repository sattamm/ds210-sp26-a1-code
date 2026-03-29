use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    
    // creating a new dataset, that is empty, but has the same columns as the original
    let mut result: Dataset = Dataset::new(dataset.columns().clone());
        
    
    // check every row in the dataset, pass by ref 
    for row in dataset.iter() {
        // only if the row matches the condition, then you can clone it to keep it 
        if row_matches_condition(dataset, row, filter) {
            result.add_row(row.clone());
        }
    }
    //return the result 
    result
}



fn row_matches_condition(dataset: &Dataset, row: &Row, condition: &Condition) -> bool {
   //function will return if matches conditions, passes only by ref because we just need to check, we don't need a copy
    match condition {
        Condition::Equal(column_name, target_value) => {
            // Find which index this column is in.
            let col_index = dataset.column_index(column_name);

            // Get the row's value at that column.
            let row_value = row.get_value(col_index);

            // Compare it to the target value.
            row_value == target_value
        }

        Condition::Not(inner_condition) => {
            !row_matches_condition(dataset, row, inner_condition)
        }

        Condition::And(left, right) => {
            row_matches_condition(dataset, row, left) //recursively check
                && row_matches_condition(dataset, row, right)
        }

        Condition::Or(left, right) => {
            row_matches_condition(dataset, row, left)
                || row_matches_condition(dataset, row, right)
        }
    }
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
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