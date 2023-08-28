# example-api-quick-hello
*How quick can we say hello?*

This repo has several examples of a dead-simple API implementation.

The reason for this is to demonstrate a simple concept in multiple languages, as well as to have a basis for understanding *client bottlenecks*.  By having a baseline of a server that *should* respond very quickly, we can reason over how to make it better (on both the client and server side).

And, this will probably be fun.

## API Specification

When a request is made to GET `/api/hello`, a HTTP 200 response is delivered with a JSON-parseable response containing the following:

```json
{
    "data":"world"
}

```


## Repo Tooling
Because multiple languages will be represented here, .gitignore will be in multiple folders.  In the short term, `npx gitignore <lang>` will be used to quickly create gitignores for each folder.

## Servers
Within the `servers/` directory, examples of a server which meets the API spec above will be given.  
A dockerfile will be maintained in each folder, so that the example can be built and launched quickly.
