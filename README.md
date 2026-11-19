# Vexa-CLI-test

# What's Vexa?
Vexa is a terminal test, performs manipulations with files, The terminal was designed for simplicity and security

# How many extensions does Vexa supports?
The terminal test only supports 6 extensions (Rust,C++,C,JavaScript,Python,Lua) and all C++ extensions (.cpp,.cxx,.c++,.cc,.cp) 

# How many code editors does vexa support?
The terminal test only supports 3 editors (Nano,Vim,Neovim)

# Explaining Components
- Rename -> change the name of a file or folder, if the renamed file is C++ or Rust or C the executable is renamed too.

- Create -> create files and folders

- Move -> can move files and folders to another folder

- Remove -> remove a file and remove the folder and all your content, this action is permanent 

- Checker -> the component returns a true or false to a existence of a file or folder, the component can check if programs supported is installed or not

- configurations -> Change the default code editor by modifying the code in the vexa.lock file

- InvalidCharacter -> detect if the argument has invalid characters

- Edit -> can change the file code

- Compile -> responsible for compiling/executing files or programs

- Error -> Tell's what happend and why the operation can't be executed 

# Basic Commands:

## [WARNING]: 
The letters in the commands must be lowercase, because the input function aren't lowercase, this command don't work: "Create RS file", but this work: "create rs file"

## Commands: 
- ## Create file: 

```bash
create file 
```

```bash
create rs file
```

```bash
create c++ file
```

```bash
create c file
```

```bash
create js file
```

```bash
create py file
```

```bash
create lua file
```

## Rename file: 

```bash
rnm file
```

```bash
rnm rs file
```

```bash
rnm c++ file
```

```bash
rnm c file 
```

```bash
rnm js file
```

```bash
rnm py file
```

```bash
rnm lua file
```

## Move file: 
mv file, mv rs file, mv c++ file, mv c file, mv js file, mv py file, mv lua file

- Remove: rm file, rm rs file, rm c++ file, rm c file, rm js file, rm py file, rn lua file

- Create dir: mkdir, create dir, create folder

- Rename dir: rnm dir, rnm folder

- Move dir: mv dir, mv folder

- Remove dir: rm -f dir 

- Check files and folders: ls

- Check files: checker rs file,checker c file, checker c++ file, checker c file, checker js file, checker py file, checker lua file

- Check dir: checker folder

- Check compiler/interpreter/program supported: checker compiler


To compile the file you just need to enter the interpreter/compiler name:

rustc

clang++

clang

nodejs

python

lua


after this the terminal will ask the file name 


# Vexa problems
- 1° -> immobile cursor.

- 2° -> doesn't have cd, which can make things difficult in large projects. 

- 3° -> the commands used in code can work on some operating systems

- 4° -> doesn't have support to GitHub

- 5° -> cannot install more programs

- 6° -> few extensions, can make difficult who don't program on those 6 languages 

- 7° -> don't have support to extension .txt

- 8° -> if your file or folder have invalid characters any operation is impossible


# How to run Vexa:

## Requirements:

- Rust compiler version: 1.85+
- Cargo
- Git
- Clang
- Python 3
- Node.js
- Lua 5.4

Nano is optional and is only required if you want to edit files from the terminal.

## Termux:

Update packages:

```bash
pkg update && upgrade
```

Install the required tools:

```bash
pkg install git rust clang python nodejs Lua nano
```

Clone the project:

```bash
git clone https://github.com/william-vext/Vexa-terminal-test.git
```

## macOS

Install Homebrew if you don't have it.

Install the required tools:

```bash
brew install git rust llvm python node lua
```

Clone the project:

```bash
git clone https://github.com/william-vext/Vexa-terminal-test.git
```

## Linux (Ubuntu / Debian)

Update packages:

```bash
sudo apt update
```

Install the required tools:

```bash
sudo apt install git curl build-essential clang python3 nodejs lua5.4 nano
```

Install Rust using rustup:

https://rustup.rs/

Clone the project:

```bash
git clone https://github.com/william-vext/Vexa-terminal-test.git
```

## Windows 

Install Git:

```powershell
winget install -e --id Git.Git
```

Install Rust:

```powershell
winget install -e --id Rustlang.Rustup
```

Install Python:

```powershell
winget install -e --id Python.Python.3.12
```

Install Node.js:

```powershell
winget install -e --id OpenJS.NodeJS
```

Install LLVM/Clang:

```powershell
winget install -e --id LLVM.LLVM
```

Lua 5.4 must also be installed and added to PATH.

Clone the project:

```powershell
git clone https://github.com/william-vext/Vexa-terminal-test.git
```

## Running Vexa

Enter the project directory:

```bash
cd Vexa-terminal-test
```

Run Vexa:

```bash
cargo run
```



