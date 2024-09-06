use std::collections::HashMap;
use std::io::{self, Write};
use std::num::ParseIntError;

fn main() {
    let mut employee_db: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("OPTIONS\n-------");
        println!("[0] Exit");
        println!("[1] Add Department");
        println!("[2] Add Employee");
        println!("[3] Remove Department");
        println!("[4] Remove Employee");
        println!("[5] List Department");
        println!("[6] List All Departments");
        println!("[7] List Employee");

        if let Ok(option) = get_option() {
            match option {
                0 => break,
                1 => add_department(&mut employee_db),
                2 => add_employee(&mut employee_db),
                3 => remove_department(&mut employee_db),
                4 => remove_employee(&mut employee_db),
                5 => list_department(get_department().as_str(), &mut employee_db),
                6 => list_all_departments(&mut employee_db),
                7 => list_employee(&employee_db),
                _ => println!("Only provided options are supported."),
            }
        } else {
            println!("[PANIC] Provided option must be an integer.");
        }

        println!();

        // dbg!(&employee_db);
    }
}

fn handle_input(variable: &mut String) {
    io::stdin()
        .read_line(variable)
        .expect("Failed reading standard input");
}

fn handle_output(text: &str) {
    print!("{text}");
    io::stdout()
        .flush()
        .expect("Failed writing to standard output");
}

fn get_option() -> Result<u8, ParseIntError> {
    handle_output("> ");

    let mut choice = String::new();
    handle_input(&mut choice);

    choice.trim().parse()
}

fn get_department() -> String {
    handle_output("Department> ");

    let mut department = String::new();
    handle_input(&mut department);

    department.trim().to_lowercase()
}

fn get_employee() -> String {
    handle_output("Employee> ");

    let mut employee = String::new();
    handle_input(&mut employee);

    employee.trim().to_lowercase()
}

fn add_department(db: &mut HashMap<String, Vec<String>>) {
    let department = get_department();

    db.entry(department.clone()).or_default();
    println!("[SUCCESS] Department {department} has been created successfully.");
}

fn add_employee(db: &mut HashMap<String, Vec<String>>) {
    let employee = get_employee();
    let department = get_department();

    if let Some(employees) = db.get_mut(department.as_str()) {
        employees.push(employee.clone());
        println!(
            "[SUCCESS] Employee {employee} has been added to department {department} successfully."
        );
    } else {
        println!("[ERROR] Department {department} does not exist!");
    }
}

fn remove_department(db: &mut HashMap<String, Vec<String>>) {
    let department = get_department();

    db.remove(department.as_str());
    println!("[SUCCESS] Department {department} has been removed successfully.");
}

fn remove_employee(db: &mut HashMap<String, Vec<String>>) {
    let employee = get_employee();
    let department = get_department();

    if let Some(employees) = db.get_mut(department.as_str()) {
        if let Some(index) = employees.iter().position(|x| *x == employee) {
            employees.swap_remove(index);
            println!("[SUCCESS] Employee {employee} has been added to department {department}.");
        } else {
            println!("[ERROR] Employee {employee} does not work in department {department}!");
        }
    } else {
        println!("[ERROR] Department {department} does not exist!");
    }
}

fn list_department(department: &str, db: &mut HashMap<String, Vec<String>>) {
    if let Some(employees) = db.get_mut(department) {
        employees.sort();

        println!("{department}\n{}", "-".repeat(department.len()));
        for employee in employees {
            println!("{employee}");
        }
    } else {
        println!("[ERROR] Department {department} does not exist!");
    }
}

fn list_all_departments(db: &mut HashMap<String, Vec<String>>) {
    let mut departments: Vec<String> = db.keys().cloned().collect();
    departments.sort();

    for department in departments {
        list_department(&department, db);
        println!();
    }
}

fn list_employee(db: &HashMap<String, Vec<String>>) {
    let employee = get_employee();

    println!("{employee}\n{}", "-".repeat(employee.len()));
    for (department, employees) in db.iter() {
        if employees.contains(&employee) {
            println!("{department}");
        }
    }
}
