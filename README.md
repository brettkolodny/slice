# Slice
A hybrid functional and imperative scripting language

## Data types
Slice has the following data types:
* character
* string
* int
* bool
* Option

## Variables
Variables are declared using the `let` keyword. Variables must be declared with a value.
```
let name = "Brett"
let favorite_number = 6
let slice_is_great = True
```
Variables can also be declared with a set type. If a variable is declared in such a way and is then reassigned to a different type an error is thrown.
```
let name = "Brett"
let last_name: string = "Kolodny"
name = "John" #Ok
last_name = True #Bad
```

## Lists
Lists are declared with brackets.
```
let my_list = [1, 2, 3]
```

## Arrays
Arrays are declared with brackets followed by a forward slash.
```
let my_array = [/1, 2, 3/]
```

## Dictionaries
Dictionaries are key value stores that are declared with braces and colons seperating the keys and values.
```
let my_dictionary = {language: "Slice", awesome: True}
```

## Functions
Functions are declared with the `fn` keyword followed by the function name and arguments and closed with the `end` keyword.
```
fn greeting(name):
  puts ("Hello " + name)
end
```

Just like variable declaration, function arguments can be declared with types.
```
fn greet(name: string):
  puts ("Hello " + name)
end
```

If a function returns a value, that is designated with `->` followed by its type.
```
fn add_one(num: int) -> int:
  return num + 1
end
```

Functions returning boolean values should be named in the format of `function_name?`.
```
fn language_is_awesome?(language: string) -> bool:
  if language == "slice":
    return true
  end
  
  false
end
```

If the value you're returning is the last expression in a function, return can be omitted.
```
fn add_one(num: int) -> int:
  num + 1
end
```

Functions can be declared multiple times with different arities or types. Matches will be attempted in the order that they appear in the file.
When an argument is not going to be used, it can be replaced with `_`.
```
fn greet(name: string, greeting: string):
  puts (greeting + " " + name)
end

fn greet(name: string):
  puts ("Hello " + name)
end

fn greet(_):
  puts "Huh, that's a strange name."
end
```

### Calling a function
When a function only has one argument, parenthesis can be omitted.
```
fn add_one(num: int) -> int:
  num + 1
end

let four = add_one 3
```

### Function piping
Function calls can be piped with the pipe operator `->` for better redability.
```
let favorite_number: int = "6" -> int_of_string #With pipe
let favorite_number: int = int_of_string("6") #Without pipe
```

## Pattern Matching
Pattern matching can done on variable declaration as well function declaration.
```
fn favorite_bread("white"):
  puts "A bland basic choice"
end

fn favorite_bread("rye"):
  puts "A good bread for a tunna melt"
end

fn favorite_bread(bread: string):
  puts "Really? That's your favorite?"
end
```
