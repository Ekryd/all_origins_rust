# The put certificate files here

> cert.pem

> privkey.pem

Self-signed certificates are probably ok, it all depends on your infrastructure setup.
My certificates are not part of this repo, and I generated them a long time ago.
I wish that I kept some more details, but my only personal note was "append chain to cert.pem" 🤦‍♂️ 

## Generate a self-signed ceritficate

```console
openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout privkey.pem -out cert.pem
```
