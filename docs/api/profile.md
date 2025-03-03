## profile api

profile

```bash
curl http://127.0.0.1:8080/v1/profile -H "Authorization: Bearer $TOKEN"
```

update profile

```bash
curl -X PUT -H "Content-Type: application/json" -H "Authorization: Bearer $TOKEN" http://127.0.0.1:8080/v1/profile -d '
{
    "nickname": "nickname",
    "avatar": "avatar",
    "current_status": "current_status",
    "bio": "bio"
}
'
```

change password

```bash
curl -X PUT -H "Content-Type: application/json" -H "Authorization: Bearer $TOKEN" http://127.0.0.1:8080/v1/profile/password -d '
{
    "current_password": "current_password",
    "new_password": "new_password",
    "new_password_confirmation": "new_password_confirmation"
}
'
```