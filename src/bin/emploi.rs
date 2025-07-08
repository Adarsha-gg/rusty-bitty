// Using a hash map and vectors, 
// create a text interface to allow a user to add employee names to a department in a company; for example,
//  “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve 
// a list of all people in a department or all people in the company by department, sorted alphabetically.


// # this is my solution to all of this shit

// use std::{io};
// use std::{collections::HashMap};


// enum Department{
//     Engineering,
//     Sales
// }
// struct  Engineering {
//     name: String
// }

// struct  Sales{
//     name: String
// }
// fn main(){
//     let vector = vec!("Engineering","Sales");
//     let mut department_Collecter:HashMap<&String,&String> = HashMap::new();

//     println!("Enter name");
//     let mut name = String::new();
//     io::stdin().read_line(&mut name).expect("Put string brother");
//     println!("Enter department");
//     let mut depart = String::new();
//     io::stdin().read_line(&mut depart).expect("Put string brother");
//     println!("{depart:?}");
    
//     for item in vector{
//         println!("{item:?}");
//         if &depart.strip_suffix("\r\n").expect("wtf is this msg") == &item{
//             department_Collecter.insert(&depart.strip_suffix("\r\n").expect("wtf is this msg")    , &name);
//         }
//     }
//     println!("{department_Collecter:?}",);

    

// }


// This is gpt man wtf 
use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    
    loop {
        println!("\nOptions:");
        println!("1. Add employee to department");
        println!("2. List all employees in a department");
        println!("3. List all employees by department");
        println!("4. Exit");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => add_employee(&mut departments),
            "2" => list_department(&departments),
            "3" => list_all_departments(&departments),
            "4" => break,
            _ => println!("Invalid option, please try again"),
        }
    }
}

fn add_employee(departments: &mut HashMap<String, Vec<String>>) {
    println!("Enter employee name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim().to_string(); // Clean and convert to owned String
    
    println!("Enter department:");
    let mut dept = String::new();
    io::stdin().read_line(&mut dept).expect("Failed to read input");
    let dept = dept.trim().to_string(); // Clean and convert to owned String
    
    // Add employee to department (creates the department if it doesn't exist)
    departments.entry(dept.clone()).or_insert_with(Vec::new).push(name.clone());
    
    println!("Added {} to {}", name, dept);
}

fn list_department(departments: &HashMap<String, Vec<String>>) {
    println!("Enter department name:");
    let mut dept = String::new();
    io::stdin().read_line(&mut dept).expect("Failed to read input");
    let dept = dept.trim();
    
    if let Some(employees) = departments.get(dept) {
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();
        println!("Employees in {}:", dept);
        for employee in sorted_employees {
            println!("  - {}", employee);
        }
    } else {
        println!("Department '{}' not found", dept);
    }
}

fn list_all_departments(departments: &HashMap<String, Vec<String>>) {
    if departments.is_empty() {
        println!("No departments found");
        return;
    }
    
    let mut sorted_departments: Vec<_> = departments.iter().collect();
    sorted_departments.sort_by_key(|(dept, _)| dept.as_str());
    
    println!("All employees by department:");
    for (dept, employees) in sorted_departments {
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();
        println!("{}:", dept);
        for employee in sorted_employees {
            println!("  - {}", employee);
        }
    }
}