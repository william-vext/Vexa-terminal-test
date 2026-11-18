# Vexa-CLI-test

# What's Vexa?
Vexa is a terminal test, performs manipulations with files, The terminal was designed for simplicity and security

# How many extensions does Vexa supports?
The terminal test only supports 6 extensions (Rust,C++,C,JavaScript,Python,Lua) and all C++ extensions (.cpp,.cxx,.c++,.cc,.cp) 

# How many code editors does vexa support?
The terminal test only supports 3 editors (Nano,Vim,Neovim)

# Explaining Components
Rename -> change the name of a file or folder, if the renamed file is C++ or Rust or C the executable is renamed too.

Create -> create files and folders

Move -> can move files and folders to another folder

Remove -> remove a file and remove the folder and all your content, this action is permanent 

Checker -> the component returns a true or false to a existence of a file or folder, the component can check if programs supported is installed or not

configurations -> Change the default code editor by modifying the code in the vexa.lock file

InvalidCharacter -> detect if the argument has invalid characters

Edit -> can change the file code

Compile -> responsible for compiling/executing files or programs

Error -> Tell's what happend and why the operation can't be executed 

# Basic Commands:

[WARNING]: The letters in the commands must be lowercase, because the input function aren't lowercase, this command don't work: "Create RS file", but this work: "create rs file"


Create file: create file, create rs file, create c++ file, create c file, create js file, create py file, create lua file

Rename file: rnm file, rnm rs file, rnm c++ file, rnm c file, rnm js file, rnm py file, rnm lua file

Move file: mv file, mv rs file, mv c++ file, mv c file, mv js file, mv py file, mv lua file

Remove: rm file, rm rs file, rm c++ file, rm c file, rm js file, rm py file, rn lua file

Create dir: mkdir, create dir, create folder

Rename dir: rnm dir, rnm folder

Move dir: mv dir, mv folder

Remove dir: rm -f dir 

Check files and folders: ls

Check files: checker rs file,checker c file, checker c++ file, checker c file, checker js file, checker py file, checker lua file

Check dir: checker folder

Check compiler/interpreter/program supported: checker compiler


To compile the file you just need to enter the interpreter/compiler name:

rustc

clang++

clang

nodejs

python

lua


after this the terminal will ask the file name 



