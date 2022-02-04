use std::collections::HashMap;
use std::io;

fn main() {
    println!("\nEMPLOYEE DATABASE\n-----------------");

    let mut departments = vec![];

    let mut employees = vec![];

    loop {
        println!("\nPlease select an option by typing the number.");

        println!("1. Add a new employee");
        println!("2. Retrieve all employees in a department");
        println!("3. Retrieve all company employees sorted by department");
        println!("4. Quit");

        //user chooses option
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failure to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut employees_and_dept = HashMap::new();

        match input {
            1 => {
                //new_employee
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

                departments.push(department);

                employees.push(name);

                println!("{:?}:{:?}", departments, employees);
            }
            2 => {
                //employee_in_dept
                println!("Please type the department name.");

                let mut dept_name = String::new();

                io::stdin()
                    .read_line(&mut dept_name)
                    .expect("Failure to read line");

                let dept_name = dept_name.trim().to_string();

                let mut employee_list = employees.clone();

                let mut depts_list = departments.clone();

                println!("Tester {:?}:{:?}", depts_list, employee_list);

                employees_and_dept = depts_list
                    .into_iter()
                    .zip(employee_list.into_iter())
                    .collect();

                //employees_and_dept = add_to_hashmap(departments, employees);

                println!("{:#?}", employees_and_dept);
            }
            //3 => //all_employees_sorted_by_dept(),
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

fn add_to_hashmap(departments: Vec<String>, employees: Vec<String>) -> HashMap<String, String> {
    let employee_and_dept: HashMap<String, String> =
        departments.into_iter().zip(employees.into_iter()).collect();

    employee_and_dept
}
