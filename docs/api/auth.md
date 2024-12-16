## auth api

register

```bash
curl -X POST -H "Content-Type: application/json" http://127.0.0.1:8080/v1/auth/register -d '
{
    "username": "test",
    "email": "test@test.com",
    "password": "123456",
    "password_confirmation": "123456"
}
'
```

login

```bash
curl -X POST -H "Content-Type: application/json" http://127.0.0.1:8080/v1/auth/login -d '
{
    "username": "test",
    "password": "123456"
}
'
```
