use poll_promise::Promise;

use crate::app;


pub fn lines_to_csv(lines: &[app::Product], skiplist: &[u8]) -> String {
    
    let mut printer = String::new();
    
    if skiplist[0] != 0 {
        printer.push_str("id,i");
        printer.push(';');
    }
    if skiplist[1] != 0 {
        printer.push_str("name,t");
        printer.push(';');
    }
    if skiplist[2] != 0 {
        printer.push_str("description,t");
        printer.push(';');
    }
    if skiplist[3] != 0 {
        printer.push_str("price,f");
        printer.push(';');
    }
    if skiplist[4] != 0 {
        printer.push_str("location,t");
        printer.push(';');
    }
    printer.pop();
    printer.push('\n');


    for item in lines {
        if skiplist[0] != 0 {
            printer.push_str(&item.id);
            printer.push(';');
        }
        if skiplist[1] != 0 {
            printer.push_str(&item.name);
            printer.push(';');
        }
        if skiplist[2] != 0 {
            printer.push_str(&item.description);
            printer.push(';');
        }
        if skiplist[3] != 0 {
            printer.push_str(&item.price);
            printer.push(';');
        }
        if skiplist[4] != 0 {
            printer.push_str(&item.location);
            printer.push(';');
        }
        printer.pop();
        printer.push('\n');
    }

    printer.pop();

    printer
}


pub fn lines_to_ezcsv(lines: &Vec<Vec<String>>) -> String {

    let mut ezcsv = String::new();

    for line in lines {
        for cell in line {
            ezcsv.push_str(cell);
            ezcsv.push_str(";");
        }
        ezcsv.pop();
        ezcsv.push('\n');
    }
    ezcsv.pop();

    ezcsv

}