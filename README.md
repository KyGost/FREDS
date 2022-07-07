# FREDS
**F**lexible **R**eferencing **E**xtensible **D**ata **S**tore

## Two Kinds

### Sized
64b/8B
Inline for Unsized

#### Common
- u64
- i64
- f64


### Unsized

#### Common
- Map
- Array
- String

#### Made of
- (Type)
- Reference
- Size
- Data



## Allows
- Partial Read
- Partial Write
- Custom Types

## Format

### File
```
- TYPE:SIZE
  - SIZE
    - DATA (eg. TYPE:DATA)
```

### Rust
```
All: [Vec<Box<impl Unsized>>; 256]
```
