use std::fmt;

// Define the Department enum
enum Department {
    Cooking,
    HR,
    Finance,
    IT,
}

// Implement Display for Department to print the enum values as strings
impl fmt::Display for Department {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let department_name = match self {
            Department::Cooking => "Cooking",
            Department::HR => "HR",
            Department::Finance => "Finance",
            Department::IT => "IT",
        };
        write!(f, "{}", department_name)
    }
}

// Define the Employee struct
struct Employee {
    active: bool,
    name: String,
    department: Department,
    position: String,
    experience: u32,
    age: u32,
    salary: u32,
}

// Implement methods for Employee
impl Employee {
    fn new(active: bool, name: &str, department: Department, position: &str, experience: u32, age: u32, salary: u32) -> Self {
        Self {
            active,
            name: name.to_string(),
            department,
            position: position.to_string(),
            experience,
            age,
            salary,
        }
    }

    fn adjust_salary(&mut self) {
        if self.experience > 10 {
            self.salary += 5000;
             // Example increment for experienced employees
        }
    }
}

fn main() {
    // Initialize a vector of employees
    let mut employees = vec![
        Employee::new(true, "Kabir", Department::Cooking, "Head Chef", 1, 30, 50000),
        Employee::new(true, "Fehim Kabir", Department::Cooking, "Assistant Chef", 12, 29, 1000000),
    ];

    // Adjust salaries based on experience
    for employee in &mut employees {
        employee.adjust_salary();
    }

    // Print details of each employee
    for employee in &employees {
        println!(
            "Employee active: {} name: {} department: {} position: {} experience: {} age: {} salary: {}", 
            employee.active, 
            employee.name, 
            employee.department, 
            employee.position, 
            employee.experience, 
            employee.age, 
            employee.salary
        );
    }
}
