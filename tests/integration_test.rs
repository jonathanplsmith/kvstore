use kvstore::store::{KVStore, Reponse};

#[test]
fn get_set_get() {
    let mut store = KVStore::new();
    let get = "GET$3$foo".as_bytes();
    let set = "SET$3$foo$3$bar".as_bytes();

    assert_eq!(Some(Reponse::Err), store.exec_command(get));
    assert_eq!(Some(Reponse::Ok), store.exec_command(set));

    let value = store.exec_command(get).unwrap().unwrap();
    eprintln!("FOOOOFOOOO {:?}", value.as_ref());
    assert_eq!("bar".as_bytes(), value.as_ref());
}

#[test]
fn barely_invalid_1() {
    let mut store = KVStore::new();
    let get = "GET$4$foo".as_bytes();

    assert_eq!(None, store.exec_command(get));
}

#[test]
fn barely_invalid_2() {
    let mut store = KVStore::new();
    let get = "BET$3$foo".as_bytes();

    assert_eq!(None, store.exec_command(get));
}

#[test]
fn barely_invalid_3() {
    let mut store = KVStore::new();
    let get = "GET$$foo".as_bytes();

    assert_eq!(None, store.exec_command(get));
}
