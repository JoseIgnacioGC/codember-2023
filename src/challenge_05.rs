use regex::Regex;
use std::io::prelude::*;
use std::{fs::File, io::BufReader, path::Path};

#[derive(Debug, Clone)]
pub struct UserModel {
    pub id: String,
    pub username: String,
    pub email: String,
    pub age: Option<u16>,
    pub location: Option<String>,
}

fn get_user(column: &str) -> UserModel {
    let filds = column.split(',').collect::<Vec<&str>>();

    UserModel {
        id: filds.first().unwrap().to_string(),
        username: filds.get(1).unwrap().to_string(),
        email: filds.get(2).unwrap().to_string(),
        age: filds.get(3).unwrap().parse::<u16>().ok(),
        location: Some(filds.get(4).unwrap().to_string()).filter(|s| !s.is_empty()),
    }
}

fn is_the_user_valid(user: UserModel) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+\.[a-zA-Z0-9]+$").unwrap();
    let alphanumeric_regex = Regex::new(r"^[[:alnum:]]+$").unwrap();

    if user.id.is_empty() || !alphanumeric_regex.is_match(&user.id) {
        return false;
    }

    if user.username.is_empty() || !alphanumeric_regex.is_match(&user.id) {
        return false;
    }

    if !email_regex.is_match(&user.email) {
        return false;
    }

    true
}

#[derive(Debug, Default)]
pub struct Users {
    pub valid: Vec<UserModel>,
    pub invalid: Vec<UserModel>,
}

fn get_users(users_raw: Vec<UserModel>) -> Users {
    users_raw.iter().fold(Users::default(), |mut users, user| {
        if is_the_user_valid(user.clone()) {
            users.valid.push(user.clone());
        } else {
            users.invalid.push(user.clone());
        };
        users
    })
}

fn get_hidden_message(users: Users) -> String {
    users.invalid.iter().fold(String::new(), |message, user| {
        format!("{}{}", message, user.username.chars().next().unwrap())
    })
}

pub fn print_challenge_hidden_message() {
    let filepath = Path::new("data/database_attacked.txt");
    let file = File::open(filepath).unwrap();
    let database_attacked = BufReader::new(file);

    let users_raw = database_attacked
        .lines()
        .map(|line| get_user(&line.unwrap()))
        .collect::<Vec<UserModel>>();

    let users = get_users(users_raw);

    let hidden_message = get_hidden_message(users);

    assert_eq!("youh4v3beenpwnd", hidden_message);
    println!("challenge_05: {}", hidden_message);
}

#[test]
fn valid_with_optional() {
    const COLUMN: &str = "1a421fa,alex,alex9@gmail.com,18,Barcelona";
    let user = get_user(COLUMN);
    let is_valid = is_the_user_valid(user);
    assert!(is_valid);
}

#[test]
fn valid_without_optional() {
    const COLUMN: &str = "494ee0,madeval,mdv@twitch.tv,,";
    let user = get_user(COLUMN);
    let is_valid = is_the_user_valid(user);
    assert!(is_valid);
}

#[test]
fn invalid_id() {
    const COLUMN: &str = "9412p_m,maria,mb@hotmail.com,22,CDMX";
    let user = get_user(COLUMN);
    let is_not_valid = !is_the_user_valid(user);
    assert!(is_not_valid);
}

#[test]
fn invalid_mail() {
    const COLUMN: &str = "494ee0,madeval,twitch.tv,22,Montevideo";
    let user = get_user(COLUMN);
    let is_not_valid = !is_the_user_valid(user);
    assert!(is_not_valid);
}
