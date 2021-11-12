# TOTP Command Line Tool

Don't like Gogol authenticator, or other vault apps to store your one-time password? Build this tool and you can access your TOTP via cli. 


## TL;DR

```
$ > git checkout git@github.com/felsweg:totp-cli.gi
$ > cargo build --release
$ > target/release/totp-cli <YOUR BASE32 ENCODED SECRET>

```

## Integrations

Remembering the base32 encoded secrets might be a skill only a few people possess, here are some possible integrations that might make your life easier and possibly more secure. Please use **AT YOUR OWN RISK**. All recommendations given here are done to best knowledge and intent. **No guarantee about the actual safetey of employing those methods can be given at all.** 


### Secret Encrypted by Random Key Protected by GPG Identity

Assuming you have gpg identity called "user@identity.io", encryption of the base32 secret could be done as follows

```
$ > printf <INSERT YOUR BASE32 SECRET HERE> > totp-context1.totp 
$ > dd if=/dev/random bs=1 count=512 status=none > key.entropy
$ > openssl enc -chacha20 -pbkdf -iter 30 -in totp-context1.totp -out secret.openssl -kfile key.entropy
$ > gpg --encrypt --output key.gpg --recipient user@identity.io key.gpg
$ > rm totp-context1.totp key.entropy
```

This will create two files:
- `key.gpg` the encrypted key, that encryted the base32 secret value
- `secret.openssl` the base32 secret value


We create a small bash script, that will allow use to access multiple TOTPS 

```
#! /usr/bin/env bash
set -euo pipefail

SECRETS_PATH=/home/$USER/.secrets

decrypt() {
    openssl enc -d -chacha20 -kfile <(gpg --decrypt $SECRETS_PATH/$1/key.gpg) -pbkdf -iter 30 -in $SECRETS_PATH/$1/secret.openssl 
}

case $1:
    "context1") /usr/bin/totp-cli $(decrypt "context1")
esac

```

# License

MIT