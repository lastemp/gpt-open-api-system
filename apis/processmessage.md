# ProcessMessage API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Initiate ProcessMessage

```sh
curl -d '{"user_message": ["hello"]}' -H 'Content-Type: application/json' http://localhost:8080/processmessage

http POST :8080/processmessage user_message=["hello"]
```

The response should be a 200 OK with the following JSON body:

```json
{
    "user_response": "Hello! How may I assist you today?"
}
```

### Format of the request message

a. first message

{"user_message": ["hello"]}

b. second message

{"user_message": ["hello","what is stock exchange?"]}

c. third message

{"user_message": ["hello","what is stock exchange?","Is it best to invest for the shortterm?"]}

#### NB

For a new conversation, json request message should only contain the new message eg {"user_message": ["hello"]}
For a continuation of the conversation, json request message should also contain the previous messages sent
eg {"user_message": ["hello","what is stock exchange?","Is it best to invest for the shortterm?"]}
