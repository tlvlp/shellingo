# Shellingo
[![Built With Ratatui](https://img.shields.io/badge/Built_With_Ratatui-000?logo=ratatui&logoColor=fff)](https://ratatui.rs/)

A simple command line tool for custom vocabulary practice.
(Without the constant harassment of the Duolingo owl)

> This is the Rust version of the [original java-based project](https://github.com/tlvlp/shellingo-java), because why not.
 
> Note: Currently it's a learning project, and not yet a functional tool.

## How to use 

1. [Download the latest release for your OS](https://github.com/tlvlp/shellingo/releases)
2. Create a subfolder named **questions** next to the executable. This is the configurable default path to store your questions in.
3. Add questions (see the 'How to' section below)
4. Run the tool with:

```shell
./shellingo
```

## Input paths

By default, the app reads files in and under its parent directory, 
but it also takes an unrestricted number of arguments with paths to override the default path.
- These arguments can contain either a parent folder to be traversed or an exact file path.
- Folders will be traversed without a depth limit, but will only pick up shellingo files. 

 ```shell
./shellingo mypath/my_parent_folder
./shellingo mypath/my_parent_folder/selected_file.sll
```

## How to add vocabularies/questions

> Shellingo will read all files under the [Input paths](#input-paths). 
> Make sure not to mix languages, or if you do indicate the language in each file name or question :)
> All the questions will be presented in a random order. Each word will be repeated until a correct solution is given.

- Create a text file and add one word/question per row and provide the expected answer,
separated with a pipe **|** character.
- You can also add comments with the hash **#** character. These lines will be ignored during the practice.
- Both questions and answers will be formatted to remove leading, trailing and duplicate white spaces and punctuation
- Letter casing will be ignored during the practice

```text
# These are the number 0-5 from the first Polish lesson
1|jeden
2|dwa
3|trzy
4|cztery
5|pięć
```

```text
# More complex sentences with exotic white space use that will be corrected

the elephant   likes  milk |  słoń  lubi  mleko

# will be presented as:
# the elephant likes milk: słoń lubi mleko
```A command line vocabulary practice tool that
