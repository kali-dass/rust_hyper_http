# http rust server

- http endpoint
- response in json format as {"test":"van"}
- default support both http and http2
- allow fallback option to go back to http

## call

```
# force h2c
curl -v --http2-prior-knowledge http://localhost:3000/echo

# fallback to http
curl -v --http2 http://localhost:3000/echo
```

## updates
17-july-2026 Kalidass Mookkaiah updated to respond as json, earlier was plain text