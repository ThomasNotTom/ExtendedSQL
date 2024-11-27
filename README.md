# ExtendedSQL

## What is it?

ExtendedSQL (ESQL) is a programming language designed to link simple backend requests directly to the database layer of the stack.

## How is it used?

(syntax subject to change)

```
CREATE SERVER server;
INSERT INTO SERVER server (
    6543, // The port number
)

CREATE TABLE users (
    user_id INT PRIMARY_KEY,
    first_name STRING,
    last_name STRING,
    age INT NOT_NULL,
);

Server ON GET TO "/get_user" WITH QUERY (
    user_id INT,
) RETURN (
    USER (SELECT * FROM users WHERE users.user_id=HEADER.user_id)
);
```

Given a user exists with `user_id` `1234` and a GET request to `/get_user?user_id=1234` on port `6543` would return

```JS
{
    user: {
        user_id: 1234,
        first_name: 'John',
        last_name: 'Doe',
        age: 40
    }
}
```

## Why is this needed?

ESQL is not designed to be a replacement to a backend or database, however is intended to be used as a temporary placeholder for both. This is particularly useful for programmers who want to quickly spin up a backend/database, and have it be configured quickly so other functionality (mainly frontend) can be worked on.
