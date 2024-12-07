trait Logger {
    fn module_name(&self) -> &str;

    fn log(&self, level: &str, message: &str) {
        println!("[{}] {}: {}", level, self.module_name(), message);
    }
}

struct DatabaseConnection {
    name: String,
}

struct FileLogger {
    file_name: String,
}

impl Logger for FileLogger {
    fn module_name(&self) -> &str {
        "FileLogger"
    }
}

impl DatabaseConnection {
    fn new(name: &str) -> DatabaseConnection {
        DatabaseConnection {
            name: name.to_string(),
        }
    }

    fn connect(&self) {
        println!("Connected to {}", self.name);
    }
}

impl Logger for DatabaseConnection {
    fn module_name(&self) -> &str {
        "DatabaseConnection"
    }
}

fn get_logger() -> impl Logger {
    DatabaseConnection {
        name: "users.db".to_string(),
    }
}

fn save_log<T: Logger>(logger: &T, level: &str, message: &str) {
    logger.log(level, message);
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("Dropping {}", self.name)
    }
}

pub fn main() {
    let db = DatabaseConnection::new("users.db");
    db.connect();
    db.log("INFO", "New database connection established");

    let logger = FileLogger {
        file_name: "app.log".to_string(),
    };

    save_log(&db, "INFO", "Database Saved");
    save_log(&logger, "INFO", "File Saved");
}
