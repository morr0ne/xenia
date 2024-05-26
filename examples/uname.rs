use xenia::uname;

fn main() {
    let uname = uname();

    println!("sysname: {}", uname.sysname().to_string_lossy());
    println!("nodename: {}", uname.nodename().to_string_lossy());
    println!("release: {}", uname.release().to_string_lossy());
    println!("version: {}", uname.version().to_string_lossy());
    println!("machine: {}", uname.machine().to_string_lossy());
    println!("domainname: {}", uname.domainname().to_string_lossy());
}
