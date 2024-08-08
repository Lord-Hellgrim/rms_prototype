use mysql::*;
use mysql::prelude::*;

use crate::rms_error::RmsError;


pub fn initialize_database(connection: &mut mysql::PooledConn) -> Result<(), RmsError> {

    
    connection.query_drop("DROP TABLE IF EXISTS transfer_items")?;
    connection.query_drop("DROP TABLE IF EXISTS transfers")?;
    connection.query_drop("DROP TABLE IF EXISTS transaction_items")?;
    connection.query_drop("DROP TABLE IF EXISTS transactions")?;
    connection.query_drop("DROP TABLE IF EXISTS pictures")?;
    connection.query_drop("DROP TABLE IF EXISTS warehouses")?;
    connection.query_drop("DROP TABLE IF EXISTS products")?;
    
    connection.query_drop("CREATE TABLE products (id INT PRIMARY KEY, price FLOAT, name VARCHAR(64), description VARCHAR(255), picture VARCHAR(255))")?;
    println!("8");
    connection.query_drop("CREATE TABLE warehouses (id VARCHAR(255) PRIMARY KEY)")?;
    println!("9");
    connection.query_drop("CREATE TABLE pictures (id VARCHAR(255) PRIMARY KEY, data LONGBLOB)")?;
    println!("10");
    connection.query_drop("CREATE TABLE transactions (id INT PRIMARY KEY, date DATETIME, salesman VARCHAR(255), client VARCHAR(255), warehouse VARCHAR(255), amount FLOAT, FOREIGN KEY (warehouse) REFERENCES warehouses (id))")?;
    println!("11");
    connection.query_drop("CREATE TABLE transaction_items (line_id INT PRIMARY KEY AUTO_INCREMENT, transaction_id INT, product_id INT, amount FLOAT, FOREIGN KEY (product_id) REFERENCES products(id))")?;
    println!("12");
    connection.query_drop("CREATE TABLE transfers (id INT PRIMARY KEY, date DATETIME, employee VARCHAR(255), transfer_from VARCHAR(255), transfer_to VARCHAR(255), FOREIGN KEY (transfer_from) REFERENCES warehouses (id), FOREIGN KEY (transfer_to) REFERENCES warehouses (id))")?;
    println!("13");
    connection.query_drop("CREATE TABLE transfer_items (line_id INT PRIMARY KEY AUTO_INCREMENT, transaction_id INT, product_id INT, amount FLOAT, FOREIGN KEY (product_id) REFERENCES products(id))")?;
println!("14");

    Ok(())
}


#[cfg(test)]
mod tests {
    
    use crate::utilities::csv_to_insert;
    
    use super::*;

    #[test]
    fn test_mysql() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let url = "mysql://eztester:test@localhost:3306/ezdbtest";
        let pool = Pool::new(url)?;

        let mut conn = pool.get_conn()?;

        let csv = csv_to_insert("id, price, name, description, picture\n18575517, 2000, 'Fúgusement' ,'Schomburg cristallfuge 6kg poki', 'cristallfuge.jpg'\n18572013, 4000, 'Flotsteypa' ,'Schomburg Soloplan 25kg poki', 'soloplan.jpg'").unwrap();

        match conn.query_drop(format!("INSERT INTO products {}", csv)) {
            Ok(_) => (),
            Err(e) => println!("Could not perform query because:\n#################################\n{}\n#################################", e)
        };

        let selected_payments = conn
            .query_map(
                "SELECT * from products",
                |(id, price, name, description, picture)| {
                    Product { id, price, name, description, picture }
                },
            )?;

        let select_statement = conn.prep("SELECT * FROM products WHERE id = :param").unwrap();

        let result = conn.exec_map(select_statement, params!{ "param" => 18575517 }, 
            |(id, price, name, description, picture)| {
                Product { id, price, name, description, picture }
            },
        ).unwrap();

        println!("{:?}", selected_payments);
        println!();
        println!("{:?}", result);

        Ok(())
    }
}