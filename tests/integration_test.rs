use kvstore::parse::Command;
use kvstore::store::{KVStore, Reponse};

#[test]
fn get_set_get() {
    let mut store = KVStore::new();
    let get = Command::Get("foo".as_bytes().to_vec());
    let set = Command::Set("foo".as_bytes().to_vec(), "bar".as_bytes().to_vec());

    assert_eq!(Reponse::Err, store.exec_command(get.clone()));
    assert_eq!(Reponse::Ok, store.exec_command(set));

    let value = store.exec_command(get).unwrap();
    assert_eq!("bar".as_bytes(), value.as_ref());
}

// #[test]
// fn barely_invalid_1() {
//     let mut store = KVStore::new();
//     let get = "GET$4$foo".as_bytes();

//     assert_eq!(None, store.exec_command(get));
// }

// #[test]
// fn barely_invalid_2() {
//     let mut store = KVStore::new();
//     let get = "BET$3$foo".as_bytes();

//     assert_eq!(None, store.exec_command(get));
// }

// #[test]
// fn barely_invalid_3() {
//     let mut store = KVStore::new();
//     let get = "GET$$foo".as_bytes();

//     assert_eq!(None, store.exec_command(get));
// }
