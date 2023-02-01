// Import necessary libraries
use std::collections::HashMap;
use icp_contracts::{account, env, sdk};
use icp_contracts::hash::H256;
use icp_contracts::bytesrepr::{ToBytes, FromBytes};

// Define struct for user data
#[derive(Debug, ToBytes, FromBytes)]
struct User {
    name: String,
    email: String,
    password: H256,
    tokens: u64,
}

// Define struct for contract state
#[derive(Debug, ToBytes, FromBytes)]
struct State {
    users: HashMap<account::Id, User>,
}

// Define function for registering a new user
fn register_user(name: String, email: String, password: H256) {
    let state = env::state();
    // Check if email is already in use
    if state.users.contains_key(&env::predecessor_account_id()) {
        panic!("Email is already in use");
    }
    // Create new user
    let new_user = User {
        name: name,
        email: email,
        password: password,
        tokens: 0
    };
    // Add new user to state
    state.users.insert(env::predecessor_account_id(), new_user);
    env::commit_state(state);
}

// Define function for user login
fn login(email: String, password: H256) {
    let state = env::state();
    // Check if email exists in state
    if !state.users.contains_key(&env::predecessor_account_id()) {
        panic!("Invalid email or password");
    }
    // Get user from state
    let user = state.users.get(&env::predecessor_account_id()).unwrap();
    // Check if password matches
    if user.password != password {
        panic!("Invalid email or password");
    }
    // Return user data
    env::ret(user.to_bytes().unwrap());
}

// Define function for updating user data
fn update_user(name: String, email: String, password: H256) {
    let state = env::state();
    // Check if email exists in state
    if !state.users.contains_key(&env::predecessor_account_id()) {
        panic!("User not found");
    }
    // Get user from state
    let mut user = state.users.get_mut(&env::predecessor_account_id()).unwrap();
    // Update user data
    user.name = name;
    user.email = email;
    user.password = password;
    // Commit state
    env::commit_state(state);
}

// Define function for getting user data
fn get_user() {
    let state = env::state();
    // Check if email exists in state
    if !state.users.contains_key(&env::predecessor_account_id()) {
        panic!("User not found");
    }
    // Get user from state
    let user = state.users.get(&env::predecessor_account_id()).unwrap();
    // Return user data
    env::ret(user.to_bytes().unwrap());
}

// Define function for deleting user
fn delete_user() {
    let state = env::state();
    // Check if email exists in state
    if !state.users.contains_key(&env::predecessor_account_id()) {
        panic!("User not found");
    }
    // Remove user from state
    state.users.remove(&env::predecessor_account_id());
    // Commit state
    env::commit_state(state);
}

// Define function for user logout
fn logout() {
    let state = env::state();
    // Check if email exists in state
    if !state.users.contains_key(&env::predecessor_account_id()) {
        panic!("User not found");
    }
    // Get user from state
    let user = state.users.get(&env::predecessor_account_id()).unwrap();
    // Return user data
    env::ret(user.to_bytes().unwrap());
}

 

