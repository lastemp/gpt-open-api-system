# gpt-open-api-system

This RESTful Actix Web API examples illustrates how to intergrate with [OpenAI’s chat API](https://platform.openai.com/docs/guides/gpt/chat-completions-api) and exchange messages.

Currently this RESTful API supports: 
- Chat Completions

The RESTful Actix Web API has below listed dependencies:
- [Actix Web](https://github.com/actix/actix-web) web framework for Rust
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [Reqwest](https://github.com/seanmonstar/reqwest) Rust HTTP Client

## Usage

All the following commands assume that your current working directory is _this_ directory. I.e.:

```console
$ pwd
.../gpt-open-api-system
```

1. Create a `.env` file in this this directory:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   ```

1. Run the server:

   ```sh
   cargo run
   ```

1. Using a different terminal send requests to the running server. For example, using [HTTPie]:

   ```sh
   http POST :8080/processmessage user_message=["hello"]
   ```

   See [the API documentation pages](./apis/) for more info.

[HTTPie]: https://httpie.io/cli