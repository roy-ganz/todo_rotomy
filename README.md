# Todo (Rocket, Toql, MySQL)
This is a little REST server built with Rocket, Toql and MySQL. 

The server allows to call CRUD functions on a `Todo` item. 

It demonstrates how easy it is to write REST backends with the Rust programming language.


## Get it running

You need a running MySQL server to play around with it.

Then create the database `todo_rotomy` and insert the `Todo` table with
```sql
CREATE TABLE `Todo` (`id` int(11) NOT NULL AUTO_INCREMENT,`title` varchar(200) NOT NULL, `completed` tinyint(1) DEFAULT 0, PRIMARY KEY (`id`))` 
```

Now clone this repository 
```bash
git clone https://github.com/roy-ganz/todo_rotomy.git
```

Run the server with your database username and password
```bash
ROCKET_DATABASES='{todo_rotomy={url=mysql://USER:PASS@localhost:3306/todo_rotomy}}' cargo run
```

## Use the server

Open another terminal and use `curl`:

#### Insert a new item  
```bash
curl localhost:8000/todo -X POST -d '{\"title\":\"Water plants\"}'
```

#### Update an item  
```bash
curl localhost:8000/todo/ID -X PUT -d '{\"completed\":\"true\"}'
```

#### List all todo items
```bash
curl localhost:8000/todo
```
#### Get a single item
```bash
curl localhost:8000/todo/ID
```

#### Get only completed todos in descending order (example of Toql query)
```bash
curl localhost:8000/todo?query=-id,completed+eq+1
```

#### Delete an item
```bash
curl -X DELETE localhost:8000/todo/ID
```

## Make your own
This project may also serve as a starting point for your own REST server. 

However if you plan a big project, it's important to split up your Rust
project into separate crates to keep compilation time sane. 
The `#[derive(Toql)` adds _a lot_ of code to your structs.


## License
Todo-rotomy is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).
