# rust-daily-cli
A daily habit-tracker CLI app.

# Why?
On my journey of teaching myself Rust through The Rust Programming Language, Rust By Example, and supplemental YouTube videos, I wanted to create a CLI app that uses the fundamentals of any programming language; that is, control flow, file I/O, data structures, etc. </br>
CLI apps are a common type of lab I've done while in my university's earlier courses, so I thought creating one in a language I was unfamiliar with would be good practice to create something I would actually use (as opposed to making 3210948321 temporary `new.txt`s in Notepad++), while simultaneously getting my hands dirty in the language and its features (ie. the borrow checker). 

# Reflection (ongoing)
What I thought would be a small one or two day project has taken me longer than I expected. I underestimated how strict Rust's compiler *actually* is, as well as how verbose the syntax is (although this is for good reason as laid out in The Book).
I actually redid the entirety of the project from scratch because of how convoluted my first implementation was. </br>
I will say though, developing this mini-project has made me think about a lot of things that I had never considered while programming in Python or C++ (such as design choices revolving around typing, scopes, etc.). My main takeaway so far is that borrowing and ownership are concepts that are *really* important in Rust. </br>
With that being said, all functio

# Features
- `Reading` and `Writing` using the `CSV crate`
- Creating new CSV files in terminal
- Editing files in terminal

# To-Do/Refactoring/Bugs
Check out [issues](https://github.com/loganjaymes/rust-daily-cli/issues).
