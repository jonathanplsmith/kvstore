use kvstore::parse::Command;
use kvstore::store::{KVStore, Reponse};

#[test]
fn store_get_set_get() {
    let mut store = KVStore::new();
    let get = Command::Get("foo".as_bytes().to_vec());
    let set = Command::Set("foo".as_bytes().to_vec(), "bar".as_bytes().to_vec());

    assert_eq!(Reponse::Err, store.exec_command(get.clone()));
    assert_eq!(Reponse::Ok, store.exec_command(set));

    let value = store.exec_command(get).unwrap();
    assert_eq!("bar".as_bytes(), value.as_ref());
}

#[test]
fn store_set_get_clr_get() {
    let mut store = KVStore::new();
    let get = Command::Get("foo".as_bytes().to_vec());
    let set = Command::Set("foo".as_bytes().to_vec(), "bar".as_bytes().to_vec());
    let clr = Command::Clear;

    assert_eq!(Reponse::Ok, store.exec_command(set));
    assert_eq!(
        "bar".as_bytes(),
        store.exec_command(get.clone()).unwrap().as_ref()
    );
    assert_eq!(Reponse::Ok, store.exec_command(clr));

    assert_eq!(Reponse::Err, store.exec_command(get));
}

#[test]
fn store_set_set_get_get_del_get_get() {
    let mut store = KVStore::new();
    let get1 = Command::Get("foo".as_bytes().to_vec());
    let set1 = Command::Set("foo".as_bytes().to_vec(), "bar".as_bytes().to_vec());
    let get2 = Command::Get("a".as_bytes().to_vec());
    let del1 = Command::Delete("foo".as_bytes().to_vec());
    let set2 = Command::Set("a".as_bytes().to_vec(), "b".as_bytes().to_vec());

    assert_eq!(Reponse::Ok, store.exec_command(set1));
    assert_eq!(Reponse::Ok, store.exec_command(set2));
    assert_eq!(
        "bar".as_bytes(),
        store.exec_command(get1.clone()).unwrap().as_ref()
    );
    assert_eq!(
        "b".as_bytes(),
        store.exec_command(get2.clone()).unwrap().as_ref()
    );

    assert_eq!(Reponse::Ok, store.exec_command(del1));
    assert_eq!(Reponse::Err, store.exec_command(get1));
    assert_eq!("b".as_bytes(), store.exec_command(get2).unwrap().as_ref());
}

#[test]
fn store_set_get_set_get() {
    let mut store = KVStore::new();
    let get = Command::Get("foo".as_bytes().to_vec());
    let set1 = Command::Set("foo".as_bytes().to_vec(), "bar".as_bytes().to_vec());
    let set2 = Command::Set("foo".as_bytes().to_vec(), "baz".as_bytes().to_vec());

    assert_eq!(Reponse::Ok, store.exec_command(set1));
    assert_eq!(
        "bar".as_bytes(),
        store.exec_command(get.clone()).unwrap().as_ref()
    );
    assert_eq!(Reponse::Ok, store.exec_command(set2));

    assert_eq!("baz".as_bytes(), store.exec_command(get).unwrap().as_ref());
}
