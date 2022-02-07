use itertools::Itertools;
use std::collections::HashMap;
use std::io;

fn main() {
    println!("\nEMPLOYEE DATABASE\n-----------------");

    let mut employees_and_dept = HashMap::new();

    loop {
        println!("\nPlease select an option by typing the number.");

        println!("1. Add a new employee");
        println!("2. Retrieve all employees in a department");
        println!("3. Retrieve all company employees sorted by department");
        println!("4. Quit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failure to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => {
                println!("\nPlease input the Employee's full name.");

                let mut name = String::new();

                io::stdin()
                    .read_line(&mut name)
                    .expect("Failure to read line");

                let name = name.trim().to_string();

                println!("\n\nPlease input the employee's department.");

                let mut department = String::new();

                io::stdin()
                    .read_line(&mut department)
                    .expect("Failure to read line");

                let department = department.trim().to_string();

                println!(
                    "\n{} has been entered into the database as working in the {} department.",
                    name, department
                );

                employees_and_dept.insert(name, department);

                println!("FOLKS IN THE HASHMAP {:?}", employees_and_dept);
            }
            2 => {
                println!("\nPlease type the department name.");

                let mut dept_name = String::new();

                io::stdin()
                    .read_line(&mut dept_name)
                    .expect("Failure to read line");

                let dept_name = dept_name.trim().to_string();

                println!("");

                for (ab, bc) in &employees_and_dept {
                    if bc.eq(&dept_name) {
                        println!("{}", ab);
                    } else if !bc.is_empty() {
                        ();
                    };
                }
            }
            3 =>
            //all_employees_sorted_by_dept()
            {
                /* println!(
                    "SORTED BY DEPT IN THE HASHMAP {:?}",
                    &employees_and_dept
                        .iter()
                        .sorted_by(|b, a| Ord::cmp(&b.1, &a.1))
                        .map(|(person, _age)| person)
                ); */

                let sorted_list = &employees_and_dept
                    .iter()
                    .sorted_by(|b, a| Ord::cmp(&b.1, &a.1))
                    .map(|(person, _age)| person);

                println!("{:#?}", &sorted_list);
            }
            4 => {
                println!("You've exited the application.");
                std::process::exit(exitcode::OK);
            }
            _ => continue,
        };
    }
}

//fn new_employee(mut departments: Vec<String>, mut employees: Vec<String>)
//ADD A NEW EMPLOYEE TO THE DB
