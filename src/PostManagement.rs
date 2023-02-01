use icp_contracts::balance;
use icp_contracts::balance::{TransferError};

// Define struct for post data
#[derive(Debug, ToBytes, FromBytes)]
struct Post {
    category: Category,
    heading: String,
    content: String,
    image: H256,
    user_id: account::Id,
    likes: u64,
    dislikes: u64,
    time: u64,
}

// Define struct for contract state
#[derive(Debug, ToBytes, FromBytes)]
struct State {
    posts: HashMap<H256, Post>,
    nfts: HashMap<H256, H256>,
}

// Define function for creating a new post
fn create_post(category: Category, heading: String, content: String, image: H256) {
    let state = env::state();
    let user_id = env::predecessor_account_id();
    // Check if the user has enough $TIME tokens to create a post
    let time_cost = 5;
    if let Err(TransferError::InsufficientBalance) = balance::transfer(user_id, env::predecessor_contract_id(), time_cost) {
        panic!("Insufficient balance")
    }
    // Create new post
    let new_post = Post {
        category: category,
        heading: heading,
        content: content,
        image: image,
        user_id: user_id,
        likes: 0,
        dislikes: 0,
        time: 5,
    };
    // Generate a unique id for the post
    let post_id = H256::random();
    // Add new post to state
    state.posts.insert(post_id, new_post);
    // Create an NFT for the image
    let nft_id = H256::random();
    state.nfts.insert(image, nft_id);
    env::commit_state(state);
    // Return the post id
    env::ret(post_id.to_bytes().unwrap());
}

// Define function for liking a post
fn like_post(post_id: H256) {
    let state = env::state();
    // Check if post exists in state
   
    if !state.posts.contains_key(&post_id) {
        panic!("Post not found");
    }
    // Check if the user has enough $TIME tokens to like a post
    let time_cost = 1;
    if let Err(TransferError::InsufficientBalance) = balance::transfer(env::predecessor_account_id(), env::predecessor_contract_id(), time_cost) {
        panic!("Insufficient balance")
    }
    // Get post from state
    let mut post = state.posts.get_mut(&post_id).unwrap();
    // Update post data
    post.likes += 1;
    post.time += 1;
    // Commit state
    env::commit_state(state);
}

// Define function for disliking a post
fn dislike_post(post_id: H256) {
    let state = env::state();
    // Check if post exists in state
    if !state.posts.contains_key(&post_id) {
        panic!("Post not found");
    }
    // Check if the user has enough $TIME tokens to dislike a post
    let time_cost = 1;
    if let Err(TransferError::InsufficientBalance) = balance::transfer(env::predecessor_account_id(), env::predecessor_contract_id(), time_cost) {
        panic!("Insufficient balance")
    }
    // Get post from state
    let mut post = state.posts.get_mut(&post_id).unwrap();
    // Update post data
    post.dislikes += 1;
    post.time -= 1;
        // Check if post time is less than or equal to 0
        if post.time <= 0 {
            state.posts.remove(&post_id);
        }
        // Commit state
        env::commit_state(state);
    }

// Define function for getting a post
fn get_post(post_id: H256) {
    let state = env::state();
    // Check if post exists in state
    if !state.posts.contains_key(&post_id) {
        panic!("Post not found");
    }
    // Get post from state
    let post = state.posts.get(&post_id).unwrap();
    // Return post data
    env::ret(post.to_bytes().unwrap());
}

// Define function for getting all posts
fn get_all_posts() {
    let state = env::state();
    // Return all posts
    env::ret(state.posts.to_bytes().unwrap());
}

// Define function for getting all posts in a category
fn get_posts_by_category(category: Category) {
    let state = env::state();
    // Create a new hashmap for posts in the category
    let mut posts_in_category = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is in the category
        if post.category == category {
            // Add post to hashmap
            posts_in_category.insert(*post_id, *post);
        }
    }
    // Return posts in category
    env::ret(posts_in_category.to_bytes().unwrap());
}

// Define function for getting all posts by a user
fn get_posts_by_user(user_id: account::Id) {
    let state = env::state();
    // Create a new hashmap for posts by the user
    let mut posts_by_user = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is by the user
        if post.user_id == user_id {
            // Add post to hashmap
            posts_by_user.insert(*post_id, *post);
        }
    }
    // Return posts by user
    env::ret(posts_by_user.to_bytes().unwrap());
}


// Define function for getting all posts by a user in a category
fn get_posts_by_user_and_category(user_id: account::Id, category: Category) {
    let state = env::state();
    // Create a new hashmap for posts by the user in the category
    let mut posts_by_user_and_category = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is by the user and in the category
        if post.user_id == user_id && post.category == category {
            // Add post to hashmap
            posts_by_user_and_category.insert(*post_id, *post);
        }
    }
    // Return posts by user in category
    env::ret(posts_by_user_and_category.to_bytes().unwrap());
}

//  Define function for getting highest post time of an user
fn get_highest_post_time(user_id: account::Id) {
    let state = env::state();
    // Create a new hashmap for posts by the user
    let mut posts_by_user = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is by the user
        if post.user_id == user_id {
            // Add post to hashmap
            posts_by_user.insert(*post_id, *post);
        }
    }
    // Create a new vector for post times
    let mut post_times = Vec::new();
    // Iterate through all posts in the hashmap
    for post in posts_by_user.values() {
        // Add post time to vector
        post_times.push(post.time);
    }
    // Sort vector in descending order
    post_times.sort_by(|a, b| b.cmp(a));
    // Return highest post time
    env::ret(post_times[0].to_bytes().unwrap());
}

// Define function for getting highest post time of an user in a category
fn get_highest_post_time_by_category(user_id: account::Id, category: Category) {
    let state = env::state();
    // Create a new hashmap for posts by the user in the category
    let mut posts_by_user_and_category = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is by the user and in the category
        if post.user_id == user_id && post.category == category {
            // Add post to hashmap
            posts_by_user_and_category.insert(*post_id, *post);
        }
    }
    // Create a new vector for post times
    let mut post_times = Vec::new();
    // Iterate through all posts in the hashmap
    for post in posts_by_user_and_category.values() {
        // Add post time to vector
        post_times.push(post.time);
    }
    // Sort vector in descending order
    post_times.sort_by(|a, b| b.cmp(a));
    // Return highest post time
    env::ret(post_times[0].to_bytes().unwrap());
}

// Define a function for getting the number of posts by a user
fn get_number_of_posts_by_user(user_id: account::Id) {
    let state = env::state();
    // Create a new hashmap for posts by the user
    let mut posts_by_user = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is by the user
        if post.user_id == user_id {
            // Add post to hashmap
            posts_by_user.insert(*post_id, *post);
        }
    }
    // Return number of posts by user
    env::ret(posts_by_user.len().to_bytes().unwrap());
}

// Define a function for getting the number of posts by a user in a category
fn get_number_of_posts_by_user_and_category(user_id: account::Id, category: Category) {
    let state = env::state();
    // Create a new hashmap for posts by the user in the category
    let mut posts_by_user_and_category = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is by the user and in the category
        if post.user_id == user_id && post.category == category {
            // Add post to hashmap
            posts_by_user_and_category.insert(*post_id, *post);
        }
    }
    // Return number of posts by user in category
    env::ret(posts_by_user_and_category.len().to_bytes().unwrap());
}

// Define a function for getting total likes for all the post for a user
fn get_total_likes_by_user(user_id: account::Id) {
    let state = env::state();
    // Create a new hashmap for posts by the user
    let mut posts_by_user = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is by the user
        if post.user_id == user_id {
            // Add post to hashmap
            posts_by_user.insert(*post_id, *post);
        }
    }
    // Create a new vector for post likes
    let mut post_likes = Vec::new();
    // Iterate through all posts in the hashmap
    for post in posts_by_user.values() {
        // Add post likes to vector
        post_likes.push(post.likes);
    }
    // Return total likes for all posts by user
    env::ret(post_likes.iter().sum::<u64>().to_bytes().unwrap());
}

// Define a function for getting total dislikes for all the post for a user 
fn get_total_dislikes_by_user(user_id: account::Id) {
    let state = env::state();
    // Create a new hashmap for posts by the user
    let mut posts_by_user = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is by the user
        if post.user_id == user_id {
            // Add post to hashmap
            posts_by_user.insert(*post_id, *post);
        }
    }
    // Create a new vector for post dislikes
    let mut post_dislikes = Vec::new();
    // Iterate through all posts in the hashmap
    for post in posts_by_user.values() {
        // Add post dislikes to vector
        post_dislikes.push(post.dislikes);
    }
    // Return total dislikes for all posts by user
    env::ret(post_dislikes.iter().sum::<u64>().to_bytes().unwrap());
}

// Define a function for getting total time for all the post for a user
fn get_total_time_by_user(user_id: account::Id) {
    let state = env::state();
    // Create a new hashmap for posts by the user
    let mut posts_by_user = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is by the user
        if post.user_id == user_id {
            // Add post to hashmap
            posts_by_user.insert(*post_id, *post);
        }
    }
    // Create a new vector for post time
    let mut post_time = Vec::new();
    // Iterate through all posts in the hashmap
    for post in posts_by_user.values() {
        // Add post time to vector
        post_time.push(post.time);
    }
    // Return total time for all posts by user
    env::ret(post_time.iter().sum::<u64>().to_bytes().unwrap());
}

// Define a function for getting total likes for all the post for a user in a category
fn get_total_likes_by_user_and_category(user_id: account::Id, category: String) {
    let state = env::state();
    // Create a new hashmap for posts by the user in the category
    let mut posts_by_user_and_category = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is by the user and in the category
        if post.user_id == user_id && post.category == category {
            // Add post to hashmap
            posts_by_user_and_category.insert(*post_id, *post);
        }
    }
    // Create a new vector for post likes
    let mut post_likes = Vec::new();
    // Iterate through all posts in the hashmap
    for post in posts_by_user_and_category.values() {
        // Add post likes to vector
        post_likes.push(post.likes);
    }
    // Return total likes for all posts by user in category
    env::ret(post_likes.iter().sum::<u64>().to_bytes().unwrap());
}

// Define a function for getting total dislikes for all the post for a user in a category
fn get_total_dislikes_by_user_and_category(user_id: account::Id, category: String) {
    let state = env::state();
    // Create a new hashmap for posts by the user in the category
    let mut posts_by_user_and_category = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is by the user and in the category
        if post.user_id == user_id && post.category == category {
            // Add post to hashmap
            posts_by_user_and_category.insert(*post_id, *post);
        }
    }
    // Create a new vector for post dislikes
    let mut post_dislikes = Vec::new();
    // Iterate through all posts in the hashmap
    for post in posts_by_user_and_category.values() {
        // Add post dislikes to vector
        post_dislikes.push(post.dislikes);
    }
    // Return total dislikes for all posts by user in category
    env::ret(post_dislikes.iter().sum::<u64>().to_bytes().unwrap());
}

// Define a function for getting total time for all the post for a user in a category
fn get_total_time_by_user_and_category(user_id: account::Id, category: String) {
    let state = env::state();
    // Create a new hashmap for posts by the user in the category
    let mut posts_by_user_and_category = HashMap::new();
    // Iterate through all posts in state
    for (post_id, post) in state.posts.iter() {
        // Check if post is by the user and in the category
        if post.user_id == user_id && post.category == category {
            // Add post to hashmap
            posts_by_user_and_category.insert(*post_id, *post);
        }
    }
    // Create a new vector for post time
    let mut post_time = Vec::new();
    // Iterate through all posts in the hashmap
    for post in posts_by_user_and_category.values() {
        // Add post time to vector
        post_time.push(post.time);
    }
    // Return total time for all posts by user in category
    env::ret(post_time.iter().sum::<u64>().to_bytes().unwrap());
}

// Define a function for getting total likes for a specific post by user
fn get_total_likes_by_user_and_post(user_id: account::Id, post_id: H256) {
    let state = env::state();
    // Check if post exists in state
    if !state.posts.contains_key(&post_id) {
        panic!("Post not found");
    }
    // Check if post is by the user
    let post = state.posts.get(&post_id).unwrap();
    if post.user_id != user_id {
        panic!("Post not by user");
    }
    // Return total likes for post by user
    env::ret(post.likes.to_bytes().unwrap());
}

// Define a function for getting total dislikes for a specific post by user
fn get_total_dislikes_by_user_and_post(user_id: account::Id, post_id: H256) {
    let state = env::state();
    // Check if post exists in state
    if !state.posts.contains_key(&post_id) {
        panic!("Post not found");
    }
    // Check if post is by the user
    let post = state.posts.get(&post_id).unwrap();
    if post.user_id != user_id {
        panic!("Post not by user");
    }
    // Return total dislikes for post by user
    env::ret(post.dislikes.to_bytes().unwrap());
}


// Define a function for getting total time for a specific post by user
fn get_total_time_by_user_and_post(user_id: account::Id, post_id: H256) {
    let state = env::state();
    // Check if post exists in state
    if !state.posts.contains_key(&post_id) {
        panic!("Post not found");
    }
    // Check if post is by the user
    let post = state.posts.get(&post_id).unwrap();
    if post.user_id != user_id {
        panic!("Post not by user");
    }
    // Return total time for post by user
    env::ret(post.time.to_bytes().unwrap());
}

// Define a function for getting total time and being able to withdraw it for a specific post by a user which will decrease post time depending on how much time is withdrawn
fn withdraw_time_by_user_and_post(user_id: account::Id, post_id: H256) {
    let state = env::state();
    // Check if post exists in state
    if !state.posts.contains_key(&post_id) {
        panic!("Post not found");
    }
    // Check if post is by the user
    let post = state.posts.get(&post_id).unwrap();
    if post.user_id != user_id {
        panic!("Post not by user");
    }
    // Check if the user has enough $TIME tokens to withdraw
    let time_cost = 1;
    if let Err(TransferError::InsufficientBalance) = balance::transfer(user_id, env::predecessor_contract_id(), time_cost) {
        panic!("Insufficient balance");
    }
    // Update post data
    let mut post = state.posts.get_mut(&post_id).unwrap();
    post.time -= 1;
    // Commit state
    env::commit_state(state);
}



/* This will be reviewed in future when we implement free likes/dislikes for posts and free post 

Add a new field in the Post struct for tracking the number of free likes/dislikes used 
by each user for a particular post.

#[derive(Debug, ToBytes, FromBytes)]
struct Post {
    // existing fields
    free_likes_dislikes: HashMap<account::Id, u64>,
    // existing fields
}

Add a new field in the State struct for 
tracking the number of free posts used by each user.

#[derive(Debug, ToBytes, FromBytes)]
struct State {
    // existing fields
    free_posts: HashMap<account::Id, u64>,
    // existing fields
}

Modify the like_post() and dislike_post() functions to check if the user has used up their 
free likes/dislikes for the post before proceeding with the like/dislike action.

fn like_post(post_id: H256) {
    let state = env::state();
    // Check if post exists in state
    if !state.posts.contains_key(&post_id) {
        panic!("Post not found");
    }
    let user_id = env::predecessor_account_id();
    // Check if the user has used up their free likes for the post
    let post = state.posts.get_mut(&post_id).unwrap();
    let free_likes_dislikes = post.free_likes_dislikes.entry(user_id).or_insert(0);
    if *free_likes_dislikes >= 1 {
        *free_likes_dislikes -= 1;
    } else {
                // Check if the user has enough $TIME tokens to like a post
        let time_cost = 1;
        if let Err(TransferError::InsufficientBalance) = balance::transfer(user_id, env::predecessor_contract_id(), time_cost) {
            panic!("Insufficient balance");
        }
    }
    // Update post data
    post.likes += 1;
    post.time += 1;
    // Commit state
    env::commit_state(state);
}

fn dislike_post(post_id: H256) {
    let state = env::state();
    // Check if post exists in state
    if !state.posts.contains_key(&post_id) {
        panic!("Post not found");
    }
    let user_id = env::predecessor_account_id();
    // Check if the user has used up their free dislikes for the post
    let post = state.posts.get_mut(&post_id).unwrap();
    let free_likes_dislikes = post.free_likes_dislikes.entry(user_id).or_insert(0);
    if *free_likes_dislikes >= 1 {
        *free_likes_dislikes -= 1;
    } else {
        // Check if the user has enough $TIME tokens to dislike a post
        let time_cost = 1;
        if let Err(TransferError::InsufficientBalance) = balance::transfer(user_id, env::predecessor_contract_id(), time_cost) {
            panic!("Insufficient balance");
        }
    }
    // Update post data
    post.dislikes += 1;
    post.time -= 1;
    // Check if post time is less than or equal to 0
    if post.time <= 0 {
        state.posts.remove(&post_id);
    }
    // Commit state
    env::commit_state(state);
}

This code checks whether the user has used up their free likes/dislikes before
proceeding with the like/dislike action. If they have used up their free likes/dislikes then they 
have to spend $TIME tokens to like/dislike otherwise they can use the free likes/dislikes.




// Create Free Post 
fn create_free_post(category: Category, heading: String, content: String, image: String) {
    let state = env::state();
    let user_id = env::predecessor_account_id();
    // Create new post
    let new_post = Post {
        category: category,
        heading: heading,
        content: content,
        image: image,
        user_id: user_id,
        likes: 0,
        dislikes: 0,
        time: 0,
    };
    // Generate a unique id for the post
    let post_id = H256::random();
    // Add new post to state
    state.posts.insert(post_id, new_post);
    // Create an NFT for the image
    let nft_id = H256::random();
    state.nfts.insert(image, nft_id);
    env::commit_state(state);
    // Return the post id
    env::ret(post_id.to_bytes().unwrap());
}

fn create_free_post(category: Category, heading: String, content: String, image: H256) {
    let state = env::state();
    let user_id = env::predecessor_account_id();
    // Check if the user has used up their free post
    let free_posts = state.free_posts.entry(user_id).or_insert(0);
    if *free_posts >= 1 {
        *free_posts -= 1;
    } else {
        // Check if the user has enough $TIME tokens to create a post
        let time_cost = 5;
        if let Err(TransferError::InsufficientBalance) = balance::transfer(user_id, env::predecessor_contract_id(), time_cost) {
            panic!("Insufficient balance");
        }
    }
    let post_time = 5;
    // Create post
    let post_id = create_post(category, heading, content, post_time, image);
    // Commit state
    env::commit_state(state);
}

This code checks whether the user has used up their free post before proceeding with creating 
a post. If they have used up their free post then they have to spend $TIME tokens to
 create post otherwise they can use the free post.
*/


