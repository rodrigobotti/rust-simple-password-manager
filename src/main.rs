use simple_password_manager::PasswordManager;

pub fn main() {
    let manager = PasswordManager::new("super secret master password".to_owned());
    // let passwords = manager.list_passwords(); <- compilation error because the method does not exist on PasswordManager<Locked>
    // manager.add_password("user".to_owned(), "pass".to_owned()); <- compilation error because the method does not exist on PasswordManager<Locked>

    let mut manager = manager.unlock();
    // let manager = manager.unlock(); <- compilation error because the method does not exist on PasswordManager<Unlocked>

    println!(
        "unlocked manager: encryption = {}, versions = {}",
        manager.encryption(),
        manager.version()
    );
    manager
        .add_password("user".to_owned(), "pass".to_owned())
        .add_password("u1".to_owned(), "p1".to_owned())
        .add_password("u2".to_owned(), "p2".to_owned());

    let passwords = manager.list_passwords();
    println!("passwords are {passwords:?}");

    let manager = manager.lock();
    println!(
        "locked manager: encryption = {}, versions = {}",
        manager.encryption(),
        manager.version()
    );
}
