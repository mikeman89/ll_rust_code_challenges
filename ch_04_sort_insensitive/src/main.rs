fn sort_usernames<T: AsRef<str> + Ord>(users: &mut Vec<T>) {
    users.sort_by_cached_key(|x| x.as_ref().to_lowercase());
}

fn main() {
    let mut users = vec!["Todd".to_owned(), "amy".to_owned(), "Xander".to_owned()];
    sort_usernames(&mut users);
    println!("{:?}", users);
}

#[test]
fn sorted_users_lower() {
    let mut input = vec!["amy".to_owned(), "todd".to_owned(), "xander".to_owned()];
    let expected = input.clone();
    sort_usernames(&mut input);
    assert_eq!(input, expected)
}
#[test]
fn sorted_users_mixed() {
    let mut input = vec!["amy".to_owned(), "Todd".to_owned(), "Xander".to_owned()];
    let expected = input.clone();
    sort_usernames(&mut input);
    assert_eq!(input, expected)
}

#[test]
fn unsorted_users_lower() {
    let mut input = vec!["todd".to_owned(), "amy".to_owned(), "xander".to_owned()];
    let expected = vec!["amy".to_owned(), "todd".to_owned(), "xander".to_owned()];
    sort_usernames(&mut input);
    assert_eq!(input, expected)
}

#[test]
fn unsorted_users_mixed() {
    let mut input = vec!["Todd".to_owned(), "amy".to_owned(), "Xander".to_owned()];
    let expected = vec!["amy".to_owned(), "Todd".to_owned(), "Xander".to_owned()];
    sort_usernames(&mut input);
    assert_eq!(input, expected)
}
