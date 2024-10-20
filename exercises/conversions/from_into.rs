#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // Step 1: Check if the string is empty
        if s.is_empty() {
            return Person::default();
        }

        // Step 2: Split the string by commas
        let parts: Vec<&str> = s.split(',').collect();
        
        // Step 3: Extract the name
        let name = parts.get(0).unwrap_or(&"").trim();
        
        // Step 4: Check if the name is empty
        if name.is_empty() {
            return Person::default();
        }
        
        // Step 5: Extract and parse the age
        let age = parts.get(1)
            .and_then(|age_str| age_str.trim().parse::<usize>().ok())
            .unwrap_or(0);
        
        // Step 6: Check if age is valid and there are no extra parts
        if age == 0 || parts.len() > 2 {
            return Person::default();
        }
        
        // Return a new Person instance
        Person { name: name.to_string(), age }
    }
}

// The rest of the code (main function and tests) remains unchanged