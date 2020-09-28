# rust-csv-averager
I only ever used Rust for Project Euler problems, and I wanted to do something "useful" with it, so I wrote a small program to do somethning I normally use awk for.

This program takes a CSV file as an argument, then reads the file and averages the second values in lines where the first value matches. The output is CSV-formatted, de-duplicated by average.
