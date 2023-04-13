# Welcome to pg_base32 👋
![Version](https://img.shields.io/badge/version-0.0.1-blue.svg?cacheSeconds=2592000)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](#)

> A base32 postgres extension. Supports RFC4648 and Crockford.

## Install

First, install [pgx](https://github.com/tcdi/pgx).

```sh
cargo pgx install pg14 --release
```

Add the extension to your database.

```sql
CREATE EXTENSION pg_base32;
```

## Usage

Schema name: `base32`

```sql
$ SELECT base32.encode('hello');
  encode  
----------
 NBSWY3DP
(1 row)
```

```sql
$ SELECT base32.decode('NBSWY3DP');
    decode    
--------------
 \x68656c6c6f
(1 row)
```

```sql
$ SELECT convert_from(base32.decode('NBSWY3DP'), 'UTF8');
 convert_from 
--------------
 hello
(1 row)
```

### base32.encode

`encode(data: text, variant: 'rfc4648' | 'crockford' = 'rfc4648', padding: bool = false) -> text`

- `data: text`: Input data
- `variant: 'rfc4648' | 'crockford'`: The variant to use
- `padding: bool`: If padding should be used (only for `rfc4648`)

### base32.decode

`decode(data: text, variant: 'rfc4648' | 'crockford' = 'rfc4648', padding: bool = false) -> bytea`

- `data: text`: Encoded data
- `variant: 'rfc4648' | 'crockford'`: The variant to use
- `padding: bool`: If padding should be used (only for `rfc4648`)

## Author

👤 **vabatta**

* Github: [@vabatta](https://github.com/vabatta)

## Show your support

Give a ⭐️ if this project helped you!



***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_