# TimeFeed-Backend-v1.0.0

TimeFeed is a web-based application for managing and tracking multiple users and posts. It is built with Rust and uses a MongoDB database for data storage.

![timefeed-logo-background](https://user-images.githubusercontent.com/58028401/215619014-876965d0-b9d2-4a68-acd3-ed9d0244ad2f.png)


## Features

- Create, update and delete users
- Create, update and delete posts
- Manage user permissions
- Connect with other users
- Monitor user activity

## Installation

1. Clone the repository: `git clone https://github.com/shreeyesh/TimeFeed-Backend-v1.0.0.git`
2. Install the dependencies: `cargo install`
3. Run the application: `cargo run`

## Documentation

## PostManagement
PostManagement is a module of the TimeFeed backend application. It allows users to create, edit, and delete posts. It also includes functions for retrieving posts, as well as upvoting and downvoting.

### Functions
* `createPost()`: Creates a new post
* `editPost()`: Edits an existing post
* `deletePost()`: Deletes an existing post
* `getPost()`: Retrieves an existing post
* `upvotePost()`: Upvotes a post
* `downvotePost()`: Downvotes a post

## UserManagement
UserManagement is a module of the TimeFeed backend application. It allows users to register, log in, and log out. It also includes functions for retrieving user information and updating user profiles.

### Functions
* `registerUser()`: Registers a new user
* `loginUser()`: Logs in an existing user
* `logoutUser()`: Logs out an existing user
* `getUserInfo()`: Retrieves user information
* `updateUserProfile()`: Updates user profile information

## Contributing

We welcome contributions in any form. If you would like to contribute, please read our [contributing guidelines](https://github.com/shreeyesh/TimeFeed-Backend-v1.0.0/blob/master/CONTRIBUTING.md).

## License

TimeFeed Backend is released under the [MIT License](https://github.com/shreeyesh/TimeFeed-Backend-v1.0.0/blob/master/LICENSE).

## Bugs

If you discover any bugs, please [open an issue](https://github.com/shreeyesh/TimeFeed-Backend-v1.0.0/issues/new) on GitHub.
