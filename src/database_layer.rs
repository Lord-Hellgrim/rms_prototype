use mysql::*;
use mysql::prelude::*;

use crate::rms_error::RmsError;


pub fn initialize_database(connection: &mut mysql::PooledConn) -> Result<(), RmsError> {

    let create_products_statement = connection.prep("CREATE TABLE products (id INT PRIMARY KEY, price FLOAT, name VARCHAR(64), description VARCHAR(255), picture VARCHAR(255)")?;
    let create_warehouses_statement = connection.prep("CREATE TABLE warehouses (id VARCHAR(255) PRIMARY KEY)")?;
    let create_pictures_statement = connection.prep("CREATE TABLE pictures (id VARCHAR(255), data LONGBLOB)")?;
    let create_transactions_statement = connection.prep("CREATE TABLE transactions (id INT, price FLOAT, name VARCHAR(64), description VARCHAR(255), picture VARCHAR(255)")?;


    Ok(())
}


#[cfg(test)]
mod tests {
    
    use crate::utilities::csv_to_insert;
    
    use super::*;
    
    
    #[derive(Debug, PartialEq)]
    struct Product {
        id: i32,
        price: f32,
        name: String,
        description: String,
        picture: String,
    }

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