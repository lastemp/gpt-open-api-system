# gpt-open-api-system

This RESTful Actix Web API examples illustrates how to intergrate with OpenAI’s chat API and exchange messages.
[OpenAI’ chat API](https://platform.openai.com/docs/guides/gpt/chat-completions-api)

Currently this RESTful API supports: 
- Chat Completions

The RESTful Actix Web API has below listed dependencies:
- [Actix Web](https://github.com/actix/actix-web) web framework for Rust
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [Reqwest](https://github.com/seanmonstar/reqwest) Rust HTTP Client
- [MySQL](https://github.com/mysql/mysql-server) MySQL database server
- [mysql](https://github.com/blackbeam/rust-mysql-simple) MySql database driver

## Instructions

### NOTE:

You may need to ensure that you are running the commands with the correct MySQL user/password.

1. Create `.env` file:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   ```

5. Run the server:

   ```shell
   cargo run
   ```

6. Using a different terminal send an HTTP POST requests to the running server:

   Directory "gpt-open-api-system\apis" contains below listed api file:
   - processmessage.txt

   Copy the curl request on the "processmessage.txt" and execute it on a terminal. the "processmessage.txt" contains curl request and expected json reponse data.