// File: structs_enums.rs

pub mod structs {
    #[derive(Debug)]
    pub struct Employee {
        pub active: bool,
        pub name: String,
        pub department: String,
        pub position: String,
        pub experience: u32, // years of experience
        pub age: u32,        // age of the employee
        pub salary: u64,     // salary in a currency unit (e.g., cents)
    }

    impl Employee {
        // Constructor method to create a new Employee
        pub fn new(active: bool, name: &str, department: &str, position: &str, experience: u32, age: u32, salary: u64) -> Employee {
            Employee {
                active,
                name: name.to_string(),
                department: department.to_string(),
                position: position.to_string(),
                experience,
                age,
                salary,
            }
        }

        // Method to adjust salary based on experience
        pub fn adjust_salary(&mut self) {
            if self.experience > 5 {
                self.salary += 60_000; // Increase salary by $60,000
            }
        }
}
    }
    