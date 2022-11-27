use std::{io, collections::HashMap};
use itertools::Itertools;

pub fn run () {
    let mut workforce: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        menu();
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        match ans.trim() {
            "1"=>{
                let (employee, department) = add_employee();
                let dep_employees = workforce.entry(department).or_default();
                dep_employees.push(employee);
            },
            "2"=>{
                show_workforce(&workforce);
            },
            _ => {println!("That's all!"); break;},
        }

    }
}

fn show_workforce(workforce: &HashMap<String, Vec<String>>) {
    for (department, employees) in workforce {
        println!("{}:", department);
        for (i, employee) in employees.iter().sorted().enumerate() {
            println!("{}. {}", i, employee)
        }
    }
}

fn menu() {
    println!("Select operation:");
    println!("1. Add employee");
    println!("2. Show employees");
}

fn add_employee() -> (String, String) {
    let employee = get_answer("Enter employee name");
    let dpt = get_answer("Enter department");
    (employee, dpt)
}

fn get_answer(msg: &str) -> String {
    let mut ans = String::new();
    println!("{}",msg);
    io::stdin().read_line(&mut ans).unwrap();
    String::from(ans.trim())
}