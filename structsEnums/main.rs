mod structs;  

fn main() {
    
    let mut employees = vec![

        structs::structs::Employee::new(true, "Kabir", "Cooking", "Head Chef", 1, 30, 50000),

        structs::structs::Employee::new(true, "Fehim Kabir", "Cooking", "Assistant Chef", 12, 29, 1000000),
    ];

   
    for employee in &mut employees {
        employee.adjust_salary(); 
    }

   
    for employee in &employees {
        println!("Employee active: {} name: {} department: {} position: {} experience: {} age: {} salary: {}", 
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
