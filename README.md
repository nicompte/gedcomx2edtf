# gedcomx2edtf

GedcomxDate to edtf converter.

## setup

Install a rust compiler (with [rustup](https://rustup.rs)):
```bash
curl https://sh.rustup.rs -sSf | sh
```
And build
```bash
cargo build --release
```

## usage
```ruby
require './gedcomx2edtf'

puts Gedcomx2edtf.convert("+1988-03-29T03:19Z") # 1988-03-29T03:19-00:00
puts Gedcomx2edtf.convert("A+2003/+2003-05") # 2003~/2003-05~
puts Gedcomx2edtf.convert("/+1789") # [..,1789]
puts Gedcomx2edtf.convert("-0001") # -2
```
