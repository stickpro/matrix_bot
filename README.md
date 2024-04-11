## Matrix bot

Run code 

```bash
cargo run --bin app
```

Example to use 

```
curl --request POST \
  --url http://localhost:8081/api/v1/matrix/send_message \
  --header 'Content-Type: application/json' \
  --data '{
	"room_id": "!oeryIrjgrtgbDEgSIh:example.com",
	"message": "hello world"
}'
```