# Persistent Tree
<img src="image.png" width="350">  

This is an interactive Rust program demonstrating the functionality of a persistent binary search tree. The persistent tree allows creating immutable versions of a tree with each operation, enabling users to track history and revert to previous versions.


## Features
- **Insert values**: Add values to the persistent tree, creating new versions.
- **Rollback to previous versions**: Navigate back in time to earlier tree states.
- **Display current tree**: View the inorder traversal of the current tree version.
  
  
## How It Works 📢
The program maintains a history of tree versions in a vector. Each operation (such as inserting a new value) creates a new version of the tree while preserving the previous versions. Users can interactively add values to the tree or revert to earlier states.

## Installation ⚙️

To run this program on your machine, follow these steps:

1. **Clone the repository**:  
   ```git clone https://github.com/mzums/persistent_tree```
2. **Navigate to the project directory**:  
    ```cd persistent_tree```
3. **Build and run the program**:  
    ```cargo run```
4. **Interactive Menu Options**:
    - Option 1: **Insert** a new value into the tree. This creates a new version.
    - Option 2: **Roll back** by a user-specified number of versions.
    - Option 3: **Exit** the program.

## Contributing 🖋️
Contributing is always welcome!  
Steps to contribute:
1. Fork the repository.
2. Create a new branch  
    ```git checkout -b feature/your-feature```
3. Make your changes and commit them.  
    ```git commit -am 'Add new feature'```
4. Push to your branch.  
    ```git push origin feature/your-feature```
5. Create a new Pull Request.