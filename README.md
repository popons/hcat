### horizontal cat(concatenate) program

Reads multiple files and combines them horizontally for output.  
Typical use is to create a csv file.  


```usage
horizontal cat(concatenate) program

USAGE:
    hcat [OPTIONS] [FILES]...

ARGS:
    <FILES>...    Names of read files

OPTIONS:
    -h, --head               print header as file name
        --help               Print help information
    -o, --output <OUTPUT>    output file to write
    -p, --pretty             Align the output width
    -s, --sep <SEP>          Separator when print items [default: ,]
    -V, --version            Print version information
```
